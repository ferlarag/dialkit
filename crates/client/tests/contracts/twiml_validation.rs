use dialkit::twiml::{
    Connect, Dial, Enqueue, Gather, MessageNode, Pay, Refer, RejectReason, Start, Stop,
    TwimlResponse,
};

#[test]
fn complete_voice_families_render_canonical_ordered_xml() {
    let response = TwimlResponse::voice()
        .connect(
            Connect::new()
                .action("https://example.test/connect")
                .conversation_relay("wss://example.test/relay")
                .room("support")
                .stream("wss://example.test/stream")
                .virtual_agent("assistant"),
        )
        .dial(
            Dial::nouns()
                .action("https://example.test/dial")
                .application("AP00000000000000000000000000000000")
                .client("alice")
                .conference("daily")
                .number("+15005550006")
                .queue("support")
                .sip("sip:alice@example.test"),
        )
        .gather(
            Gather::new()
                .input("dtmf speech")
                .timeout(5)
                .say("Press <1>")
                .play("https://example.test/tone?a=1&b=2")
                .pause(1),
        )
        .pay(
            Pay::new()
                .action("https://example.test/pay")
                .payment_method("card")
                .prompt("Card number")
                .parameter("attempt", "1"),
        )
        .start(Start::new().recording("primary").stream("live"))
        .stop(Stop::new().recording("primary").stream("live"))
        .enqueue(Enqueue::new("support").action("https://example.test/enqueue"))
        .echo()
        .leave()
        .refer(Refer::sip("sip:bob@example.test").action("https://example.test/refer"))
        .reject(RejectReason::Busy)
        .build()
        .unwrap();
    assert_eq!(
        response.to_xml().unwrap(),
        include_str!("../fixtures/twiml/voice/all_nodes.xml").trim()
    );
}

#[test]
fn messaging_repetition_attributes_unicode_and_escaping_are_canonical() {
    let response = TwimlResponse::messaging()
        .message(
            MessageNode::new()
                .action("https://example.test/action")
                .method("POST")
                .status_callback("https://example.test/status")
                .body("Hello ✓ & goodbye")
                .media("https://example.test/one.png?a=1&b=2")
                .media("https://example.test/two.png"),
        )
        .build()
        .unwrap();
    assert_eq!(
        response.to_xml().unwrap(),
        include_str!("../fixtures/twiml/messaging/all_nodes.xml").trim()
    );
}

#[test]
fn invalid_constraints_and_empty_required_nouns_are_rejected_without_values() {
    let cases = [
        TwimlResponse::voice().connect(Connect::new()).build(),
        TwimlResponse::voice().dial(Dial::nouns()).build(),
        TwimlResponse::voice()
            .gather(Gather::new().input("invalid").say("x"))
            .build(),
        TwimlResponse::voice()
            .pay(Pay::new().payment_method("secret-canary"))
            .build(),
        TwimlResponse::voice().start(Start::new()).build(),
    ];
    for result in cases {
        let error = result.unwrap_err();
        let formatted = format!("{error:?} {error}");
        assert!(!formatted.contains("secret-canary"));
    }
}

#[test]
fn debug_output_redacts_text_attributes_and_payment_values() {
    let response = TwimlResponse::voice()
        .pay(
            Pay::new()
                .payment_method("card")
                .prompt("payment-secret-canary"),
        )
        .build()
        .unwrap();
    assert!(!format!("{response:?}").contains("payment-secret-canary"));
}

#[test]
fn dial_record_mode_rejects_unknown_value_without_echoing_it() {
    let error = TwimlResponse::voice()
        .dial(
            Dial::nouns()
                .number("+15005550006")
                .record("private-record-mode"),
        )
        .build()
        .unwrap_err();
    assert!(!format!("{error:?} {error}").contains("private-record-mode"));
    assert!(
        TwimlResponse::voice()
            .dial(
                Dial::nouns()
                    .number("+15005550006")
                    .record("record-from-answer"),
            )
            .build()
            .is_ok()
    );
}

#[test]
fn gather_speech_timeout_accepts_auto_and_rejects_unknown_value() {
    assert!(
        TwimlResponse::voice()
            .gather(Gather::new().say("Press a key").speech_timeout("auto"))
            .build()
            .is_ok()
    );
    let error = TwimlResponse::voice()
        .gather(
            Gather::new()
                .say("Press a key")
                .speech_timeout("private-timeout"),
        )
        .build()
        .unwrap_err();
    assert!(!format!("{error:?} {error}").contains("private-timeout"));
}

