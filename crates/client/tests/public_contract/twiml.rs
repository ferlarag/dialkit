use dialkit::twiml::{Dial, Gather, MessageNode, TwimlResponse};
use proptest::prelude::*;
use quick_xml::{Reader, events::Event};

#[test]
fn renders_voice_nodes_in_order_with_escaping() {
    let response = TwimlResponse::voice()
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
        response.to_xml().unwrap(),
        include_str!("../fixtures/twiml/voice.xml").trim()
    );
}

proptest! {
    #[test]
    fn arbitrary_nonempty_text_always_renders_as_well_formed_xml(suffix in ".{0,127}") {
        let response = TwimlResponse::voice().say(format!("x{suffix}")).build().unwrap();
        let xml = response.to_xml().unwrap();
        let mut reader = Reader::from_str(&xml);
        loop {
            match reader.read_event() {
                Ok(Event::Eof) => break,
                Ok(_) => {}
                Err(error) => prop_assert!(false, "invalid XML: {error}"),
            }
        }
    }
}

#[test]
fn renders_messaging_nodes_and_rejects_missing_content() {
    let response = TwimlResponse::messaging()
        .message(
            MessageNode::new()
                .body("Hello & goodbye")
                .media("https://example.test/a.png?x=1&y=2"),
        )
        .redirect("https://example.test/next")
        .build()
        .unwrap();
    assert_eq!(
        response.to_xml().unwrap(),
        include_str!("../fixtures/twiml/messaging.xml").trim()
    );
    assert!(
        TwimlResponse::messaging()
            .message(MessageNode::new())
            .build()
            .is_err()
    );
}

#[test]
fn rejects_missing_or_invalid_values_for_initial_nodes() {
    assert!(TwimlResponse::voice().say("").build().is_err());
    assert!(TwimlResponse::voice().play("").build().is_err());
    assert!(
        TwimlResponse::voice()
            .play("ftp://example.test/a")
            .build()
            .is_err()
    );
    assert!(
        TwimlResponse::voice()
            .gather(Gather::new())
            .build()
            .is_err()
    );
    assert!(
        TwimlResponse::voice()
            .dial(Dial::new("   "))
            .build()
            .is_err()
    );
    assert!(TwimlResponse::voice().pause(0).build().is_err());
    assert!(TwimlResponse::voice().pause(61).build().is_err());
    assert!(
        TwimlResponse::voice()
            .redirect("relative/path")
            .build()
            .is_err()
    );
    assert!(
        TwimlResponse::messaging()
            .message(MessageNode::new().body(""))
            .build()
            .is_err()
    );
    assert!(
        TwimlResponse::messaging()
            .message(MessageNode::new().media("not-a-url"))
            .build()
            .is_err()
    );
    assert!(TwimlResponse::messaging().redirect("").build().is_err());
}
