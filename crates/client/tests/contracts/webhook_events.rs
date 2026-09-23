use base64::{Engine as _, engine::general_purpose::STANDARD};
use dialkit::{WebhookFamily, WebhookValidator};
use hmac::{Hmac, Mac as _};
use sha1::Sha1;
use sha2::{Digest as _, Sha256};

fn signature(token: &str, url: &str, pairs: &[(String, String)]) -> String {
    let mut pairs = pairs.to_vec();
    pairs.sort_by(|left, right| left.0.cmp(&right.0).then_with(|| left.1.cmp(&right.1)));
    let mut input = url.to_owned();
    for (name, value) in pairs {
        input.push_str(&name);
        input.push_str(&value);
    }
    let mut mac = Hmac::<Sha1>::new_from_slice(token.as_bytes()).unwrap();
    mac.update(input.as_bytes());
    STANDARD.encode(mac.finalize().into_bytes())
}

fn signed_json(token: &str, body: &serde_json::Value) -> (Vec<u8>, String, String) {
    let body = serde_json::to_vec(body).unwrap();
    let digest = Sha256::digest(&body)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    let url = format!("https://example.test/status?bodySHA256={digest}");
    let mut mac = Hmac::<Sha1>::new_from_slice(token.as_bytes()).unwrap();
    mac.update(url.as_bytes());
    let signature = STANDARD.encode(mac.finalize().into_bytes());
    (body, url, signature)
}

#[test]
fn message_status_json_exposes_structured_channel_data_and_unknown_fields() {
    use dialkit::FamilyJsonWebhookEvent;
    let token = "synthetic-channel-token";
    let body = serde_json::json!({
        "MessageStatus": "introduced-after-pin",
        "ChannelData": {"provider": "future-channel", "nested": {"attempt": 2}},
        "FutureProperty": ["one", "two"]
    });
    let (raw, url, signature) = signed_json(token, &body);
    let verified = WebhookValidator::new(token)
        .verify_json(WebhookFamily::MessageStatus, &url, &raw, &signature)
        .unwrap();
    let FamilyJsonWebhookEvent::MessageStatus(event) = verified.parse_family_specific().unwrap()
    else {
        panic!("expected message status event")
    };
    assert_eq!(event.status().unwrap().as_str(), "introduced-after-pin");
    assert_eq!(event.channel_data().unwrap()["nested"]["attempt"], 2);
    assert!(!event.extras().contains_key("ChannelData"));
    assert_eq!(event.extras()["FutureProperty"][1], "two");
    assert!(!format!("{event:?}").contains("future-channel"));
}

#[test]
fn message_status_json_rejects_malformed_channel_data_without_echoing_it() {
    let token = "synthetic-channel-token";
    let body = serde_json::json!({"ChannelData": "private-malformed-canary"});
    let (raw, url, signature) = signed_json(token, &body);
    let error = WebhookValidator::new(token)
        .verify_json(WebhookFamily::MessageStatus, &url, &raw, &signature)
        .unwrap_err();
    assert!(!format!("{error:?} {error}").contains("private-malformed-canary"));
}

#[test]
fn verified_voice_families_parse_open_values_and_retain_duplicate_empty_extras() {
    let token = "synthetic-webhook-token";
    let url = "https://example.test/call-status";
    let pairs = vec![
        (
            "CallSid".into(),
            "CA00000000000000000000000000000000".into(),
        ),
        ("CallStatus".into(), "introduced-after-pin".into()),
        ("Future".into(), String::new()),
        ("Future".into(), "second".into()),
    ];
    let verified = WebhookValidator::new(token)
        .verify_form(
            WebhookFamily::CallProgress,
            url,
            &pairs,
            &signature(token, url, &pairs),
        )
        .unwrap();
    let event = verified.parse_voice().unwrap();
    assert_eq!(event.status().unwrap().as_str(), "introduced-after-pin");
    assert_eq!(event.extras().len(), 2);
    assert!(!format!("{verified:?} {event:?}").contains("CA00000000000000000000000000000000"));
}

#[test]
fn inbound_mms_indexes_media_and_redacts_content() {
    let token = "synthetic-message-token";
    let url = "https://example.test/inbound";
    let pairs = vec![
        (
            "MessageSid".into(),
            "SM00000000000000000000000000000000".into(),
        ),
        ("Body".into(), "message-body-canary".into()),
        ("NumMedia".into(), "2".into()),
        ("MediaUrl0".into(), "https://media.test/one".into()),
        ("MediaContentType0".into(), "image/png".into()),
        ("MediaUrl1".into(), "https://media.test/two".into()),
        ("Unknown".into(), "future".into()),
    ];
    let verified = WebhookValidator::new(token)
        .verify_form(
            WebhookFamily::IncomingMessage,
            url,
            &pairs,
            &signature(token, url, &pairs),
        )
        .unwrap();
    let event = verified.parse_messaging().unwrap();
    assert_eq!(event.media().len(), 2);
    assert_eq!(event.extras()[0].name(), "Unknown");
    assert!(!format!("{event:?}").contains("message-body-canary"));
    assert!(!format!("{event:?}").contains("https://media.test/one"));
}

#[test]
fn family_selection_is_explicit_and_parse_errors_are_value_free() {
    let token = "synthetic-family-token";
    let url = "https://example.test/family";
    let pairs = vec![("CallSid".into(), "secret-call-canary".into())];
    let verified = WebhookValidator::new(token)
        .verify_form(
            WebhookFamily::IncomingMessage,
            url,
            &pairs,
            &signature(token, url, &pairs),
        )
        .unwrap();
    let error = verified.parse_voice().unwrap_err();
    assert_eq!(error.field(), "family");
    assert!(!format!("{error:?} {error}").contains("secret-call-canary"));
}

#[test]
fn fixture_families_are_synthetic_and_declared() {
    for fixture in [
        include_str!("../fixtures/webhooks/voice/call-progress.json"),
        include_str!("../fixtures/webhooks/callbacks/recording.json"),
        include_str!("../fixtures/webhooks/messaging/inbound-mms.json"),
    ] {
        let value: serde_json::Value = serde_json::from_str(fixture).unwrap();
        assert_eq!(value["synthetic"], true);
    }
}
