use dialkit_core::{
    auth::{AccountSid, ApiKeySid, Credentials},
    error::Error,
    request::{ClientConfiguration, HttpClient, RequestSpec},
    retry::{OperationSafety, RetryPolicy},
};
use secrecy::SecretString;
use std::{
    num::NonZeroU32,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use url::Url;
use wiremock::{
    Mock, MockServer, Request, Respond, ResponseTemplate,
    matchers::{header, method},
};

fn client(
    server: &MockServer,
    credentials: Credentials,
    policy: RetryPolicy,
    timeout: Duration,
) -> HttpClient {
    HttpClient::new(ClientConfiguration {
        credentials,
        base_url: Url::parse(&server.uri()).unwrap(),
        connect_timeout: Duration::from_millis(50),
        request_timeout: timeout,
        retry_policy: policy,
        allow_http_for_tests: true,
    })
    .unwrap()
}

fn get(safety: OperationSafety) -> RequestSpec {
    RequestSpec {
        operation: "test",
        method: http::Method::GET,
        route_template: "/test",
        path: "test".into(),
        query: vec![],
        form: vec![],
        safety,
    }
}

#[test]
fn credential_identifiers_reject_invalid_shapes() {
    for invalid in ["", "AC01", "../../admin", "AC space", "AC✓"] {
        assert!(AccountSid::new(invalid).is_err());
    }
    for invalid in ["", "SK01", "../../admin", "SK space", "SK✓"] {
        assert!(ApiKeySid::new(invalid).is_err());
    }
}

#[derive(Clone)]
struct Sequence {
    calls: Arc<AtomicUsize>,
}
impl Respond for Sequence {
    fn respond(&self, _: &Request) -> ResponseTemplate {
        if self.calls.fetch_add(1, Ordering::SeqCst) == 0 {
            ResponseTemplate::new(429)
                .insert_header("Retry-After", "0")
                .set_body_json(serde_json::json!({"code":20429,"message":"rate limited"}))
        } else {
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"ok":true}))
        }
    }
}

#[tokio::test]
async fn account_token_and_api_key_authenticate_with_the_expected_username() {
    for (credentials, username) in [
        (
            Credentials::account_token(
                AccountSid::new("ACaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap(),
                SecretString::from("token".to_owned()),
            ),
            "ACaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        ),
        (
            Credentials::api_key(
                AccountSid::new("ACaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap(),
                ApiKeySid::new("SKbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb").unwrap(),
                SecretString::from("secret".to_owned()),
            ),
            "SKbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        ),
    ] {
        let server = MockServer::start().await;
        let encoded = base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            format!(
                "{username}:{}",
                if username == "ACaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" {
                    "token"
                } else {
                    "secret"
                }
            ),
        );
        Mock::given(method("GET"))
            .and(header("authorization", format!("Basic {encoded}")))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"ok":true})))
            .expect(1)
            .mount(&server)
            .await;
        let result: serde_json::Value = client(
            &server,
            credentials,
            RetryPolicy::conservative(),
            Duration::from_secs(1),
        )
        .execute_json(&get(OperationSafety::Read))
        .await
        .unwrap();
        assert_eq!(result["ok"], true);
    }
}

#[tokio::test]
async fn honors_retry_after_but_does_not_retry_ambiguous_mutations_by_default() {
    let server = MockServer::start().await;
    let calls = Arc::new(AtomicUsize::new(0));
    Mock::given(method("GET"))
        .respond_with(Sequence {
            calls: Arc::clone(&calls),
        })
        .mount(&server)
        .await;
    let credentials = Credentials::account_token(
        AccountSid::new("ACaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap(),
        SecretString::from("token".to_owned()),
    );
    let policy = RetryPolicy::conservative().max_attempts(NonZeroU32::new(2).unwrap());
    let value: serde_json::Value = client(&server, credentials, policy, Duration::from_secs(1))
        .execute_json(&get(OperationSafety::Read))
        .await
        .unwrap();
    assert_eq!(value["ok"], true);
    assert_eq!(calls.load(Ordering::SeqCst), 2);

    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(503))
        .expect(1)
        .mount(&server)
        .await;
    let credentials = Credentials::account_token(
        AccountSid::new("ACaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap(),
        SecretString::from("token".to_owned()),
    );
    assert!(
        client(
            &server,
            credentials,
            RetryPolicy::conservative(),
            Duration::from_secs(1)
        )
        .execute_json::<serde_json::Value>(&get(OperationSafety::Mutation))
        .await
        .is_err()
    );
}

#[tokio::test]
async fn total_timeout_is_categorized() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_millis(100)))
        .mount(&server)
        .await;
    let credentials = Credentials::account_token(
        AccountSid::new("ACaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap(),
        SecretString::from("token".to_owned()),
    );
    let result = client(
        &server,
        credentials,
        RetryPolicy::conservative().max_attempts(NonZeroU32::new(1).unwrap()),
        Duration::from_millis(60),
    )
    .execute_json::<serde_json::Value>(&get(OperationSafety::Read))
    .await;
    assert!(matches!(result, Err(Error::Timeout { attempts: 1 })));
}

