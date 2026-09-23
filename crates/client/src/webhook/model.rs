use std::{collections::BTreeMap, fmt};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum WebhookFamily {
    VoiceInstruction,
    CallProgress,
    AnsweringMachine,
    VoiceAction,
    Recording,
    Conference,
    Queue,
    Gather,
    Stream,
    Siprec,
    Transcription,
    Payment,
    UserDefinedMessage,
    IncomingMessage,
    MessageStatus,
    MessagingServiceRouting,
}

impl WebhookFamily {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VoiceInstruction => "voice-instruction",
            Self::CallProgress => "call-progress",
            Self::AnsweringMachine => "answering-machine",
            Self::VoiceAction => "twiml-action",
            Self::Recording => "recording",
            Self::Conference => "conference",
            Self::Queue => "queue",
            Self::Gather => "gather",
            Self::Stream | Self::Siprec => "stream-siprec",
            Self::Transcription => "transcription",
            Self::Payment => "payment",
            Self::UserDefinedMessage => "user-defined-message",
            Self::IncomingMessage => "incoming-message",
            Self::MessageStatus => "message-status",
            Self::MessagingServiceRouting => "messaging-service-routing",
        }
    }

    #[must_use]
    pub const fn known_fields(self) -> &'static [WebhookField] {
        use WebhookField as F;
        match self {
            Self::VoiceInstruction => &[
                F::AccountSid,
                F::CallSid,
                F::From,
                F::To,
                F::CallStatus,
                F::Direction,
                F::ApiVersion,
                F::CallerName,
                F::ForwardedFrom,
            ],
            Self::CallProgress => &[
                F::AccountSid,
                F::CallSid,
                F::ParentCallSid,
                F::CallStatus,
                F::Direction,
                F::Timestamp,
                F::SequenceNumber,
                F::CallbackSource,
            ],
            Self::AnsweringMachine => &[
                F::AccountSid,
                F::CallSid,
                F::AnsweredBy,
                F::MachineDetectionDuration,
            ],
            Self::VoiceAction => &[
                F::AccountSid,
                F::CallSid,
                F::DialCallSid,
                F::DialCallStatus,
                F::DialCallDuration,
                F::DialBridged,
                F::QueueResult,
                F::Digits,
                F::SpeechResult,
                F::Result,
            ],
            Self::Recording => &[
                F::AccountSid,
                F::CallSid,
                F::ConferenceSid,
                F::RecordingSid,
                F::RecordingStatus,
                F::RecordingUrl,
                F::RecordingDuration,
            ],
            Self::Conference => &[
                F::AccountSid,
                F::CallSid,
                F::ConferenceSid,
                F::FriendlyName,
                F::StatusCallbackEvent,
                F::ParticipantLabel,
                F::Timestamp,
                F::SequenceNumber,
                F::Muted,
                F::Hold,
            ],
            Self::Queue => &[
                F::AccountSid,
                F::CallSid,
                F::QueueSid,
                F::QueueResult,
                F::QueuePosition,
                F::QueueTime,
            ],
            Self::Gather => &[
                F::AccountSid,
                F::CallSid,
                F::Digits,
                F::SpeechResult,
                F::Confidence,
            ],
            Self::Stream => &[
                F::AccountSid,
                F::CallSid,
                F::StreamSid,
                F::StreamEvent,
                F::Timestamp,
            ],
            Self::Siprec => &[
                F::AccountSid,
                F::CallSid,
                F::SiprecSid,
                F::SiprecEvent,
                F::Timestamp,
            ],
            Self::Transcription => &[
                F::AccountSid,
                F::CallSid,
                F::TranscriptionSid,
                F::TranscriptionEvent,
                F::Result,
            ],
            Self::Payment => &[F::AccountSid, F::CallSid, F::Result, F::PaymentToken],
            Self::UserDefinedMessage => &[F::AccountSid, F::CallSid, F::Event, F::Payload],
            Self::IncomingMessage => &[
                F::AccountSid,
                F::MessageSid,
                F::SmsSid,
                F::From,
                F::To,
                F::Body,
                F::NumMedia,
                F::MessagingServiceSid,
            ],
            Self::MessageStatus => &[
                F::AccountSid,
                F::MessageSid,
                F::SmsSid,
                F::MessageStatus,
                F::SmsStatus,
                F::ErrorCode,
                F::ErrorMessage,
                F::ChannelData,
                F::RawDlrDoneDate,
                F::MessagingServiceSid,
            ],
            Self::MessagingServiceRouting => &[
                F::AccountSid,
                F::MessageSid,
                F::SmsSid,
                F::MessagingServiceSid,
                F::From,
                F::To,
                F::Body,
                F::MessageStatus,
            ],
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum WebhookField {
    AccountSid,
    CallSid,
    ParentCallSid,
    From,
    To,
    CallStatus,
    Direction,
    ApiVersion,
    CallerName,
    ForwardedFrom,
    Timestamp,
    SequenceNumber,
    CallbackSource,
    AnsweredBy,
    MachineDetectionDuration,
    DialCallSid,
    DialCallStatus,
    DialCallDuration,
    DialBridged,
    RecordingSid,
    RecordingStatus,
    RecordingUrl,
    RecordingDuration,
    ConferenceSid,
    FriendlyName,
    StatusCallbackEvent,
    ParticipantLabel,
    Muted,
    Hold,
    QueueSid,
    QueueResult,
    QueuePosition,
    QueueTime,
    Digits,
    SpeechResult,
    Confidence,
    StreamSid,
    StreamEvent,
    SiprecSid,
    SiprecEvent,
    TranscriptionSid,
    TranscriptionEvent,
    Result,
    PaymentToken,
    Event,
    Payload,
    MessageSid,
    SmsSid,
    Body,
    NumMedia,
    MessageStatus,
    SmsStatus,
    MessagingServiceSid,
    ErrorCode,
    ErrorMessage,
    ChannelData,
    RawDlrDoneDate,
}

impl WebhookField {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AccountSid => "AccountSid",
            Self::CallSid => "CallSid",
            Self::ParentCallSid => "ParentCallSid",
            Self::From => "From",
            Self::To => "To",
            Self::CallStatus => "CallStatus",
            Self::Direction => "Direction",
            Self::ApiVersion => "ApiVersion",
            Self::CallerName => "CallerName",
            Self::ForwardedFrom => "ForwardedFrom",
            Self::Timestamp => "Timestamp",
            Self::SequenceNumber => "SequenceNumber",
            Self::CallbackSource => "CallbackSource",
            Self::AnsweredBy => "AnsweredBy",
            Self::MachineDetectionDuration => "MachineDetectionDuration",
            Self::DialCallSid => "DialCallSid",
            Self::DialCallStatus => "DialCallStatus",
            Self::DialCallDuration => "DialCallDuration",
            Self::DialBridged => "DialBridged",
            Self::RecordingSid => "RecordingSid",
            Self::RecordingStatus => "RecordingStatus",
            Self::RecordingUrl => "RecordingUrl",
            Self::RecordingDuration => "RecordingDuration",
            Self::ConferenceSid => "ConferenceSid",
            Self::FriendlyName => "FriendlyName",
            Self::StatusCallbackEvent => "StatusCallbackEvent",
            Self::ParticipantLabel => "ParticipantLabel",
            Self::Muted => "Muted",
            Self::Hold => "Hold",
            Self::QueueSid => "QueueSid",
            Self::QueueResult => "QueueResult",
            Self::QueuePosition => "QueuePosition",
            Self::QueueTime => "QueueTime",
            Self::Digits => "Digits",
            Self::SpeechResult => "SpeechResult",
            Self::Confidence => "Confidence",
            Self::StreamSid => "StreamSid",
            Self::StreamEvent => "StreamEvent",
            Self::SiprecSid => "SiprecSid",
            Self::SiprecEvent => "SiprecEvent",
            Self::TranscriptionSid => "TranscriptionSid",
            Self::TranscriptionEvent => "TranscriptionEvent",
            Self::Result => "Result",
            Self::PaymentToken => "PaymentToken",
            Self::Event => "Event",
            Self::Payload => "Payload",
            Self::MessageSid => "MessageSid",
            Self::SmsSid => "SmsSid",
            Self::Body => "Body",
            Self::NumMedia => "NumMedia",
            Self::MessageStatus => "MessageStatus",
            Self::SmsStatus => "SmsStatus",
            Self::MessagingServiceSid => "MessagingServiceSid",
            Self::ErrorCode => "ErrorCode",
            Self::ErrorMessage => "ErrorMessage",
            Self::ChannelData => "ChannelData",
            Self::RawDlrDoneDate => "RawDlrDoneDate",
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct TypedWebhookValue {
    field: WebhookField,
    value: String,
}
impl TypedWebhookValue {
    #[must_use]
    pub const fn field(&self) -> WebhookField {
        self.field
    }
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}
impl fmt::Debug for TypedWebhookValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TypedWebhookValue")
            .field("field", &self.field)
            .field("value", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone)]
pub struct TypedWebhookEvent {
    family: WebhookFamily,
    known: Vec<TypedWebhookValue>,
    extras: Vec<FormPair>,
}
impl fmt::Debug for TypedWebhookEvent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TypedWebhookEvent")
            .field("family", &self.family)
            .field("known", &format_args!("[REDACTED; {}]", self.known.len()))
            .field("extras", &format_args!("[REDACTED; {}]", self.extras.len()))
            .finish()
    }
}
impl TypedWebhookEvent {
    #[must_use]
    pub const fn family(&self) -> WebhookFamily {
        self.family
    }
    #[must_use]
    pub fn known(&self) -> &[TypedWebhookValue] {
        &self.known
    }
    #[must_use]
    pub fn extras(&self) -> &[FormPair] {
        &self.extras
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenValue(String);
impl OpenValue {
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormPair {
    pub(crate) name: String,
    pub(crate) value: String,
}
impl FormPair {
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Clone)]
pub struct VerifiedFormWebhook {
    pub(crate) family: WebhookFamily,
    pub(crate) pairs: Vec<(String, String)>,
}
impl VerifiedFormWebhook {
    #[must_use]
    pub const fn family(&self) -> WebhookFamily {
        self.family
    }
    pub(crate) fn first(&self, name: &str) -> Option<&str> {
        self.pairs
            .iter()
            .find(|(key, _)| key == name)
            .map(|(_, value)| value.as_str())
    }
    pub(crate) fn extras(&self, consumed: &[&str]) -> Vec<FormPair> {
        self.pairs
            .iter()
            .filter(|(name, _)| !consumed.contains(&name.as_str()))
            .map(|(name, value)| FormPair {
                name: name.clone(),
                value: value.clone(),
            })
            .collect()
    }

    #[must_use]
    pub fn parse_typed(&self) -> TypedWebhookEvent {
        let known = self
            .family
            .known_fields()
            .iter()
            .flat_map(|field| {
                self.pairs
                    .iter()
                    .filter(move |(name, _)| name == field.as_str())
                    .map(move |(_, value)| TypedWebhookValue {
                        field: *field,
                        value: value.clone(),
                    })
            })
            .collect();
        let consumed = self
            .family
            .known_fields()
            .iter()
            .map(|field| field.as_str())
            .collect::<Vec<_>>();
        TypedWebhookEvent {
            family: self.family,
            known,
            extras: self.extras(&consumed),
        }
    }
}
impl fmt::Debug for VerifiedFormWebhook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VerifiedFormWebhook")
            .field("family", &self.family)
            .field("pairs", &format_args!("[REDACTED; {}]", self.pairs.len()))
            .finish()
    }
}

