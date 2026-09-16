use super::{Node, TwimlResponse};
use crate::Error;

#[derive(Clone, Debug, Default)]
pub struct MessageNode {
    children: Vec<Node>,
}
impl MessageNode {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn body(mut self, text: impl Into<String>) -> Self {
        self.children.push(Node::text("Body", text));
        self
    }
    #[must_use]
    pub fn media(mut self, url: impl Into<String>) -> Self {
        self.children.push(Node::text("Media", url));
        self
    }
    fn node(self) -> Result<Node, Error> {
        if self.children.is_empty() {
            return Err(Error::Xml("Message requires Body or Media content".into()));
        }
        Ok(Node::children("Message", self.children))
    }
}

#[derive(Debug, Default)]
pub struct MessagingResponseBuilder {
    nodes: Vec<Node>,
    error: Option<Error>,
}
impl MessagingResponseBuilder {
    pub(crate) fn new() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn message(mut self, message: MessageNode) -> Self {
        match message.node() {
            Ok(node) => self.nodes.push(node),
            Err(error) => self.error = Some(error),
        }
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
