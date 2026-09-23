use dialkit::{
    InvalidReason, PortCompatibility, ValidationResult, WebhookValidator,
    twiml::{Dial, Gather, MessageNode, TwimlResponse},
};

#[test]
fn existing_wire_and_xml_fixtures_are_frozen_byte_for_byte() {
    assert_eq!(
        include_bytes!("../fixtures/compatibility/calls.json"),
        include_bytes!("../fixtures/calls.json")
    );
    assert_eq!(
        include_bytes!("../fixtures/compatibility/messages.json"),
        include_bytes!("../fixtures/messages.json")
    );
    let voice = TwimlResponse::voice()
        .say("Hello <world> & friends")
        .gather(
            Gather::new()
                .say("Press 1")
                .play("https://example.test/tone?a=1&b=2"),
        )
        .dial(Dial::new("+15005550006"))
        .record()
        .pause(2)
        .hangup()
        .redirect("https://example.test/next")
        .build()
        .unwrap();
    assert_eq!(
        voice.to_xml().unwrap(),
        include_str!("../fixtures/compatibility/voice.xml").trim()
    );
    let messaging = TwimlResponse::messaging()
        .message(
            MessageNode::new()
                .body("Hello & goodbye")
                .media("https://example.test/a.png?x=1&y=2"),
        )
        .redirect("https://example.test/next")
        .build()
        .unwrap();
    assert_eq!(
        messaging.to_xml().unwrap(),
        include_str!("../fixtures/compatibility/messaging.xml").trim()
    );
}

#[test]
fn webhook_result_distinctions_remain_stable() {
    let vector: serde_json::Value =
        serde_json::from_str(include_str!("../fixtures/compatibility/webhook-form.json")).unwrap();
    let params = vector["parameters"]
        .as_array()
        .unwrap()
        .iter()
        .map(|pair| {
            (
                pair[0].as_str().unwrap().to_owned(),
                pair[1].as_str().unwrap().to_owned(),
            )
        })
        .collect::<Vec<_>>();
    let validator = WebhookValidator::new(vector["token"].as_str().unwrap())
        .port_compatibility(PortCompatibility::Strict);
    assert_eq!(
        validator
            .validate_form(
                vector["url"].as_str().unwrap(),
                &params,
                vector["signature"].as_str().unwrap(),
            )
            .unwrap(),
        ValidationResult::Valid
    );
    assert_eq!(
        validator
            .validate_form(
                vector["url"].as_str().unwrap(),
                &params,
                "AAAAAAAAAAAAAAAAAAAAAAAAAAAA",
            )
            .unwrap(),
        ValidationResult::Invalid(InvalidReason::SignatureMismatch)
    );
}
