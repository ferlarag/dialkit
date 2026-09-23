use super::Node;
use crate::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TwimlFamily {
    Voice,
    Messaging,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TwimlNodeKind {
    Connect,
    ConversationRelay,
    Room,
    ConnectStream,
    VirtualAgent,
    Dial,
    Application,
    Client,
    Conference,
    Number,
    Queue,
    Sip,
    Gather,
    Say,
    Play,
    Pause,
    Pay,
    Prompt,
    Parameter,
    Start,
    Stop,
    Recording,
    Siprec,
    Stream,
    Transcription,
    Enqueue,
    Echo,
    Hangup,
    Leave,
    Record,
    VoiceRedirect,
    Refer,
    Reject,
    Message,
    MessagingRedirect,
    Body,
    Media,
}

impl TwimlNodeKind {
    pub const fn xml_name(self) -> &'static str {
        match self {
            Self::Connect => "Connect",
            Self::ConversationRelay => "ConversationRelay",
            Self::Room => "Room",
            Self::ConnectStream | Self::Stream => "Stream",
            Self::VirtualAgent => "VirtualAgent",
            Self::Dial => "Dial",
            Self::Application => "Application",
            Self::Client => "Client",
            Self::Conference => "Conference",
            Self::Number => "Number",
            Self::Queue => "Queue",
            Self::Sip => "Sip",
            Self::Gather => "Gather",
            Self::Say => "Say",
            Self::Play => "Play",
            Self::Pause => "Pause",
            Self::Pay => "Pay",
            Self::Prompt => "Prompt",
            Self::Parameter => "Parameter",
            Self::Start => "Start",
            Self::Stop => "Stop",
            Self::Recording => "Recording",
            Self::Siprec => "Siprec",
            Self::Transcription => "Transcription",
            Self::Enqueue => "Enqueue",
            Self::Echo => "Echo",
            Self::Hangup => "Hangup",
            Self::Leave => "Leave",
            Self::Record => "Record",
            Self::VoiceRedirect | Self::MessagingRedirect => "Redirect",
            Self::Refer => "Refer",
            Self::Reject => "Reject",
            Self::Message => "Message",
            Self::Body => "Body",
            Self::Media => "Media",
        }
    }

    const fn family(self) -> TwimlFamily {
        match self {
            Self::Message | Self::MessagingRedirect | Self::Body | Self::Media => {
                TwimlFamily::Messaging
            }
            _ => TwimlFamily::Voice,
        }
    }

    const fn root(self) -> bool {
        matches!(
            self,
            Self::Connect
                | Self::Dial
                | Self::Gather
                | Self::Say
                | Self::Play
                | Self::Pause
                | Self::Pay
                | Self::Start
                | Self::Stop
                | Self::Enqueue
                | Self::Echo
                | Self::Hangup
                | Self::Leave
                | Self::Record
                | Self::VoiceRedirect
                | Self::Refer
                | Self::Reject
                | Self::Message
                | Self::MessagingRedirect
        )
    }

    #[must_use]
    pub fn supports_attribute(self, name: TwimlAttributeName) -> bool {
        allows_attribute(self, name)
    }

    #[must_use]
    pub fn supports_child(self, child: Self) -> bool {
        allows_child(self, child)
    }

    /// Reports whether Twilio transfers or ends control after this verb, so
    /// later sibling nodes will not execute.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Hangup | Self::Reject | Self::VoiceRedirect | Self::MessagingRedirect
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TwimlAttributeName {
    Action,
    Method,
    Url,
    StatusCallback,
    StatusCallbackMethod,
    StatusCallbackEvent,
    Name,
    Track,
    ConnectorName,
    WelcomeGreeting,
    Language,
    Voice,
    TtsProvider,
    TranscriptionProvider,
    SpeechModel,
    Interruptible,
    DtmfDetection,
    Timeout,
    TimeLimit,
    HangupOnStar,
    CallerId,
    Record,
    RecordingStatusCallback,
    RecordingStatusCallbackMethod,
    RecordingStatusCallbackEvent,
    AnswerOnBridge,
    RingTone,
    Trim,
    Input,
    SpeechTimeout,
    MaxSpeechTime,
    ProfanityFilter,
    FinishOnKey,
    NumDigits,
    PartialResultCallback,
    PartialResultCallbackMethod,
    Hints,
    BargeIn,
    ActionOnEmptyResult,
    Loop,
    Digits,
    Length,
    PaymentMethod,
    PaymentConnector,
    ChargeAmount,
    Currency,
    Description,
    MaxAttempts,
    SecurityCode,
    PostalCode,
    TokenType,
    WaitUrl,
    WaitUrlMethod,
    WorkflowSid,
    PlayBeep,
    MaxLength,
    Reason,
    ContentType,
    CustomParameters,
    Value,
    For,
    Attempt,
}

impl TwimlAttributeName {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Action => "action",
            Self::Method => "method",
            Self::Url => "url",
            Self::StatusCallback => "statusCallback",
            Self::StatusCallbackMethod => "statusCallbackMethod",
            Self::StatusCallbackEvent => "statusCallbackEvent",
            Self::Name => "name",
            Self::Track => "track",
            Self::ConnectorName => "connectorName",
            Self::WelcomeGreeting => "welcomeGreeting",
            Self::Language => "language",
            Self::Voice => "voice",
            Self::TtsProvider => "ttsProvider",
            Self::TranscriptionProvider => "transcriptionProvider",
            Self::SpeechModel => "speechModel",
            Self::Interruptible => "interruptible",
            Self::DtmfDetection => "dtmfDetection",
            Self::Timeout => "timeout",
            Self::TimeLimit => "timeLimit",
            Self::HangupOnStar => "hangupOnStar",
            Self::CallerId => "callerId",
            Self::Record => "record",
            Self::RecordingStatusCallback => "recordingStatusCallback",
            Self::RecordingStatusCallbackMethod => "recordingStatusCallbackMethod",
            Self::RecordingStatusCallbackEvent => "recordingStatusCallbackEvent",
            Self::AnswerOnBridge => "answerOnBridge",
            Self::RingTone => "ringTone",
            Self::Trim => "trim",
            Self::Input => "input",
            Self::SpeechTimeout => "speechTimeout",
            Self::MaxSpeechTime => "maxSpeechTime",
            Self::ProfanityFilter => "profanityFilter",
            Self::FinishOnKey => "finishOnKey",
            Self::NumDigits => "numDigits",
            Self::PartialResultCallback => "partialResultCallback",
            Self::PartialResultCallbackMethod => "partialResultCallbackMethod",
            Self::Hints => "hints",
            Self::BargeIn => "bargeIn",
            Self::ActionOnEmptyResult => "actionOnEmptyResult",
            Self::Loop => "loop",
            Self::Digits => "digits",
            Self::Length => "length",
            Self::PaymentMethod => "paymentMethod",
            Self::PaymentConnector => "paymentConnector",
            Self::ChargeAmount => "chargeAmount",
            Self::Currency => "currency",
            Self::Description => "description",
            Self::MaxAttempts => "maxAttempts",
            Self::SecurityCode => "securityCode",
            Self::PostalCode => "postalCode",
            Self::TokenType => "tokenType",
            Self::WaitUrl => "waitUrl",
            Self::WaitUrlMethod => "waitUrlMethod",
            Self::WorkflowSid => "workflowSid",
            Self::PlayBeep => "playBeep",
            Self::MaxLength => "maxLength",
            Self::Reason => "reason",
            Self::ContentType => "contentType",
            Self::CustomParameters => "customParameters",
            Self::Value => "value",
            Self::For => "for",
            Self::Attempt => "attempt",
        }
    }
}

