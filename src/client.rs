use std::{
    collections::hash_map::RandomState,
    fmt,
    hash::{BuildHasher, Hasher},
    net::IpAddr,
    time::{Duration, SystemTime},
};

use reqwest::{Client, ClientBuilder, Method, RequestBuilder, Url, header::HeaderValue};
use serde::de::DeserializeOwned;
use tokio::time::Instant;

use crate::{ApiError, Error};

const BASE_URL: &str = "https://roxyapi.com/api/v2";

/// Retries for transient failures.
///
/// A call is retried only after HTTP 429, 500, 502, 503 or 504, or a
/// connection failure that happened before the request was sent. Other
/// statuses, read timeouts and decode errors are returned immediately. The
/// default policy makes up to two retries; [`RetryPolicy::new(0)`](RetryPolicy::new)
/// turns retries off.
///
/// When the response carries `Retry-After`, the next attempt waits that long
/// plus a random padding of up to `initial_delay`, capped at `max_delay`. A
/// `Retry-After` longer than `max_delay` is not retried: the error is returned
/// with [`ApiError::retry_after`] set. Without `Retry-After`, the wait is a
/// random duration up to `initial_delay * 2^attempt`, capped at `max_delay`.
/// No wait runs past the client deadline set with [`Builder::timeout`]; the
/// last error is returned instead.
///
/// ```
/// use std::time::Duration;
///
/// let policy = roxyapi::RetryPolicy::new(3)
///     .with_initial_delay(Duration::from_millis(250))
///     .with_max_delay(Duration::from_secs(10));
/// assert_eq!(policy.max_retries(), 3);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct RetryPolicy {
    max_retries: u8,
    initial_delay: Duration,
    max_delay: Duration,
}

impl Default for RetryPolicy {
    /// Up to two retries with the default delays.
    fn default() -> Self {
        Self::new(2)
    }
}

impl RetryPolicy {
    /// A policy allowing up to `max_retries` retries after the first attempt,
    /// with a 500 millisecond initial delay and a 30 second maximum delay.
    pub const fn new(max_retries: u8) -> Self {
        Self {
            max_retries,
            initial_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(30),
        }
    }

    /// Set the base backoff delay, which is also the upper bound of the
    /// random padding added to `Retry-After`.
    #[must_use]
    pub const fn with_initial_delay(mut self, delay: Duration) -> Self {
        self.initial_delay = delay;
        self
    }

    /// Set the longest single wait between attempts.
    #[must_use]
    pub const fn with_max_delay(mut self, delay: Duration) -> Self {
        self.max_delay = delay;
        self
    }

    /// The number of retries after the first attempt.
    pub const fn max_retries(&self) -> u8 {
        self.max_retries
    }

    /// The base backoff delay.
    pub const fn initial_delay(&self) -> Duration {
        self.initial_delay
    }

    /// The longest single wait between attempts.
    pub const fn max_delay(&self) -> Duration {
        self.max_delay
    }

    /// The wait before retry number `attempt + 1`, or `None` when no retry is
    /// allowed. `unit` is a uniform sample in `[0, 1)`.
    fn delay(&self, attempt: u8, retry_after: Option<Duration>, unit: f64) -> Option<Duration> {
        if attempt >= self.max_retries {
            return None;
        }
        match retry_after {
            Some(server) if server > self.max_delay => None,
            Some(server) => Some(
                server
                    .saturating_add(self.initial_delay.mul_f64(unit))
                    .min(self.max_delay),
            ),
            None => {
                let ceiling = (0..attempt).fold(self.initial_delay, |delay, _| {
                    delay.saturating_mul(2).min(self.max_delay)
                });
                Some(ceiling.min(self.max_delay).mul_f64(unit))
            }
        }
    }
}

/// A uniform sample in `[0, 1)`. The standard library hasher keys are seeded
/// randomly per process and advance per instance.
fn unit_sample() -> f64 {
    let mut hasher = RandomState::new().build_hasher();
    hasher.write_u128(
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos(),
    );
    (hasher.finish() >> 11) as f64 / (1_u64 << 53) as f64
}

/// A reusable, cloneable asynchronous client. Clones share the connection pool.
#[derive(Clone, Debug)]
pub struct Roxy {
    pub(crate) core: Core,
}

impl Roxy {
    /// A client with the default configuration: a 30 second deadline per call
    /// and up to two retries for transient failures.
    pub fn new(api_key: impl Into<String>) -> Result<Self, Error> {
        Self::builder(api_key).build()
    }