#[derive(Clone)]
pub struct VerifiedJsonWebhook {
    pub(crate) family: WebhookFamily,
    pub(crate) properties: BTreeMap<String, serde_json::Value>,
}

#[derive(Clone)]
pub struct TypedJsonWebhookEvent {
    family: WebhookFamily,
    known: BTreeMap<WebhookField, serde_json::Value>,
    extras: BTreeMap<String, serde_json::Value>,
}
impl TypedJsonWebhookEvent {
    #[must_use]
    pub const fn family(&self) -> WebhookFamily {
        self.family
    }
    #[must_use]
    pub fn known(&self) -> &BTreeMap<WebhookField, serde_json::Value> {
        &self.known
    }
    #[must_use]
    pub fn extras(&self) -> &BTreeMap<String, serde_json::Value> {
        &self.extras
    }
}
impl fmt::Debug for TypedJsonWebhookEvent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TypedJsonWebhookEvent")
            .field("family", &self.family)
            .field("known", &format_args!("[REDACTED; {}]", self.known.len()))
            .field("extras", &format_args!("[REDACTED; {}]", self.extras.len()))
            .finish()
    }
}
impl VerifiedJsonWebhook {
    #[must_use]
    pub const fn family(&self) -> WebhookFamily {
        self.family
    }
    #[must_use]
    pub fn properties(&self) -> &BTreeMap<String, serde_json::Value> {
        &self.properties
    }
    #[must_use]
    pub fn parse_typed(&self) -> TypedJsonWebhookEvent {
        let mut known = BTreeMap::new();
        let mut extras = self.properties.clone();
        for field in self.family.known_fields() {
            if let Some(value) = extras.remove(field.as_str()) {
                known.insert(*field, value);
            }
        }
        TypedJsonWebhookEvent {
            family: self.family,
            known,
            extras,
        }
    }
}
impl fmt::Debug for VerifiedJsonWebhook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VerifiedJsonWebhook")
            .field("family", &self.family)
            .field(
                "properties",
                &format_args!("[REDACTED; {}]", self.properties.len()),
            )
            .finish()
    }
}

pub(crate) fn required<'a>(
    input: &'a VerifiedFormWebhook,
    name: &'static str,
    family: &'static str,
) -> Result<&'a str, super::WebhookParseError> {
    input
        .first(name)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| super::WebhookParseError::new(family, name, "is required"))
}
