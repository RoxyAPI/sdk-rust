pub mod common;

use std::time::{Duration, SystemTime};

use common::{ERROR, FIXTURE_KEY, LANGUAGES, RATE_LIMITED, Reply, SATURATED, Server};
use roxyapi::{Error, ErrorCode, RetryPolicy, Roxy, types};

fn policy(max_retries: u8) -> RetryPolicy {
    RetryPolicy::new(max_retries)
        .with_initial_delay(Duration::from_millis(25))
        .with_max_delay(Duration::from_millis(60))
}

fn retrying(server: &Server, policy: RetryPolicy) -> Roxy {
    Roxy::builder(FIXTURE_KEY)
        .base_url(&server.url)
        .retry_policy(policy)
        .build()
        .unwrap()
}

/// A loopback address with nothing listening on it.
fn closed_address() -> std::net::SocketAddr {
    let reserved = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    reserved.local_addr().unwrap()
}

fn api_error(error: Error) -> Box<roxyapi::ApiError> {
    match error {
        Error::Api(error) => error,
        other => panic!("expected an HTTP error, received {other:?}"),
    }
}

#[test]
fn client_configuration_validates_inputs_and_redacts_credentials() {
    fn clone_send_sync<T: Clone + Send + Sync>() {}
    fn error_send_sync<T: std::error::Error + Send + Sync + 'static>() {}
    clone_send_sync::<Roxy>();
    error_send_sync::<Error>();
    for value in ["", "  ", "bad\nheader"] {
        assert!(matches!(Roxy::new(value), Err(Error::Configuration(_))));
    }
    for url in [
        "ftp://example.com",
        "http://example.com",
        "http://127.0.0.1.example.com",
        "https://fixture@example.com",
        "https://example.com?query=1",
        "https://example.com#fragment",
        "not a URL",
    ] {
        assert!(matches!(
            Roxy::builder(FIXTURE_KEY).base_url(url).build(),
            Err(Error::Configuration(_))
        ));
    }
    for timeout in [Duration::ZERO, Duration::MAX] {
        assert!(matches!(
            Roxy::builder(FIXTURE_KEY).timeout(timeout).build(),
            Err(Error::Configuration(_))
        ));
    }
    for url in [
        "https://example.com/api/v2",
        "http://localhost:3000/api/v2",
        "http://127.0.0.1:3000",
        "http://[::1]:3000",
    ] {
        assert!(Roxy::builder(FIXTURE_KEY).base_url(url).build().is_ok());
    }
    for (initial_delay, max_delay) in [
        (Duration::ZERO, Duration::from_secs(1)),
        (Duration::from_secs(2), Duration::from_secs(1)),
        (Duration::from_millis(25), Duration::MAX),
    ] {
        assert!(matches!(
            Roxy::builder(FIXTURE_KEY)
                .retry_policy(
                    RetryPolicy::new(1)
                        .with_initial_delay(initial_delay)
                        .with_max_delay(max_delay)
                )
                .build(),
            Err(Error::Configuration(_))
        ));
    }
    let builder = Roxy::builder(FIXTURE_KEY);
    assert!(!format!("{builder:?}").contains(FIXTURE_KEY));
    let client = builder.build().unwrap();
    assert!(!format!("{client:?}").contains(FIXTURE_KEY));
}

#[tokio::test]
async fn sends_credentials_sdk_version_and_preserves_url_prefix() {
    let server = Server::start(vec![Reply::new(200, LANGUAGES)]).await;
    for suffix in ["/api/v2", "/api/v2/"] {
        let client = Roxy::builder(FIXTURE_KEY)
            .base_url(format!("{}{suffix}", server.url))
            .build()
            .unwrap();
        assert!(
            client
                .languages()
                .list_languages()
                .await
                .unwrap()
                .languages
                .is_empty()
        );
    }
    for request in server.observed() {
        assert_eq!(request.method, "GET");
        assert_eq!(request.target, "/api/v2/languages");
        assert_eq!(request.headers["x-api-key"], FIXTURE_KEY);
        assert_eq!(
            request.headers["x-sdk-client"],
            format!("roxy-sdk-rust/{}", roxyapi::VERSION)
        );
        assert!(request.body.is_empty());
    }
}

