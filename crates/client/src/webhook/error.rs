use std::fmt;

#[derive(Clone, Eq, PartialEq)]
pub struct WebhookParseError {
    family: &'static str,
    field: &'static str,
    rule: &'static str,
}

impl WebhookParseError {
    pub(crate) const fn new(family: &'static str, field: &'static str, rule: &'static str) -> Self {
        Self {
            family,
            field,
            rule,
        }
    }
    #[must_use]
    pub const fn family(&self) -> &'static str {
        self.family
    }
    #[must_use]
    pub const fn field(&self) -> &'static str {
        self.field
    }
    #[must_use]
    pub const fn rule(&self) -> &'static str {
        self.rule
    }
}

impl fmt::Debug for WebhookParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WebhookParseError")
            .field("family", &self.family)
            .field("field", &self.field)
            .field("rule", &self.rule)
            .finish()
    }
}
impl fmt::Display for WebhookParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} webhook field {} {}",
            self.family, self.field, self.rule
        )
    }
}
impl std::error::Error for WebhookParseError {}
