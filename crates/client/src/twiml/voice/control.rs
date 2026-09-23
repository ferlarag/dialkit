use super::super::Node;

#[derive(Clone, Debug)]
pub struct Enqueue {
    name: String,
    action: Option<String>,
}
impl Enqueue {
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            action: None,
        }
    }
    #[must_use]
    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.action = Some(value.into());
        self
    }
    pub(crate) fn node(self) -> Node {
        let mut node = Node::text("Enqueue", self.name);
        if let Some(value) = self.action {
            node = node.attribute("action", value);
        }
        node
    }
}

#[derive(Clone, Debug)]
pub struct Refer {
    sip: String,
    action: Option<String>,
}
impl Refer {
    #[must_use]
    pub fn sip(uri: impl Into<String>) -> Self {
        Self {
            sip: uri.into(),
            action: None,
        }
    }
    #[must_use]
    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.action = Some(value.into());
        self
    }
    pub(crate) fn node(self) -> Node {
        let child = Node::text("Sip", self.sip);
        let mut node = Node::children("Refer", vec![child]);
        if let Some(value) = self.action {
            node = node.attribute("action", value);
        }
        node
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RejectReason {
    Rejected,
    Busy,
}
impl RejectReason {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Rejected => "rejected",
            Self::Busy => "busy",
        }
    }
}
