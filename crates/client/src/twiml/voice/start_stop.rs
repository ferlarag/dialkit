use super::super::Node;

#[derive(Clone, Debug, Default)]
pub struct Start {
    children: Vec<Node>,
}
#[derive(Clone, Debug, Default)]
pub struct Stop {
    children: Vec<Node>,
}

macro_rules! nouns {
    ($type:ident) => {
        impl $type {
            #[must_use]
            pub fn new() -> Self {
                Self::default()
            }
            #[must_use]
            pub fn recording(mut self, name: impl Into<String>) -> Self {
                self.children
                    .push(Node::empty("Recording").attribute("name", name));
                self
            }
            #[must_use]
            pub fn siprec(mut self, name: impl Into<String>) -> Self {
                self.children
                    .push(Node::empty("Siprec").attribute("name", name));
                self
            }
            #[must_use]
            pub fn stream(mut self, name: impl Into<String>) -> Self {
                self.children
                    .push(Node::empty("Stream").attribute("name", name));
                self
            }
            #[must_use]
            pub fn transcription(mut self, name: impl Into<String>) -> Self {
                self.children
                    .push(Node::empty("Transcription").attribute("name", name));
                self
            }
            pub(crate) fn node(self) -> Node {
                Node::children(stringify!($type), self.children)
            }
        }
    };
}
nouns!(Start);
nouns!(Stop);

impl Start {
    /// Starts a named media stream at a `wss` WebSocket URL.
    ///
    /// This is the documented form for starting a stream. The older
    /// [`Self::stream`] method remains available for source compatibility,
    /// but emits only a name and is not sufficient to establish a stream.
    /// A URL with a query string is rejected when the response is built;
    /// use child `Parameter` values for custom data instead.
    ///
    /// ```
    /// use dialkit::twiml::{Start, TwimlResponse};
    /// use url::Url;
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let response = TwimlResponse::voice()
    ///     .start(Start::new().stream_to(
    ///         "live",
    ///         Url::parse("wss://example.invalid/audio")?,
    ///     ))
    ///     .build()?;
    /// assert!(response.to_xml()?.contains("wss://example.invalid/audio"));
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn stream_to(mut self, name: impl Into<String>, url: url::Url) -> Self {
        self.children.push(
            Node::empty("Stream")
                .attribute("name", name)
                .attribute("url", url.as_str()),
        );
        self
    }

    /// Starts a named SIPREC session through a configured connector.
    ///
    /// The older [`Self::siprec`] method remains available for source
    /// compatibility, but does not identify a connector.
    ///
    /// ```
    /// use dialkit::twiml::{Start, TwimlResponse};
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let response = TwimlResponse::voice()
    ///     .start(Start::new().siprec_with_connector("session", "configured-connector"))
    ///     .build()?;
    /// assert!(response.to_xml()?.contains("connectorName=\"configured-connector\""));
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn siprec_with_connector(
        mut self,
        name: impl Into<String>,
        connector_name: impl Into<String>,
    ) -> Self {
        self.children.push(
            Node::empty("Siprec")
                .attribute("name", name)
                .attribute("connectorName", connector_name),
        );
        self
    }
}
