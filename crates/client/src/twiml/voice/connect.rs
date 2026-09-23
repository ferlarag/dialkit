use super::super::Node;

#[derive(Clone, Debug, Default)]
pub struct Connect {
    children: Vec<Node>,
    action: Option<String>,
}

impl Connect {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.action = Some(value.into());
        self
    }
    #[must_use]
    pub fn conversation_relay(mut self, url: impl Into<String>) -> Self {
        self.children
            .push(Node::empty("ConversationRelay").attribute("url", url));
        self
    }
    #[must_use]
    pub fn room(mut self, name: impl Into<String>) -> Self {
        self.children.push(Node::text("Room", name));
        self
    }
    #[must_use]
    pub fn stream(mut self, url: impl Into<String>) -> Self {
        self.children
            .push(Node::empty("Stream").attribute("url", url));
        self
    }
    #[must_use]
    pub fn virtual_agent(mut self, connector_name: impl Into<String>) -> Self {
        self.children
            .push(Node::empty("VirtualAgent").attribute("connectorName", connector_name));
        self
    }
    pub(crate) fn node(self) -> Node {
        let mut node = Node::children("Connect", self.children);
        if let Some(action) = self.action {
            node = node.attribute("action", action);
        }
        node
    }
}