#[tokio::test]
async fn custom_http_builder_is_used() {
    let server = Server::start(vec![Reply::new(200, LANGUAGES)]).await;
    let client = Roxy::builder(FIXTURE_KEY)
        .base_url(&server.url)
        .http_client_builder(roxyapi::reqwest::Client::builder().user_agent("fixture-app/1.0"))
        .build()
        .unwrap();
    client.languages().list_languages().await.unwrap();
    assert_eq!(
        server.observed()[0].headers["user-agent"],
        "fixture-app/1.0"
    );
}

#[tokio::test]
async fn typed_api_error_preserves_status_fields_and_extra_details() {
    let server = Server::start(vec![Reply::new(400, ERROR)]).await;
    let error = api_error(
        server
            .client()
            .languages()
            .list_languages()
            .await
            .unwrap_err(),
    );
    assert_eq!(error.status, 400);
    assert_eq!(error.error, "date: Invalid date");
    assert_eq!(error.code, "validation_error");
    assert_eq!(error.error_code(), ErrorCode::ValidationError);
    assert_eq!(
        error.doc_url.as_deref(),
        Some("https://roxyapi.com/docs/errors#validation_error")
    );
    assert_eq!(error.details["issues"][0]["path"], "date");
    assert_eq!(error.details["issues"][0]["message"], "Invalid date");
    assert!(error.retry_after.is_none());
    assert!(!error.is_retryable());
    assert!(error.to_string().contains("400 validation_error"));
}

#[tokio::test]
async fn error_details_carry_allowed_methods_and_path_suggestions() {
    let server = Server::start(vec![
        Reply::new(
            405,
            r#"{"error":"Method not allowed","code":"method_not_allowed","doc_url":"https://roxyapi.com/docs/errors#method_not_allowed","allow":["POST"]}"#,
        ),
        Reply::new(
            404,
            r#"{"error":"Not found","code":"not_found","doc_url":"https://roxyapi.com/docs/errors#not_found","suggestions":["/api/v2/languages"]}"#,
        ),
    ])
    .await;
    let client = server.client();
    let method = api_error(client.languages().list_languages().await.unwrap_err());
    assert_eq!(method.code, "method_not_allowed");
    assert_eq!(method.details["allow"], serde_json::json!(["POST"]));
    let missing = api_error(client.languages().list_languages().await.unwrap_err());
    assert_eq!(missing.code, "not_found");
    assert_eq!(missing.details["suggestions"][0], "/api/v2/languages");
}

#[tokio::test]
async fn non_json_http_error_uses_safe_status_fallback() {
    let server = Server::start(vec![Reply::new(502, "<html>upstream unavailable</html>")]).await;
    let error = api_error(
        retrying(&server, RetryPolicy::new(0))
            .languages()
            .list_languages()
            .await
            .unwrap_err(),
    );
    assert_eq!(error.status, 502);
    assert_eq!(error.error, "Bad Gateway");
    assert_eq!(error.code, "");
    assert!(error.doc_url.is_none());
    assert!(!error.to_string().contains("<html>"));
}

