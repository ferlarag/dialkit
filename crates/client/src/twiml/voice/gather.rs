use super::Gather;

impl Gather {
    #[must_use]
    pub fn timeout(mut self, seconds: u32) -> Self {
        self.attributes.push(("timeout", seconds.to_string()));
        self
    }

    /// Uses Twilio's automatic timeout for speech input.
    #[must_use]
    pub fn timeout_auto(mut self) -> Self {
        self.attributes.push(("timeout", "auto".into()));
        self
    }

    #[must_use]
    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.attributes.push(("language", value.into()));
        self
    }

    #[must_use]
    pub fn speech_timeout(mut self, value: impl Into<String>) -> Self {
        self.attributes.push(("speechTimeout", value.into()));
        self
    }
}
