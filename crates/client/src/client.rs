//! Client configuration and service accessors.

use crate::{Calls, Error, Messages};
pub use dialkit_core::auth::{AccountSid, ApiKeySid};
pub use dialkit_core::request::{TlsBackend, active_tls_backend};
pub use dialkit_core::retry::RetryPolicy;
use dialkit_core::{
    auth,
    request::{ClientConfiguration, HttpClient},
};
use secrecy::SecretString;
use std::{fmt, sync::Arc, time::Duration};
use url::Url;

pub trait IntoSecret {
    fn into_secret(self) -> SecretString;
}

impl IntoSecret for SecretString {
    fn into_secret(self) -> SecretString {
        self
    }
}

impl IntoSecret for String {
    fn into_secret(self) -> SecretString {
        SecretString::from(self)
    }
}

impl IntoSecret for &str {
    fn into_secret(self) -> SecretString {
        SecretString::from(self.to_owned())
    }
}

#[derive(Clone)]
pub struct Credentials(auth::Credentials);

impl Credentials {
    #[must_use]
    pub fn account_token(account_sid: AccountSid, auth_token: impl IntoSecret) -> Self {
        Self(auth::Credentials::account_token(
            account_sid,
            auth_token.into_secret(),
        ))
    }

    #[must_use]
    pub fn api_key(
        account_sid: AccountSid,
        key_sid: ApiKeySid,
        key_secret: impl IntoSecret,
    ) -> Self {
        Self(auth::Credentials::api_key(
            account_sid,
            key_sid,
            key_secret.into_secret(),
        ))
    }
}

impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Credentials([REDACTED])")
    }
}

#[derive(Clone)]
pub struct Client {
    pub(crate) inner: Arc<HttpClient>,
}

impl Client {
    pub fn new(account_sid: AccountSid, auth_token: impl IntoSecret) -> Result<Self, Error> {
        Self::builder(Credentials::account_token(account_sid, auth_token)).build()
    }

    #[must_use]
    pub fn builder(credentials: Credentials) -> ClientBuilder {
        ClientBuilder::new(credentials)
    }
    #[must_use]
    pub fn calls(&self) -> Calls {
        Calls::new(self.clone())
    }
    #[must_use]
    pub fn messages(&self) -> Messages {
        Messages::new(self.clone())
    }
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Client { .. }")
    }
}

pub struct ClientBuilder {
    credentials: Credentials,
    base_url: Url,
    connect_timeout: Duration,
    request_timeout: Duration,
    retry_policy: RetryPolicy,
}

impl ClientBuilder {
    fn new(credentials: Credentials) -> Self {
        Self {
            credentials,
            base_url: Url::parse("https://api.twilio.com/").expect("constant Twilio URL is valid"),
            connect_timeout: Duration::from_secs(10),
            request_timeout: Duration::from_secs(30),
            retry_policy: RetryPolicy::conservative(),
        }
    }

    #[must_use]
    pub fn base_url(mut self, url: Url) -> Self {
        self.base_url = url;
        self
    }
    #[must_use]
    pub const fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }
    #[must_use]
    pub const fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = timeout;
        self
    }
    #[must_use]
    pub fn retry_policy(mut self, policy: RetryPolicy) -> Self {
        self.retry_policy = policy;
        self
    }

    pub fn build(self) -> Result<Client, Error> {
        let allow_http_for_tests = self.base_url.scheme() == "http";
        let core = HttpClient::new(ClientConfiguration {
            credentials: self.credentials.0,
            base_url: self.base_url,
            connect_timeout: self.connect_timeout,
            request_timeout: self.request_timeout,
            retry_policy: self.retry_policy,
            allow_http_for_tests,
        })?;
        Ok(Client {
            inner: Arc::new(core),
        })
    }
}
