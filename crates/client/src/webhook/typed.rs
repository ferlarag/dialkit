use super::{FormPair, OpenValue, VerifiedFormWebhook, VerifiedJsonWebhook, WebhookFamily};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fmt;

macro_rules! family_event {
    ($name:ident { $($field:ident => $wire:literal),+ $(,)? }) => {
        #[derive(Clone)]
        pub struct $name {
            $( $field: Option<String>, )+
            extras: Vec<FormPair>,
        }
        impl $name {
            $(
                #[must_use]
                pub fn $field(&self) -> Option<&str> { self.$field.as_deref() }
            )+
            #[must_use]
            pub fn extras(&self) -> &[FormPair] { &self.extras }
            fn parse(input: &VerifiedFormWebhook) -> Self {
                Self {
                    $( $field: input.first($wire).map(str::to_owned), )+
                    extras: input.extras(&[$($wire),+]),
                }
            }
        }
        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.debug_struct(stringify!($name))
                    $(.field(stringify!($field), &self.$field.as_ref().map(|_| "[REDACTED]")))+
                    .field("extras", &format_args!("[REDACTED; {}]", self.extras.len()))
                    .finish()
            }
        }
    };
}

family_event!(VoiceInstructionWebhook {
    account_sid => "AccountSid", call_sid => "CallSid", from_value => "From", to => "To",
    call_status => "CallStatus", direction => "Direction", api_version => "ApiVersion",
    caller_name => "CallerName", forwarded_from => "ForwardedFrom",
});
family_event!(CallProgressWebhook {
    account_sid => "AccountSid", call_sid => "CallSid", parent_call_sid => "ParentCallSid",
    call_status => "CallStatus", direction => "Direction", timestamp => "Timestamp",
    sequence_number => "SequenceNumber", callback_source => "CallbackSource",
});
family_event!(AnsweringMachineWebhook {
    account_sid => "AccountSid", call_sid => "CallSid", answered_by => "AnsweredBy",
    machine_detection_duration => "MachineDetectionDuration",
});
family_event!(VoiceActionWebhook {
    account_sid => "AccountSid", call_sid => "CallSid", dial_call_sid => "DialCallSid",
    dial_call_status => "DialCallStatus", dial_call_duration => "DialCallDuration",
    dial_bridged => "DialBridged", queue_result => "QueueResult", digits => "Digits",
    speech_result => "SpeechResult", result => "Result",
});
family_event!(RecordingWebhook {
    account_sid => "AccountSid", call_sid => "CallSid", conference_sid => "ConferenceSid",
    recording_sid => "RecordingSid", recording_status => "RecordingStatus",
    recording_url => "RecordingUrl", recording_duration => "RecordingDuration",
});
family_event!(ConferenceWebhook {
    account_sid => "AccountSid", call_sid => "CallSid", conference_sid => "ConferenceSid",
    friendly_name => "FriendlyName", status_callback_event => "StatusCallbackEvent",
    participant_label => "ParticipantLabel", timestamp => "Timestamp",
    sequence_number => "SequenceNumber", muted => "Muted", hold => "Hold",
});
family_event!(QueueWebhook {
    account_sid => "AccountSid", call_sid => "CallSid", queue_sid => "QueueSid",
    queue_result => "QueueResult", queue_position => "QueuePosition", queue_time => "QueueTime",
});
family_event!(GatherWebhook {
    account_sid => "AccountSid", call_sid => "CallSid", digits => "Digits",
    speech_result => "SpeechResult", confidence => "Confidence",
});
family_event!(StreamWebhook {
    account_sid => "AccountSid", call_sid => "CallSid", stream_sid => "StreamSid",
    stream_event => "StreamEvent", timestamp => "Timestamp",
});
family_event!(SiprecWebhook {
    account_sid => "AccountSid", call_sid => "CallSid", siprec_sid => "SiprecSid",
    siprec_event => "SiprecEvent", timestamp => "Timestamp",
});
family_event!(TranscriptionWebhook {
    account_sid => "AccountSid", call_sid => "CallSid", transcription_sid => "TranscriptionSid",
    transcription_event => "TranscriptionEvent", result => "Result",
});
family_event!(PaymentWebhook {
    account_sid => "AccountSid", call_sid => "CallSid", result => "Result",
    payment_token => "PaymentToken",
});
family_event!(UserDefinedMessageWebhook {
    account_sid => "AccountSid", call_sid => "CallSid", event => "Event", payload => "Payload",
});
family_event!(IncomingMessageWebhook {
    account_sid => "AccountSid", message_sid => "MessageSid", sms_sid => "SmsSid",
    from_value => "From", to => "To", body => "Body", num_media => "NumMedia",
    messaging_service_sid => "MessagingServiceSid",
});
family_event!(MessageStatusWebhook {
    account_sid => "AccountSid", message_sid => "MessageSid", sms_sid => "SmsSid",
    message_status => "MessageStatus", sms_status => "SmsStatus", error_code => "ErrorCode",
    error_message => "ErrorMessage", channel_data => "ChannelData",
    raw_dlr_done_date => "RawDlrDoneDate", messaging_service_sid => "MessagingServiceSid",
});
family_event!(MessagingServiceRoutingWebhook {
    account_sid => "AccountSid", message_sid => "MessageSid", sms_sid => "SmsSid",
    messaging_service_sid => "MessagingServiceSid", from_value => "From", to => "To",
    body => "Body", message_status => "MessageStatus",
});

