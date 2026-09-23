use dialkit::{
    AccountSid, ApplicationSid, CallSid, Client, ConferenceSid, ConfigureApplication,
    ConfigurePhoneNumber, IncomingPhoneNumberSid, MessageMediaRef, MessageMediaSid, MessageSid,
    MessagingServiceSid, ParticipantCallSid, ParticipantUpdate, QueueSid,
};
use url::Url;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_string_contains, method, path},
};

const ACCOUNT: &str = "AC00000000000000000000000000000000";

fn client(server: &MockServer) -> Client {
    let origin = Url::parse(&server.uri()).unwrap();
    Client::builder(dialkit::Credentials::account_token(
        AccountSid::new(ACCOUNT).unwrap(),
        "synthetic-workflow-token",
    ))
    .base_url(origin.clone())
    .messaging_base_url(origin)
    .build()
    .unwrap()
}

async fn mount_post(
    server: &MockServer,
    request_path: impl Into<String>,
    body_fragment: &'static str,
    response: serde_json::Value,
) {
    Mock::given(method("POST"))
        .and(path(request_path.into()))
        .and(body_string_contains(body_fragment))
        .respond_with(ResponseTemplate::new(200).set_body_json(response))
        .expect(1)
        .mount(server)
        .await;
}

#[tokio::test]
async fn voice_control_facades_issue_operational_requests() {
    let server = MockServer::start().await;
    let client = client(&server);
    let conference = ConferenceSid::new("CF00000000000000000000000000000000").unwrap();
    let participant = ParticipantCallSid::new("CA00000000000000000000000000000000").unwrap();
    let queue = QueueSid::new("QU00000000000000000000000000000000").unwrap();
    let call = CallSid::new("CA11111111111111111111111111111111").unwrap();

    mount_post(
        &server,
        format!(
            "/2010-04-01/Accounts/{ACCOUNT}/Conferences/{}/Participants/{}.json",
            conference.as_str(),
            participant.as_str()
        ),
        "Muted=true",
        serde_json::json!({"call_sid": participant.as_str()}),
    )
    .await;
    mount_post(
        &server,
        format!(
            "/2010-04-01/Accounts/{ACCOUNT}/Queues/{}/Members/{}.json",
            queue.as_str(),
            call.as_str()
        ),
        "Url=https%3A%2F%2Fexample.test%2Fqueue-exit",
        serde_json::json!({"call_sid": call.as_str()}),
    )
    .await;
    mount_post(
        &server,
        format!(
            "/2010-04-01/Accounts/{ACCOUNT}/Calls/{}/Recordings.json",
            call.as_str()
        ),
        "",
        serde_json::json!({"sid": "RE00000000000000000000000000000000"}),
    )
    .await;

    client
        .conferences()
        .participants(conference)
        .update(&participant, ParticipantUpdate::new().muted(true))
        .await
        .unwrap();
    client
        .queues()
        .members(queue)
        .redirect(
            &call,
            &Url::parse("https://example.test/queue-exit").unwrap(),
        )
        .await
        .unwrap();
    let recording = client.recordings().start(&call).await.unwrap();
    assert_eq!(recording.as_str(), "RE00000000000000000000000000000000");
}

#[tokio::test]
async fn sip_setup_facade_creates_each_resource() {
    let server = MockServer::start().await;
    let client = client(&server);
    let list_sid = "CL00000000000000000000000000000000";

    mount_post(
        &server,
        format!("/2010-04-01/Accounts/{ACCOUNT}/SIP/Domains.json"),
        "DomainName=example.sip.twilio.com",
        serde_json::json!({"sid": "SD00000000000000000000000000000000"}),
    )
    .await;
    mount_post(
        &server,
        format!("/2010-04-01/Accounts/{ACCOUNT}/SIP/CredentialLists.json"),
        "FriendlyName=production+agents",
        serde_json::json!({"sid": list_sid}),
    )
    .await;
    mount_post(
        &server,
        format!("/2010-04-01/Accounts/{ACCOUNT}/SIP/CredentialLists/{list_sid}/Credentials.json"),
        "Username=agent",
        serde_json::json!({"sid": "CR00000000000000000000000000000000"}),
    )
    .await;

    let sip = client.sip();
    let domain = sip.create_domain("example.sip.twilio.com").await.unwrap();
    let list = sip
        .create_credential_list("production agents")
        .await
        .unwrap();
    let credential = sip
        .create_credential(&list, "agent", "synthetic-password")
        .await
        .unwrap();
    assert_eq!(domain.as_str(), "SD00000000000000000000000000000000");
    assert_eq!(list.as_str(), list_sid);
    assert_eq!(credential.as_str(), "CR00000000000000000000000000000000");
}

