use super::{Dial, Node};

impl Dial {
    #[must_use]
    pub fn application(mut self, sid: impl Into<String>) -> Self {
        self.children.push(Node::text("Application", sid));
        self
    }
    #[must_use]
    pub fn client(mut self, identity: impl Into<String>) -> Self {
        self.children.push(Node::text("Client", identity));
        self
    }
    #[must_use]
    pub fn conference(mut self, name: impl Into<String>) -> Self {
        self.children.push(Node::text("Conference", name));
        self
    }
    #[must_use]
    pub fn number(mut self, number: impl Into<String>) -> Self {
        self.children.push(Node::text("Number", number));
        self
    }
    #[must_use]
    pub fn queue(mut self, name: impl Into<String>) -> Self {
        self.children.push(Node::text("Queue", name));
        self
    }
    #[must_use]
    pub fn sip(mut self, uri: impl Into<String>) -> Self {
        self.children.push(Node::text("Sip", uri));
        self
    }
    #[must_use]
    pub fn action(mut self, url: impl Into<String>) -> Self {
        self.attributes.push(("action", url.into()));
        self
    }
    #[must_use]
    pub fn record(mut self, value: impl Into<String>) -> Self {
        self.attributes.push(("record", value.into()));
        self
    }
}
