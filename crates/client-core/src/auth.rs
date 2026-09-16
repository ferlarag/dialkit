//! Credential types and the narrow Basic-auth exposure boundary.

use base64::{Engine as _, engine::general_purpose::STANDARD};
use secrecy::{ExposeSecret, SecretString};
use std::fmt;

/// A Twilio account identifier used to scope REST resources.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct AccountSid(String);

impl AccountSid {
    /// Validates and constructs an account SID.
    pub fn new(value: impl Into<String>) -> Result<Self, &'static str> {
        let value = value.into();
        if !valid_sid(&value, "AC") {
            return Err("account SID must be AC followed by 32 hexadecimal characters");
        }
        Ok(Self(value))
    }

    /// Returns the SID without exposing any credential secret.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for AccountSid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AccountSid([REDACTED])")
    }
}

/// An API key identifier used as the Basic-auth username.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct ApiKeySid(String);

impl ApiKeySid {
    /// Validates and constructs an API key SID.
    pub fn new(value: impl Into<String>) -> Result<Self, &'static str> {
        let value = value.into();
        if !valid_sid(&value, "SK") {
            return Err("API key SID must be SK followed by 32 hexadecimal characters");
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn valid_sid(value: &str, prefix: &str) -> bool {
    value.len() == 34
        && value.starts_with(prefix)
        && value[2..].bytes().all(|byte| byte.is_ascii_hexdigit())
}

impl fmt::Debug for ApiKeySid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ApiKeySid([REDACTED])")
    }
}

/// Supported Twilio Basic-auth credential forms.
#[derive(Clone)]
pub enum Credentials {
    AccountToken {
        account_sid: AccountSid,
        auth_token: SecretString,
    },
    ApiKey {
        account_sid: AccountSid,
        key_sid: ApiKeySid,
        key_secret: SecretString,
    },
}

impl Credentials {
    #[must_use]
    pub fn account_token(account_sid: AccountSid, auth_token: SecretString) -> Self {
        Self::AccountToken {
            account_sid,
            auth_token,
        }
    }

    #[must_use]
    pub fn api_key(account_sid: AccountSid, key_sid: ApiKeySid, key_secret: SecretString) -> Self {
        Self::ApiKey {
            account_sid,
            key_sid,
            key_secret,
        }
    }

    #[must_use]
    pub fn account_sid(&self) -> &AccountSid {
        match self {
            Self::AccountToken { account_sid, .. } | Self::ApiKey { account_sid, .. } => {
                account_sid
            }
        }
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        let secret = match self {
            Self::AccountToken { auth_token, .. } => auth_token.expose_secret(),
            Self::ApiKey { key_secret, .. } => key_secret.expose_secret(),
        };
        if secret.is_empty() {
            return Err("credential secret cannot be empty");
        }
        Ok(())
    }

    /// Constructs the authorization value at the final transport boundary.
    pub(crate) fn authorization_value(&self) -> String {
        let (username, secret) = match self {
            Self::AccountToken {
                account_sid,
                auth_token,
            } => (account_sid.as_str(), auth_token.expose_secret()),
            Self::ApiKey {
                key_sid,
                key_secret,
                ..
            } => (key_sid.as_str(), key_secret.expose_secret()),
        };
        format!("Basic {}", STANDARD.encode(format!("{username}:{secret}")))
    }

    pub(crate) fn sensitive_values(&self) -> Vec<String> {
        match self {
            Self::AccountToken {
                account_sid,
                auth_token,
            } => vec![
                account_sid.as_str().to_owned(),
                auth_token.expose_secret().to_owned(),
            ],
            Self::ApiKey {
                account_sid,
                key_sid,
                key_secret,
            } => vec![
                account_sid.as_str().to_owned(),
                key_sid.as_str().to_owned(),
                key_secret.expose_secret().to_owned(),
            ],
        }
    }
}

impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AccountToken { .. } => f.write_str("Credentials::AccountToken([REDACTED])"),
            Self::ApiKey { .. } => f.write_str("Credentials::ApiKey([REDACTED])"),
        }
    }
}
