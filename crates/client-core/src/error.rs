//! Stable, redacted transport errors.

use std::{fmt, time::Duration};
use thiserror::Error;

const MAX_EXCERPT: usize = 512;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ResponseMetadata {
    pub status: Option<u16>,
    pub request_id: Option<String>,
    pub concurrent_requests: Option<String>,
    pub request_duration: Option<String>,
    pub retry_after: Option<Duration>,
    pub attempts: u32,
}

#[derive(Clone, Eq, PartialEq)]
pub struct ApiError {
    status: u16,
    code: Option<u32>,
    message: String,
    more_info: Option<String>,
    request_id: Option<String>,
    raw_excerpt: Option<String>,
    metadata: ResponseMetadata,
}

impl ApiError {
    #[must_use]
    pub fn new(
        status: u16,
        code: Option<u32>,
        message: impl Into<String>,
        more_info: Option<String>,
        request_id: Option<String>,
        raw_excerpt: Option<&str>,
    ) -> Self {
        Self {
            status,
            code,
            message: bounded(message.into()),
            more_info: more_info.map(bounded),
            request_id,
            raw_excerpt: raw_excerpt.map(|value| bounded(value.to_owned())),
            metadata: ResponseMetadata::default(),
        }
    }

    #[must_use]
    pub fn new_redacted(
        status: u16,
        code: Option<u32>,
        message: impl Into<String>,
        more_info: Option<String>,
        request_id: Option<String>,
        raw_excerpt: Option<&str>,
        sensitive_values: &[String],
    ) -> Self {
        let raw_excerpt = raw_excerpt.map(|value| redact(value.to_owned(), sensitive_values));
        Self::new(
            status,
            code,
            redact(message.into(), sensitive_values),
            more_info.map(|value| redact(value, sensitive_values)),
            request_id,
            raw_excerpt.as_deref(),
        )
    }

    #[must_use]
    pub fn with_metadata(mut self, metadata: ResponseMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    #[must_use]
    pub const fn status(&self) -> u16 {
        self.status
    }
    #[must_use]
    pub const fn code(&self) -> Option<u32> {
        self.code
    }
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
    #[must_use]
    pub fn more_info(&self) -> Option<&str> {
        self.more_info.as_deref()
    }
    #[must_use]
    pub fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }
    #[must_use]
    pub const fn attempts(&self) -> u32 {
        self.metadata.attempts
    }
    #[must_use]
    pub const fn retry_after(&self) -> Option<Duration> {
        self.metadata.retry_after
    }
    #[must_use]
    pub fn concurrent_requests(&self) -> Option<&str> {
        self.metadata.concurrent_requests.as_deref()
    }
    #[must_use]
    pub fn request_duration(&self) -> Option<&str> {
        self.metadata.request_duration.as_deref()
    }
}

impl fmt::Debug for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ApiError")
            .field("status", &self.status)
            .field("code", &self.code)
            .field("message", &self.message)
            .field("more_info", &self.more_info)
            .field("request_id", &self.request_id)
            .field("attempts", &self.metadata.attempts)
            .field("retry_after", &self.metadata.retry_after)
            .field(
                "raw_excerpt",
                &self.raw_excerpt.as_ref().map(|_| "[REDACTED]"),
            )
            .finish()
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Twilio API error (HTTP {}", self.status)?;
        if let Some(code) = self.code {
            write!(f, ", code {code}")?;
        }
        write!(f, "): {}", self.message)
    }
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("authentication configuration is invalid: {0}")]
    Authentication(String),
    #[error("request validation failed: {0}")]
    Validation(String),
    #[error("request was rate limited after {attempts} attempt(s)")]
    RateLimited {
        attempts: u32,
        metadata: Box<ResponseMetadata>,
        api: Option<Box<ApiError>>,
    },
    #[error("{0}")]
    Api(Box<ApiError>),
    #[error("transport failed after {attempts} attempt(s): {message}")]
    Transport { attempts: u32, message: String },
    #[error("request timed out after {attempts} attempt(s)")]
    Timeout { attempts: u32 },
    #[error("response decoding failed: {message}")]
    Decode {
        message: String,
        metadata: ResponseMetadata,
    },
    #[error("webhook input is invalid: {0}")]
    WebhookValidation(String),
    #[error("TwiML is invalid: {0}")]
    Xml(String),
}

fn bounded(mut value: String) -> String {
    if value.len() > MAX_EXCERPT {
        value.truncate(MAX_EXCERPT);
        value.push('…');
    }
    value
}

pub(crate) fn redact(mut value: String, sensitive_values: &[String]) -> String {
    for sensitive in sensitive_values
        .iter()
        .filter(|candidate| candidate.len() >= 3)
    {
        value = value.replace(sensitive, "[REDACTED]");
    }
    bounded(value)
}