macro_rules! open_status {
    ($name:ident { $($variant:ident => $wire:literal),+ $(,)? }) => {
        #[derive(Clone, Debug, Eq, PartialEq)]
        #[non_exhaustive]
        pub enum $name {
            $($variant,)+
            Unknown(OpenValue),
        }
        impl $name {
            #[must_use]
            pub fn parse(value: &str) -> Self {
                match value {
                    $($wire => Self::$variant,)+
                    other => Self::Unknown(OpenValue::new(other)),
                }
            }
            #[must_use]
            pub fn as_str(&self) -> &str {
                match self {
                    $(Self::$variant => $wire,)+
                    Self::Unknown(value) => value.as_str(),
                }
            }
        }
    };
}

open_status!(CallProgressStatus {
    Initiated => "initiated", Ringing => "ringing", Answered => "answered",
    Completed => "completed", Busy => "busy", Failed => "failed",
    NoAnswer => "no-answer", Canceled => "canceled",
});
open_status!(RecordingStatus {
    InProgress => "in-progress", Completed => "completed", Absent => "absent",
});
open_status!(ConferenceEvent {
    Start => "start", End => "end", Join => "join", Leave => "leave",
    Mute => "mute", Hold => "hold", Speaker => "speaker",
});
open_status!(MessageDeliveryStatus {
    Accepted => "accepted", Scheduled => "scheduled", Canceled => "canceled",
    Queued => "queued", Sending => "sending", Sent => "sent",
    Receiving => "receiving", Received => "received", Delivered => "delivered",
    Undelivered => "undelivered", Failed => "failed", Read => "read",
});