#[test]
fn pay_input_is_dtmf_only_while_legacy_payment_mode_still_renders() {
    use dialkit::twiml::{TwimlAttributeName as A, TwimlNode, TwimlNodeKind as N};
    let invalid = TwimlNode::new(N::Pay)
        .attribute(A::Input, "speech")
        .unwrap()
        .child(
            TwimlNode::new(N::Prompt)
                .text("Pay now")
                .attribute(A::For, "payment-card-number")
                .unwrap(),
        )
        .unwrap();
    assert!(TwimlResponse::voice().typed(invalid).build().is_err());
    let valid = TwimlNode::new(N::Pay)
        .attribute(A::PaymentMethod, "credit-card")
        .unwrap()
        .child(
            TwimlNode::new(N::Prompt)
                .text("Pay now")
                .attribute(A::For, "payment-card-number")
                .unwrap(),
        )
        .unwrap();
    assert!(TwimlResponse::voice().typed(valid).build().is_ok());
    assert!(
        TwimlResponse::voice()
            .pay(Pay::new().payment_method("card").prompt("Pay now"))
            .build()
            .is_ok()
    );
}

#[test]
fn gather_empty_finish_key_is_valid_but_multicharacter_key_is_not() {
    use dialkit::twiml::{TwimlAttributeName as A, TwimlNode, TwimlNodeKind as N};
    let valid = TwimlNode::new(N::Gather)
        .attribute(A::FinishOnKey, "")
        .unwrap()
        .child(TwimlNode::new(N::Say).text("Enter a code"))
        .unwrap();
    assert!(TwimlResponse::voice().typed(valid).build().is_ok());
    assert!(
        TwimlNode::new(N::Gather)
            .attribute(A::FinishOnKey, "12")
            .is_err()
    );
}

#[test]
fn recording_callback_events_reject_unknown_tokens() {
    use dialkit::twiml::{TwimlAttributeName as A, TwimlNode, TwimlNodeKind as N};
    let dial = TwimlNode::new(N::Dial)
        .attribute(A::RecordingStatusCallbackEvent, "completed future")
        .unwrap()
        .child(TwimlNode::new(N::Number).text("+15005550006"))
        .unwrap();
    assert!(TwimlResponse::voice().typed(dial).build().is_err());
}

#[test]
fn conference_status_events_use_conference_vocabulary() {
    use dialkit::twiml::{TwimlAttributeName as A, TwimlNode, TwimlNodeKind as N};
    let conference = TwimlNode::new(N::Conference)
        .text("daily")
        .attribute(A::StatusCallbackEvent, "join ringing")
        .unwrap();
    let dial = TwimlNode::new(N::Dial).child(conference).unwrap();
    assert!(TwimlResponse::voice().typed(dial).build().is_err());
}

#[test]
fn conference_record_mode_uses_conference_values() {
    use dialkit::twiml::{TwimlAttributeName as A, TwimlNode, TwimlNodeKind as N};
    let conference = TwimlNode::new(N::Conference)
        .text("daily")
        .attribute(A::Record, "record-from-answer")
        .unwrap();
    let dial = TwimlNode::new(N::Dial).child(conference).unwrap();
    assert!(TwimlResponse::voice().typed(dial).build().is_err());
    let conference = TwimlNode::new(N::Conference)
        .text("daily")
        .attribute(A::Record, "record-from-start")
        .unwrap();
    let dial = TwimlNode::new(N::Dial).child(conference).unwrap();
    assert!(TwimlResponse::voice().typed(dial).build().is_ok());
}

#[test]
fn conversation_relay_requires_a_secure_websocket_url() {
    use dialkit::twiml::{TwimlAttributeName as A, TwimlNode, TwimlNodeKind as N};
    let missing = TwimlNode::new(N::Connect)
        .child(TwimlNode::new(N::ConversationRelay))
        .unwrap();
    assert!(TwimlResponse::voice().typed(missing).build().is_err());
    let insecure = TwimlNode::new(N::Connect)
        .child(
            TwimlNode::new(N::ConversationRelay)
                .attribute(A::Url, "https://example.test/relay")
                .unwrap(),
        )
        .unwrap();
    assert!(TwimlResponse::voice().typed(insecure).build().is_err());
    assert!(
        TwimlResponse::voice()
            .connect(Connect::new().conversation_relay("https://example.test/relay"))
            .build()
            .is_err()
    );
}

#[test]
fn virtual_agent_requires_a_connector_name() {
    use dialkit::twiml::{TwimlAttributeName as A, TwimlNode, TwimlNodeKind as N};
    let missing = TwimlNode::new(N::Connect)
        .child(TwimlNode::new(N::VirtualAgent))
        .unwrap();
    assert!(TwimlResponse::voice().typed(missing).build().is_err());
    assert!(
        TwimlNode::new(N::VirtualAgent)
            .attribute(A::ConnectorName, "")
            .is_err()
    );
    let configured = TwimlNode::new(N::Connect)
        .child(
            TwimlNode::new(N::VirtualAgent)
                .attribute(A::ConnectorName, "configured-connector")
                .unwrap(),
        )
        .unwrap();
    assert!(TwimlResponse::voice().typed(configured).build().is_ok());
}

