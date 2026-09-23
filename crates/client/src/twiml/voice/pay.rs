use super::super::Node;

#[derive(Clone, Debug, Default)]
pub struct Pay {
    children: Vec<Node>,
    attributes: Vec<(&'static str, String)>,
}

impl Pay {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.attributes.push(("action", value.into()));
        self
    }
    #[must_use]
    pub fn payment_method(mut self, value: impl Into<String>) -> Self {
        self.attributes.push(("paymentMethod", value.into()));
        self
    }
    #[must_use]
    pub fn prompt(mut self, text: impl Into<String>) -> Self {
        self.children.push(Node::text("Prompt", text));
        self
    }
    /// Adds a prompt for a documented payment step.
    ///
    /// The older [`Self::prompt`] method remains available for XML
    /// compatibility, but omits the payment step required by Twilio.
    #[must_use]
    pub fn prompt_for(mut self, step: impl Into<String>, text: impl Into<String>) -> Self {
        self.children
            .push(Node::text("Prompt", text).attribute("for", step));
        self
    }
    #[must_use]
    pub fn parameter(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.children.push(
            Node::empty("Parameter")
                .attribute("name", name)
                .attribute("value", value),
        );
        self
    }
    pub(crate) fn node(self) -> Node {
        let mut node = Node::children("Pay", self.children);
        for (name, value) in self.attributes {
            node = node.attribute(name, value);
        }
        node
    }
}