impl CallProgressWebhook {
    #[must_use]
    pub fn status(&self) -> Option<CallProgressStatus> {
        self.call_status.as_deref().map(CallProgressStatus::parse)
    }
}
impl RecordingWebhook {
    #[must_use]
    pub fn status(&self) -> Option<RecordingStatus> {
        self.recording_status.as_deref().map(RecordingStatus::parse)
    }
}
impl ConferenceWebhook {
    #[must_use]
    pub fn event(&self) -> Option<ConferenceEvent> {
        self.status_callback_event
            .as_deref()
            .map(ConferenceEvent::parse)
    }
}
impl MessageStatusWebhook {
    #[must_use]
    pub fn status(&self) -> Option<MessageDeliveryStatus> {
        self.message_status
            .as_deref()
            .or(self.sms_status.as_deref())
            .map(MessageDeliveryStatus::parse)
    }
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum FamilyWebhookEvent {
    VoiceInstruction(VoiceInstructionWebhook),
    CallProgress(CallProgressWebhook),
    AnsweringMachine(AnsweringMachineWebhook),
    VoiceAction(VoiceActionWebhook),
    Recording(RecordingWebhook),
    Conference(ConferenceWebhook),
    Queue(QueueWebhook),
    Gather(GatherWebhook),
    Stream(StreamWebhook),
    Siprec(SiprecWebhook),
    Transcription(TranscriptionWebhook),
    Payment(PaymentWebhook),
    UserDefinedMessage(UserDefinedMessageWebhook),
    IncomingMessage(IncomingMessageWebhook),
    MessageStatus(MessageStatusWebhook),
    MessagingServiceRouting(MessagingServiceRoutingWebhook),
}

impl FamilyWebhookEvent {
    #[must_use]
    pub const fn family(&self) -> WebhookFamily {
        match self {
            Self::VoiceInstruction(_) => WebhookFamily::VoiceInstruction,
            Self::CallProgress(_) => WebhookFamily::CallProgress,
            Self::AnsweringMachine(_) => WebhookFamily::AnsweringMachine,
            Self::VoiceAction(_) => WebhookFamily::VoiceAction,
            Self::Recording(_) => WebhookFamily::Recording,
            Self::Conference(_) => WebhookFamily::Conference,
            Self::Queue(_) => WebhookFamily::Queue,
            Self::Gather(_) => WebhookFamily::Gather,
            Self::Stream(_) => WebhookFamily::Stream,
            Self::Siprec(_) => WebhookFamily::Siprec,
            Self::Transcription(_) => WebhookFamily::Transcription,
            Self::Payment(_) => WebhookFamily::Payment,
            Self::UserDefinedMessage(_) => WebhookFamily::UserDefinedMessage,
            Self::IncomingMessage(_) => WebhookFamily::IncomingMessage,
            Self::MessageStatus(_) => WebhookFamily::MessageStatus,
            Self::MessagingServiceRouting(_) => WebhookFamily::MessagingServiceRouting,
        }
    }

