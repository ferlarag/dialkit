use super::{FormPair, OpenValue, VerifiedFormWebhook, WebhookFamily, WebhookParseError};
use std::fmt;

#[derive(Clone)]
pub struct VoiceRealtimeEvent {
    call_sid: Option<String>,
    event: Option<OpenValue>,
    safe_token: Option<String>,
    extras: Vec<FormPair>,
}
impl VoiceRealtimeEvent {
    #[must_use]
    pub fn call_sid(&self) -> Option<&str> {
        self.call_sid.as_deref()
    }
    #[must_use]
    pub fn event(&self) -> Option<&OpenValue> {
        self.event.as_ref()
    }
    #[must_use]
    pub fn safe_token(&self) -> Option<&str> {
        self.safe_token.as_deref()
    }
    #[must_use]
    pub fn extras(&self) -> &[FormPair] {
        &self.extras
    }
}
impl fmt::Debug for VoiceRealtimeEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VoiceRealtimeEvent")
            .field("call_sid", &self.call_sid.as_ref().map(|_| "[REDACTED]"))
            .field("event", &self.event)
            .field(
                "safe_token",
                &self.safe_token.as_ref().map(|_| "[REDACTED]"),
            )
            .field("extras", &format_args!("[REDACTED; {}]", self.extras.len()))
            .finish()
    }
}
impl VerifiedFormWebhook {
    pub fn parse_voice_realtime(&self) -> Result<VoiceRealtimeEvent, WebhookParseError> {
        if !matches!(
            self.family,
            WebhookFamily::Stream
                | WebhookFamily::Siprec
                | WebhookFamily::Transcription
                | WebhookFamily::Payment
                | WebhookFamily::UserDefinedMessage
        ) {
            return Err(WebhookParseError::new(
                "voice-realtime",
                "family",
                "does not match parser",
            ));
        }
        Ok(VoiceRealtimeEvent {
            call_sid: self.first("CallSid").map(str::to_owned),
            event: [
                "StreamEvent",
                "SiprecEvent",
                "TranscriptionEvent",
                "Result",
                "Event",
            ]
            .into_iter()
            .find_map(|name| self.first(name).map(OpenValue::new)),
            safe_token: self.first("PaymentToken").map(str::to_owned),
            extras: self.extras(&[
                "CallSid",
                "StreamEvent",
                "SiprecEvent",
                "TranscriptionEvent",
                "Result",
                "Event",
                "PaymentToken",
            ]),
        })
    }
}
