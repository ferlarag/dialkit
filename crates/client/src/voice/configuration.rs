use super::{ApplicationSid, sid_type};
use crate::{Client, Error};
use dialkit_core::{request::RequestSpec, retry::OperationSafety};
use http::Method;
use serde::Deserialize;
use url::Url;

sid_type!(IncomingPhoneNumberSid, "PN", "incoming phone-number SID");

/// Fetches and configures an incoming phone number through [`Client::phone_numbers`].
///
/// # Example
///
/// ```no_run
/// use dialkit::{AccountSid, ApplicationSid, Client, ConfigurePhoneNumber, IncomingPhoneNumberSid};
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new(
///     AccountSid::new("AC00000000000000000000000000000000")?,
///     "synthetic-token",
/// )?;
/// let number = IncomingPhoneNumberSid::new("PN00000000000000000000000000000000")?;
/// let application = ApplicationSid::new("AP00000000000000000000000000000000")?;
/// let current = client.phone_numbers().get(&number).await?;
/// let settings = ConfigurePhoneNumber::new().voice_application(application);
/// let updated = client.phone_numbers().configure(&number, settings).await?;
/// assert_eq!(current.sid(), updated.sid());
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct PhoneNumbers {
    client: Client,
}

/// Callback URLs or application SIDs to update on an incoming phone number.
/// A URL and application SID for the same channel are mutually exclusive.
/// At least one setting is required by [`PhoneNumbers::configure`].
#[derive(Clone, Default)]
pub struct ConfigurePhoneNumber {
    voice_url: Option<Url>,
    sms_url: Option<Url>,
    voice_application_sid: Option<ApplicationSid>,
    sms_application_sid: Option<ApplicationSid>,
}
impl ConfigurePhoneNumber {
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
    #[must_use]
    pub fn voice_application(mut self, value: ApplicationSid) -> Self {
        self.voice_application_sid = Some(value);
        self
    }
    #[must_use]
    pub fn sms_application(mut self, value: ApplicationSid) -> Self {
        self.sms_application_sid = Some(value);
        self
    }
}

/// A fetched incoming phone number with typed SID and callback URLs.
#[derive(Clone)]
pub struct IncomingPhoneNumber {
    sid: IncomingPhoneNumberSid,
    phone_number: String,
    voice_url: Option<Url>,
    sms_url: Option<Url>,
}
impl IncomingPhoneNumber {
    #[must_use]
    pub fn sid(&self) -> &IncomingPhoneNumberSid {
        &self.sid
    }
    #[must_use]
    pub fn phone_number(&self) -> &str {
        &self.phone_number
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
impl std::fmt::Debug for IncomingPhoneNumber {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("IncomingPhoneNumber([REDACTED])")
    }
}

#[derive(Deserialize)]
struct PhoneNumberWire {
    sid: String,
    phone_number: String,
    voice_url: Option<String>,
    sms_url: Option<String>,
}
impl TryFrom<PhoneNumberWire> for IncomingPhoneNumber {
    type Error = Error;
    fn try_from(wire: PhoneNumberWire) -> Result<Self, Self::Error> {
        Ok(Self {
            sid: IncomingPhoneNumberSid::new(wire.sid)?,
            phone_number: wire.phone_number,
            voice_url: wire
                .voice_url
                .map(|value| {
                    Url::parse(&value).map_err(|_| {
                        Error::Validation("phone-number response has invalid voice URL".into())
                    })
                })
                .transpose()?,
            sms_url: wire
                .sms_url
                .map(|value| {
                    Url::parse(&value).map_err(|_| {
                        Error::Validation("phone-number response has invalid SMS URL".into())
                    })
                })
                .transpose()?,
        })
    }
}

impl PhoneNumbers {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }
    #[must_use]
    pub fn account_sid(&self) -> &str {
        self.client.inner.account_sid()
    }

    /// Fetches an incoming phone number by its `PN` SID.
    pub async fn get(&self, sid: &IncomingPhoneNumberSid) -> Result<IncomingPhoneNumber, Error> {
        self.execute(sid, Method::GET, vec![]).await
    }

    /// Updates one or more settings, rejecting conflicting URL/application pairs.
    pub async fn configure(
        &self,
        sid: &IncomingPhoneNumberSid,
        settings: ConfigurePhoneNumber,
    ) -> Result<IncomingPhoneNumber, Error> {
        if settings.voice_url.is_some() && settings.voice_application_sid.is_some() {
            return Err(Error::Validation(
                "phone-number voice URL and voice application are mutually exclusive".into(),
            ));
        }
        if settings.sms_url.is_some() && settings.sms_application_sid.is_some() {
            return Err(Error::Validation(
                "phone-number SMS URL and SMS application are mutually exclusive".into(),
            ));
        }
        let mut form = Vec::new();
        if let Some(value) = settings.voice_url {
            form.push(("VoiceUrl".into(), value.to_string()));
        }
        if let Some(value) = settings.sms_url {
            form.push(("SmsUrl".into(), value.to_string()));
        }
        if let Some(value) = settings.voice_application_sid {
            form.push(("VoiceApplicationSid".into(), value.as_str().to_owned()));
        }
        if let Some(value) = settings.sms_application_sid {
            form.push(("SmsApplicationSid".into(), value.as_str().to_owned()));
        }
        if form.is_empty() {
            return Err(Error::Validation(
                "phone-number configuration requires a voice or SMS setting".into(),
            ));
        }
        self.execute(sid, Method::POST, form).await
    }

    async fn execute(
        &self,
        sid: &IncomingPhoneNumberSid,
        method: Method,
        form: Vec<(String, String)>,
    ) -> Result<IncomingPhoneNumber, Error> {
        let account = self.client.inner.account_sid();
        let spec = RequestSpec {
            operation: if method == Method::GET {
                "fetch_incoming_phone_number"
            } else {
                "update_incoming_phone_number"
            },
            method: method.clone(),
            route_template: "/2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{Sid}.json",
            path: format!(
                "2010-04-01/Accounts/{}/IncomingPhoneNumbers/{}.json",
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
        let wire: PhoneNumberWire = self.client.inner.execute_json(&spec).await?;
        IncomingPhoneNumber::try_from(wire)
    }
}