#[derive(Clone, Debug)]
pub struct TwimlNode {
    kind: TwimlNodeKind,
    text: Option<String>,
    attributes: Vec<(TwimlAttributeName, String)>,
    children: Vec<Self>,
}

impl TwimlNode {
    #[must_use]
    pub const fn new(kind: TwimlNodeKind) -> Self {
        Self {
            kind,
            text: None,
            attributes: Vec::new(),
            children: Vec::new(),
        }
    }

    #[must_use]
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn attribute(
        mut self,
        name: TwimlAttributeName,
        value: impl Into<String>,
    ) -> Result<Self, Error> {
        if !allows_attribute(self.kind, name) {
            return Err(Error::Xml(format!(
                "{} does not allow {}",
                self.kind.xml_name(),
                name.as_str()
            )));
        }
        let value = value.into();
        if self
            .attributes
            .iter()
            .any(|(existing, _)| *existing == name)
        {
            return Err(Error::Xml(format!(
                "{} cannot repeat {}",
                self.kind.xml_name(),
                name.as_str()
            )));
        }
        if !(self.kind == TwimlNodeKind::Gather
            && name == TwimlAttributeName::Timeout
            && value == "auto")
            && !(self.kind == TwimlNodeKind::Record
                && name == TwimlAttributeName::Timeout
                && value == "0")
            && !(self.kind == TwimlNodeKind::Record && name == TwimlAttributeName::FinishOnKey)
        {
            validate_attribute(name, &value)?;
        }
        if self.kind == TwimlNodeKind::Record
            && name == TwimlAttributeName::FinishOnKey
            && (value.is_empty()
                || !value
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || byte == b'*' || byte == b'#'))
        {
            return Err(Error::Xml(
                "Record finishOnKey must contain digits, * or #".into(),
            ));
        }
        if self.kind == TwimlNodeKind::ConnectStream
            && name == TwimlAttributeName::Track
            && value != "inbound_track"
        {
            return Err(Error::Xml(
                "Connect Stream can receive only inbound_track".into(),
            ));
        }
        self.attributes.push((name, value));
        Ok(self)
    }

    pub fn child(mut self, child: Self) -> Result<Self, Error> {
        if !allows_child(self.kind, child.kind) {
            return Err(Error::Xml(format!(
                "{} is not allowed under {}",
                child.kind.xml_name(),
                self.kind.xml_name()
            )));
        }
        self.children.push(child);
        Ok(self)
    }

    pub(crate) fn into_root_node(self, family: TwimlFamily) -> Result<Node, Error> {
        if self.kind.family() != family || !self.kind.root() {
            return Err(Error::Xml(format!(
                "{} is not a top-level node for this response",
                self.kind.xml_name()
            )));
        }
        self.into_node(None)
    }

    fn into_node(self, parent: Option<TwimlNodeKind>) -> Result<Node, Error> {
        if self.kind == TwimlNodeKind::VirtualAgent
            && !self
                .attributes
                .iter()
                .any(|(name, _)| *name == TwimlAttributeName::ConnectorName)
        {
            return Err(Error::Xml("VirtualAgent requires connectorName".into()));
        }
        if self.kind == TwimlNodeKind::ConversationRelay
            && !self
                .attributes
                .iter()
                .any(|(name, _)| *name == TwimlAttributeName::Url)
        {
            return Err(Error::Xml("ConversationRelay requires a wss URL".into()));
        }
        if matches!(
            (parent, self.kind),
            (Some(TwimlNodeKind::Start), TwimlNodeKind::Stream)
                | (Some(TwimlNodeKind::Connect), TwimlNodeKind::ConnectStream)
        ) && !self
            .attributes
            .iter()
            .any(|(name, _)| *name == TwimlAttributeName::Url)
        {
            return Err(Error::Xml("Stream requires a wss URL".into()));
        }
        if parent == Some(TwimlNodeKind::Start)
            && self.kind == TwimlNodeKind::Siprec
            && !self
                .attributes
                .iter()
                .any(|(name, _)| *name == TwimlAttributeName::ConnectorName)
        {
            return Err(Error::Xml("Siprec requires connectorName".into()));
        }
        if parent == Some(TwimlNodeKind::Stop)
            && matches!(
                self.kind,
                TwimlNodeKind::Recording
                    | TwimlNodeKind::Siprec
                    | TwimlNodeKind::Stream
                    | TwimlNodeKind::Transcription
            )
            && !self
                .attributes
                .iter()
                .any(|(name, _)| *name == TwimlAttributeName::Name)
        {
            return Err(Error::Xml(format!(
                "{} requires name under Stop",
                self.kind.xml_name()
            )));
        }
        if parent == Some(TwimlNodeKind::Pay)
            && self.kind == TwimlNodeKind::Prompt
            && !self
                .attributes
                .iter()
                .any(|(name, _)| *name == TwimlAttributeName::For)
        {
            return Err(Error::Xml("Prompt requires a payment step".into()));
        }
        if parent == Some(TwimlNodeKind::Gather)
            && self.kind == TwimlNodeKind::Play
            && self
                .attributes
                .iter()
                .any(|(name, _)| *name == TwimlAttributeName::Digits)
        {
            return Err(Error::Xml("Gather cannot contain Play digits".into()));
        }
        let name = self.kind.xml_name();
        let mut node = if self.children.is_empty() {
            self.text
                .map_or_else(|| Node::empty(name), |text| Node::text(name, text))
        } else {
            if self.text.is_some() {
                return Err(Error::Xml(format!(
                    "{name} cannot mix text and child nodes"
                )));
            }
            let children = self
                .children
                .into_iter()
                .map(|child| child.into_node(Some(self.kind)))
                .collect::<Result<Vec<_>, _>>()?;
            Node::children(name, children)
        };
        for (attribute, value) in self.attributes {
            node = node.attribute(attribute.as_str(), value);
        }
        Ok(node)
    }
}

