use base64::{Engine as _, engine::general_purpose::STANDARD};
use dialkit::{WebhookFamily, WebhookValidator};
use hmac::{Hmac, Mac as _};
use sha1::Sha1;
use sha2::{Digest as _, Sha256};

fn manifest() -> toml::Value {
    toml::from_str(include_str!(
        "../../../../codegen/coverage/webhook-families.toml"
    ))
    .unwrap()
}

fn family(name: &str) -> WebhookFamily {
    match name {
        "voice-instruction" => WebhookFamily::VoiceInstruction,
        "call-progress" => WebhookFamily::CallProgress,
        "answering-machine" => WebhookFamily::AnsweringMachine,
        "twiml-action" => WebhookFamily::VoiceAction,
        "recording" => WebhookFamily::Recording,
        "conference" => WebhookFamily::Conference,
        "queue" => WebhookFamily::Queue,
        "gather" => WebhookFamily::Gather,
        "stream-siprec" => WebhookFamily::Stream,
        "transcription" => WebhookFamily::Transcription,
        "payment" => WebhookFamily::Payment,
        "user-defined-message" => WebhookFamily::UserDefinedMessage,
        "incoming-message" => WebhookFamily::IncomingMessage,
        "message-status" | "json-verification" => WebhookFamily::MessageStatus,
        "messaging-service-routing" => WebhookFamily::MessagingServiceRouting,
        value => panic!("unmapped webhook family {value}"),
    }
}

fn form_signature(token: &str, url: &str, pairs: &[(String, String)]) -> String {
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

fn url_signature(token: &str, url: &str) -> String {
    let mut mac = Hmac::<Sha1>::new_from_slice(token.as_bytes()).unwrap();
    mac.update(url.as_bytes());
    STANDARD.encode(mac.finalize().into_bytes())
}

fn row_fields(row: &toml::Value) -> Vec<&str> {
    row["known_fields"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap())
        .collect()
}

#[test]
fn manifest_known_fields_match_typed_family_fields() {
    let manifest = manifest();
    let fixture: serde_json::Value =
        serde_json::from_str(include_str!("../fixtures/webhooks/webhook-matrix.json")).unwrap();
    let fixture_families = fixture["families"].as_array().unwrap();
    for row in manifest["families"].as_array().unwrap() {
        let name = row["family"].as_str().unwrap();
        assert!(fixture_families.iter().any(|value| value == name));
        if name == "json-verification" {
            continue;
        }
        let mut actual = family(name)
            .known_fields()
            .iter()
            .map(|field| field.as_str())
            .collect::<Vec<_>>();
        if name == "stream-siprec" {
            actual.extend(
                WebhookFamily::Siprec
                    .known_fields()
                    .iter()
                    .map(|field| field.as_str()),
            );
            actual.sort_unstable();
            actual.dedup();
        }
        let mut expected = row_fields(row);
        expected.sort_unstable();
        actual.sort_unstable();
        assert_eq!(actual, expected, "typed field mismatch for {name}");
    }
}

#[test]
fn every_webhook_family_has_signed_typed_forward_compatible_evidence() {
    let token = "synthetic-matrix-token";
    for row in manifest()["families"].as_array().unwrap() {
        let name = row["family"].as_str().unwrap();
        if name == "json-verification" {
            continue;
        }
        let family = family(name);
        let url = format!("https://example.test/hooks/{name}");
        let mut pairs = family
            .known_fields()
            .iter()
            .map(|field| (field.as_str().to_owned(), "introduced-after-pin".to_owned()))
            .collect::<Vec<_>>();
        pairs.push(("FutureParameter".into(), "future-value".into()));
        let verified = WebhookValidator::new(token)
            .verify_form(family, &url, &pairs, &form_signature(token, &url, &pairs))
            .unwrap();
        let event = verified.parse_typed();
        assert_eq!(event.family(), family);
        assert_eq!(event.known().len(), family.known_fields().len());
        assert_eq!(event.extras().len(), 1);
        assert_eq!(event.extras()[0].name(), "FutureParameter");
        assert!(!format!("{event:?}").contains("future-value"));
    }
}

#[test]
fn every_webhook_family_rejects_an_invalid_signature() {
    for row in manifest()["families"].as_array().unwrap() {
        let name = row["family"].as_str().unwrap();
        if name == "json-verification" {
            continue;
        }
        assert!(
            WebhookValidator::new("synthetic-matrix-token")
                .verify_form(
                    family(name),
                    &format!("https://example.test/hooks/{name}"),
                    &[("FutureParameter".into(), "future-value".into())],
                    "aW52YWxpZA==",
                )
                .is_err()
        );
    }
}

#[test]
fn every_applicable_json_encoding_has_typed_evidence() {
    let token = "synthetic-json-matrix-token";
    for row in manifest()["families"].as_array().unwrap() {
        let encodings = row["encodings"].as_array().unwrap();
        if !encodings
            .iter()
            .any(|value| value.as_str().unwrap().starts_with("json"))
        {
            continue;
        }
        let family = family(row["family"].as_str().unwrap());
        let field = family
            .known_fields()
            .first()
            .map_or("Event", |field| field.as_str());
        let body = serde_json::to_vec(&serde_json::json!({
            field: "introduced-after-pin",
            "FutureProperty": {"nested": true}
        }))
        .unwrap();
        let digest = Sha256::digest(&body)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();
        let url = format!("https://example.test/json?bodySHA256={digest}");
        let verified = WebhookValidator::new(token)
            .verify_json(family, &url, &body, &url_signature(token, &url))
            .unwrap();
        let event = verified.parse_typed();
        assert_eq!(event.family(), family);
        assert_eq!(
            event.extras().get("FutureProperty").unwrap()["nested"],
            true
        );
    }
}
