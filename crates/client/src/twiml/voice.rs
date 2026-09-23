use super::{Node, TwimlResponse};
use crate::Error;

mod connect;
mod control;
mod dial;
mod gather;
mod pay;
mod start_stop;

pub use connect::*;
pub use control::*;
pub use pay::*;
pub use start_stop::*;

#[derive(Clone, Debug)]
pub struct Gather {
    children: Vec<Node>,
    attributes: Vec<(&'static str, String)>,
}
impl Gather {
    #[must_use]
    pub fn new() -> Self {
        Self {
            children: vec![],
            attributes: vec![],
        }
    }
    #[must_use]
    pub fn say(mut self, text: impl Into<String>) -> Self {
        self.children.push(Node::text("Say", text));
        self
    }
    #[must_use]
    pub fn play(mut self, url: impl Into<String>) -> Self {
        self.children.push(Node::text("Play", url));
        self
    }
    #[must_use]
    pub fn pause(mut self, seconds: u32) -> Self {
        self.children
            .push(Node::empty("Pause").attribute("length", seconds.to_string()));
        self
    }
    #[must_use]
    pub fn input(mut self, value: impl Into<String>) -> Self {
        self.attributes.push(("input", value.into()));
        self
    }
    #[must_use]
    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.attributes.push(("action", value.into()));
        self
    }
    fn node(self) -> Result<Node, Error> {
        if self.children.is_empty() {
            return Err(Error::Xml("Gather requires Say or Play content".into()));
        }
        let mut node = Node::children("Gather", self.children);
        for (name, value) in self.attributes {
            node = node.attribute(name, value);
        }
        Ok(node)
    }
}
impl Default for Gather {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct Dial {
    destination: Option<String>,
    children: Vec<Node>,
    attributes: Vec<(&'static str, String)>,
}
impl Dial {
    #[must_use]
    pub fn new(destination: impl Into<String>) -> Self {
        Self {
            destination: Some(destination.into()),
            children: vec![],
            attributes: vec![],
        }
    }

    #[must_use]
    pub fn nouns() -> Self {
        Self {
            destination: None,
            children: vec![],
            attributes: vec![],
        }
    }

    fn node(self) -> Result<Node, Error> {
        if let Some(destination) = self.destination {
            return Ok(Node::text("Dial", destination));
        }
        if self.children.is_empty() {
            return Err(Error::Xml("Dial requires a destination noun".into()));
        }
        let mut node = Node::children("Dial", self.children);
        for (name, value) in self.attributes {
            node = node.attribute(name, value);
        }
        Ok(node)
    }
}

#[derive(Debug, Default)]
pub struct VoiceResponseBuilder {
    nodes: Vec<Node>,
    error: Option<Error>,
}
impl VoiceResponseBuilder {
    pub(crate) fn new() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn say(mut self, text: impl Into<String>) -> Self {
        self.nodes.push(Node::text("Say", text));
        self
    }
    #[must_use]
    pub fn play(mut self, url: impl Into<String>) -> Self {
        self.nodes.push(Node::text("Play", url));
        self
    }
    #[must_use]
    pub fn gather(mut self, gather: Gather) -> Self {
        match gather.node() {
            Ok(node) => self.nodes.push(node),
            Err(error) => self.error = Some(error),
        }
        self
    }
    #[must_use]
    pub fn dial(mut self, dial: Dial) -> Self {
        match dial.node() {
            Ok(node) => self.nodes.push(node),
            Err(error) => self.error = Some(error),
        }
        self
    }
    #[must_use]
    pub fn connect(mut self, value: Connect) -> Self {
        self.nodes.push(value.node());
        self
    }
    #[must_use]
    pub fn pay(mut self, value: Pay) -> Self {
        self.nodes.push(value.node());
        self
    }
    #[must_use]
    pub fn start(mut self, value: Start) -> Self {
        self.nodes.push(value.node());
        self
    }
    #[must_use]
    pub fn stop(mut self, value: Stop) -> Self {
        self.nodes.push(value.node());
        self
    }
    #[must_use]
    pub fn enqueue(mut self, value: Enqueue) -> Self {
        self.nodes.push(value.node());
        self
    }
    #[must_use]
    pub fn echo(mut self) -> Self {
        self.nodes.push(Node::empty("Echo"));
        self
    }
    #[must_use]
    pub fn leave(mut self) -> Self {
        self.nodes.push(Node::empty("Leave"));
        self
    }
    #[must_use]
    pub fn refer(mut self, value: Refer) -> Self {
        self.nodes.push(value.node());
        self
    }
    #[must_use]
    pub fn reject(mut self, reason: RejectReason) -> Self {
        self.nodes
            .push(Node::empty("Reject").attribute("reason", reason.as_str()));
        self
    }
    #[must_use]
    pub fn record(mut self) -> Self {
        self.nodes.push(Node::empty("Record"));
        self
    }
    #[must_use]
    pub fn hangup(mut self) -> Self {
        self.nodes.push(Node::empty("Hangup"));
        self
    }
    #[must_use]
    pub fn pause(mut self, seconds: u32) -> Self {
        self.nodes
            .push(Node::empty("Pause").attribute("length", seconds.to_string()));
        self
    }
    #[must_use]
    pub fn redirect(mut self, url: impl Into<String>) -> Self {
        self.nodes.push(Node::text("Redirect", url));
        self
    }
    #[must_use]
    pub fn typed(mut self, node: super::TwimlNode) -> Self {
        match node.into_root_node(super::grammar::TwimlFamily::Voice) {
            Ok(node) => self.nodes.push(node),
            Err(error) => self.error = Some(error),
        }
        self
    }
    pub fn build(self) -> Result<TwimlResponse, Error> {
        if let Some(error) = self.error {
            Err(error)
        } else {
            TwimlResponse::new(self.nodes)
        }
    }
}