#[tokio::test]
async fn messaging_service_sender_facade_uses_messaging_origin() {
    let server = MockServer::start().await;
    let client = client(&server);
    let service = MessagingServiceSid::new("MG00000000000000000000000000000000").unwrap();
    let phone = IncomingPhoneNumberSid::new("PN00000000000000000000000000000000").unwrap();

    mount_post(
        &server,
        format!("/v1/Services/{}/PhoneNumbers", service.as_str()),
        "PhoneNumberSid=PN00000000000000000000000000000000",
        serde_json::json!({"phone_number_sid": phone.as_str()}),
    )
    .await;

    client
        .messaging_services()
        .senders(service)
        .add_phone_number(&phone)
        .await
        .unwrap();
}

#[tokio::test]
async fn empty_control_updates_and_sip_fields_are_rejected_before_transport() {
    let server = MockServer::start().await;
    let client = client(&server);
    let conference = ConferenceSid::new("CF00000000000000000000000000000000").unwrap();
    let participant = ParticipantCallSid::new("CA00000000000000000000000000000000").unwrap();

    let participant_error = client
        .conferences()
        .participants(conference)
        .update(&participant, ParticipantUpdate::new())
        .await
        .unwrap_err();
    assert!(participant_error.to_string().contains("must change"));

    let sip_error = client.sip().create_domain("  ").await.unwrap_err();
    assert!(sip_error.to_string().contains("cannot be empty"));
}

#[tokio::test]
async fn application_phone_and_media_facades_use_pinned_routes_and_typed_results() {
    let server = MockServer::start().await;
    let client = client(&server);
    let application = ApplicationSid::new("AP00000000000000000000000000000000").unwrap();
    let phone = IncomingPhoneNumberSid::new("PN00000000000000000000000000000000").unwrap();
    let message = MessageSid::new("SM00000000000000000000000000000000").unwrap();
    let media = MessageMediaSid::new("ME00000000000000000000000000000000").unwrap();
    let application_path = format!(
        "/2010-04-01/Accounts/{ACCOUNT}/Applications/{}.json",
        application.as_str()
    );
    let phone_path = format!(
        "/2010-04-01/Accounts/{ACCOUNT}/IncomingPhoneNumbers/{}.json",
        phone.as_str()
    );
    let media_path = format!(
        "/2010-04-01/Accounts/{ACCOUNT}/Messages/{}/Media/{}.json",
        message.as_str(),
        media.as_str()
    );
    let application_response = serde_json::json!({"sid": application.as_str(), "friendly_name": "Synthetic app", "voice_url": "https://example.test/voice"});
    let phone_response = serde_json::json!({"sid": phone.as_str(), "phone_number": "+15005550006", "voice_url": "https://example.test/phone"});
    Mock::given(method("GET"))
        .and(path(application_path.clone()))
        .respond_with(ResponseTemplate::new(200).set_body_json(application_response.clone()))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path(application_path))
        .and(body_string_contains(
            "VoiceUrl=https%3A%2F%2Fexample.test%2Fvoice",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(application_response))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path(phone_path.clone()))
        .respond_with(ResponseTemplate::new(200).set_body_json(phone_response.clone()))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path(phone_path))
        .and(body_string_contains(
            "VoiceApplicationSid=AP00000000000000000000000000000000",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(phone_response))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET")).and(path(media_path))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"sid": media.as_str(), "parent_sid": message.as_str(), "content_type": "image/png"})))
        .expect(1).mount(&server).await;

    let apps = client.applications();
    assert_eq!(
        apps.get(&application).await.unwrap().sid().as_str(),
        application.as_str()
    );
    let configured = apps
        .configure(
            &application,
            ConfigureApplication::new()
                .voice_url(Url::parse("https://example.test/voice").unwrap()),
        )
        .await
        .unwrap();
    assert_eq!(
        configured.voice_url().unwrap().as_str(),
        "https://example.test/voice"
    );
    let numbers = client.phone_numbers();
    assert_eq!(
        numbers.get(&phone).await.unwrap().phone_number(),
        "+15005550006"
    );
    let configured = numbers
        .configure(
            &phone,
            ConfigurePhoneNumber::new().voice_application(application.clone()),
        )
        .await
        .unwrap();
    assert_eq!(configured.sid().as_str(), phone.as_str());
    let metadata = client
        .media()
        .metadata(&MessageMediaRef::new(message.clone(), media.clone()))
        .await
        .unwrap();
    assert_eq!(metadata.sid().as_str(), media.as_str());
    assert_eq!(metadata.message_sid().as_str(), message.as_str());
    assert_eq!(metadata.content_type(), "image/png");

    assert!(
        apps.configure(&application, ConfigureApplication::new())
            .await
            .is_err()
    );
    assert!(
        numbers
            .configure(
                &phone,
                ConfigurePhoneNumber::new()
                    .voice_application(application)
                    .voice_url(Url::parse("https://example.test/voice").unwrap())
            )
            .await
            .is_err()
    );
}