#[test]
fn new_start_stream_builder_requires_a_secure_websocket_url() {
    use url::Url;
    let response = TwimlResponse::voice()
        .start(Start::new().stream_to("live", Url::parse("wss://example.invalid/audio").unwrap()))
        .build()
        .unwrap();
    assert!(
        response
            .to_xml()
            .unwrap()
            .contains("url=\"wss://example.invalid/audio\"")
    );
    assert!(
        TwimlResponse::voice()
            .start(
                Start::new()
                    .stream_to("live", Url::parse("https://example.invalid/audio").unwrap(),)
            )
            .build()
            .is_err()
    );
}

#[test]
fn new_start_siprec_builder_requires_a_connector_name() {
    let response = TwimlResponse::voice()
        .start(Start::new().siprec_with_connector("session", "synthetic-connector"))
        .build()
        .unwrap();
    assert!(
        response
            .to_xml()
            .unwrap()
            .contains("connectorName=\"synthetic-connector\"")
    );
    assert!(
        TwimlResponse::voice()
            .start(Start::new().siprec_with_connector("session", ""))
            .build()
            .is_err()
    );
}

#[test]
fn typed_node_start_requires_documented_stream_and_siprec_inputs() {
    use dialkit::twiml::{TwimlAttributeName as A, TwimlNode, TwimlNodeKind as N};
    let invalid_stream = TwimlNode::new(N::Start)
        .child(
            TwimlNode::new(N::Stream)
                .attribute(A::Name, "live")
                .unwrap(),
        )
        .unwrap();
    assert!(
        TwimlResponse::voice()
            .typed(invalid_stream)
            .build()
            .is_err()
    );
    let invalid_siprec = TwimlNode::new(N::Start)
        .child(
            TwimlNode::new(N::Siprec)
                .attribute(A::Name, "session")
                .unwrap(),
        )
        .unwrap();
    assert!(
        TwimlResponse::voice()
            .typed(invalid_siprec)
            .build()
            .is_err()
    );
}

#[test]
fn gather_auto_timeout_is_valid_but_dial_timeout_remains_numeric() {
    use dialkit::twiml::{TwimlAttributeName as A, TwimlNode, TwimlNodeKind as N};
    assert!(
        TwimlResponse::voice()
            .gather(Gather::new().timeout_auto().say("Speak"))
            .build()
            .is_ok()
    );
    let gather = TwimlNode::new(N::Gather)
        .attribute(A::Timeout, "auto")
        .unwrap()
        .child(TwimlNode::new(N::Say).text("Speak"))
        .unwrap();
    assert!(TwimlResponse::voice().typed(gather).build().is_ok());
    assert!(
        TwimlNode::new(N::Dial)
            .attribute(A::Timeout, "auto")
            .is_err()
    );
}

#[test]
fn prompt_payment_step_and_attempt_are_constrained_without_breaking_legacy_prompt() {
    use dialkit::twiml::{TwimlAttributeName as A, TwimlNode, TwimlNodeKind as N};
    let prompt = TwimlNode::new(N::Prompt)
        .text("Enter card number")
        .attribute(A::For, "payment-card-number")
        .unwrap()
        .attribute(A::Attempt, "2 3")
        .unwrap();
    let pay = TwimlNode::new(N::Pay).child(prompt).unwrap();
    assert!(TwimlResponse::voice().typed(pay).build().is_ok());
    assert!(
        TwimlNode::new(N::Prompt)
            .attribute(A::For, "private-step")
            .is_err()
    );
    let missing_step = TwimlNode::new(N::Prompt).text("Enter card number");
    assert!(
        TwimlResponse::voice()
            .typed(TwimlNode::new(N::Pay).child(missing_step).unwrap())
            .build()
            .is_err()
    );
    assert!(
        TwimlNode::new(N::Prompt)
            .attribute(A::Attempt, "11")
            .is_err()
    );
    assert!(
        TwimlResponse::voice()
            .pay(Pay::new().prompt_for("payment-card-number", "Enter card number"))
            .build()
            .is_ok()
    );
}

#[test]
fn play_digits_are_valid_without_url_but_not_inside_gather() {
    use dialkit::twiml::{TwimlAttributeName as A, TwimlNode, TwimlNodeKind as N};
    let standalone = TwimlNode::new(N::Play)
        .attribute(A::Digits, "12w#")
        .unwrap();
    assert!(TwimlResponse::voice().typed(standalone).build().is_ok());
    assert!(TwimlNode::new(N::Play).attribute(A::Digits, "12x").is_err());
    let nested = TwimlNode::new(N::Gather)
        .child(TwimlNode::new(N::Play).attribute(A::Digits, "12").unwrap())
        .unwrap();
    assert!(TwimlResponse::voice().typed(nested).build().is_err());
}

