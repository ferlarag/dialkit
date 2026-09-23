use dialkit::twiml::{TwimlAttributeName as A, TwimlNode, TwimlNodeKind as N, TwimlResponse};

fn manifest() -> toml::Value {
    toml::from_str(include_str!(
        "../../../../codegen/coverage/twiml-nodes.toml"
    ))
    .unwrap()
}

fn kind(family: &str, name: &str, parent: &str) -> N {
    match (family, name, parent) {
        ("voice", "Connect", _) => N::Connect,
        ("voice", "ConversationRelay", _) => N::ConversationRelay,
        ("voice", "Room", _) => N::Room,
        ("voice", "Stream", "Connect") => N::ConnectStream,
        ("voice", "Stream", _) => N::Stream,
        ("voice", "VirtualAgent", _) => N::VirtualAgent,
        ("voice", "Dial", _) => N::Dial,
        ("voice", "Application", _) => N::Application,
        ("voice", "Client", _) => N::Client,
        ("voice", "Conference", _) => N::Conference,
        ("voice", "Number", _) => N::Number,
        ("voice", "Queue", _) => N::Queue,
        ("voice", "Sip", _) => N::Sip,
        ("voice", "Gather", _) => N::Gather,
        ("voice", "Say", _) => N::Say,
        ("voice", "Play", _) => N::Play,
        ("voice", "Pause", _) => N::Pause,
        ("voice", "Pay", _) => N::Pay,
        ("voice", "Prompt", _) => N::Prompt,
        ("voice", "Parameter", _) => N::Parameter,
        ("voice", "Start", _) => N::Start,
        ("voice", "Stop", _) => N::Stop,
        ("voice", "Recording", _) => N::Recording,
        ("voice", "Siprec", _) => N::Siprec,
        ("voice", "Transcription", _) => N::Transcription,
        ("voice", "Enqueue", _) => N::Enqueue,
        ("voice", "Echo", _) => N::Echo,
        ("voice", "Hangup", _) => N::Hangup,
        ("voice", "Leave", _) => N::Leave,
        ("voice", "Record", _) => N::Record,
        ("voice", "Redirect", _) => N::VoiceRedirect,
        ("voice", "Refer", _) => N::Refer,
        ("voice", "Reject", _) => N::Reject,
        ("messaging", "Message", _) => N::Message,
        ("messaging", "Redirect", _) => N::MessagingRedirect,
        ("messaging", "Body", _) => N::Body,
        ("messaging", "Media", _) => N::Media,
        key => panic!("unmapped TwiML node {key:?}"),
    }
}

fn attribute(name: &str) -> A {
    [
        A::Action,
        A::Method,
        A::Url,
        A::StatusCallback,
        A::StatusCallbackMethod,
        A::StatusCallbackEvent,
        A::Name,
        A::Track,
        A::ConnectorName,
        A::WelcomeGreeting,
        A::Language,
        A::Voice,
        A::TtsProvider,
        A::TranscriptionProvider,
        A::SpeechModel,
        A::Interruptible,
        A::DtmfDetection,
        A::Timeout,
        A::TimeLimit,
        A::HangupOnStar,
        A::CallerId,
        A::Record,
        A::RecordingStatusCallback,
        A::RecordingStatusCallbackMethod,
        A::RecordingStatusCallbackEvent,
        A::AnswerOnBridge,
        A::RingTone,
        A::Trim,
        A::Input,
        A::SpeechTimeout,
        A::MaxSpeechTime,
        A::ProfanityFilter,
        A::FinishOnKey,
        A::NumDigits,
        A::PartialResultCallback,
        A::PartialResultCallbackMethod,
        A::Hints,
        A::BargeIn,
        A::ActionOnEmptyResult,
        A::Loop,
        A::Digits,
        A::Length,
        A::PaymentMethod,
        A::PaymentConnector,
        A::ChargeAmount,
        A::Currency,
        A::Description,
        A::MaxAttempts,
        A::SecurityCode,
        A::PostalCode,
        A::TokenType,
        A::WaitUrl,
        A::WaitUrlMethod,
        A::WorkflowSid,
        A::PlayBeep,
        A::MaxLength,
        A::Reason,
        A::ContentType,
        A::CustomParameters,
        A::Value,
        A::For,
        A::Attempt,
    ]
    .into_iter()
    .find(|attribute| attribute.as_str() == name)
    .unwrap_or_else(|| panic!("unmapped TwiML attribute {name}"))
}

