use std::{
    fmt,
    time::{Duration, SystemTime},
};

use reqwest::{Client, ClientBuilder, Method, RequestBuilder, Url, header::HeaderValue};
use serde::de::DeserializeOwned;

use crate::{ApiError, Error};

const BASE_URL: &str = "https://roxyapi.com/api/v2";

/// Optional retries for HTTP 429, 502, 503, and 504 responses.
#[derive(Clone, Copy, Debug)]
pub struct RetryPolicy {
    pub max_retries: u8,
    pub initial_delay: Duration,
    pub max_delay: Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 0,
            initial_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(30),
        }
    }
}

impl RetryPolicy {
    fn delay(&self, attempt: u8, retry_after: Option<Duration>) -> Option<Duration> {
        if attempt >= self.max_retries {
            return None;
        }
        let exponential = (0..attempt).fold(self.initial_delay, |delay, _| {
            delay.saturating_mul(2).min(self.max_delay)
        });
        let delay = exponential.max(retry_after.unwrap_or_default());
        (delay <= self.max_delay).then_some(delay)
    }
}

/// A reusable, cloneable asynchronous client. Clones share the connection pool.
#[derive(Clone, Debug)]
pub struct Roxy {
    pub(crate) core: Core,
}

impl Roxy {
    pub fn new(api_key: impl Into<String>) -> Result<Self, Error> {
        Self::builder(api_key).build()
    }

    pub fn builder(api_key: impl Into<String>) -> Builder {
        Builder {
            api_key: api_key.into(),
            base_url: BASE_URL.to_owned(),
            timeout: Duration::from_secs(30),
            retries: RetryPolicy::default(),
            http: Client::builder(),
        }
    }
}

/// Configure a client, including an overall deadline and optional retries.
pub struct Builder {
    api_key: String,
    base_url: String,
    timeout: Duration,
    retries: RetryPolicy,
    http: ClientBuilder,
}

impl fmt::Debug for Builder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Builder")
            .field("timeout", &self.timeout)
            .field("retries", &self.retries)
            .finish_non_exhaustive()
    }
}

impl Builder {
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Total time for one call, including retries and response body reading.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn retry_policy(mut self, policy: RetryPolicy) -> Self {
        self.retries = policy;
        self
    }

    /// Configure proxies, TLS, or connection pooling. Redirects and transport retries stay disabled.
    pub fn http_client_builder(mut self, builder: ClientBuilder) -> Self {
        self.http = builder;
        self
    }