#[test]
fn gather_hints_reject_oversized_entry_and_too_many_entries() {
    use dialkit::twiml::{TwimlAttributeName as A, TwimlNode, TwimlNodeKind as N};
    assert!(
        TwimlNode::new(N::Gather)
            .attribute(A::Hints, "item, another phrase")
            .is_ok()
    );
    assert!(
        TwimlNode::new(N::Gather)
            .attribute(A::Hints, "x".repeat(101))
            .is_err()
    );
    assert!(
        TwimlNode::new(N::Gather)
            .attribute(A::Hints, vec!["x"; 501].join(","))
            .is_err()
    );
}

#[test]
fn record_timeout_zero_and_multiple_finish_keys_are_valid() {
    use dialkit::twiml::{TwimlAttributeName as A, TwimlNode, TwimlNodeKind as N};
    let record = TwimlNode::new(N::Record)
        .attribute(A::Timeout, "0")
        .unwrap()
        .attribute(A::FinishOnKey, "12#*")
        .unwrap();
    assert!(TwimlResponse::voice().typed(record).build().is_ok());
    assert!(
        TwimlNode::new(N::Record)
            .attribute(A::FinishOnKey, "12x")
            .is_err()
    );
    assert!(
        TwimlNode::new(N::Gather)
            .attribute(A::FinishOnKey, "12")
            .is_err()
    );
}

#[test]
fn typed_nodes_reject_duplicate_or_wrongly_scoped_attributes() {
    use dialkit::twiml::{TwimlAttributeName as A, TwimlNode, TwimlNodeKind as N};
    assert!(
        TwimlNode::new(N::Gather)
            .attribute(A::Input, "dtmf")
            .unwrap()
            .attribute(A::Input, "speech")
            .is_err()
    );
    for (kind, attribute) in [
        (N::Conference, A::Url),
        (N::Number, A::Record),
        (N::Siprec, A::Url),
        (N::Recording, A::Track),
        (N::Transcription, A::Track),
    ] {
        assert!(!kind.supports_attribute(attribute));
        assert!(TwimlNode::new(kind).attribute(attribute, "x").is_err());
    }
}

#[test]
fn payment_action_requires_https_and_post() {
    use dialkit::twiml::{TwimlAttributeName as A, TwimlNode, TwimlNodeKind as N};
    let prompt = || {
        TwimlNode::new(N::Prompt)
            .text("Enter card number")
            .attribute(A::For, "payment-card-number")
            .unwrap()
    };
    for url in ["http://example.invalid/pay", "wss://example.invalid/pay"] {
        let pay = TwimlNode::new(N::Pay)
            .attribute(A::Action, url)
            .unwrap()
            .child(prompt())
            .unwrap();
        assert!(TwimlResponse::voice().typed(pay).build().is_err());
        assert!(
            TwimlResponse::voice()
                .pay(
                    Pay::new()
                        .action(url)
                        .prompt_for("payment-card-number", "Enter card number")
                )
                .build()
                .is_err()
        );
    }
    let get = TwimlNode::new(N::Pay)
        .attribute(A::Method, "GET")
        .unwrap()
        .child(prompt())
        .unwrap();
    assert!(TwimlResponse::voice().typed(get).build().is_err());
    let post = TwimlNode::new(N::Pay)
        .attribute(A::Method, "POST")
        .unwrap()
        .child(prompt())
        .unwrap();
    assert!(TwimlResponse::voice().typed(post).build().is_ok());
}

#[test]
fn conversation_relay_provider_and_interrupt_options_use_documented_values() {
    use dialkit::twiml::{TwimlAttributeName as A, TwimlNode, TwimlNodeKind as N};
    for value in ["none", "dtmf", "speech", "any", "true", "false"] {
        assert!(
            TwimlNode::new(N::ConversationRelay)
                .attribute(A::Interruptible, value)
                .is_ok()
        );
    }
    assert!(
        TwimlNode::new(N::ConversationRelay)
            .attribute(A::Interruptible, "private-invalid-mode")
            .is_err()
    );
    for value in ["Google", "Amazon", "ElevenLabs", "google"] {
        assert!(
            TwimlNode::new(N::ConversationRelay)
                .attribute(A::TtsProvider, value)
                .is_ok()
        );
    }
    for value in ["Google", "Deepgram", "deepgram"] {
        assert!(
            TwimlNode::new(N::ConversationRelay)
                .attribute(A::TranscriptionProvider, value)
                .is_ok()
        );
    }
    assert!(
        TwimlNode::new(N::ConversationRelay)
            .attribute(A::TtsProvider, "private-provider")
            .is_err()
    );
    assert!(
        TwimlNode::new(N::ConversationRelay)
            .attribute(A::TranscriptionProvider, "private-provider")
            .is_err()
    );
}