#[test]
fn every_typed_node_accepts_exactly_its_pinned_attribute_set() {
    let manifest = manifest();
    let rows = manifest["nodes"].as_array().unwrap();
    let all_names = rows
        .iter()
        .flat_map(|row| row["attributes"].as_array().unwrap())
        .filter_map(toml::Value::as_str)
        .chain(["customParameters"])
        .collect::<std::collections::BTreeSet<_>>();
    for row in rows {
        let family = row["family"].as_str().unwrap();
        let name = row["name"].as_str().unwrap();
        let parent = row["parent"].as_str().unwrap();
        let kind = kind(family, name, parent);
        let declared = row["attributes"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(toml::Value::as_str)
            .collect::<std::collections::BTreeSet<_>>();
        for attribute_name in &all_names {
            assert_eq!(
                kind.supports_attribute(attribute(attribute_name)),
                declared.contains(attribute_name),
                "{family}:{parent}/{name} attribute {attribute_name} disagrees with the pinned manifest"
            );
        }
    }
}

fn parent_kind(parent: &str) -> Option<N> {
    match parent {
        "Response" => None,
        "Connect" => Some(N::Connect),
        "Dial" => Some(N::Dial),
        "Gather" | "Response|Gather" => Some(N::Gather),
        "Pay" => Some(N::Pay),
        "Message" => Some(N::Message),
        "Start|Stop" => Some(N::Start),
        "Pay|Stream|VirtualAgent|Client" => Some(N::Stream),
        value => panic!("unmapped parent context {value}"),
    }
}

fn attribute_value(attribute: A) -> &'static str {
    match attribute {
        A::Action
        | A::Url
        | A::StatusCallback
        | A::RecordingStatusCallback
        | A::PartialResultCallback
        | A::WaitUrl => "https://example.invalid/callback?a=1&b=2",
        A::Method
        | A::StatusCallbackMethod
        | A::RecordingStatusCallbackMethod
        | A::PartialResultCallbackMethod
        | A::WaitUrlMethod => "POST",
        A::Timeout
        | A::TimeLimit
        | A::MaxSpeechTime
        | A::NumDigits
        | A::Loop
        | A::Length
        | A::MaxAttempts
        | A::MaxLength => "2",
        A::SpeechTimeout => "auto",
        A::HangupOnStar
        | A::AnswerOnBridge
        | A::ProfanityFilter
        | A::BargeIn
        | A::ActionOnEmptyResult
        | A::PlayBeep
        | A::DtmfDetection
        | A::SecurityCode => "true",
        A::Track => "inbound_track",
        A::Interruptible => "any",
        A::TtsProvider => "Google",
        A::TranscriptionProvider => "Deepgram",
        A::Trim => "trim-silence",
        A::Record => "record-from-answer",
        A::TokenType => "reusable",
        A::FinishOnKey => "#",
        A::Digits => "12w#",
        A::ChargeAmount => "12.34",
        A::For => "payment-card-number",
        A::Attempt => "2",
        A::StatusCallbackEvent | A::RecordingStatusCallbackEvent => "completed",
        A::Input => "dtmf speech",
        A::PaymentMethod => "card",
        A::Reason => "busy",
        _ => "synthetic & value",
    }
}