    /// Start configuring a client.
    pub fn builder(api_key: impl Into<String>) -> Builder {
        Builder {
            api_key: api_key.into(),
            base_url: BASE_URL.to_owned(),
            timeout: Duration::from_secs(30),
            retries: RetryPolicy::default(),
            http: None,
        }
    }
}

/// Configure a client, including an overall deadline and optional retries.
pub struct Builder {
    api_key: String,
    base_url: String,
    timeout: Duration,
    retries: RetryPolicy,
    http: Option<ClientBuilder>,
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
    /// Override the API base URL. It must use HTTPS, except for loopback hosts
    /// in local testing, and carry no credentials, query or fragment.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Total time for one call, including retries, waits and reading the
    /// response body.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Replace the default retry policy. See [`RetryPolicy`].
    pub fn retry_policy(mut self, policy: RetryPolicy) -> Self {
        self.retries = policy;
        self
    }

    /// Configure proxies, TLS or connection pooling. Redirects stay disabled,
    /// and any retry policy set on this builder is replaced with none, so only
    /// [`RetryPolicy`] decides when a request is sent again.
    pub fn http_client_builder(mut self, builder: ClientBuilder) -> Self {
        self.http = Some(builder);
        self
    }

    /// Validate the configuration and build the client.
    pub fn build(self) -> Result<Roxy, Error> {
        if self.api_key.trim().is_empty() {
            return Err(Error::Configuration("an API key is required"));
        }
        let mut api_key = HeaderValue::from_str(&self.api_key)
            .map_err(|_| Error::Configuration("invalid API key header"))?;
        api_key.set_sensitive(true);
        let base_url =
            Url::parse(&self.base_url).map_err(|_| Error::Configuration("invalid base URL"))?;
        let loopback = base_url.host_str().is_some_and(|host| {
            host.eq_ignore_ascii_case("localhost")
                || host
                    .trim_start_matches('[')
                    .trim_end_matches(']')
                    .parse::<IpAddr>()
                    .is_ok_and(|ip| ip.is_loopback())
        });
        if !(base_url.scheme() == "https" || (loopback && base_url.scheme() == "http"))
            || base_url.host_str().is_none()
            || !base_url.username().is_empty()
            || base_url.password().is_some()
            || base_url.query().is_some()
            || base_url.fragment().is_some()
        {
            return Err(Error::Configuration(
                "base URL must use HTTPS (HTTP only for loopback hosts) with no credentials, query, or fragment",
            ));
        }
        let now = std::time::Instant::now();
        if self.timeout.is_zero()
            || now.checked_add(self.timeout).is_none()
            || self.retries.initial_delay.is_zero()
            || now.checked_add(self.retries.max_delay).is_none()
            || self.retries.max_delay < self.retries.initial_delay
        {
            return Err(Error::Configuration(
                "deadline and retry delays must be positive and representable, with max_delay at least initial_delay",
            ));
        }
        // The stock reqwest policy only replays protocol refusals the server
        // never processed, so the default client keeps it. A caller builder may
        // carry any policy, which would stack under ours.
        let http = match self.http {
            Some(builder) => builder.retry(reqwest::retry::never()),
            None => Client::builder(),
        }
        .redirect(reqwest::redirect::Policy::none())
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
        let deadline = Instant::now() + self.timeout;
        tokio::time::timeout_at(deadline, self.send_inner(request, deadline))
            .await
            .map_err(|_| Error::Timeout)?
    }

    /// The wait before the next attempt, when the policy allows one and it
    /// ends before the deadline.
    fn wait(
        &self,
        attempt: u8,
        retry_after: Option<Duration>,
        deadline: Instant,
    ) -> Option<Duration> {
        let delay = self.retries.delay(attempt, retry_after, unit_sample())?;
        Instant::now()
            .checked_add(delay)
            .is_some_and(|wake| wake <= deadline)
            .then_some(delay)
    }