    #[must_use]
    pub fn extras(&self) -> &[FormPair] {
        match self {
            Self::VoiceInstruction(value) => value.extras(),
            Self::CallProgress(value) => value.extras(),
            Self::AnsweringMachine(value) => value.extras(),
            Self::VoiceAction(value) => value.extras(),
            Self::Recording(value) => value.extras(),
            Self::Conference(value) => value.extras(),
            Self::Queue(value) => value.extras(),
            Self::Gather(value) => value.extras(),
            Self::Stream(value) => value.extras(),
            Self::Siprec(value) => value.extras(),
            Self::Transcription(value) => value.extras(),
            Self::Payment(value) => value.extras(),
            Self::UserDefinedMessage(value) => value.extras(),
            Self::IncomingMessage(value) => value.extras(),
            Self::MessageStatus(value) => value.extras(),
            Self::MessagingServiceRouting(value) => value.extras(),
        }
    }
}

impl VerifiedFormWebhook {
    #[must_use]
    pub fn parse_family_specific(&self) -> FamilyWebhookEvent {
        match self.family {
            WebhookFamily::VoiceInstruction => {
                FamilyWebhookEvent::VoiceInstruction(VoiceInstructionWebhook::parse(self))
            }
            WebhookFamily::CallProgress => {
                FamilyWebhookEvent::CallProgress(CallProgressWebhook::parse(self))
            }
            WebhookFamily::AnsweringMachine => {
                FamilyWebhookEvent::AnsweringMachine(AnsweringMachineWebhook::parse(self))
            }
            WebhookFamily::VoiceAction => {
                FamilyWebhookEvent::VoiceAction(VoiceActionWebhook::parse(self))
            }
            WebhookFamily::Recording => {
                FamilyWebhookEvent::Recording(RecordingWebhook::parse(self))
            }
            WebhookFamily::Conference => {
                FamilyWebhookEvent::Conference(ConferenceWebhook::parse(self))
            }
            WebhookFamily::Queue => FamilyWebhookEvent::Queue(QueueWebhook::parse(self)),
            WebhookFamily::Gather => FamilyWebhookEvent::Gather(GatherWebhook::parse(self)),
            WebhookFamily::Stream => FamilyWebhookEvent::Stream(StreamWebhook::parse(self)),
            WebhookFamily::Siprec => FamilyWebhookEvent::Siprec(SiprecWebhook::parse(self)),
            WebhookFamily::Transcription => {
                FamilyWebhookEvent::Transcription(TranscriptionWebhook::parse(self))
            }
            WebhookFamily::Payment => FamilyWebhookEvent::Payment(PaymentWebhook::parse(self)),
            WebhookFamily::UserDefinedMessage => {
                FamilyWebhookEvent::UserDefinedMessage(UserDefinedMessageWebhook::parse(self))
            }
            WebhookFamily::IncomingMessage => {
                FamilyWebhookEvent::IncomingMessage(IncomingMessageWebhook::parse(self))
            }
            WebhookFamily::MessageStatus => {
                FamilyWebhookEvent::MessageStatus(MessageStatusWebhook::parse(self))
            }
            WebhookFamily::MessagingServiceRouting => FamilyWebhookEvent::MessagingServiceRouting(
                MessagingServiceRoutingWebhook::parse(self),
            ),
        }
    }
}

macro_rules! json_family_event {
    ($name:ident { $($field:ident => $wire:literal),+ $(,)? }) => {
        #[derive(Clone)]
        pub struct $name {
            $( $field: Option<String>, )+
            extras: BTreeMap<String, Value>,
        }
        impl $name {
            $(
                #[must_use]
                pub fn $field(&self) -> Option<&str> { self.$field.as_deref() }
            )+
            #[must_use]
            pub fn extras(&self) -> &BTreeMap<String, Value> { &self.extras }
            fn parse(input: &VerifiedJsonWebhook) -> Self {
                let mut extras = input.properties().clone();
                $(
                    let $field = extras.get($wire).and_then(Value::as_str).map(str::to_owned);
                    if $field.is_some() { extras.remove($wire); }
                )+
                Self { $($field,)+ extras }
            }
        }
        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.debug_struct(stringify!($name))
                    $(.field(stringify!($field), &self.$field.as_ref().map(|_| "[REDACTED]")))+
                    .field("extras", &format_args!("[REDACTED; {}]", self.extras.len()))
                    .finish()
            }
        }
    };
}

json_family_event!(StreamJsonWebhook {
    account_sid => "AccountSid", call_sid => "CallSid", stream_sid => "StreamSid",
    stream_event => "StreamEvent", timestamp => "Timestamp",
});
json_family_event!(SiprecJsonWebhook {
    account_sid => "AccountSid", call_sid => "CallSid", siprec_sid => "SiprecSid",
    siprec_event => "SiprecEvent", timestamp => "Timestamp",
});
json_family_event!(TranscriptionJsonWebhook {
    account_sid => "AccountSid", call_sid => "CallSid", transcription_sid => "TranscriptionSid",
    transcription_event => "TranscriptionEvent", result => "Result",
});
#[derive(Clone)]
pub struct MessageStatusJsonWebhook {
    account_sid: Option<String>,
    message_sid: Option<String>,
    sms_sid: Option<String>,
    message_status: Option<String>,
    sms_status: Option<String>,
    error_code: Option<String>,
    error_message: Option<String>,
    channel_data: Option<serde_json::Map<String, Value>>,
    raw_dlr_done_date: Option<String>,
    messaging_service_sid: Option<String>,
    extras: BTreeMap<String, Value>,
}