fn minimal(kind: N) -> TwimlNode {
    let node = TwimlNode::new(kind);
    match kind {
        N::Say
        | N::Body
        | N::Enqueue
        | N::Room
        | N::Application
        | N::Client
        | N::Conference
        | N::Number
        | N::Queue
        | N::Sip
        | N::Dial => node.text("synthetic content"),
        N::Prompt => node
            .text("synthetic content")
            .attribute(A::For, "payment-card-number")
            .unwrap(),
        N::Play | N::Media | N::VoiceRedirect | N::MessagingRedirect => {
            node.text("https://example.invalid/content?a=1&b=2")
        }
        N::Pause => node.attribute(A::Length, "1").unwrap(),
        N::Parameter => node
            .attribute(A::Name, "synthetic-name")
            .unwrap()
            .attribute(A::Value, "synthetic-value")
            .unwrap(),
        N::ConnectStream => node
            .attribute(A::Url, "wss://example.invalid/audio")
            .unwrap(),
        N::ConversationRelay => node
            .attribute(A::Url, "wss://example.invalid/relay")
            .unwrap(),
        N::VirtualAgent => node
            .attribute(A::ConnectorName, "synthetic-connector")
            .unwrap(),
        N::Stream => node
            .attribute(A::Name, "synthetic-stream")
            .unwrap()
            .attribute(A::Url, "wss://example.invalid/audio")
            .unwrap(),
        N::Siprec => node
            .attribute(A::Name, "synthetic-siprec")
            .unwrap()
            .attribute(A::ConnectorName, "synthetic-connector")
            .unwrap(),
        N::Recording | N::Transcription => node.attribute(A::Name, "synthetic-name").unwrap(),
        N::Connect => node.child(minimal(N::Room)).unwrap(),
        N::Gather => node.child(minimal(N::Say)).unwrap(),
        N::Pay => node.child(minimal(N::Prompt)).unwrap(),
        N::Start | N::Stop => node.child(minimal(N::Recording)).unwrap(),
        N::Refer => node.child(minimal(N::Sip)).unwrap(),
        N::Message => node.child(minimal(N::Body)).unwrap(),
        _ => node,
    }
}

fn render_root(family: &str, node: TwimlNode) -> String {
    let response = match family {
        "voice" => TwimlResponse::voice().typed(node).build(),
        "messaging" => TwimlResponse::messaging().typed(node).build(),
        value => panic!("unknown family {value}"),
    }
    .unwrap();
    response.to_xml().unwrap()
}

fn wrap_for_parent(family: &str, parent: &str, node: TwimlNode) -> TwimlNode {
    match parent {
        "Response" | "Response|Gather" => node,
        "Connect" => TwimlNode::new(N::Connect).child(node).unwrap(),
        "Dial" => TwimlNode::new(N::Dial).child(node).unwrap(),
        "Gather" => TwimlNode::new(N::Gather).child(node).unwrap(),
        "Pay" | "Pay|Stream|VirtualAgent|Client" => TwimlNode::new(N::Pay).child(node).unwrap(),
        "Message" => TwimlNode::new(N::Message).child(node).unwrap(),
        "Start|Stop" => TwimlNode::new(N::Start).child(node).unwrap(),
        value => panic!("unmapped parent {family}:{value}"),
    }
}