    async fn send_inner<T: DeserializeOwned>(
        &self,
        request: RequestBuilder,
        deadline: Instant,
    ) -> Result<T, Error> {
        let mut attempt = 0;
        loop {
            let response = match request
                .try_clone()
                .ok_or(Error::Configuration("request body cannot be replayed"))?
                .send()
                .await
            {
                Ok(response) => response,
                Err(error) => {
                    if error.is_connect()
                        && let Some(delay) = self.wait(attempt, None, deadline)
                    {
                        tokio::time::sleep(delay).await;
                        attempt += 1;
                        continue;
                    }
                    return Err(error.into());
                }
            };
            let status = response.status();
            let retry_after = response
                .headers()
                .get(reqwest::header::RETRY_AFTER)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| retry_after_delay(v, SystemTime::now()));
            if retryable_status(status.as_u16())
                && let Some(delay) = self.wait(attempt, retry_after, deadline)
            {
                drop(response);
                tokio::time::sleep(delay).await;
                attempt += 1;
                continue;
            }
            let body = response.bytes().await?;
            if status.is_success() {
                return serde_json::from_slice(&body).map_err(|source| Error::Decode {
                    status: status.as_u16(),
                    body: body.to_vec(),
                    source,
                });
            }
            let mut error: ApiError = serde_json::from_slice(&body).unwrap_or_else(|_| ApiError {
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

/// The statuses that [`RetryPolicy`] retries.
pub(crate) const fn retryable_status(status: u16) -> bool {
    matches!(status, 429 | 500 | 502 | 503 | 504)
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

    const ALMOST_ONE: f64 = 1.0 - f64::EPSILON;

    fn policy() -> RetryPolicy {
        RetryPolicy::new(4)
            .with_initial_delay(Duration::from_secs(1))
            .with_max_delay(Duration::from_secs(3))
    }

    #[test]
    fn full_jitter_stays_within_the_capped_exponential_ceiling() {
        let policy = policy();
        for (attempt, ceiling) in [(0, 1), (1, 2), (2, 3), (3, 3)] {
            let ceiling = Duration::from_secs(ceiling);
            assert_eq!(policy.delay(attempt, None, 0.0), Some(Duration::ZERO));
            assert_eq!(policy.delay(attempt, None, 0.5), Some(ceiling / 2));
            let upper = policy.delay(attempt, None, ALMOST_ONE).unwrap();
            assert!(upper <= ceiling && upper > ceiling - Duration::from_millis(1));
        }
        assert_eq!(policy.delay(4, None, 0.5), None);
        assert_eq!(RetryPolicy::new(0).delay(0, None, 0.5), None);
        let tiny = RetryPolicy::new(40)
            .with_initial_delay(Duration::from_nanos(1))
            .with_max_delay(Duration::from_secs(60));
        assert_eq!(
            tiny.delay(33, None, 0.5),
            Some(Duration::from_nanos(1_u64 << 32))
        );
        assert_eq!(tiny.delay(39, None, 0.5), Some(Duration::from_secs(30)));
    }

    #[test]
    fn retry_after_is_padded_capped_or_refused() {
        let policy = policy();
        let two = Some(Duration::from_secs(2));
        assert_eq!(policy.delay(0, two, 0.0), Some(Duration::from_secs(2)));
        assert_eq!(
            policy.delay(0, two, 0.25),
            Some(Duration::from_millis(2250))
        );
        assert_eq!(
            policy.delay(0, Some(Duration::from_millis(2500)), ALMOST_ONE),
            Some(Duration::from_secs(3))
        );
        assert_eq!(
            policy.delay(0, Some(Duration::from_secs(3)), 0.5),
            Some(Duration::from_secs(3))
        );
        assert_eq!(policy.delay(0, Some(Duration::from_secs(4)), 0.0), None);
        assert_eq!(policy.delay(0, Some(Duration::MAX), 0.0), None);
        assert_eq!(policy.delay(4, two, 0.0), None);
    }

    #[test]
    fn unit_samples_are_in_range_and_vary() {
        let samples: Vec<f64> = (0..64).map(|_| unit_sample()).collect();
        assert!(samples.iter().all(|value| (0.0..1.0).contains(value)));
        assert!(samples.windows(2).any(|pair| pair[0] != pair[1]));
    }

    #[test]
    fn policy_accessors_and_defaults() {
        let policy = RetryPolicy::new(3);
        assert_eq!(policy.max_retries(), 3);
        assert_eq!(policy.initial_delay(), Duration::from_millis(500));
        assert_eq!(policy.max_delay(), Duration::from_secs(30));
        assert_eq!(RetryPolicy::default(), RetryPolicy::new(2));
        for status in [429, 500, 502, 503, 504] {
            assert!(retryable_status(status));
        }
        for status in [400, 404, 408, 422, 501, 505] {
            assert!(!retryable_status(status));
        }
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
