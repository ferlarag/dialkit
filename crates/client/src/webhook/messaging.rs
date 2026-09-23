use super::{FormPair, OpenValue, VerifiedFormWebhook, WebhookFamily, WebhookParseError};
use std::fmt;

#[derive(Clone)]
pub struct IncomingMedia {
    index: usize,
    url: String,
    content_type: Option<String>,
}
impl IncomingMedia {
    #[must_use]
    pub const fn index(&self) -> usize {
        self.index
    }
    #[must_use]
    pub fn url(&self) -> &str {
        &self.url
    }
    #[must_use]
    pub fn content_type(&self) -> Option<&str> {
        self.content_type.as_deref()
    }
}
impl fmt::Debug for IncomingMedia {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IncomingMedia")
            .field("index", &self.index)
            .field("url", &"[REDACTED]")
            .field("content_type", &self.content_type)
            .finish()
    }
}

#[derive(Clone)]
pub struct MessagingWebhookEvent {
    message_sid: String,
    status: Option<OpenValue>,
    body: Option<String>,
    media: Vec<IncomingMedia>,
    extras: Vec<FormPair>,
}
impl MessagingWebhookEvent {
    #[must_use]
    pub fn message_sid(&self) -> &str {
        &self.message_sid
    }
    #[must_use]
    pub fn status(&self) -> Option<&OpenValue> {
        self.status.as_ref()
    }
    #[must_use]
    pub fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }
    #[must_use]
    pub fn media(&self) -> &[IncomingMedia] {
        &self.media
    }
    #[must_use]
    pub fn extras(&self) -> &[FormPair] {
        &self.extras
    }
}
impl fmt::Debug for MessagingWebhookEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MessagingWebhookEvent")
            .field("message_sid", &"[REDACTED]")
            .field("status", &self.status)
            .field("body", &self.body.as_ref().map(|_| "[REDACTED]"))
            .field("media", &self.media)
            .field("extras", &format_args!("[REDACTED; {}]", self.extras.len()))
            .finish()
    }
}
impl VerifiedFormWebhook {
    pub fn parse_messaging(&self) -> Result<MessagingWebhookEvent, WebhookParseError> {
        if !matches!(
            self.family,
            WebhookFamily::IncomingMessage
                | WebhookFamily::MessageStatus
                | WebhookFamily::MessagingServiceRouting
        ) {
            return Err(WebhookParseError::new(
                "messaging",
                "family",
                "does not match parser",
            ));
        }
        let message_sid = self
            .first("MessageSid")
            .or_else(|| self.first("SmsSid"))
            .ok_or_else(|| WebhookParseError::new("messaging", "MessageSid", "is required"))?
            .to_owned();
        let mut media = Vec::new();
        let mut consumed = vec!["MessageSid", "SmsSid", "MessageStatus", "SmsStatus", "Body"];
        let count = self
            .first("NumMedia")
            .and_then(|value| value.parse::<usize>().ok())
            .unwrap_or(0);
        consumed.push("NumMedia");
        for index in 0..count {
            let url_name = format!("MediaUrl{index}");
            let type_name = format!("MediaContentType{index}");
            if let Some(url) = self.first(&url_name) {
                media.push(IncomingMedia {
                    index,
                    url: url.to_owned(),
                    content_type: self.first(&type_name).map(str::to_owned),
                });
            }
        }
        let extras = self
            .pairs
            .iter()
            .filter(|(name, _)| {
                !consumed.contains(&name.as_str())
                    && !name.starts_with("MediaUrl")
                    && !name.starts_with("MediaContentType")
            })
            .map(|(name, value)| FormPair {
                name: name.clone(),
                value: value.clone(),
            })
            .collect();
        Ok(MessagingWebhookEvent {
            message_sid,
            status: self
                .first("MessageStatus")
                .or_else(|| self.first("SmsStatus"))
                .map(OpenValue::new),
            body: self.first("Body").map(str::to_owned),
            media,
            extras,
        })
    }
}