impl MessageStatusJsonWebhook {
    fn parse(input: &VerifiedJsonWebhook) -> Self {
        let mut extras = input.properties().clone();
        let mut take_string = |name: &str| {
            let value = extras.get(name).and_then(Value::as_str).map(str::to_owned);
            if value.is_some() {
                extras.remove(name);
            }
            value
        };
        let account_sid = take_string("AccountSid");
        let message_sid = take_string("MessageSid");
        let sms_sid = take_string("SmsSid");
        let message_status = take_string("MessageStatus");
        let sms_status = take_string("SmsStatus");
        let error_code = take_string("ErrorCode");
        let error_message = take_string("ErrorMessage");
        let raw_dlr_done_date = take_string("RawDlrDoneDate");
        let messaging_service_sid = take_string("MessagingServiceSid");
        let channel_data = extras
            .remove("ChannelData")
            .and_then(|value| value.as_object().cloned());
        Self {
            account_sid,
            message_sid,
            sms_sid,
            message_status,
            sms_status,
            error_code,
            error_message,
            channel_data,
            raw_dlr_done_date,
            messaging_service_sid,
            extras,
        }
    }

    #[must_use]
    pub fn account_sid(&self) -> Option<&str> {
        self.account_sid.as_deref()
    }
    #[must_use]
    pub fn message_sid(&self) -> Option<&str> {
        self.message_sid.as_deref()
    }
    #[must_use]
    pub fn sms_sid(&self) -> Option<&str> {
        self.sms_sid.as_deref()
    }
    #[must_use]
    pub fn message_status(&self) -> Option<&str> {
        self.message_status.as_deref()
    }
    #[must_use]
    pub fn sms_status(&self) -> Option<&str> {
        self.sms_status.as_deref()
    }
    #[must_use]
    pub fn error_code(&self) -> Option<&str> {
        self.error_code.as_deref()
    }
    #[must_use]
    pub fn error_message(&self) -> Option<&str> {
        self.error_message.as_deref()
    }
    #[must_use]
    pub fn channel_data(&self) -> Option<&serde_json::Map<String, Value>> {
        self.channel_data.as_ref()
    }
    #[must_use]
    pub fn raw_dlr_done_date(&self) -> Option<&str> {
        self.raw_dlr_done_date.as_deref()
    }
    #[must_use]
    pub fn messaging_service_sid(&self) -> Option<&str> {
        self.messaging_service_sid.as_deref()
    }
    #[must_use]
    pub fn extras(&self) -> &BTreeMap<String, Value> {
        &self.extras
    }
}

impl fmt::Debug for MessageStatusJsonWebhook {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MessageStatusJsonWebhook")
            .field("known", &"[REDACTED]")
            .field(
                "channel_data",
                &self.channel_data.as_ref().map(|_| "[REDACTED]"),
            )
            .field("extras", &format_args!("[REDACTED; {}]", self.extras.len()))
            .finish()
    }
}

impl MessageStatusJsonWebhook {
    #[must_use]
    pub fn status(&self) -> Option<MessageDeliveryStatus> {
        self.message_status
            .as_deref()
            .or(self.sms_status.as_deref())
            .map(MessageDeliveryStatus::parse)
    }
}