fn validate_attribute(name: TwimlAttributeName, value: &str) -> Result<(), Error> {
    if value.trim().is_empty() && name != TwimlAttributeName::FinishOnKey {
        return Err(Error::Xml(format!("{} cannot be empty", name.as_str())));
    }
    if matches!(
        name,
        TwimlAttributeName::Action
            | TwimlAttributeName::Url
            | TwimlAttributeName::StatusCallback
            | TwimlAttributeName::RecordingStatusCallback
            | TwimlAttributeName::PartialResultCallback
            | TwimlAttributeName::WaitUrl
    ) {
        let url = url::Url::parse(value)
            .map_err(|_| Error::Xml(format!("{} requires an absolute URL", name.as_str())))?;
        if !matches!(url.scheme(), "http" | "https" | "wss") {
            return Err(Error::Xml(format!(
                "{} URL scheme is unsupported",
                name.as_str()
            )));
        }
    }
    if matches!(
        name,
        TwimlAttributeName::Method
            | TwimlAttributeName::StatusCallbackMethod
            | TwimlAttributeName::RecordingStatusCallbackMethod
            | TwimlAttributeName::PartialResultCallbackMethod
            | TwimlAttributeName::WaitUrlMethod
    ) && !matches!(value, "GET" | "POST")
    {
        return Err(Error::Xml(format!("{} must be GET or POST", name.as_str())));
    }
    if matches!(
        name,
        TwimlAttributeName::Timeout
            | TwimlAttributeName::TimeLimit
            | TwimlAttributeName::MaxSpeechTime
            | TwimlAttributeName::NumDigits
            | TwimlAttributeName::Length
            | TwimlAttributeName::MaxAttempts
            | TwimlAttributeName::MaxLength
    ) && value.parse::<u32>().ok().is_none_or(|number| number == 0)
    {
        return Err(Error::Xml(format!(
            "{} must be a positive integer",
            name.as_str()
        )));
    }
    if name == TwimlAttributeName::Loop && value.parse::<u32>().is_err() {
        return Err(Error::Xml("loop must be a non-negative integer".into()));
    }
    if name == TwimlAttributeName::SpeechTimeout
        && value != "auto"
        && value.parse::<u32>().ok().is_none_or(|number| number == 0)
    {
        return Err(Error::Xml(
            "speechTimeout must be auto or a positive integer".into(),
        ));
    }
    if matches!(
        name,
        TwimlAttributeName::HangupOnStar
            | TwimlAttributeName::AnswerOnBridge
            | TwimlAttributeName::ProfanityFilter
            | TwimlAttributeName::BargeIn
            | TwimlAttributeName::ActionOnEmptyResult
            | TwimlAttributeName::PlayBeep
            | TwimlAttributeName::DtmfDetection
            | TwimlAttributeName::SecurityCode
    ) && !matches!(value, "true" | "false")
    {
        return Err(Error::Xml(format!(
            "{} must be true or false",
            name.as_str()
        )));
    }
    if name == TwimlAttributeName::Interruptible
        && !matches!(value, "none" | "dtmf" | "speech" | "any" | "true" | "false")
    {
        return Err(Error::Xml("interruptible has an unsupported value".into()));
    }
    if name == TwimlAttributeName::TtsProvider
        && !["Google", "Amazon", "ElevenLabs"]
            .iter()
            .any(|provider| value.eq_ignore_ascii_case(provider))
    {
        return Err(Error::Xml("ttsProvider has an unsupported value".into()));
    }
    if name == TwimlAttributeName::TranscriptionProvider
        && !["Google", "Deepgram"]
            .iter()
            .any(|provider| value.eq_ignore_ascii_case(provider))
    {
        return Err(Error::Xml(
            "transcriptionProvider has an unsupported value".into(),
        ));
    }
    if name == TwimlAttributeName::Track
        && !matches!(value, "inbound_track" | "outbound_track" | "both_tracks")
    {
        return Err(Error::Xml("track has an unsupported value".into()));
    }
    if name == TwimlAttributeName::Trim && !matches!(value, "trim-silence" | "do-not-trim") {
        return Err(Error::Xml("trim has an unsupported value".into()));
    }
    if name == TwimlAttributeName::TokenType
        && !matches!(value, "one-time" | "reusable" | "payment-method")
    {
        return Err(Error::Xml("tokenType has an unsupported value".into()));
    }
    if name == TwimlAttributeName::FinishOnKey
        && !value.is_empty()
        && (value.len() != 1
            || !value
                .bytes()
                .all(|byte| byte.is_ascii_digit() || byte == b'*' || byte == b'#'))
    {
        return Err(Error::Xml("finishOnKey must be one digit, * or #".into()));
    }
    if name == TwimlAttributeName::MaxAttempts
        && value
            .parse::<u32>()
            .ok()
            .is_none_or(|number| !(1..=3).contains(&number))
    {
        return Err(Error::Xml("maxAttempts must be between 1 and 3".into()));
    }
    if name == TwimlAttributeName::MaxLength
        && value
            .parse::<u32>()
            .ok()
            .is_none_or(|number| !(2..=86_400).contains(&number))
    {
        return Err(Error::Xml("maxLength must be between 2 and 86400".into()));
    }
    if name == TwimlAttributeName::ChargeAmount
        && value
            .parse::<f64>()
            .ok()
            .is_none_or(|number| !number.is_finite() || !(0.0..=1_000_000.0).contains(&number))
    {
        return Err(Error::Xml(
            "chargeAmount must be between 0 and 1000000".into(),
        ));
    }
    if name == TwimlAttributeName::For
        && !matches!(
            value,
            "payment-card-number"
                | "expiration-date"
                | "security-code"
                | "postal-code"
                | "bank-routing-number"
                | "bank-account-number"
                | "payment-processing"
        )
    {
        return Err(Error::Xml(
            "Prompt for has an unsupported payment step".into(),
        ));
    }
    if name == TwimlAttributeName::Attempt
        && (value.split_ascii_whitespace().next().is_none()
            || !value.split_ascii_whitespace().all(|token| {
                token
                    .parse::<u8>()
                    .ok()
                    .is_some_and(|number| (1..=10).contains(&number))
            }))
    {
        return Err(Error::Xml(
            "Prompt attempt must contain values from 1 to 10".into(),
        ));
    }
    if name == TwimlAttributeName::Digits
        && !value.bytes().all(|byte| {
            byte.is_ascii_digit()
                || matches!(byte, b'A' | b'B' | b'C' | b'D' | b'W' | b'w' | b'#' | b'*')
        })
    {
        return Err(Error::Xml(
            "Play digits has an unsupported character".into(),
        ));
    }
    if name == TwimlAttributeName::Hints
        && (value.split(',').count() > 500
            || value
                .split(',')
                .any(|hint| hint.trim().is_empty() || hint.chars().count() > 100))
    {
        return Err(Error::Xml(
            "Gather hints requires 1-500 entries of at most 100 characters".into(),
        ));
    }
    Ok(())
}

