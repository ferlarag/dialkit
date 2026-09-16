use base64::{Engine as _, engine::general_purpose::STANDARD};
use dialkit::{
    AccountSid, CallInstructions, CallSid, Client, CreateCall, Error, ListCalls, PhoneEndpoint,
};
use url::Url;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_string_contains, header, method, path},
};

#[tokio::test]
async fn creates_a_call_with_a_stable_typed_result() {
    let server = MockServer::start().await;
    let account = "AC00000000000000000000000000000000";
    let token = "test-token-not-secret";
    Mock::given(method("POST"))
        .and(path(format!("/2010-04-01/Accounts/{account}/Calls.json")))
        .and(header(
            "authorization",
            format!("Basic {}", STANDARD.encode(format!("{account}:{token}"))),
        ))
        .and(body_string_contains("From=%2B15005550006"))
        .and(body_string_contains("To=%2B15005550009"))
        .and(body_string_contains(
            "Url=https%3A%2F%2Fexample.test%2Fvoice",
        ))
        .respond_with(
            ResponseTemplate::new(201)
                .set_body_raw(include_str!("../fixtures/calls.json"), "application/json"),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::builder(dialkit::Credentials::account_token(
        AccountSid::new(account).unwrap(),
        token,
    ))
    .base_url(Url::parse(&server.uri()).unwrap())
    .build()
    .unwrap();
    let call = client
        .calls()
        .create(
            CreateCall::new(
                PhoneEndpoint::new("+15005550006").unwrap(),
                PhoneEndpoint::new("+15005550009").unwrap(),
                CallInstructions::Url(Url::parse("https://example.test/voice").unwrap()),
            )
            .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(call.sid().as_str(), "CA00000000000000000000000000000000");
    assert_eq!(call.status().as_str(), "queued");
}

#[test]
fn rejects_conflicting_or_empty_call_input() {
    assert!(PhoneEndpoint::new("").is_err());
    for invalid in ["", "CA01", "../../admin", "CA space", "CA✓"] {
        assert!(CallSid::new(invalid).is_err());
    }
}

#[tokio::test]
async fn rejected_call_preserves_safe_twilio_context() {
    let server = MockServer::start().await;
    let account = "AC00000000000000000000000000000000";
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(400)
            .insert_header("Twilio-Request-Id", "RQ-safe-test")
            .insert_header("Twilio-Concurrent-Requests", "4")
            .insert_header("Twilio-Request-Duration", "0.125")
            .insert_header("Retry-After", "2")
            .set_body_json(serde_json::json!({"code": 21212, "message": "Invalid To number", "more_info": "https://example.test/errors/21212"})))
        .mount(&server).await;
    let client = Client::builder(dialkit::Credentials::account_token(
        AccountSid::new(account).unwrap(),
        "secret-canary",
    ))
    .base_url(Url::parse(&server.uri()).unwrap())
    .build()
    .unwrap();
    let request = CreateCall::new(
        PhoneEndpoint::new("+15005550006").unwrap(),
        PhoneEndpoint::new("+15005550009").unwrap(),
        CallInstructions::Url(Url::parse("https://example.test/voice").unwrap()),
    )
    .unwrap();
    let error = client.calls().create(request).await.unwrap_err();
    match error {
        Error::Api(api) => {
            assert_eq!(api.code(), Some(21212));
            assert_eq!(api.request_id(), Some("RQ-safe-test"));
            assert_eq!(api.attempts(), 1);
            assert_eq!(api.retry_after(), Some(std::time::Duration::from_secs(2)));
            assert_eq!(api.concurrent_requests(), Some("4"));
            assert_eq!(api.request_duration(), Some("0.125"));
            assert!(!format!("{api:?}").contains("secret-canary"));
        }
        other => panic!("expected API error, got {other:?}"),
    }
}

#[tokio::test]
async fn paginates_calls_in_order() {
    let server = MockServer::start().await;
    let account = "AC00000000000000000000000000000000";
    let route = format!("/2010-04-01/Accounts/{account}/Calls.json");
    Mock::given(method("GET")).and(path(route.clone()))
        .and(wiremock::matchers::query_param("PageSize", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "calls": [{"sid":"CA00000000000000000000000000000001","status":"queued"}],
            "next_page_uri": format!("{}/{}?PageToken=opaque", server.uri(), route.trim_start_matches('/'))
        }))).expect(1).mount(&server).await;
    Mock::given(method("GET"))
        .and(path(route))
        .and(wiremock::matchers::query_param("PageToken", "opaque"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "calls": [{"sid":"CA00000000000000000000000000000002","status":"future-status"}], "next_page_uri": null
        })))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::builder(dialkit::Credentials::account_token(
        AccountSid::new(account).unwrap(),
        "test-token",
    ))
    .base_url(Url::parse(&server.uri()).unwrap())
    .build()
    .unwrap();
    let mut pager = client.calls().list(ListCalls::new().page_size(1));
    assert_eq!(
        pager.next().await.unwrap().unwrap().sid().as_str(),
        "CA00000000000000000000000000000001"
    );
    let second = pager.next().await.unwrap().unwrap();
    assert_eq!(second.sid().as_str(), "CA00000000000000000000000000000002");
    assert_eq!(second.status().as_str(), "future-status");
    assert!(pager.next().await.is_none());
}

#[tokio::test]
async fn exposes_page_boundaries_without_continuation_bookkeeping() {
    use futures_util::StreamExt as _;

    let server = MockServer::start().await;
    let account = "AC00000000000000000000000000000000";
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "calls": [{"sid":"CA00000000000000000000000000000001","status":"queued"}],
            "next_page_uri": null
        })))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::builder(dialkit::Credentials::account_token(
        AccountSid::new(account).unwrap(),
        "test-token",
    ))
    .base_url(Url::parse(&server.uri()).unwrap())
    .build()
    .unwrap();
    let pages = client
        .calls()
        .list(ListCalls::new())
        .pages()
        .collect::<Vec<_>>()
        .await;
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].as_ref().unwrap().items().len(), 1);
}
