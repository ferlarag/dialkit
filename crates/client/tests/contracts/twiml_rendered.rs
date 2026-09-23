// Each manifest row has its own test selector; the shared verifier renders the
// node with every documented attribute, every legal child, and one repeated
// child where content repetition applies.

macro_rules! contract {
    ($test:ident, $family:literal, $name:literal, $parent:literal) => {
        #[test]
        fn $test() {
            super::twiml_manifest::verify_manifest_row($family, $name, $parent);
        }
    };
}

contract!(voice_response_connect, "voice", "Connect", "Response");
contract!(
    voice_connect_conversation_relay,
    "voice",
    "ConversationRelay",
    "Connect"
);
contract!(voice_connect_room, "voice", "Room", "Connect");
contract!(voice_connect_stream, "voice", "Stream", "Connect");
contract!(
    voice_connect_virtual_agent,
    "voice",
    "VirtualAgent",
    "Connect"
);
contract!(voice_response_dial, "voice", "Dial", "Response");
contract!(voice_dial_application, "voice", "Application", "Dial");
contract!(voice_dial_client, "voice", "Client", "Dial");
contract!(voice_dial_conference, "voice", "Conference", "Dial");
contract!(voice_dial_number, "voice", "Number", "Dial");
contract!(voice_dial_queue, "voice", "Queue", "Dial");
contract!(voice_dial_sip, "voice", "Sip", "Dial");
contract!(voice_response_gather, "voice", "Gather", "Response");
contract!(voice_response_gather_say, "voice", "Say", "Response|Gather");
contract!(
    voice_response_gather_play,
    "voice",
    "Play",
    "Response|Gather"
);
contract!(
    voice_response_gather_pause,
    "voice",
    "Pause",
    "Response|Gather"
);
contract!(voice_response_pay, "voice", "Pay", "Response");
contract!(voice_pay_prompt, "voice", "Prompt", "Pay");
contract!(
    voice_parameter_parents,
    "voice",
    "Parameter",
    "Pay|Stream|VirtualAgent|Client"
);
contract!(voice_response_start, "voice", "Start", "Response");
contract!(voice_response_stop, "voice", "Stop", "Response");
contract!(
    voice_start_stop_recording,
    "voice",
    "Recording",
    "Start|Stop"
);
contract!(voice_start_stop_siprec, "voice", "Siprec", "Start|Stop");
contract!(voice_start_stop_stream, "voice", "Stream", "Start|Stop");
contract!(
    voice_start_stop_transcription,
    "voice",
    "Transcription",
    "Start|Stop"
);
contract!(voice_response_enqueue, "voice", "Enqueue", "Response");
contract!(voice_response_echo, "voice", "Echo", "Response");
contract!(voice_response_hangup, "voice", "Hangup", "Response");
contract!(voice_response_leave, "voice", "Leave", "Response");
contract!(voice_response_record, "voice", "Record", "Response");
contract!(voice_response_redirect, "voice", "Redirect", "Response");
contract!(voice_response_refer, "voice", "Refer", "Response");
contract!(voice_response_reject, "voice", "Reject", "Response");
contract!(
    messaging_response_message,
    "messaging",
    "Message",
    "Response"
);
contract!(
    messaging_response_redirect,
    "messaging",
    "Redirect",
    "Response"
);
contract!(messaging_message_body, "messaging", "Body", "Message");
contract!(messaging_message_media, "messaging", "Media", "Message");

macro_rules! invalid_contract {
    ($test:ident, $family:literal, $name:literal, $parent:literal) => {
        #[test]
        fn $test() {
            super::twiml_manifest::verify_invalid_row($family, $name, $parent);
        }
    };
}

