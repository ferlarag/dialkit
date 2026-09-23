//! Typed, ordered TwiML builders.

mod grammar;
mod messaging;
mod voice;

pub use grammar::{TwimlAttributeName, TwimlNode, TwimlNodeKind};
pub use messaging::{MessageNode, MessagingResponseBuilder};
pub use voice::*;

use crate::Error;
use quick_xml::{
    Writer,
    events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event},
};

#[derive(Clone)]
pub(crate) struct Node {
    name: &'static str,
    attributes: Vec<(&'static str, String)>,
    text: Option<String>,
    children: Vec<Node>,
}

impl std::fmt::Debug for Node {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Node")
            .field("name", &self.name)
            .field(
                "attributes",
                &self
                    .attributes
                    .iter()
                    .map(|(name, _)| name)
                    .collect::<Vec<_>>(),
            )
            .field("text", &self.text.as_ref().map(|_| "[REDACTED]"))
            .field("children", &self.children)
            .finish()
    }
}

impl Node {
    pub(crate) fn text(name: &'static str, text: impl Into<String>) -> Self {
        Self {
            name,
            attributes: vec![],
            text: Some(text.into()),
            children: vec![],
        }
    }
    pub(crate) fn empty(name: &'static str) -> Self {
        Self {
            name,
            attributes: vec![],
            text: None,
            children: vec![],
        }
    }
    pub(crate) fn children(name: &'static str, children: Vec<Self>) -> Self {
        Self {
            name,
            attributes: vec![],
            text: None,
            children,
        }
    }
    pub(crate) fn attribute(mut self, name: &'static str, value: impl Into<String>) -> Self {
        self.attributes.push((name, value.into()));
        self
    }

