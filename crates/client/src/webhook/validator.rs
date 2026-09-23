use super::{VerifiedFormWebhook, VerifiedJsonWebhook, WebhookFamily};
use crate::{Error, IntoSecret};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use hmac::{Hmac, Mac as _};
use secrecy::{ExposeSecret as _, SecretString};
use sha1::Sha1;
use sha2::{Digest as _, Sha256};
use std::{collections::BTreeMap, fmt};
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
    /// Validates a GET callback against the exact public URL, including its raw query string.
    /// Query fields are part of the URL signature and are not appended as form fields.
    pub fn validate_get(
        &self,
        external_url: &str,
        signature: &str,
    ) -> Result<ValidationResult, Error> {
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
    pub fn verify_form(
        &self,
        family: WebhookFamily,
        external_url: &str,
        parameters: &[(String, String)],
        signature: &str,
    ) -> Result<VerifiedFormWebhook, Error> {
        match self.validate_form(external_url, parameters, signature)? {
            ValidationResult::Valid => Ok(VerifiedFormWebhook {
                family,
                pairs: parameters.to_vec(),
            }),
            ValidationResult::Invalid(_) => {
                Err(Error::WebhookValidation("signature mismatch".into()))
            }
        }
    }
    /// Verifies a signed GET URL, then exposes its decoded query pairs to the selected parser.
    /// Repeated and empty pairs are retained in their original order.
    pub fn verify_get(
        &self,
        family: WebhookFamily,
        external_url: &str,
        signature: &str,
    ) -> Result<VerifiedFormWebhook, Error> {
        match self.validate_get(external_url, signature)? {
            ValidationResult::Valid => {
                let parsed = Url::parse(external_url)
                    .map_err(|_| Error::WebhookValidation("external URL is invalid".into()))?;
                let pairs = parsed
                    .query_pairs()
                    .map(|(name, value)| (name.into_owned(), value.into_owned()))
                    .collect();
                Ok(VerifiedFormWebhook { family, pairs })
            }
            ValidationResult::Invalid(_) => {
                Err(Error::WebhookValidation("signature mismatch".into()))
            }
        }
    }
    pub fn verify_json(
        &self,
        family: WebhookFamily,
        external_url: &str,
        raw_body: &[u8],
        signature: &str,
    ) -> Result<VerifiedJsonWebhook, Error> {
        match self.validate_json(external_url, raw_body, signature)? {
            ValidationResult::Valid => {
                let value: serde_json::Value = serde_json::from_slice(raw_body)
                    .map_err(|_| Error::WebhookValidation("JSON body is malformed".into()))?;
                let properties = value
                    .as_object()
                    .ok_or_else(|| Error::WebhookValidation("JSON body must be an object".into()))?
                    .iter()
                    .map(|(key, value)| (key.clone(), value.clone()))
                    .collect::<BTreeMap<_, _>>();
                if family == WebhookFamily::MessageStatus
                    && properties
                        .get("ChannelData")
                        .is_some_and(|value| !value.is_object())
                {
                    return Err(Error::WebhookValidation(
                        "ChannelData must be a JSON object".into(),
                    ));
                }
                Ok(VerifiedJsonWebhook { family, properties })
            }
            ValidationResult::Invalid(_) => Err(Error::WebhookValidation(
                "signature or body digest mismatch".into(),
            )),
        }
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
            let start = external_url.find("://").unwrap_or(0) + 3;
            let end = external_url[start..]
                .find(['/', '?', '#'])
                .map_or(external_url.len(), |offset| start + offset);
            let authority = &external_url[start..end];
            let alternate = if authority.ends_with(port) {
                format!(
                    "{}{}",
                    &external_url[..end - port.len()],
                    &external_url[end..]
                )
            } else if parsed.port().is_none() {
                format!("{}{}{}", &external_url[..end], port, &external_url[end..])
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