pub(super) fn verify_manifest_row(family: &str, name: &str, parent: &str) {
    let manifest = manifest();
    let row = manifest["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| {
            row["family"].as_str() == Some(family)
                && row["name"].as_str() == Some(name)
                && row["parent"].as_str() == Some(parent)
        })
        .unwrap_or_else(|| panic!("missing manifest row {family}:{parent}/{name}"));
    let node_kind = kind(family, name, parent);
    let mut node = TwimlNode::new(node_kind);
    for value in row["attributes"].as_array().unwrap() {
        let attribute = attribute(value.as_str().unwrap());
        let value = match (node_kind, attribute) {
            (N::Pay, A::Input) => "dtmf",
            (N::Conference, A::StatusCallbackEvent) => "join",
            (N::ConnectStream | N::Stream | N::ConversationRelay, A::Url) => {
                "wss://example.invalid/audio"
            }
            (N::Conference, A::Record) => "record-from-start",
            _ => attribute_value(attribute),
        };
        node = node.attribute(attribute, value).unwrap();
    }
    if row["children"].as_array().unwrap().is_empty() {
        if node_kind != N::Play {
            node = minimal_content(node_kind, node);
        }
    } else {
        for value in row["children"].as_array().unwrap() {
            node = node
                .child(minimal(kind(family, value.as_str().unwrap(), name)))
                .unwrap();
        }
        let repeated = row["children"][0].as_str().unwrap();
        node = node.child(minimal(kind(family, repeated, name))).unwrap();
    }
    let roots = match parent {
        "Response|Gather" if node_kind == N::Play => vec![
            node.clone(),
            TwimlNode::new(N::Gather).child(minimal(N::Play)).unwrap(),
        ],
        "Response|Gather" => vec![
            node.clone(),
            TwimlNode::new(N::Gather).child(node.clone()).unwrap(),
        ],
        "Start|Stop" => vec![
            TwimlNode::new(N::Start).child(node.clone()).unwrap(),
            TwimlNode::new(N::Stop).child(node.clone()).unwrap(),
        ],
        "Pay|Stream|VirtualAgent|Client" => vec![
            TwimlNode::new(N::Pay).child(node.clone()).unwrap(),
            TwimlNode::new(N::Start)
                .child(minimal(N::Stream).child(node.clone()).unwrap())
                .unwrap(),
            TwimlNode::new(N::Connect)
                .child(minimal(N::VirtualAgent).child(node.clone()).unwrap())
                .unwrap(),
            TwimlNode::new(N::Dial)
                .child(TwimlNode::new(N::Client).child(node.clone()).unwrap())
                .unwrap(),
        ],
        _ => vec![wrap_for_parent(family, parent, node)],
    };
    for root in roots {
        let xml = render_root(family, root);
        assert!(xml.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?><Response>"));
        assert!(xml.contains(&format!("<{name}")));
        if node_kind == N::Play && xml.contains("<Gather") {
            // <Play digits> is legal at Response but forbidden inside Gather.
            continue;
        }
        for value in row["attributes"].as_array().unwrap() {
            assert!(xml.contains(&format!(" {}=\"", value.as_str().unwrap())));
        }
        if row["attributes"]
            .as_array()
            .unwrap()
            .iter()
            .any(|value| attribute_value(attribute(value.as_str().unwrap())).contains('&'))
        {
            assert!(xml.contains("&amp;"));
        }
    }
}