    fn validate(&self) -> Result<(), Error> {
        match self.name {
            "Say" | "Body" | "Enqueue" | "Room" | "Application" | "Conference" | "Number"
            | "Queue" | "Sip" | "Prompt" => {
                if self
                    .text
                    .as_deref()
                    .is_none_or(|value| value.trim().is_empty())
                {
                    return Err(Error::Xml(format!(
                        "{} requires a non-empty value",
                        self.name
                    )));
                }
            }
            "Client" => {
                if self.children.is_empty()
                    && self
                        .text
                        .as_deref()
                        .is_none_or(|value| value.trim().is_empty())
                {
                    return Err(Error::Xml(
                        "Client requires an identity or Parameter child".into(),
                    ));
                }
            }
            "Dial" => {
                if self.children.is_empty()
                    && self
                        .text
                        .as_deref()
                        .is_none_or(|value| value.trim().is_empty())
                {
                    return Err(Error::Xml("Dial requires a destination".into()));
                }
            }
            "Play" => {
                let digits = self.attributes.iter().any(|(name, _)| *name == "digits");
                if digits && self.text.as_deref().is_some_and(|value| !value.is_empty()) {
                    return Err(Error::Xml("Play cannot combine digits and a URL".into()));
                }
                if !digits {
                    let value = self
                        .text
                        .as_deref()
                        .filter(|value| !value.trim().is_empty())
                        .ok_or_else(|| Error::Xml("Play requires a URL or digits".into()))?;
                    let url = url::Url::parse(value)
                        .map_err(|_| Error::Xml("Play requires an absolute URL".into()))?;
                    if !matches!(url.scheme(), "http" | "https") {
                        return Err(Error::Xml("Play URL must use HTTP or HTTPS".into()));
                    }
                }
            }
            "Media" | "Redirect" => {
                let value = self
                    .text
                    .as_deref()
                    .filter(|value| !value.trim().is_empty())
                    .ok_or_else(|| Error::Xml(format!("{} requires a URL", self.name)))?;
                let url = url::Url::parse(value)
                    .map_err(|_| Error::Xml(format!("{} requires an absolute URL", self.name)))?;
                if !matches!(url.scheme(), "http" | "https") {
                    return Err(Error::Xml(format!(
                        "{} URL must use HTTP or HTTPS",
                        self.name
                    )));
                }
            }
            "Pause" => {
                let length = self
                    .attributes
                    .iter()
                    .find(|(name, _)| *name == "length")
                    .and_then(|(_, value)| value.parse::<u32>().ok())
                    .unwrap_or(0);
                if !(1..=60).contains(&length) {
                    return Err(Error::Xml(
                        "Pause length must be between 1 and 60 seconds".into(),
                    ));
                }
            }
            "Connect" | "Start" | "Stop" | "Pay" | "Refer" => {
                if self.children.is_empty() {
                    return Err(Error::Xml(format!("{} requires a child noun", self.name)));
                }
            }
            "Parameter" => {
                for required in ["name", "value"] {
                    if self
                        .attributes
                        .iter()
                        .find(|(name, _)| *name == required)
                        .is_none_or(|(_, value)| value.is_empty())
                    {
                        return Err(Error::Xml(format!("Parameter requires {required}")));
                    }
                }
            }
            _ => {}
        }
        for (name, value) in &self.attributes {
            if matches!(*name, "action" | "url" | "statusCallback") {
                let url = url::Url::parse(value)
                    .map_err(|_| Error::Xml(format!("{} requires an absolute URL", name)))?;
                if !matches!(url.scheme(), "http" | "https" | "wss") {
                    return Err(Error::Xml(format!(
                        "{} uses an unsupported URL scheme",
                        name
                    )));
                }
            }
            if self.name == "Pay" && *name == "action" {
                let url = url::Url::parse(value)
                    .map_err(|_| Error::Xml("Pay action requires an absolute HTTPS URL".into()))?;
                if url.scheme() != "https" {
                    return Err(Error::Xml("Pay action requires HTTPS".into()));
                }
            }
            if self.name == "Pay" && *name == "method" && value != "POST" {
                return Err(Error::Xml("Pay action requires POST".into()));
            }
            if *name == "input"
                && ((self.name == "Gather"
                    && !matches!(value.as_str(), "dtmf" | "speech" | "dtmf speech"))
                    || (self.name == "Pay" && value != "dtmf"))
            {
                return Err(Error::Xml(format!("{} input is not supported", self.name)));
            }
            if *name == "paymentMethod"
                && !matches!(value.as_str(), "card" | "ach" | "credit-card" | "ach-debit")
            {
                return Err(Error::Xml("Pay paymentMethod is not supported".into()));
            }
            if self.name == "Reject"
                && *name == "reason"
                && !matches!(value.as_str(), "rejected" | "busy")
            {
                return Err(Error::Xml("Reject reason must be rejected or busy".into()));
            }
            if *name == "speechTimeout"
                && value != "auto"
                && value.parse::<u32>().ok().is_none_or(|number| number == 0)
            {
                return Err(Error::Xml(
                    "speechTimeout must be auto or a positive integer".into(),
                ));
            }
            if *name == "record"
                && self.name == "Dial"
                && !matches!(
                    value.as_str(),
                    "do-not-record"
                        | "record-from-answer"
                        | "record-from-ringing"
                        | "record-from-answer-dual"
                        | "record-from-ringing-dual"
                        | "true"
                        | "false"
                )
            {
                return Err(Error::Xml("Dial record has an unsupported value".into()));
            }
            if *name == "trim" && !matches!(value.as_str(), "trim-silence" | "do-not-trim") {
                return Err(Error::Xml("trim has an unsupported value".into()));
            }
            if *name == "maxLength"
                && value
                    .parse::<u32>()
                    .ok()
                    .is_none_or(|number| !(2..=86_400).contains(&number))
            {
                return Err(Error::Xml("maxLength must be between 2 and 86400".into()));
            }
            if *name == "finishOnKey"
                && !(self.name == "Gather" && value.is_empty())
                && ((self.name != "Record" && value.len() != 1)
                    || value.is_empty()
                    || !value
                        .bytes()
                        .all(|byte| byte.is_ascii_digit() || byte == b'*' || byte == b'#'))
            {
                return Err(Error::Xml("finishOnKey must be one digit, * or #".into()));
            }
            if *name == "track" {
                let valid = if self.name == "Stream" {
                    matches!(
                        value.as_str(),
                        "inbound_track" | "outbound_track" | "both_tracks"
                    )
                } else {
                    true
                };
                if !valid {
                    return Err(Error::Xml("Stream track has an unsupported value".into()));
                }
            }
            if *name == "maxAttempts"
                && value
                    .parse::<u32>()
                    .ok()
                    .is_none_or(|number| !(1..=3).contains(&number))
            {
                return Err(Error::Xml("Pay maxAttempts must be between 1 and 3".into()));
            }
            if *name == "tokenType"
                && !matches!(value.as_str(), "one-time" | "reusable" | "payment-method")
            {
                return Err(Error::Xml("Pay tokenType has an unsupported value".into()));
            }
            if *name == "chargeAmount"
                && value.parse::<f64>().ok().is_none_or(|number| {
                    !number.is_finite() || !(0.0..=1_000_000.0).contains(&number)
                })
            {
                return Err(Error::Xml(
                    "Pay chargeAmount must be between 0 and 1000000".into(),
                ));
            }
            if self.name == "Prompt"
                && *name == "for"
                && !matches!(
                    value.as_str(),
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
            if matches!(self.name, "Stream" | "ConversationRelay") && *name == "url" {
                let url = url::Url::parse(value).map_err(|_| {
                    Error::Xml("WebSocket noun requires an absolute wss URL".into())
                })?;
                if url.scheme() != "wss" || (self.name == "Stream" && url.query().is_some()) {
                    return Err(Error::Xml(
                        "WebSocket noun URL must use wss; Stream URL must omit query parameters"
                            .into(),
                    ));
                }
            }
            if self.name == "Conference"
                && *name == "record"
                && !matches!(value.as_str(), "do-not-record" | "record-from-start")
            {
                return Err(Error::Xml(
                    "Conference record has an unsupported value".into(),
                ));
            }
            if self.name == "Siprec" && *name == "connectorName" && value.trim().is_empty() {
                return Err(Error::Xml("Siprec requires a connector name".into()));
            }
            if *name == "recordingStatusCallbackEvent"
                && !event_list_allowed(value, &["in-progress", "completed", "absent"])
            {
                return Err(Error::Xml(
                    "recordingStatusCallbackEvent has an unsupported event".into(),
                ));
            }
            if *name == "statusCallbackEvent" {
                let valid = if self.name == "Conference" {
                    event_list_allowed(
                        value,
                        &[
                            "start",
                            "end",
                            "join",
                            "leave",
                            "mute",
                            "hold",
                            "modify",
                            "speaker",
                            "announcement",
                        ],
                    )
                } else if matches!(self.name, "Number" | "Sip" | "Client") {
                    event_list_allowed(value, &["initiated", "ringing", "answered", "completed"])
                } else {
                    true
                };
                if !valid {
                    return Err(Error::Xml(
                        "statusCallbackEvent has an unsupported event".into(),
                    ));
                }
            }
        }
        for child in &self.children {
            child.validate()?;
        }
        Ok(())
    }
}

fn event_list_allowed(value: &str, allowed: &[&str]) -> bool {
    let mut events = value.split_ascii_whitespace();
    events.next().is_some_and(|event| allowed.contains(&event))
        && events.all(|event| allowed.contains(&event))
}

#[derive(Clone)]
pub struct TwimlResponse {
    nodes: Vec<Node>,
}

impl std::fmt::Debug for TwimlResponse {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("TwimlResponse")
            .field("nodes", &self.nodes)
            .finish()
    }
}

