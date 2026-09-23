use super::{Node, TwimlResponse};
use crate::Error;

#[derive(Clone, Debug, Default)]
pub struct MessageNode {
    children: Vec<Node>,
    attributes: Vec<(&'static str, String)>,
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
    #[must_use]
    pub fn action(mut self, url: impl Into<String>) -> Self {
        self.attributes.push(("action", url.into()));
        self
    }
    #[must_use]
    pub fn method(mut self, value: impl Into<String>) -> Self {
        self.attributes.push(("method", value.into()));
        self
    }
    #[must_use]
    pub fn status_callback(mut self, url: impl Into<String>) -> Self {
        self.attributes.push(("statusCallback", url.into()));
        self
    }
    fn node(self) -> Result<Node, Error> {
        if self.children.is_empty() {
            return Err(Error::Xml("Message requires Body or Media content".into()));
        }
        let mut node = Node::children("Message", self.children);
        for (name, value) in self.attributes {
            node = node.attribute(name, value);
        }
        Ok(node)
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
    #[must_use]
    pub fn typed(mut self, node: super::TwimlNode) -> Self {
        match node.into_root_node(super::grammar::TwimlFamily::Messaging) {
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