#[derive(Clone)]
pub struct UserDefinedMessageJsonWebhook {
    account_sid: Option<String>,
    call_sid: Option<String>,
    event: Option<String>,
    payload: Option<Value>,
    extras: BTreeMap<String, Value>,
}
impl UserDefinedMessageJsonWebhook {
    fn parse(input: &VerifiedJsonWebhook) -> Self {
        let mut extras = input.properties().clone();
        let account_sid = extras
            .get("AccountSid")
            .and_then(Value::as_str)
            .map(str::to_owned);
        if account_sid.is_some() {
            extras.remove("AccountSid");
        }
        let call_sid = extras
            .get("CallSid")
            .and_then(Value::as_str)
            .map(str::to_owned);
        if call_sid.is_some() {
            extras.remove("CallSid");
        }
        let event = extras
            .get("Event")
            .and_then(Value::as_str)
            .map(str::to_owned);
        if event.is_some() {
            extras.remove("Event");
        }
        let payload = extras.remove("Payload");
        Self {
            account_sid,
            call_sid,
            event,
            payload,
            extras,
        }
    }
    #[must_use]
    pub fn account_sid(&self) -> Option<&str> {
        self.account_sid.as_deref()
    }
    #[must_use]
    pub fn call_sid(&self) -> Option<&str> {
        self.call_sid.as_deref()
    }
    #[must_use]
    pub fn event(&self) -> Option<&str> {
        self.event.as_deref()
    }
    #[must_use]
    pub fn payload(&self) -> Option<&Value> {
        self.payload.as_ref()
    }
    #[must_use]
    pub fn extras(&self) -> &BTreeMap<String, Value> {
        &self.extras
    }
}
impl fmt::Debug for UserDefinedMessageJsonWebhook {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("UserDefinedMessageJsonWebhook")
            .field(
                "account_sid",
                &self.account_sid.as_ref().map(|_| "[REDACTED]"),
            )
            .field("call_sid", &self.call_sid.as_ref().map(|_| "[REDACTED]"))
            .field("event", &self.event.as_ref().map(|_| "[REDACTED]"))
            .field("payload", &self.payload.as_ref().map(|_| "[REDACTED]"))
            .field("extras", &format_args!("[REDACTED; {}]", self.extras.len()))
            .finish()
    }
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum FamilyJsonWebhookEvent {
    Stream(StreamJsonWebhook),
    Siprec(SiprecJsonWebhook),
    Transcription(TranscriptionJsonWebhook),
    UserDefinedMessage(UserDefinedMessageJsonWebhook),
    MessageStatus(MessageStatusJsonWebhook),
}
impl FamilyJsonWebhookEvent {
    #[must_use]
    pub const fn family(&self) -> WebhookFamily {
        match self {
            Self::Stream(_) => WebhookFamily::Stream,
            Self::Siprec(_) => WebhookFamily::Siprec,
            Self::Transcription(_) => WebhookFamily::Transcription,
            Self::UserDefinedMessage(_) => WebhookFamily::UserDefinedMessage,
            Self::MessageStatus(_) => WebhookFamily::MessageStatus,
        }
    }
    #[must_use]
    pub fn extras(&self) -> &BTreeMap<String, Value> {
        match self {
            Self::Stream(value) => value.extras(),
            Self::Siprec(value) => value.extras(),
            Self::Transcription(value) => value.extras(),
            Self::UserDefinedMessage(value) => value.extras(),
            Self::MessageStatus(value) => value.extras(),
        }
    }
}

impl VerifiedJsonWebhook {
    #[must_use]
    pub fn parse_family_specific(&self) -> Option<FamilyJsonWebhookEvent> {
        match self.family() {
            WebhookFamily::Stream => Some(FamilyJsonWebhookEvent::Stream(
                StreamJsonWebhook::parse(self),
            )),
            WebhookFamily::Siprec => Some(FamilyJsonWebhookEvent::Siprec(
                SiprecJsonWebhook::parse(self),
            )),
            WebhookFamily::Transcription => Some(FamilyJsonWebhookEvent::Transcription(
                TranscriptionJsonWebhook::parse(self),
            )),
            WebhookFamily::UserDefinedMessage => Some(FamilyJsonWebhookEvent::UserDefinedMessage(
                UserDefinedMessageJsonWebhook::parse(self),
            )),
            WebhookFamily::MessageStatus => Some(FamilyJsonWebhookEvent::MessageStatus(
                MessageStatusJsonWebhook::parse(self),
            )),
            _ => None,
        }
    }
}
