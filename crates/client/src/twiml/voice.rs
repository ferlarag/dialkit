use super::{Node, TwimlResponse};
use crate::Error;

#[derive(Clone, Debug)]
pub struct Gather {
    children: Vec<Node>,
}
impl Gather {
    #[must_use]
    pub fn new() -> Self {
        Self { children: vec![] }
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
    fn node(self) -> Result<Node, Error> {
        if self.children.is_empty() {
            return Err(Error::Xml("Gather requires Say or Play content".into()));
        }
        Ok(Node::children("Gather", self.children))
    }
}
impl Default for Gather {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct Dial {
    destination: String,
}
impl Dial {
    #[must_use]
    pub fn new(destination: impl Into<String>) -> Self {
        Self {
            destination: destination.into(),
        }
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
        self.nodes.push(Node::text("Dial", dial.destination));
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
    pub fn build(self) -> Result<TwimlResponse, Error> {
        if let Some(error) = self.error {
            Err(error)
        } else {
            TwimlResponse::new(self.nodes)
        }
    }
}
