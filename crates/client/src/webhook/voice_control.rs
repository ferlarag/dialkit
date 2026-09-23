use super::{FormPair, OpenValue, VerifiedFormWebhook, WebhookFamily, WebhookParseError};
use std::fmt;

#[derive(Clone)]
pub struct VoiceControlEvent {
    identifier: Option<String>,
    outcome: Option<OpenValue>,
    extras: Vec<FormPair>,
}
impl VoiceControlEvent {
    #[must_use]
    pub fn identifier(&self) -> Option<&str> {
        self.identifier.as_deref()
    }
    #[must_use]
    pub fn outcome(&self) -> Option<&OpenValue> {
        self.outcome.as_ref()
    }
    #[must_use]
    pub fn extras(&self) -> &[FormPair] {
        &self.extras
    }
}
impl fmt::Debug for VoiceControlEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VoiceControlEvent")
            .field(
                "identifier",
                &self.identifier.as_ref().map(|_| "[REDACTED]"),
            )
            .field("outcome", &self.outcome)
            .field("extras", &format_args!("[REDACTED; {}]", self.extras.len()))
            .finish()
    }
}
impl VerifiedFormWebhook {
    pub fn parse_voice_control(&self) -> Result<VoiceControlEvent, WebhookParseError> {
        if !matches!(
            self.family,
            WebhookFamily::Recording
                | WebhookFamily::Conference
                | WebhookFamily::Queue
                | WebhookFamily::Gather
        ) {
            return Err(WebhookParseError::new(
                "voice-control",
                "family",
                "does not match parser",
            ));
        }
        let identifier = ["RecordingSid", "ConferenceSid", "QueueSid", "CallSid"]
            .into_iter()
            .find_map(|name| self.first(name).map(str::to_owned));
        let outcome = [
            "RecordingStatus",
            "StatusCallbackEvent",
            "QueueResult",
            "Digits",
            "SpeechResult",
        ]
        .into_iter()
        .find_map(|name| self.first(name).map(OpenValue::new));
        Ok(VoiceControlEvent {
            identifier,
            outcome,
            extras: self.extras(&[
                "RecordingSid",
                "ConferenceSid",
                "QueueSid",
                "CallSid",
                "RecordingStatus",
                "StatusCallbackEvent",
                "QueueResult",
                "Digits",
                "SpeechResult",
            ]),
        })
    }
}
