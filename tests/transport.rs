pub mod common;

use std::time::{Duration, SystemTime};

use common::{ERROR, FIXTURE_KEY, LANGUAGES, Reply, Server};
use roxyapi::{Error, RetryPolicy, Roxy, types};

fn policy(max_retries: u8) -> RetryPolicy {
    RetryPolicy {
        max_retries,
        initial_delay: Duration::from_millis(25),
        max_delay: Duration::from_millis(60),
    }
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
    for (initial_delay, max_delay) in [
        (Duration::ZERO, Duration::from_secs(1)),
        (Duration::from_secs(2), Duration::from_secs(1)),
        (Duration::from_millis(25), Duration::MAX),
    ] {
        assert!(matches!(
            Roxy::builder(FIXTURE_KEY)
                .retry_policy(RetryPolicy {
                    max_retries: 1,
                    initial_delay,
                    max_delay,
                })
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
    assert_eq!(error.error, "Request rejected");
    assert_eq!(error.code, "INVALID_INPUT");
    assert_eq!(
        error.doc_url.as_deref(),
        Some("https://roxyapi.com/docs/errors")
    );
    assert_eq!(error.details["field"], "date");
    assert!(error.retry_after.is_none());
    assert!(error.to_string().contains("400 INVALID_INPUT"));
}

#[tokio::test]
async fn non_json_http_error_uses_safe_status_fallback() {
    let server = Server::start(vec![Reply::new(502, "<html>upstream unavailable</html>")]).await;
    let error = api_error(
        server
            .client()
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
async fn successful_invalid_json_is_a_decode_error() {
    let server = Server::start(vec![Reply::new(200, "not JSON")]).await;
    assert!(matches!(
        server.client().languages().list_languages().await,
        Err(Error::Decode(_))
    ));
}

#[tokio::test]
async fn retries_are_disabled_by_default() {
    let server = Server::start(vec![Reply::new(503, ERROR), Reply::new(200, LANGUAGES)]).await;
    let error = api_error(
        server
            .client()
            .languages()
            .list_languages()
            .await
            .unwrap_err(),
    );
    assert_eq!(error.status, 503);
    assert_eq!(server.observed().len(), 1);
}

#[tokio::test]
async fn opt_in_retries_use_bounded_exponential_delays_and_stop_at_success() {
    let server = Server::start(vec![
        Reply::new(502, ERROR),
        Reply::new(503, ERROR),
        Reply::new(504, ERROR),
        Reply::new(200, LANGUAGES),
    ])
    .await;
    let client = Roxy::builder(FIXTURE_KEY)
        .base_url(&server.url)
        .retry_policy(policy(4))
        .build()
        .unwrap();
    client.languages().list_languages().await.unwrap();
    let requests = server.observed();
    assert_eq!(requests.len(), 4);
    for (pair, minimum) in requests.windows(2).zip([20, 45, 55]) {
        assert!(pair[1].at.duration_since(pair[0].at) >= Duration::from_millis(minimum));
    }
}

#[tokio::test]
async fn retries_stop_when_retry_count_is_exhausted() {
    let server = Server::start(vec![Reply::new(503, ERROR)]).await;
    let client = Roxy::builder(FIXTURE_KEY)
        .base_url(&server.url)
        .retry_policy(policy(2))
        .build()
        .unwrap();
    let error = api_error(client.languages().list_languages().await.unwrap_err());
    assert_eq!(error.status, 503);
    assert_eq!(server.observed().len(), 3);
}

#[tokio::test]
async fn client_errors_and_other_server_errors_are_not_retried() {
    for status in [400, 401, 403, 404, 408, 409, 422, 500, 501] {
        let server = Server::start(vec![Reply::new(status, ERROR)]).await;
        let client = Roxy::builder(FIXTURE_KEY)
            .base_url(&server.url)
            .retry_policy(policy(2))
            .build()
            .unwrap();
        assert_eq!(
            api_error(client.languages().list_languages().await.unwrap_err()).status,
            status
        );
        assert_eq!(server.observed().len(), 1);
    }
}

#[tokio::test]
async fn retry_after_seconds_are_respected() {
    let server = Server::start(vec![
        Reply::new(429, ERROR).retry_after("1"),
        Reply::new(200, LANGUAGES),
    ])
    .await;
    let client = Roxy::builder(FIXTURE_KEY)
        .base_url(&server.url)
        .retry_policy(RetryPolicy {
            max_delay: Duration::from_secs(2),
            ..policy(1)
        })
        .build()
        .unwrap();
    client.languages().list_languages().await.unwrap();
    let requests = server.observed();
    assert_eq!(requests.len(), 2);
    assert!(requests[1].at.duration_since(requests[0].at) >= Duration::from_secs(1));
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
    let client = Roxy::builder(FIXTURE_KEY)
        .base_url(&server.url)
        .retry_policy(RetryPolicy {
            max_delay: Duration::from_secs(3),
            ..policy(1)
        })
        .build()
        .unwrap();
    client.languages().list_languages().await.unwrap();
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
        let server = Server::start(vec![Reply::new(429, ERROR).retry_after(value)]).await;
        let client = Roxy::builder(FIXTURE_KEY)
            .base_url(&server.url)
            .retry_policy(policy(2))
            .build()
            .unwrap();
        let error = api_error(client.languages().list_languages().await.unwrap_err());
        assert_eq!(error.status, 429);
        assert_eq!(server.observed().len(), 1, "Retry-After: {value}");
    }
}

#[tokio::test]
async fn retry_after_is_exposed_when_retries_are_disabled() {
    let server = Server::start(vec![Reply::new(429, ERROR).retry_after("120")]).await;
    let error = api_error(
        server
            .client()
            .languages()
            .list_languages()
            .await
            .unwrap_err(),
    );
    assert_eq!(error.retry_after, Some(Duration::from_secs(120)));
}

#[tokio::test]
async fn overall_deadline_includes_retry_sleep() {
    let server = Server::start(vec![Reply::new(503, ERROR)]).await;
    let client = Roxy::builder(FIXTURE_KEY)
        .base_url(&server.url)
        .timeout(Duration::from_millis(80))
        .retry_policy(RetryPolicy {
            max_retries: 3,
            initial_delay: Duration::from_millis(300),
            max_delay: Duration::from_secs(1),
        })
        .build()
        .unwrap();
    assert!(matches!(
        client.languages().list_languages().await,
        Err(Error::Timeout)
    ));
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
    let params = types::SearchCitiesParams {
        q: query.into(),
        ..Default::default()
    };
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
    let params = types::GetCardParams {
        id: "星 /?#+%".into(),
        ..Default::default()
    };
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
        let params = types::GetCardParams {
            id: id.into(),
            ..Default::default()
        };
        assert!(matches!(
            server.client().tarot().get_card(&params).await,
            Err(Error::Configuration(_))
        ));
    }
    assert!(server.observed().is_empty());
}

#[tokio::test]
async fn post_json_is_identical_across_retries_and_omits_unset_fields() {
    let server = Server::start(vec![Reply::new(503, ERROR), Reply::new(400, ERROR)]).await;
    let client = Roxy::builder(FIXTURE_KEY)
        .base_url(&server.url)
        .retry_policy(policy(1))
        .build()
        .unwrap();
    let body = types::GetDailyCardRequest {
        seed: Some("読み物 café".into()),
        ..Default::default()
    };
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
    assert!(
        serde_json::from_value::<types::GetDailyHoroscopeParams>(
            serde_json::json!({"sign":"invalid"})
        )
        .is_err()
    );
}
