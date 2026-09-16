use dialkit::{AccountSid, Client, CreateMessage, MessageSender, MessageSid, PhoneEndpoint};
use url::Url;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_string_contains, method, path},
};

#[tokio::test]
async fn sends_a_message_with_a_stable_typed_result() {
    let server = MockServer::start().await;
    let account = "AC00000000000000000000000000000000";
    Mock::given(method("POST"))
        .and(path(format!(
            "/2010-04-01/Accounts/{account}/Messages.json"
        )))
        .and(body_string_contains("Body=hello+from+dialkit"))
        .respond_with(ResponseTemplate::new(201).set_body_raw(
            include_str!("../fixtures/messages.json"),
            "application/json",
        ))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::builder(dialkit::Credentials::account_token(
        AccountSid::new(account).unwrap(),
        "test-token-not-secret",
    ))
    .base_url(Url::parse(&server.uri()).unwrap())
    .build()
    .unwrap();
    let request = CreateMessage::new(
        MessageSender::phone(PhoneEndpoint::new("+15005550006").unwrap()),
        PhoneEndpoint::new("+15005550009").unwrap(),
    )
    .body("hello from dialkit")
    .build()
    .unwrap();
    let message = client.messages().create(request).await.unwrap();

    assert_eq!(message.sid().as_str(), "SM00000000000000000000000000000000");
    assert_eq!(message.status().as_str(), "queued");
}

#[test]
fn requires_message_content() {
    let request = CreateMessage::new(
        MessageSender::phone(PhoneEndpoint::new("+15005550006").unwrap()),
        PhoneEndpoint::new("+15005550009").unwrap(),
    );
    assert!(request.build().is_err());
    for invalid in ["", "SM01", "../../admin", "SM space", "SM✓"] {
        assert!(MessageSid::new(invalid).is_err());
    }
}