pub(super) fn verify_invalid_row(family: &str, name: &str, parent: &str) {
    let manifest = manifest();
    let row = manifest["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| {
            row["family"].as_str() == Some(family)
                && row["name"].as_str() == Some(name)
                && row["parent"].as_str() == Some(parent)
        })
        .unwrap();
    let node = kind(family, name, parent);
    for value in row["attributes"].as_array().unwrap() {
        let attribute = attribute(value.as_str().unwrap());
        assert!(
            TwimlNode::new(node).attribute(attribute, " ").is_err(),
            "{family}:{parent}/{name}: empty {}",
            attribute.as_str()
        );
        if matches!(
            attribute,
            A::Method
                | A::StatusCallbackMethod
                | A::RecordingStatusCallbackMethod
                | A::PartialResultCallbackMethod
                | A::WaitUrlMethod
        ) {
            assert!(
                TwimlNode::new(node).attribute(attribute, "PATCH").is_err(),
                "{family}:{parent}/{name}: invalid method"
            );
        }
        if !(node == N::Record && attribute == A::Timeout)
            && matches!(
                attribute,
                A::Timeout
                    | A::TimeLimit
                    | A::MaxSpeechTime
                    | A::NumDigits
                    | A::Length
                    | A::MaxAttempts
                    | A::MaxLength
            )
        {
            assert!(
                TwimlNode::new(node).attribute(attribute, "0").is_err(),
                "{family}:{parent}/{name}: invalid range"
            );
        }
        if attribute == A::Loop {
            assert!(
                TwimlNode::new(node)
                    .attribute(attribute, "not-a-number")
                    .is_err(),
                "{family}:{parent}/{name}: invalid loop"
            );
            assert!(TwimlNode::new(node).attribute(attribute, "0").is_ok());
        }
        if matches!(
            attribute,
            A::Action
                | A::Url
                | A::StatusCallback
                | A::RecordingStatusCallback
                | A::PartialResultCallback
                | A::WaitUrl
        ) {
            assert!(
                TwimlNode::new(node)
                    .attribute(attribute, "relative/path")
                    .is_err(),
                "{family}:{parent}/{name}: invalid URL"
            );
        }
    }
    let forbidden_attribute = [A::Action, A::PaymentMethod, A::ContentType, A::Length]
        .into_iter()
        .find(|attribute| !node.supports_attribute(*attribute));
    if let Some(attribute) = forbidden_attribute {
        assert!(
            TwimlNode::new(node)
                .attribute(attribute, "synthetic")
                .is_err(),
            "{family}:{parent}/{name}: forbidden attribute"
        );
    }
    let forbidden_child = [N::Message, N::Hangup]
        .into_iter()
        .find(|child| !node.supports_child(*child))
        .unwrap();
    assert!(
        TwimlNode::new(node)
            .child(TwimlNode::new(forbidden_child))
            .is_err(),
        "{family}:{parent}/{name}: forbidden child"
    );
    if node == N::Gather {
        let invalid = TwimlNode::new(node)
            .attribute(A::Input, "fax")
            .unwrap()
            .child(minimal(N::Say))
            .unwrap();
        assert!(TwimlResponse::voice().typed(invalid).build().is_err());
    }
    if node == N::Pay {
        let invalid = TwimlNode::new(node)
            .attribute(A::PaymentMethod, "crypto")
            .unwrap()
            .child(minimal(N::Prompt))
            .unwrap();
        assert!(TwimlResponse::voice().typed(invalid).build().is_err());
    }
    if node == N::Reject {
        let invalid = TwimlNode::new(node)
            .attribute(A::Reason, "unavailable")
            .unwrap();
        assert!(render_invalid_root(family, invalid));
    }
    if matches!(
        node,
        N::Connect
            | N::Start
            | N::Stop
            | N::Pay
            | N::Refer
            | N::Dial
            | N::Play
            | N::Media
            | N::VoiceRedirect
            | N::MessagingRedirect
            | N::Parameter
    ) {
        let invalid = wrap_for_parent(family, parent, TwimlNode::new(node));
        assert!(
            render_invalid_root(family, invalid),
            "{family}:{parent}/{name}: missing required content"
        );
    }
    if node == N::Pause {
        let invalid = TwimlNode::new(node).attribute(A::Length, "61").unwrap();
        assert!(render_invalid_root(family, invalid));
    }
    if !row["children"].as_array().unwrap().is_empty() {
        let child_name = row["children"][0].as_str().unwrap();
        let child = minimal(kind(family, child_name, name));
        let mixed = TwimlNode::new(node).text("synthetic").child(child).unwrap();
        let root = wrap_for_parent(family, parent, mixed);
        assert!(
            render_invalid_root(family, root),
            "{family}:{parent}/{name}: mixed text and child"
        );
    }
}

fn render_invalid_root(family: &str, root: TwimlNode) -> bool {
    let result = if family == "voice" {
        TwimlResponse::voice().typed(root).build()
    } else {
        TwimlResponse::messaging().typed(root).build()
    };
    result.is_err()
}

