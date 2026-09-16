//! Allowlisted tracing helpers. Request values and headers are intentionally absent.

use std::time::Duration;
use tracing::{Span, field};

#[must_use]
pub fn request_span(operation: &'static str, method: &str, route_template: &'static str) -> Span {
    tracing::info_span!(
        "twilio.request",
        operation,
        http.method = method,
        http.route = route_template,
        attempt = field::Empty,
        http.status_code = field::Empty,
        twilio.request_id = field::Empty,
        elapsed_ms = field::Empty,
        outcome = field::Empty,
    )
}

pub fn record_response(span: &Span, metadata: &crate::error::ResponseMetadata) {
    if let Some(status) = metadata.status {
        span.record("http.status_code", status);
    }
    if let Some(request_id) = metadata.request_id.as_deref() {
        span.record("twilio.request_id", request_id);
    }
}

pub fn record_outcome(span: &Span, outcome: &'static str, elapsed: Duration) {
    span.record("outcome", outcome);
    span.record("elapsed_ms", elapsed.as_millis() as u64);
    tracing::info!(
        outcome,
        elapsed_ms = elapsed.as_millis() as u64,
        "request completed"
    );
}
