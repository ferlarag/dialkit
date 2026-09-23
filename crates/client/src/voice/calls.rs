use super::sid_type;
use crate::{Client, Error};
use dialkit_core::{request::RequestSpec, retry::OperationSafety};
use http::Method;
use serde::Deserialize;
use url::Url;

sid_type!(ApplicationSid, "AP", "application SID");

/// Fetches and configures a Voice application through [`Client::applications`].
///
/// # Example
///
/// ```no_run
/// use dialkit::{AccountSid, ApplicationSid, Client, ConfigureApplication};
/// use url::Url;
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new(
///     AccountSid::new("AC00000000000000000000000000000000")?,
///     "synthetic-token",
/// )?;
/// let sid = ApplicationSid::new("AP00000000000000000000000000000000")?;
/// let application = client.applications().get(&sid).await?;
/// let settings = ConfigureApplication::new()
///     .voice_url(Url::parse("https://example.invalid/voice")?);
/// let updated = client.applications().configure(&sid, settings).await?;
/// assert_eq!(application.sid(), updated.sid());
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct Applications {
    client: Client,
}

/// Optional Voice and SMS callback URLs to update on an application.
/// At least one URL is required when passed to [`Applications::configure`].
#[derive(Clone, Default)]
pub struct ConfigureApplication {
    voice_url: Option<Url>,
    sms_url: Option<Url>,
}
impl ConfigureApplication {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn voice_url(mut self, value: Url) -> Self {
        self.voice_url = Some(value);
        self
    }
    #[must_use]
    pub fn sms_url(mut self, value: Url) -> Self {
        self.sms_url = Some(value);
        self
    }
}

/// A fetched Voice application with typed identifiers and callback URLs.
#[derive(Clone)]
pub struct Application {
    sid: ApplicationSid,
    friendly_name: Option<String>,
    voice_url: Option<Url>,
    sms_url: Option<Url>,
}
impl Application {
    #[must_use]
    pub fn sid(&self) -> &ApplicationSid {
        &self.sid
    }
    #[must_use]
    pub fn friendly_name(&self) -> Option<&str> {
        self.friendly_name.as_deref()
    }
    #[must_use]
    pub fn voice_url(&self) -> Option<&Url> {
        self.voice_url.as_ref()
    }
    #[must_use]
    pub fn sms_url(&self) -> Option<&Url> {
        self.sms_url.as_ref()
    }
}
impl std::fmt::Debug for Application {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Application([REDACTED])")
    }
}

#[derive(Deserialize)]
struct ApplicationWire {
    sid: String,
    friendly_name: Option<String>,
    voice_url: Option<String>,
    sms_url: Option<String>,
}
impl TryFrom<ApplicationWire> for Application {
    type Error = Error;
    fn try_from(wire: ApplicationWire) -> Result<Self, Self::Error> {
        Ok(Self {
            sid: ApplicationSid::new(wire.sid)?,
            friendly_name: wire.friendly_name,
            voice_url: wire
                .voice_url
                .map(|value| {
                    Url::parse(&value).map_err(|_| {
                        Error::Validation("application response has invalid voice URL".into())
                    })
                })
                .transpose()?,
            sms_url: wire
                .sms_url
                .map(|value| {
                    Url::parse(&value).map_err(|_| {
                        Error::Validation("application response has invalid SMS URL".into())
                    })
                })
                .transpose()?,
        })
    }
}

impl Applications {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    #[must_use]
    pub fn account_sid(&self) -> &str {
        self.client.inner.account_sid()
    }

    /// Fetches an application by its `AP` SID.
    pub async fn get(&self, sid: &ApplicationSid) -> Result<Application, Error> {
        self.execute(sid, Method::GET, vec![]).await
    }

    /// Updates one or both callback URLs. Empty settings return a validation error.
    pub async fn configure(
        &self,
        sid: &ApplicationSid,
        settings: ConfigureApplication,
    ) -> Result<Application, Error> {
        let mut form = Vec::new();
        if let Some(url) = settings.voice_url {
            form.push(("VoiceUrl".into(), url.to_string()));
        }
        if let Some(url) = settings.sms_url {
            form.push(("SmsUrl".into(), url.to_string()));
        }
        if form.is_empty() {
            return Err(Error::Validation(
                "application configuration requires a voice or SMS URL".into(),
            ));
        }
        self.execute(sid, Method::POST, form).await
    }

    async fn execute(
        &self,
        sid: &ApplicationSid,
        method: Method,
        form: Vec<(String, String)>,
    ) -> Result<Application, Error> {
        let account = self.client.inner.account_sid();
        let spec = RequestSpec {
            operation: if method == Method::GET {
                "fetch_application"
            } else {
                "update_application"
            },
            method: method.clone(),
            route_template: "/2010-04-01/Accounts/{AccountSid}/Applications/{Sid}.json",
            path: format!(
                "2010-04-01/Accounts/{}/Applications/{}.json",
                dialkit_core::request::encode_path_segment(account),
                dialkit_core::request::encode_path_segment(sid.as_str())
            ),
            query: vec![],
            form,
            safety: if method == Method::GET {
                OperationSafety::Read
            } else {
                OperationSafety::Mutation
            },
        };
        let wire: ApplicationWire = self.client.inner.execute_json(&spec).await?;
        Application::try_from(wire)
    }
}