impl TwimlResponse {
    #[must_use]
    pub fn voice() -> VoiceResponseBuilder {
        VoiceResponseBuilder::new()
    }
    #[must_use]
    pub fn messaging() -> MessagingResponseBuilder {
        MessagingResponseBuilder::new()
    }
    pub(crate) fn new(nodes: Vec<Node>) -> Result<Self, Error> {
        if nodes.is_empty() {
            return Err(Error::Xml("response requires at least one node".into()));
        }
        for node in &nodes {
            node.validate()?;
        }
        Ok(Self { nodes })
    }
    pub fn to_xml(&self) -> Result<String, Error> {
        let mut writer = Writer::new(Vec::new());
        writer
            .write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))
            .map_err(xml_error)?;
        writer
            .write_event(Event::Start(BytesStart::new("Response")))
            .map_err(xml_error)?;
        for node in &self.nodes {
            write_node(&mut writer, node)?;
        }
        writer
            .write_event(Event::End(BytesEnd::new("Response")))
            .map_err(xml_error)?;
        String::from_utf8(writer.into_inner())
            .map_err(|_| Error::Xml("renderer produced non-UTF-8 output".into()))
    }
}

fn write_node(writer: &mut Writer<Vec<u8>>, node: &Node) -> Result<(), Error> {
    let mut start = BytesStart::new(node.name);
    for (name, value) in &node.attributes {
        start.push_attribute((*name, value.as_str()));
    }
    if node.text.is_none() && node.children.is_empty() {
        writer.write_event(Event::Empty(start)).map_err(xml_error)?;
        return Ok(());
    }
    writer.write_event(Event::Start(start)).map_err(xml_error)?;
    if let Some(text) = &node.text {
        writer
            .write_event(Event::Text(BytesText::new(text)))
            .map_err(xml_error)?;
    }
    for child in &node.children {
        write_node(writer, child)?;
    }
    writer
        .write_event(Event::End(BytesEnd::new(node.name)))
        .map_err(xml_error)?;
    Ok(())
}

fn xml_error(error: std::io::Error) -> Error {
    Error::Xml(error.to_string())
}
