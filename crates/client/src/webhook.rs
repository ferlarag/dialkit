//! Twilio webhook signature validation.

use crate::{Error, IntoSecret};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use hmac::{Hmac, Mac as _};
use secrecy::{ExposeSecret as _, SecretString};
use sha1::Sha1;
use sha2::{Digest as _, Sha256};
use std::fmt;
use url::Url;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PortCompatibility {
    #[default]
    Strict,
    AddOrRemoveStandardPort,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum InvalidReason {
    SignatureMismatch,
    BodyDigestMismatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ValidationResult {
    Valid,
    Invalid(InvalidReason),
}

#[derive(Clone)]
pub struct WebhookValidator {
    auth_token: SecretString,
    port_compatibility: PortCompatibility,
}

impl WebhookValidator {
    #[must_use]
    pub fn new(auth_token: impl IntoSecret) -> Self {
        Self {
            auth_token: auth_token.into_secret(),
            port_compatibility: PortCompatibility::Strict,
        }
    }
    #[must_use]
    pub const fn port_compatibility(mut self, mode: PortCompatibility) -> Self {
        self.port_compatibility = mode;
        self
    }

    pub fn validate_form(
        &self,
        external_url: &str,
        parameters: &[(String, String)],
        signature: &str,
    ) -> Result<ValidationResult, Error> {
        let signature = decode_signature(signature)?;
        let mut parameters = parameters.to_vec();
        parameters.sort_by(|left, right| left.0.cmp(&right.0).then_with(|| left.1.cmp(&right.1)));
        for url in candidate_urls(external_url, self.port_compatibility)? {
            let mut message = url;
            for (name, value) in &parameters {
                message.push_str(name);
                message.push_str(value);
            }
            if verify(
                self.auth_token.expose_secret().as_bytes(),
                message.as_bytes(),
                &signature,
            ) {
                return Ok(ValidationResult::Valid);
            }
        }
        Ok(ValidationResult::Invalid(InvalidReason::SignatureMismatch))
    }

    pub fn validate_json(
        &self,
        external_url: &str,
        raw_body: &[u8],
        signature: &str,
    ) -> Result<ValidationResult, Error> {
        let parsed = Url::parse(external_url)
            .map_err(|_| Error::WebhookValidation("external URL is invalid".into()))?;
        let expected = parsed
            .query_pairs()
            .find(|(name, _)| name == "bodySHA256")
            .map(|(_, value)| value.into_owned())
            .ok_or_else(|| Error::WebhookValidation("bodySHA256 is missing".into()))?;
        let actual = Sha256::digest(raw_body)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();
        if !constant_time_bytes(expected.as_bytes(), actual.as_bytes()) {
            return Ok(ValidationResult::Invalid(InvalidReason::BodyDigestMismatch));
        }
        let signature = decode_signature(signature)?;
        for url in candidate_urls(external_url, self.port_compatibility)? {
            if verify(
                self.auth_token.expose_secret().as_bytes(),
                url.as_bytes(),
                &signature,
            ) {
                return Ok(ValidationResult::Valid);
            }
        }
        Ok(ValidationResult::Invalid(InvalidReason::SignatureMismatch))
    }
}

impl fmt::Debug for WebhookValidator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WebhookValidator")
            .field("auth_token", &"[REDACTED]")
            .field("port_compatibility", &self.port_compatibility)
            .finish()
    }
}

fn decode_signature(value: &str) -> Result<Vec<u8>, Error> {
    if value.is_empty() {
        return Err(Error::WebhookValidation("signature is missing".into()));
    }
    STANDARD
        .decode(value)
        .map_err(|_| Error::WebhookValidation("signature is malformed".into()))
}

fn verify(key: &[u8], value: &[u8], signature: &[u8]) -> bool {
    Hmac::<Sha1>::new_from_slice(key).is_ok_and(|mut mac| {
        mac.update(value);
        mac.verify_slice(signature).is_ok()
    })
}

fn constant_time_bytes(left: &[u8], right: &[u8]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .fold(0_u8, |diff, (a, b)| diff | (a ^ b))
            == 0
}

fn candidate_urls(external_url: &str, mode: PortCompatibility) -> Result<Vec<String>, Error> {
    let parsed = Url::parse(external_url)
        .map_err(|_| Error::WebhookValidation("external URL is invalid".into()))?;
    let mut values = vec![external_url.to_owned()];
    if mode == PortCompatibility::AddOrRemoveStandardPort {
        let standard = match parsed.scheme() {
            "https" => Some(":443"),
            "http" => Some(":80"),
            _ => None,
        };
        if let Some(port) = standard {
            let authority_start = external_url.find("://").unwrap_or(0) + 3;
            let authority_end = external_url[authority_start..]
                .find(['/', '?', '#'])
                .map_or(external_url.len(), |offset| authority_start + offset);
            let authority = &external_url[authority_start..authority_end];
            let alternate = if authority.ends_with(port) {
                format!(
                    "{}{}{}",
                    &external_url[..authority_end - port.len()],
                    "",
                    &external_url[authority_end..]
                )
            } else if parsed.port().is_none() {
                format!(
                    "{}{}{}",
                    &external_url[..authority_end],
                    port,
                    &external_url[authority_end..]
                )
            } else {
                String::new()
            };
            if !alternate.is_empty() {
                values.push(alternate);
            }
        }
    }
    Ok(values)
}