invalid_contract!(
    invalid_voice_response_connect,
    "voice",
    "Connect",
    "Response"
);
invalid_contract!(
    invalid_voice_connect_conversation_relay,
    "voice",
    "ConversationRelay",
    "Connect"
);
invalid_contract!(invalid_voice_connect_room, "voice", "Room", "Connect");
invalid_contract!(invalid_voice_connect_stream, "voice", "Stream", "Connect");
invalid_contract!(
    invalid_voice_connect_virtual_agent,
    "voice",
    "VirtualAgent",
    "Connect"
);
invalid_contract!(invalid_voice_response_dial, "voice", "Dial", "Response");
invalid_contract!(
    invalid_voice_dial_application,
    "voice",
    "Application",
    "Dial"
);
invalid_contract!(invalid_voice_dial_client, "voice", "Client", "Dial");
invalid_contract!(invalid_voice_dial_conference, "voice", "Conference", "Dial");
invalid_contract!(invalid_voice_dial_number, "voice", "Number", "Dial");
invalid_contract!(invalid_voice_dial_queue, "voice", "Queue", "Dial");
invalid_contract!(invalid_voice_dial_sip, "voice", "Sip", "Dial");
invalid_contract!(invalid_voice_response_gather, "voice", "Gather", "Response");
invalid_contract!(
    invalid_voice_response_gather_say,
    "voice",
    "Say",
    "Response|Gather"
);
invalid_contract!(
    invalid_voice_response_gather_play,
    "voice",
    "Play",
    "Response|Gather"
);
invalid_contract!(
    invalid_voice_response_gather_pause,
    "voice",
    "Pause",
    "Response|Gather"
);
invalid_contract!(invalid_voice_response_pay, "voice", "Pay", "Response");
invalid_contract!(invalid_voice_pay_prompt, "voice", "Prompt", "Pay");
invalid_contract!(
    invalid_voice_parameter_parents,
    "voice",
    "Parameter",
    "Pay|Stream|VirtualAgent|Client"
);
invalid_contract!(invalid_voice_response_start, "voice", "Start", "Response");
invalid_contract!(invalid_voice_response_stop, "voice", "Stop", "Response");
invalid_contract!(
    invalid_voice_start_stop_recording,
    "voice",
    "Recording",
    "Start|Stop"
);
invalid_contract!(
    invalid_voice_start_stop_siprec,
    "voice",
    "Siprec",
    "Start|Stop"
);
invalid_contract!(
    invalid_voice_start_stop_stream,
    "voice",
    "Stream",
    "Start|Stop"
);
invalid_contract!(
    invalid_voice_start_stop_transcription,
    "voice",
    "Transcription",
    "Start|Stop"
);
invalid_contract!(
    invalid_voice_response_enqueue,
    "voice",
    "Enqueue",
    "Response"
);
invalid_contract!(invalid_voice_response_echo, "voice", "Echo", "Response");
invalid_contract!(invalid_voice_response_hangup, "voice", "Hangup", "Response");
invalid_contract!(invalid_voice_response_leave, "voice", "Leave", "Response");
invalid_contract!(invalid_voice_response_record, "voice", "Record", "Response");
invalid_contract!(
    invalid_voice_response_redirect,
    "voice",
    "Redirect",
    "Response"
);
invalid_contract!(invalid_voice_response_refer, "voice", "Refer", "Response");
invalid_contract!(invalid_voice_response_reject, "voice", "Reject", "Response");
invalid_contract!(
    invalid_messaging_response_message,
    "messaging",
    "Message",
    "Response"
);
invalid_contract!(
    invalid_messaging_response_redirect,
    "messaging",
    "Redirect",
    "Response"
);
invalid_contract!(
    invalid_messaging_message_body,
    "messaging",
    "Body",
    "Message"
);
invalid_contract!(
    invalid_messaging_message_media,
    "messaging",
    "Media",
    "Message"
);

// Separate selectors keep the additional pinned constraints independently auditable.
macro_rules! invalid_attribute_value {
    ($test:ident, $node:ident, $attribute:ident, $bad:literal, $good:literal) => {
        #[test]
        fn $test() {
            use dialkit::twiml::{TwimlAttributeName as A, TwimlNode, TwimlNodeKind as N};
            assert!(
                TwimlNode::new(N::$node)
                    .attribute(A::$attribute, $bad)
                    .is_err()
            );
            assert!(
                TwimlNode::new(N::$node)
                    .attribute(A::$attribute, $good)
                    .is_ok()
            );
        }
    };
}

invalid_attribute_value!(
    invalid_dial_hangup_on_star,
    Dial,
    HangupOnStar,
    "yes",
    "true"
);
invalid_attribute_value!(
    invalid_dial_answer_on_bridge,
    Dial,
    AnswerOnBridge,
    "1",
    "false"
);
invalid_attribute_value!(invalid_dial_trim, Dial, Trim, "trim-all", "trim-silence");
invalid_attribute_value!(
    invalid_gather_speech_timeout,
    Gather,
    SpeechTimeout,
    "later",
    "auto"
);
invalid_attribute_value!(invalid_gather_finish_on_key, Gather, FinishOnKey, "##", "#");
invalid_attribute_value!(
    invalid_gather_profanity_filter,
    Gather,
    ProfanityFilter,
    "yes",
    "false"
);
invalid_attribute_value!(
    invalid_start_stream_track,
    Stream,
    Track,
    "center",
    "inbound_track"
);
invalid_attribute_value!(invalid_record_play_beep, Record, PlayBeep, "yes", "true");
invalid_attribute_value!(
    invalid_record_max_length,
    Record,
    MaxLength,
    "86401",
    "86400"
);
invalid_attribute_value!(invalid_pay_max_attempts, Pay, MaxAttempts, "4", "3");
invalid_attribute_value!(
    invalid_pay_charge_amount,
    Pay,
    ChargeAmount,
    "1000000.01",
    "1000000"
);
invalid_attribute_value!(
    invalid_pay_token_type,
    Pay,
    TokenType,
    "forever",
    "reusable"
);
invalid_attribute_value!(invalid_say_loop, Say, Loop, "forever", "0");

#[test]
fn invalid_connect_stream_track() {
    use dialkit::twiml::{TwimlAttributeName as A, TwimlNode, TwimlNodeKind as N};
    assert!(
        TwimlNode::new(N::ConnectStream)
            .attribute(A::Track, "both_tracks")
            .is_err()
    );
    assert!(
        TwimlNode::new(N::ConnectStream)
            .attribute(A::Track, "inbound_track")
            .is_ok()
    );
}
