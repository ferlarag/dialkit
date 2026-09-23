use super::{FormPair, OpenValue, VerifiedFormWebhook, WebhookFamily, WebhookParseError, required};
use std::fmt;

#[derive(Clone)]
pub struct VoiceWebhookEvent {
    call_sid: String,
    status: Option<OpenValue>,
    extras: Vec<FormPair>,
}
impl VoiceWebhookEvent {
    #[must_use]
    pub fn call_sid(&self) -> &str {
        &self.call_sid
    }
    #[must_use]
    pub fn status(&self) -> Option<&OpenValue> {
        self.status.as_ref()
    }
    #[must_use]
    pub fn extras(&self) -> &[FormPair] {
        &self.extras
    }
}
impl fmt::Debug for VoiceWebhookEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VoiceWebhookEvent")
            .field("call_sid", &"[REDACTED]")
            .field("status", &self.status)
            .field("extras", &format_args!("[REDACTED; {}]", self.extras.len()))
            .finish()
    }
}

impl VerifiedFormWebhook {
    pub fn parse_voice(&self) -> Result<VoiceWebhookEvent, WebhookParseError> {
        if !matches!(
            self.family,
            WebhookFamily::VoiceInstruction
                | WebhookFamily::CallProgress
                | WebhookFamily::AnsweringMachine
                | WebhookFamily::VoiceAction
        ) {
            return Err(WebhookParseError::new(
                "voice",
                "family",
                "does not match parser",
            ));
        }
        Ok(VoiceWebhookEvent {
            call_sid: required(self, "CallSid", "voice")?.to_owned(),
            status: self.first("CallStatus").map(OpenValue::new),
            extras: self.extras(&["CallSid", "CallStatus"]),
        })
    }
}
