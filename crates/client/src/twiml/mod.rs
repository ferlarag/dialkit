//! Typed, ordered TwiML builders.

mod messaging;
mod voice;

pub use messaging::{MessageNode, MessagingResponseBuilder};
pub use voice::{Dial, Gather, VoiceResponseBuilder};

use crate::Error;
use quick_xml::{
    Writer,
    events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event},
};

#[derive(Clone, Debug)]
pub(crate) struct Node {
    name: &'static str,
    attributes: Vec<(&'static str, String)>,
    text: Option<String>,
    children: Vec<Node>,
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
            "Say" | "Dial" | "Body" => {
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
            "Play" | "Media" | "Redirect" => {
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
            _ => {}
        }
        for child in &self.children {
            child.validate()?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct TwimlResponse {
    nodes: Vec<Node>,
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