fn minimal_content(kind: N, node: TwimlNode) -> TwimlNode {
    match kind {
        N::Say
        | N::Body
        | N::Enqueue
        | N::Room
        | N::Application
        | N::Client
        | N::Conference
        | N::Number
        | N::Queue
        | N::Sip
        | N::Prompt
        | N::Dial => node.text("synthetic content"),
        N::Play | N::Media | N::VoiceRedirect | N::MessagingRedirect => {
            node.text("https://example.invalid/content?a=1&b=2")
        }
        _ => node,
    }
}

#[test]
fn twiml_manifest_entries_have_executable_evidence() {
    let manifest = manifest();
    for row in manifest["nodes"].as_array().unwrap() {
        let family = row["family"].as_str().unwrap();
        let name = row["name"].as_str().unwrap();
        let parent = row["parent"].as_str().unwrap();
        let node = kind(family, name, parent);
        assert_eq!(node.xml_name(), name);
        for value in row["attributes"].as_array().unwrap() {
            let attribute = attribute(value.as_str().unwrap());
            assert!(
                node.supports_attribute(attribute),
                "{family}:{parent}/{name} lacks {}",
                attribute.as_str()
            );
        }
        if let Some(parent) = parent_kind(parent) {
            assert!(
                parent.supports_child(node),
                "illegal manifest edge {parent:?} -> {node:?}"
            );
        }
        for value in row["children"].as_array().unwrap() {
            let child_name = value.as_str().unwrap();
            let child = kind(family, child_name, name);
            assert!(
                node.supports_child(child),
                "missing manifest edge {node:?} -> {child:?}"
            );
        }
    }
}

#[test]
fn every_twiml_node_rejects_an_unsupported_attribute_or_child() {
    let manifest = manifest();
    for row in manifest["nodes"].as_array().unwrap() {
        let node = kind(
            row["family"].as_str().unwrap(),
            row["name"].as_str().unwrap(),
            row["parent"].as_str().unwrap(),
        );
        let unsupported = [A::Action, A::PaymentMethod, A::ContentType, A::Length]
            .into_iter()
            .find(|attribute| !node.supports_attribute(*attribute));
        if let Some(attribute) = unsupported {
            assert!(
                TwimlNode::new(node)
                    .attribute(attribute, "synthetic")
                    .is_err()
            );
        }
        if !node.supports_child(N::Message) {
            assert!(
                TwimlNode::new(node)
                    .child(TwimlNode::new(N::Message))
                    .is_err()
            );
        }
    }
}

#[test]
fn terminal_control_flow_is_explicit_without_breaking_legacy_rendering() {
    assert!(N::Hangup.is_terminal());
    assert!(N::Reject.is_terminal());
    assert!(N::VoiceRedirect.is_terminal());
    assert!(N::MessagingRedirect.is_terminal());
    assert!(!N::Say.is_terminal());
}

#[test]
fn documented_constraints_and_content_exclusion_are_executable() {
    let invalid_input = TwimlNode::new(N::Gather)
        .attribute(A::Input, "fax")
        .unwrap()
        .child(minimal(N::Say))
        .unwrap();
    assert!(TwimlResponse::voice().typed(invalid_input).build().is_err());
    let invalid_payment = TwimlNode::new(N::Pay)
        .attribute(A::PaymentMethod, "crypto")
        .unwrap()
        .child(minimal(N::Prompt))
        .unwrap();
    assert!(
        TwimlResponse::voice()
            .typed(invalid_payment)
            .build()
            .is_err()
    );
    assert!(TwimlNode::new(N::Pause).attribute(A::Length, "0").is_err());
    assert!(TwimlNode::new(N::Say).attribute(A::Method, "POST").is_err());
    let mixed = TwimlNode::new(N::Gather)
        .text("not legal with children")
        .child(minimal(N::Say))
        .unwrap();
    assert!(TwimlResponse::voice().typed(mixed).build().is_err());
}
