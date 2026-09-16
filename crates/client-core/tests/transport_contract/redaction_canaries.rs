use dialkit_core::{
    auth::{AccountSid, Credentials},
    error::{ApiError, Error},
    request::{ClientConfiguration, HttpClient, RequestSpec},
    retry::{OperationSafety, RetryPolicy},
};
use http::Method;
use secrecy::SecretString;
use std::time::Duration;
use tracing_test::traced_test;
use url::Url;
use wiremock::{Mock, MockServer, ResponseTemplate, matchers::method};

#[traced_test]
#[test]
fn credential_and_error_formatting_never_emit_canaries() {
    let canaries = [
        "token-CANARY",
        "+15005550006",
        "message-CANARY",
        "https://media.test/CANARY",
    ];
    let credentials = Credentials::account_token(
        AccountSid::new("ACcccccccccccccccccccccccccccccccc").unwrap(),
        SecretString::from(canaries[0].to_owned()),
    );
    tracing::info!(
        operation = "test",
        http.method = "POST",
        http.route = "/safe/{Sid}",
        "request attempt"
    );
    let api = ApiError::new(
        400,
        Some(1),
        "safe message",
        None,
        Some("RQ-safe".into()),
        Some("message-CANARY"),
    );
    let formatted = format!(
        "{credentials:?} {api:?} {}",
        Error::Api(Box::new(api.clone()))
    );
    for canary in canaries {
        assert!(!formatted.contains(canary));
        assert!(!logs_contain(canary));
    }
}

#[traced_test]
#[tokio::test]
async fn real_request_errors_and_traces_redact_every_sensitive_position() {
    let canaries = [
        "ACdeadbeefdeadbeefdeadbeefdeadbeef",
        "token-REQUEST-CANARY",
        "+15005550006-CANARY",
        "query-REQUEST-CANARY",
        "body-REQUEST-CANARY",
        "message-REQUEST-CANARY",
        "https://media.test/REQUEST-CANARY",
        "<Say>twiml-REQUEST-CANARY</Say>",
    ];
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
            "code": 21211,
            "message": canaries.join(" "),
            "more_info": format!("https://errors.test/?value={}", canaries[3]),
        })))
        .mount(&server)
        .await;
    let client = HttpClient::new(ClientConfiguration {
        credentials: Credentials::account_token(
            AccountSid::new(canaries[0]).unwrap(),
            SecretString::from(canaries[1].to_owned()),
        ),
        base_url: Url::parse(&server.uri()).unwrap(),
        connect_timeout: Duration::from_millis(100),
        request_timeout: Duration::from_secs(1),
        retry_policy: RetryPolicy::conservative(),
        allow_http_for_tests: true,
    })
    .unwrap();
    let error = client
        .execute_json::<serde_json::Value>(&RequestSpec {
            operation: "redaction_contract",
            method: Method::POST,
            route_template: "/safe/{AccountSid}",
            path: format!("safe/{}", canaries[0]),
            query: vec![("filter".into(), canaries[3].into())],
            form: vec![
                ("From".into(), canaries[2].into()),
                ("Body".into(), canaries[4].into()),
                ("Message".into(), canaries[5].into()),
                ("MediaUrl".into(), canaries[6].into()),
                ("Twiml".into(), canaries[7].into()),
            ],
            safety: OperationSafety::Mutation,
        })
        .await
        .unwrap_err();
    let formatted = format!("{error:?} {error}");
    for canary in canaries {
        assert!(
            !formatted.contains(canary),
            "formatted error leaked {canary}"
        );
        assert!(!logs_contain(canary), "trace leaked {canary}");
    }
    assert!(logs_contain("redaction_contract"));
    assert!(logs_contain("api_error"));
}