#[tokio::test]
async fn structured_malformed_decode_rate_limit_and_transport_errors_are_distinct() {
    let server = MockServer::start().await;
    let credentials = || {
        Credentials::account_token(
            AccountSid::new("ACaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap(),
            SecretString::from("token".to_owned()),
        )
    };

    Mock::given(method("GET"))
        .and(wiremock::matchers::path("/structured"))
        .respond_with(
            ResponseTemplate::new(400)
                .insert_header("Twilio-Request-Id", "RQ-structured")
                .insert_header("Twilio-Concurrent-Requests", "7")
                .insert_header("Twilio-Request-Duration", "0.042")
                .insert_header("Retry-After", "3")
                .set_body_json(serde_json::json!({"code": 21211, "message": "invalid"})),
        )
        .mount(&server)
        .await;
    let mut structured = get(OperationSafety::Read);
    structured.path = "structured".into();
    match client(
        &server,
        credentials(),
        RetryPolicy::conservative().max_attempts(NonZeroU32::new(1).unwrap()),
        Duration::from_secs(1),
    )
    .execute_json::<serde_json::Value>(&structured)
    .await
    .unwrap_err()
    {
        Error::Api(api) => {
            assert_eq!(api.code(), Some(21211));
            assert_eq!(api.request_id(), Some("RQ-structured"));
            assert_eq!(api.attempts(), 1);
            assert_eq!(api.retry_after(), Some(Duration::from_secs(3)));
            assert_eq!(api.concurrent_requests(), Some("7"));
            assert_eq!(api.request_duration(), Some("0.042"));
        }
        other => panic!("expected API error, got {other:?}"),
    }

    Mock::given(method("GET"))
        .and(wiremock::matchers::path("/malformed"))
        .respond_with(ResponseTemplate::new(502).set_body_string("not-json"))
        .mount(&server)
        .await;
    let mut malformed = get(OperationSafety::Read);
    malformed.path = "malformed".into();
    match client(
        &server,
        credentials(),
        RetryPolicy::conservative().max_attempts(NonZeroU32::new(1).unwrap()),
        Duration::from_secs(1),
    )
    .execute_json::<serde_json::Value>(&malformed)
    .await
    .unwrap_err()
    {
        Error::Api(api) => assert_eq!(api.code(), None),
        other => panic!("expected malformed API error, got {other:?}"),
    }

    Mock::given(method("GET"))
        .and(wiremock::matchers::path("/decode"))
        .respond_with(ResponseTemplate::new(200).set_body_string("not-json"))
        .mount(&server)
        .await;
    let mut decode = get(OperationSafety::Read);
    decode.path = "decode".into();
    match client(
        &server,
        credentials(),
        RetryPolicy::conservative(),
        Duration::from_secs(1),
    )
    .execute_json::<serde_json::Value>(&decode)
    .await
    .unwrap_err()
    {
        Error::Decode { metadata, .. } => assert_eq!(metadata.attempts, 1),
        other => panic!("expected decode error, got {other:?}"),
    }

    Mock::given(method("GET"))
        .and(wiremock::matchers::path("/limited"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("Retry-After", "0")
                .set_body_json(serde_json::json!({"code": 20429, "message": "limited"})),
        )
        .expect(2)
        .mount(&server)
        .await;
    let mut limited = get(OperationSafety::Read);
    limited.path = "limited".into();
    match client(
        &server,
        credentials(),
        RetryPolicy::conservative().max_attempts(NonZeroU32::new(2).unwrap()),
        Duration::from_secs(1),
    )
    .execute_json::<serde_json::Value>(&limited)
    .await
    .unwrap_err()
    {
        Error::RateLimited {
            attempts, metadata, ..
        } => {
            assert_eq!(attempts, 2);
            assert_eq!(metadata.retry_after, Some(Duration::ZERO));
        }
        other => panic!("expected rate limit, got {other:?}"),
    }

    let refused = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let refused_address = refused.local_addr().unwrap();
    drop(refused);
    let endpoint = Url::parse(&format!("http://{refused_address}")).unwrap();
    let result = HttpClient::new(ClientConfiguration {
        credentials: credentials(),
        base_url: endpoint,
        connect_timeout: Duration::from_millis(50),
        request_timeout: Duration::from_millis(100),
        retry_policy: RetryPolicy::conservative().max_attempts(NonZeroU32::new(1).unwrap()),
        allow_http_for_tests: true,
    })
    .unwrap()
    .execute_json::<serde_json::Value>(&get(OperationSafety::Read))
    .await;
    assert!(
        matches!(result, Err(Error::Transport { attempts: 1, .. })),
        "expected one-attempt transport failure, got {result:?}"
    );
}

#[tokio::test]
async fn mutation_retry_requires_opt_in_and_cancellation_stops_work() {
    let server = MockServer::start().await;
    let calls = Arc::new(AtomicUsize::new(0));
    Mock::given(method("GET"))
        .respond_with(Sequence {
            calls: Arc::clone(&calls),
        })
        .mount(&server)
        .await;
    let credentials = Credentials::account_token(
        AccountSid::new("ACaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap(),
        SecretString::from("token".to_owned()),
    );
    let value: serde_json::Value = client(
        &server,
        credentials,
        RetryPolicy::conservative()
            .max_attempts(NonZeroU32::new(2).unwrap())
            .retry_mutations(true),
        Duration::from_secs(1),
    )
    .execute_json(&get(OperationSafety::Mutation))
    .await
    .unwrap();
    assert_eq!(value["ok"], true);
    assert_eq!(calls.load(Ordering::SeqCst), 2);

    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_secs(1)))
        .expect(1)
        .mount(&server)
        .await;
    let credentials = Credentials::account_token(
        AccountSid::new("ACaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap(),
        SecretString::from("token".to_owned()),
    );
    let client = client(
        &server,
        credentials,
        RetryPolicy::conservative(),
        Duration::from_secs(2),
    );
    let request = get(OperationSafety::Read);
    tokio::select! {
        _ = client.execute_json::<serde_json::Value>(&request) => panic!("delayed request completed"),
        () = tokio::time::sleep(Duration::from_millis(50)) => {}
    }
    tokio::time::sleep(Duration::from_millis(20)).await;
    assert_eq!(server.received_requests().await.unwrap().len(), 1);
}