fn allows_attribute(kind: TwimlNodeKind, name: TwimlAttributeName) -> bool {
    use TwimlAttributeName as A;
    use TwimlNodeKind as N;
    match kind {
        N::Connect => matches!(name, A::Action | A::Method),
        N::ConversationRelay => matches!(
            name,
            A::Url
                | A::WelcomeGreeting
                | A::Language
                | A::Voice
                | A::TtsProvider
                | A::TranscriptionProvider
                | A::SpeechModel
                | A::Interruptible
                | A::DtmfDetection
        ),
        N::Room => matches!(name, A::Name | A::StatusCallback | A::StatusCallbackMethod),
        N::ConnectStream => matches!(
            name,
            A::Url | A::Name | A::Track | A::StatusCallback | A::StatusCallbackMethod
        ),
        N::VirtualAgent => matches!(
            name,
            A::ConnectorName | A::StatusCallback | A::StatusCallbackMethod
        ),
        N::Dial => matches!(
            name,
            A::Action
                | A::Method
                | A::Timeout
                | A::TimeLimit
                | A::HangupOnStar
                | A::CallerId
                | A::Record
                | A::RecordingStatusCallback
                | A::RecordingStatusCallbackMethod
                | A::RecordingStatusCallbackEvent
                | A::AnswerOnBridge
                | A::RingTone
                | A::Trim
        ),
        N::Application | N::Queue => matches!(name, A::Url | A::Method),
        N::Client => matches!(name, A::StatusCallback | A::StatusCallbackMethod),
        N::Number => matches!(
            name,
            A::StatusCallback | A::StatusCallbackMethod | A::StatusCallbackEvent
        ),
        N::Conference => matches!(
            name,
            A::StatusCallback
                | A::StatusCallbackMethod
                | A::StatusCallbackEvent
                | A::Record
                | A::RecordingStatusCallback
                | A::RecordingStatusCallbackMethod
                | A::RecordingStatusCallbackEvent
        ),
        N::Sip => matches!(
            name,
            A::Url
                | A::Method
                | A::StatusCallback
                | A::StatusCallbackMethod
                | A::StatusCallbackEvent
        ),
        N::Gather => matches!(
            name,
            A::Input
                | A::Action
                | A::Method
                | A::Timeout
                | A::SpeechTimeout
                | A::MaxSpeechTime
                | A::ProfanityFilter
                | A::FinishOnKey
                | A::NumDigits
                | A::PartialResultCallback
                | A::PartialResultCallbackMethod
                | A::Language
                | A::Hints
                | A::BargeIn
                | A::ActionOnEmptyResult
                | A::SpeechModel
        ),
        N::Say => matches!(name, A::Voice | A::Language | A::Loop),
        N::Play => matches!(name, A::Loop | A::Digits),
        N::Pause => name == A::Length,
        N::Pay => matches!(
            name,
            A::Action
                | A::Method
                | A::PaymentMethod
                | A::PaymentConnector
                | A::ChargeAmount
                | A::Currency
                | A::Description
                | A::Input
                | A::Timeout
                | A::MaxAttempts
                | A::SecurityCode
                | A::PostalCode
                | A::TokenType
                | A::StatusCallback
        ),
        N::Prompt => matches!(name, A::For | A::Attempt),
        N::Parameter => matches!(name, A::Name | A::Value),
        N::Recording => matches!(
            name,
            A::Name | A::StatusCallback | A::StatusCallbackMethod | A::StatusCallbackEvent
        ),
        N::Siprec => matches!(
            name,
            A::Name
                | A::ConnectorName
                | A::StatusCallback
                | A::StatusCallbackMethod
                | A::StatusCallbackEvent
        ),
        N::Stream => matches!(
            name,
            A::Name | A::Url | A::Track | A::StatusCallback | A::StatusCallbackMethod
        ),
        N::Transcription => matches!(
            name,
            A::Name | A::StatusCallback | A::StatusCallbackMethod | A::Language | A::SpeechModel
        ),
        N::Enqueue => matches!(
            name,
            A::Action | A::Method | A::WaitUrl | A::WaitUrlMethod | A::WorkflowSid
        ),
        N::Record => matches!(
            name,
            A::Action
                | A::Method
                | A::Timeout
                | A::FinishOnKey
                | A::MaxLength
                | A::PlayBeep
                | A::Trim
                | A::RecordingStatusCallback
                | A::RecordingStatusCallbackMethod
                | A::RecordingStatusCallbackEvent
        ),
        N::VoiceRedirect | N::MessagingRedirect => name == A::Method,
        N::Refer => matches!(name, A::Action | A::Method),
        N::Reject => name == A::Reason,
        N::Message => matches!(name, A::Action | A::Method | A::StatusCallback),
        N::Media => name == A::ContentType,
        N::Start | N::Stop | N::Echo | N::Hangup | N::Leave | N::Body => false,
    }
}

fn allows_child(parent: TwimlNodeKind, child: TwimlNodeKind) -> bool {
    use TwimlNodeKind as N;
    matches!(
        (parent, child),
        (
            N::Connect,
            N::ConversationRelay | N::Room | N::ConnectStream | N::VirtualAgent
        ) | (
            N::Dial,
            N::Application | N::Client | N::Conference | N::Number | N::Queue | N::Sip
        ) | (N::Gather, N::Say | N::Play | N::Pause)
            | (N::Pay, N::Prompt | N::Parameter)
            | (
                N::Start | N::Stop,
                N::Recording | N::Siprec | N::Stream | N::Transcription
            )
            | (
                N::ConnectStream | N::VirtualAgent | N::Client | N::Siprec | N::Stream,
                N::Parameter
            )
            | (N::Refer, N::Sip)
            | (N::Message, N::Body | N::Media)
    )
}
