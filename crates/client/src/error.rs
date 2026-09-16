//! Stable facade errors.

use std::fmt;

#[derive(Clone, Eq, PartialEq)]
pub struct ApiError {
    status: u16,
    code: Option<u32>,
    message: String,
    more_info: Option<String>,
    request_id: Option<String>,
    attempts: u32,
    retry_after: Option<std::time::Duration>,
    concurrent_requests: Option<String>,
    request_duration: Option<String>,
}

impl ApiError {
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
        self.attempts
    }
    #[must_use]
    pub const fn retry_after(&self) -> Option<std::time::Duration> {
        self.retry_after
    }
    #[must_use]
    pub fn concurrent_requests(&self) -> Option<&str> {
        self.concurrent_requests.as_deref()
    }
    #[must_use]
    pub fn request_duration(&self) -> Option<&str> {
        self.request_duration.as_deref()
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
            .field("attempts", &self.attempts)
            .field("retry_after", &self.retry_after)
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

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("authentication configuration is invalid: {0}")]
    Authentication(String),
    #[error("request validation failed: {0}")]
    Validation(String),
    #[error("request was rate limited after {attempts} attempt(s)")]
    RateLimited {
        attempts: u32,
        retry_after: Option<std::time::Duration>,
        api: Option<Box<ApiError>>,
    },
    #[error("{0}")]
    Api(Box<ApiError>),
    #[error("transport failed after {attempts} attempt(s): {message}")]
    Transport { attempts: u32, message: String },
    #[error("request timed out after {attempts} attempt(s)")]
    Timeout { attempts: u32 },
    #[error("response decoding failed after {attempts} attempt(s): {message}")]
    Decode { attempts: u32, message: String },
    #[error("webhook input is invalid: {0}")]
    WebhookValidation(String),
    #[error("TwiML is invalid: {0}")]
    Xml(String),
}

impl From<dialkit_core::error::Error> for Error {
    fn from(value: dialkit_core::error::Error) -> Self {
        use dialkit_core::error::Error as Core;
        match value {
            Core::Authentication(message) => Self::Authentication(message),
            Core::Validation(message) => Self::Validation(message),
            Core::RateLimited {
                attempts,
                metadata,
                api,
            } => Self::RateLimited {
                attempts,
                retry_after: metadata.retry_after,
                api: api.map(|value| Box::new((*value).into())),
            },
            Core::Api(api) => Self::Api(Box::new((*api).into())),
            Core::Transport { attempts, message } => Self::Transport { attempts, message },
            Core::Timeout { attempts } => Self::Timeout { attempts },
            Core::Decode { message, metadata } => Self::Decode {
                attempts: metadata.attempts,
                message,
            },
            Core::WebhookValidation(message) => Self::WebhookValidation(message),
            Core::Xml(message) => Self::Xml(message),
            _ => Self::Transport {
                attempts: 0,
                message: "unrecognized transport failure".into(),
            },
        }
    }
}

impl From<dialkit_core::error::ApiError> for ApiError {
    fn from(value: dialkit_core::error::ApiError) -> Self {
        Self {
            status: value.status(),
            code: value.code(),
            message: value.message().to_owned(),
            more_info: value.more_info().map(str::to_owned),
            request_id: value.request_id().map(str::to_owned),
            attempts: value.attempts(),
            retry_after: value.retry_after(),
            concurrent_requests: value.concurrent_requests().map(str::to_owned),
            request_duration: value.request_duration().map(str::to_owned),
        }
    }
}