    pub fn build(self) -> Result<Roxy, Error> {
        if self.api_key.trim().is_empty() {
            return Err(Error::Configuration("an API key is required"));
        }
        let mut api_key = HeaderValue::from_str(&self.api_key)
            .map_err(|_| Error::Configuration("invalid API key header"))?;
        api_key.set_sensitive(true);
        let base_url =
            Url::parse(&self.base_url).map_err(|_| Error::Configuration("invalid base URL"))?;
        if !matches!(base_url.scheme(), "http" | "https")
            || base_url.host_str().is_none()
            || !base_url.username().is_empty()
            || base_url.password().is_some()
            || base_url.query().is_some()
            || base_url.fragment().is_some()
        {
            return Err(Error::Configuration(
                "base URL must be an HTTP URL with no credentials, query, or fragment",
            ));
        }
        if self.timeout.is_zero()
            || std::time::Instant::now()
                .checked_add(self.timeout)
                .is_none()
            || self.retries.initial_delay.is_zero()
            || std::time::Instant::now()
                .checked_add(self.retries.max_delay)
                .is_none()
            || self.retries.max_delay < self.retries.initial_delay
        {
            return Err(Error::Configuration(
                "deadline and retry delays must be positive and representable, with max_delay at least initial_delay",
            ));
        }
        let http = self
            .http
            .redirect(reqwest::redirect::Policy::none())
            .retry(reqwest::retry::never())
            .build()?;
        Ok(Roxy {
            core: Core {
                http,
                api_key,
                base_url,
                timeout: self.timeout,
                retries: self.retries,
            },
        })
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Core {
    http: Client,
    api_key: HeaderValue,
    base_url: Url,
    timeout: Duration,
    retries: RetryPolicy,
}

impl Core {
    pub(crate) fn request(
        &self,
        method: Method,
        segments: &[&str],
    ) -> Result<RequestBuilder, Error> {
        if segments
            .iter()
            .any(|s| s.is_empty() || matches!(*s, "." | ".."))
        {
            return Err(Error::Configuration("invalid path parameter"));
        }
        let mut url = self.base_url.clone();
        url.path_segments_mut()
            .map_err(|_| Error::Configuration("invalid base URL"))?
            .pop_if_empty()
            .extend(segments);
        Ok(self
            .http
            .request(method, url)
            .header("X-API-Key", self.api_key.clone())
            .header(
                "X-SDK-Client",
                concat!("roxy-sdk-rust/", env!("CARGO_PKG_VERSION")),
            ))
    }

    pub(crate) async fn send<T: DeserializeOwned>(
        &self,
        request: RequestBuilder,
    ) -> Result<T, Error> {
        tokio::time::timeout(self.timeout, self.send_inner(request))
            .await
            .map_err(|_| Error::Timeout)?
    }

    async fn send_inner<T: DeserializeOwned>(&self, request: RequestBuilder) -> Result<T, Error> {
        let mut attempt = 0;
        loop {
            let response = request
                .try_clone()
                .ok_or(Error::Configuration("request body cannot be replayed"))?
                .send()
                .await?;
            let status = response.status();
            let retry_after = response
                .headers()
                .get(reqwest::header::RETRY_AFTER)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| retry_after_delay(v, SystemTime::now()));
            if matches!(status.as_u16(), 429 | 502 | 503 | 504)
                && let Some(delay) = self.retries.delay(attempt, retry_after)
            {
                drop(response);
                let deadline = tokio::time::Instant::now()
                    .checked_add(delay)
                    .ok_or(Error::Timeout)?;
                tokio::time::sleep_until(deadline).await;
                attempt += 1;
                continue;
            }
            let bytes = response.bytes().await?;
            if status.is_success() {
                return serde_json::from_slice(&bytes).map_err(Error::Decode);
            }
            let mut error: ApiError = serde_json::from_slice(&bytes).unwrap_or_else(|_| ApiError {
                status: 0,
                error: status.canonical_reason().unwrap_or("HTTP error").to_owned(),
                code: String::new(),
                doc_url: None,
                retry_after: None,
                details: serde_json::Map::new(),
            });
            error.status = status.as_u16();
            error.retry_after = retry_after;
            return Err(Error::Api(Box::new(error)));
        }
    }
}

fn retry_after_delay(value: &str, now: SystemTime) -> Option<Duration> {
    let value = value.trim();
    if !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Some(
            value
                .parse::<u64>()
                .map(Duration::from_secs)
                .unwrap_or(Duration::MAX),
        );
    }
    httpdate::parse_http_date(value)
        .ok()
        .map(|date| date.duration_since(now).unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retry_schedule_and_server_delay() {
        let policy = RetryPolicy {
            max_retries: 4,
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(3),
        };
        assert_eq!(policy.delay(0, None), Some(Duration::from_secs(1)));
        assert_eq!(policy.delay(1, None), Some(Duration::from_secs(2)));
        assert_eq!(policy.delay(2, None), Some(Duration::from_secs(3)));
        assert_eq!(policy.delay(4, None), None);
        let tiny = RetryPolicy {
            max_retries: 40,
            initial_delay: Duration::from_nanos(1),
            max_delay: Duration::from_secs(60),
        };
        assert_eq!(
            tiny.delay(33, None),
            Some(Duration::from_nanos(1_u64 << 33))
        );
        assert_eq!(
            policy.delay(0, Some(Duration::from_secs(2))),
            Some(Duration::from_secs(2))
        );
        assert_eq!(policy.delay(0, Some(Duration::from_secs(4))), None);
    }

    #[test]
    fn retry_after_seconds_and_http_dates() {
        let now = SystemTime::UNIX_EPOCH + Duration::from_secs(784111777);
        assert_eq!(
            retry_after_delay("120", now),
            Some(Duration::from_secs(120))
        );
        assert_eq!(
            retry_after_delay("Sun, 06 Nov 1994 08:49:37 GMT", now),
            Some(Duration::ZERO)
        );
        assert_eq!(
            retry_after_delay("Sun, 06 Nov 1994 08:49:38 GMT", now),
            Some(Duration::from_secs(1))
        );
        assert_eq!(retry_after_delay("bad", now), None);
        assert_eq!(retry_after_delay("-1", now), None);
        assert_eq!(
            retry_after_delay("99999999999999999999999999999", now),
            Some(Duration::MAX)
        );
        assert_eq!(
            retry_after_delay(" 120 ", now),
            Some(Duration::from_secs(120))
        );
    }
}