#[tokio::test]
async fn decode_error_keeps_status_and_raw_body() {
    let server = Server::start(vec![Reply::new(200, r#"{"languages":"unexpected"}"#)]).await;
    let error = server
        .client()
        .languages()
        .list_languages()
        .await
        .unwrap_err();
    match &error {
        Error::Decode { status, body, .. } => {
            assert_eq!(*status, 200);
            let value: serde_json::Value = serde_json::from_slice(body).unwrap();
            assert_eq!(value["languages"], "unexpected");
        }
        other => panic!("expected a decode error, received {other:?}"),
    }
    assert_chain_has_no_repeats(&error);
}

/// Error reporters print each message in the source chain, so no message may
/// repeat the text of its source.
fn assert_chain_has_no_repeats(error: &(dyn std::error::Error + 'static)) {
    let mut current = error;
    while let Some(source) = current.source() {
        let (outer, inner) = (current.to_string(), source.to_string());
        assert!(!outer.contains(&inner), "{outer:?} repeats {inner:?}");
        current = source;
    }
}

#[tokio::test]
async fn display_never_repeats_the_source_message() {
    let server = Server::start(vec![Reply::new(400, ERROR)]).await;
    let api = server
        .client()
        .languages()
        .list_languages()
        .await
        .unwrap_err();
    assert!(std::error::Error::source(&api).is_none());
    assert_chain_has_no_repeats(&api);
    let client = Roxy::builder(FIXTURE_KEY)
        .base_url(format!("http://{}", closed_address()))
        .retry_policy(RetryPolicy::new(0))
        .build()
        .unwrap();
    let transport = client.languages().list_languages().await.unwrap_err();
    assert!(matches!(transport, Error::Http(_)));
    assert!(std::error::Error::source(&transport).is_some());
    assert_chain_has_no_repeats(&transport);
}

#[tokio::test]
async fn default_client_retries_twice_and_new_zero_opts_out() {
    let server = Server::start(vec![
        Reply::new(500, ERROR),
        Reply::new(503, SATURATED),
        Reply::new(200, LANGUAGES),
    ])
    .await;
    server.client().languages().list_languages().await.unwrap();
    let requests = server.observed();
    assert_eq!(requests.len(), 3);
    // Default full jitter: up to 500 milliseconds, then up to one second.
    for (pair, ceiling) in requests.windows(2).zip([500, 1000]) {
        let gap = pair[1].at.duration_since(pair[0].at);
        assert!(gap < Duration::from_millis(ceiling + 250), "{gap:?}");
    }
    let server = Server::start(vec![Reply::new(503, SATURATED), Reply::new(200, LANGUAGES)]).await;
    let error = api_error(
        retrying(&server, RetryPolicy::new(0))
            .languages()
            .list_languages()
            .await
            .unwrap_err(),
    );
    assert_eq!(error.status, 503);
    assert_eq!(server.observed().len(), 1);
}

#[tokio::test]
async fn opt_in_retries_use_jittered_delays_within_the_backoff_ceiling() {
    let server = Server::start(vec![
        Reply::new(429, RATE_LIMITED),
        Reply::new(500, ERROR),
        Reply::new(502, ERROR),
        Reply::new(503, SATURATED),
        Reply::new(504, ERROR),
        Reply::new(200, LANGUAGES),
    ])
    .await;
    retrying(&server, policy(5))
        .languages()
        .list_languages()
        .await
        .unwrap();
    let requests = server.observed();
    assert_eq!(requests.len(), 6);
    // Full jitter waits between zero and min(60, 25 * 2^attempt) milliseconds;
    // the slack covers scheduling and a fresh connection per attempt.
    for (pair, ceiling) in requests.windows(2).zip([25, 50, 60, 60, 60]) {
        let gap = pair[1].at.duration_since(pair[0].at);
        assert!(gap < Duration::from_millis(ceiling + 250), "{gap:?}");
    }
}

#[tokio::test]
async fn connect_failures_before_sending_are_retried() {
    let address = closed_address();
    let url = format!("http://{address}");
    let late = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(40)).await;
        let listener = tokio::net::TcpListener::bind(address).await.unwrap();
        Server::listen(listener, vec![Reply::new(200, LANGUAGES)])
    });
    let client = Roxy::builder(FIXTURE_KEY)
        .base_url(&url)
        .retry_policy(policy(50))
        .build()
        .unwrap();
    client.languages().list_languages().await.unwrap();
    assert_eq!(late.await.unwrap().observed().len(), 1);
    let without_retries = Roxy::builder(FIXTURE_KEY)
        .base_url(format!("http://{}", closed_address()))
        .retry_policy(RetryPolicy::new(0))
        .build()
        .unwrap();
    match without_retries.languages().list_languages().await {
        Err(Error::Http(error)) => assert!(error.is_connect()),
        other => panic!("expected a connect error, received {other:?}"),
    }
}

#[tokio::test]
async fn retries_stop_when_retry_count_is_exhausted() {
    let server = Server::start(vec![Reply::new(503, ERROR)]).await;
    let error = api_error(
        retrying(&server, policy(2))
            .languages()
            .list_languages()
            .await
            .unwrap_err(),
    );
    assert_eq!(error.status, 503);
    assert!(error.is_retryable());
    assert_eq!(server.observed().len(), 3);
}

#[tokio::test]
async fn client_errors_and_other_server_errors_are_not_retried() {
    for status in [400, 401, 403, 404, 408, 409, 422, 501, 505] {
        let server = Server::start(vec![Reply::new(status, ERROR)]).await;
        let error = api_error(
            retrying(&server, policy(2))
                .languages()
                .list_languages()
                .await
                .unwrap_err(),
        );
        assert_eq!(error.status, status);
        assert!(!error.is_retryable());
        assert_eq!(server.observed().len(), 1);
    }
}

#[tokio::test]
async fn retry_after_seconds_are_respected_with_bounded_padding() {
    let server = Server::start(vec![
        Reply::new(429, RATE_LIMITED).retry_after("1"),
        Reply::new(200, LANGUAGES),
    ])
    .await;
    retrying(&server, policy(1).with_max_delay(Duration::from_secs(2)))
        .languages()
        .list_languages()
        .await
        .unwrap();
    let requests = server.observed();
    assert_eq!(requests.len(), 2);
    let gap = requests[1].at.duration_since(requests[0].at);
    // One second from the server plus up to the 25 millisecond initial delay.
    assert!(gap >= Duration::from_secs(1), "{gap:?}");
    assert!(gap < Duration::from_millis(1025 + 250), "{gap:?}");
}

#[tokio::test]
async fn retry_after_http_dates_are_respected() {
    let date = httpdate::fmt_http_date(SystemTime::now() + Duration::from_secs(2));
    let target = httpdate::parse_http_date(&date).unwrap();
    let server = Server::start(vec![
        Reply::new(429, ERROR).retry_after(date),
        Reply::new(200, LANGUAGES),
    ])
    .await;
    retrying(&server, policy(1).with_max_delay(Duration::from_secs(3)))
        .languages()
        .list_languages()
        .await
        .unwrap();
    let requests = server.observed();
    assert_eq!(requests.len(), 2);
    assert!(requests[1].wall_time >= target);
}

#[tokio::test]
async fn retry_after_beyond_delay_cap_never_retries_early() {
    for value in [
        "120",
        "18446744073709551615",
        "999999999999999999999999999999999999999999",
    ] {
        let server = Server::start(vec![Reply::new(429, RATE_LIMITED).retry_after(value)]).await;
        let error = api_error(
            retrying(&server, policy(2))
                .languages()
                .list_languages()
                .await
                .unwrap_err(),
        );
        assert_eq!(error.status, 429);
        assert_eq!(error.error_code(), ErrorCode::RateLimitPerMinute);
        assert!(
            error
                .retry_after
                .is_some_and(|delay| delay > Duration::from_millis(60))
        );
        assert_eq!(server.observed().len(), 1, "Retry-After: {value}");
    }
}

#[tokio::test]
async fn retry_after_is_exposed_when_retries_are_disabled() {
    let server = Server::start(vec![Reply::new(429, RATE_LIMITED).retry_after("120")]).await;
    let error = api_error(
        retrying(&server, RetryPolicy::new(0))
            .languages()
            .list_languages()
            .await
            .unwrap_err(),
    );
    assert_eq!(error.retry_after, Some(Duration::from_secs(120)));
}

#[tokio::test]
async fn a_wait_past_the_deadline_returns_the_api_error() {
    let server = Server::start(vec![Reply::new(503, SATURATED).retry_after("1")]).await;
    let client = Roxy::builder(FIXTURE_KEY)
        .base_url(&server.url)
        .timeout(Duration::from_millis(200))
        .retry_policy(RetryPolicy::new(3).with_max_delay(Duration::from_secs(2)))
        .build()
        .unwrap();
    let started = std::time::Instant::now();
    let error = api_error(client.languages().list_languages().await.unwrap_err());
    assert!(started.elapsed() < Duration::from_millis(200));
    assert_eq!(error.status, 503);
    assert_eq!(error.error_code(), ErrorCode::ComputeSaturated);
    assert_eq!(error.retry_after, Some(Duration::from_secs(1)));
    assert_eq!(server.observed().len(), 1);
}

#[tokio::test]
async fn overall_deadline_includes_response_body() {
    let mut reply = Reply::new(200, LANGUAGES);
    reply.body_delay = Duration::from_millis(300);
    let server = Server::start(vec![reply]).await;
    let client = Roxy::builder(FIXTURE_KEY)
        .base_url(&server.url)
        .timeout(Duration::from_millis(80))
        .build()
        .unwrap();
    assert!(matches!(
        client.languages().list_languages().await,
        Err(Error::Timeout)
    ));
}

#[tokio::test]
async fn transport_timeout_is_not_retried() {
    let mut reply = Reply::new(200, LANGUAGES);
    reply.header_delay = Duration::from_millis(300);
    let server = Server::start(vec![reply]).await;
    let client = Roxy::builder(FIXTURE_KEY)
        .base_url(&server.url)
        .http_client_builder(roxyapi::reqwest::Client::builder().timeout(Duration::from_millis(50)))
        .retry_policy(policy(2))
        .build()
        .unwrap();
    match client.languages().list_languages().await {
        Err(Error::Http(error)) => {
            assert!(error.is_timeout());
            assert!(error.url().is_none());
        }
        other => panic!("expected transport timeout, received {other:?}"),
    }
    assert_eq!(server.observed().len(), 1);
}

#[tokio::test]
async fn redirects_are_returned_without_forwarding_credentials() {
    let target = Server::start(vec![Reply::new(200, LANGUAGES)]).await;
    let mut reply = Reply::new(302, "");
    reply
        .headers
        .push(("Location".into(), format!("{}/languages", target.url)));
    let server = Server::start(vec![reply]).await;
    let client = Roxy::builder(FIXTURE_KEY)
        .base_url(&server.url)
        .http_client_builder(
            roxyapi::reqwest::Client::builder()
                .redirect(roxyapi::reqwest::redirect::Policy::limited(10)),
        )
        .build()
        .unwrap();
    assert_eq!(
        api_error(client.languages().list_languages().await.unwrap_err()).status,
        302
    );
    assert!(target.observed().is_empty());
}

#[tokio::test]
async fn unicode_query_is_encoded_and_absent_options_stay_absent() {
    let server = Server::start(vec![Reply::new(
        200,
        r#"{"total":0,"limit":10,"offset":0,"cities":[]}"#,
    )])
    .await;
    let query = "São Paulo & 東京 +/ ?";
    let params = types::SearchCitiesParams::new(query.into());
    server
        .client()
        .location()
        .search_cities(&params)
        .await
        .unwrap();
    let requests = server.observed();
    let url =
        roxyapi::reqwest::Url::parse(&format!("http://example.com{}", requests[0].target)).unwrap();
    let pairs: Vec<_> = url.query_pairs().collect();
    assert_eq!(pairs.len(), 1);
    assert_eq!(pairs[0].0, "q");
    assert_eq!(pairs[0].1, query);
    assert_eq!(url.path(), "/api/v2/location/search");
}

#[tokio::test]
async fn path_parameter_remains_one_encoded_segment() {
    let server = Server::start(vec![Reply::new(400, ERROR)]).await;
    let params = types::GetCardParams::new("星 /?#+%".into());
    let error = api_error(server.client().tarot().get_card(&params).await.unwrap_err());
    assert_eq!(error.status, 400);
    let request = &server.observed()[0];
    assert_eq!(
        request.target,
        "/api/v2/tarot/cards/%E6%98%9F%20%2F%3F%23+%25"
    );
}

#[tokio::test]
async fn dot_path_parameters_are_rejected_before_sending() {
    let server = Server::start(vec![Reply::new(400, ERROR)]).await;
    for id in ["", ".", ".."] {
        let params = types::GetCardParams::new(id.into());
        assert!(matches!(
            server.client().tarot().get_card(&params).await,
            Err(Error::Configuration(_))
        ));
    }
    assert!(server.observed().is_empty());
}

#[tokio::test]
async fn post_json_is_identical_across_retries_and_omits_unset_fields() {
    let server = Server::start(vec![Reply::new(503, SATURATED), Reply::new(400, ERROR)]).await;
    let client = retrying(&server, policy(1));
    let body = types::GetDailyCardRequest::new().with_seed("読み物 café".into());
    assert_eq!(
        api_error(
            client
                .tarot()
                .get_daily_card(&Default::default(), &body)
                .await
                .unwrap_err()
        )
        .status,
        400
    );
    let requests = server.observed();
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0].method, "POST");
    assert_eq!(requests[0].target, "/tarot/daily");
    assert_eq!(requests[0].headers["content-type"], "application/json");
    assert_eq!(requests[0].body, requests[1].body);
    let value: serde_json::Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert_eq!(value, serde_json::json!({"seed":"読み物 café"}));
}

#[tokio::test]
async fn enum_path_and_language_values_use_their_wire_names() {
    let server = Server::start(vec![Reply::new(400, ERROR)]).await;
    let params: types::GetDailyHoroscopeParams = serde_json::from_value(serde_json::json!({
        "sign": "aries",
        "lang": "zh-Hans"
    }))
    .unwrap();
    let error = api_error(
        server
            .client()
            .astrology()
            .get_daily_horoscope(&params)
            .await
            .unwrap_err(),
    );
    assert_eq!(error.status, 400);
    assert_eq!(
        server.observed()[0].target,
        "/api/v2/astrology/horoscope/aries/daily?lang=zh-Hans"
    );
    let unlisted: types::GetDailyHoroscopeParams =
        serde_json::from_value(serde_json::json!({"sign": "unlisted"})).unwrap();
    assert_eq!(
        serde_json::to_value(&unlisted).unwrap(),
        serde_json::json!({"sign": "unlisted"})
    );
}
