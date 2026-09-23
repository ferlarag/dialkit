//! Stable Calls facade.

use crate::{Client, Error, Pager};
use dialkit_api_generated::models::{ApiV2010AccountCall, ListCallResponse};
use dialkit_core::{error::Error as CoreError, request::RequestSpec, retry::OperationSafety};
use futures_util::FutureExt as _;
use http::Method;
use std::fmt;
use url::Url;

macro_rules! string_value {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        pub struct $name(String);
        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, Error> {
                let value = value.into();
                if value.trim().is_empty() {
                    return Err(Error::Validation(
                        concat!(stringify!($name), " cannot be empty").into(),
                    ));
                }
                Ok(Self(value))
            }
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

string_value!(PhoneEndpoint);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CallSid(String);
impl CallSid {
    pub fn new(value: impl Into<String>) -> Result<Self, Error> {
        let value = value.into();
        if value.len() != 34
            || !value.starts_with("CA")
            || !value[2..].bytes().all(|byte| byte.is_ascii_hexdigit())
        {
            return Err(Error::Validation(
                "call SID must be CA followed by 32 hexadecimal characters".into(),
            ));
        }
        Ok(Self(value))
    }
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl fmt::Display for CallSid {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CallStatus(String);
impl CallStatus {
    pub const QUEUED: &'static str = "queued";
    pub const IN_PROGRESS: &'static str = "in-progress";
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[non_exhaustive]
pub enum CallInstructions {
    #[cfg(feature = "twiml")]
    Twiml(crate::twiml::TwimlResponse),
    Url(Url),
}

pub struct CreateCall {
    from: PhoneEndpoint,
    to: PhoneEndpoint,
    instructions: CallInstructions,
}

impl CreateCall {
    pub fn new(
        from: PhoneEndpoint,
        to: PhoneEndpoint,
        instructions: CallInstructions,
    ) -> Result<Self, Error> {
        Ok(Self {
            from,
            to,
            instructions,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Call {
    sid: CallSid,
    status: CallStatus,
}
impl Call {
    #[must_use]
    pub fn sid(&self) -> &CallSid {
        &self.sid
    }
    #[must_use]
    pub fn status(&self) -> &CallStatus {
        &self.status
    }
}

impl TryFrom<ApiV2010AccountCall> for Call {
    type Error = CoreError;
    fn try_from(value: ApiV2010AccountCall) -> Result<Self, Self::Error> {
        let sid = value.sid.flatten().ok_or_else(|| CoreError::Decode {
            message: "call response omitted sid".into(),
            metadata: Default::default(),
        })?;
        let status = value
            .status
            .map_or_else(|| "unknown".into(), |status| status.to_string());
        let sid = CallSid::new(sid).map_err(|_| CoreError::Decode {
            message: "call response contained an invalid sid".into(),
            metadata: Default::default(),
        })?;
        Ok(Self {
            sid,
            status: CallStatus(status),
        })
    }
}

#[derive(Clone, Debug, Default)]
pub struct ListCalls {
    status: Option<String>,
    page_size: Option<u32>,
}
impl ListCalls {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }
    #[must_use]
    pub const fn page_size(mut self, value: u32) -> Self {
        self.page_size = Some(value);
        self
    }
}

#[derive(Clone)]
pub struct Calls {
    client: Client,
}
impl Calls {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }
    pub async fn create(&self, request: CreateCall) -> Result<Call, Error> {
        let instruction = match request.instructions {
            CallInstructions::Url(value) => ("Url", value.to_string()),
            #[cfg(feature = "twiml")]
            CallInstructions::Twiml(value) => ("Twiml", value.to_xml()?),
        };
        let account = self.client.inner.account_sid();
        let spec = RequestSpec {
            operation: "create_call",
            method: Method::POST,
            route_template: "/2010-04-01/Accounts/{AccountSid}/Calls.json",
            path: format!(
                "2010-04-01/Accounts/{}/Calls.json",
                dialkit_core::request::encode_path_segment(account)
            ),
            query: Vec::new(),
            form: vec![
                ("From".into(), request.from.0),
                ("To".into(), request.to.0),
                (instruction.0.into(), instruction.1),
            ],
            safety: OperationSafety::Mutation,
        };
        let wire: ApiV2010AccountCall = self.client.inner.execute_json(&spec).await?;
        Call::try_from(wire).map_err(Into::into)
    }

    pub async fn get(&self, sid: &CallSid) -> Result<Call, Error> {
        let account = self.client.inner.account_sid();
        let spec = RequestSpec {
            operation: "fetch_call",
            method: Method::GET,
            route_template: "/2010-04-01/Accounts/{AccountSid}/Calls/{Sid}.json",
            path: format!(
                "2010-04-01/Accounts/{}/Calls/{}.json",
                dialkit_core::request::encode_path_segment(account),
                dialkit_core::request::encode_path_segment(sid.as_str())
            ),
            query: Vec::new(),
            form: Vec::new(),
            safety: OperationSafety::Read,
        };
        let wire: ApiV2010AccountCall = self.client.inner.execute_json(&spec).await?;
        Call::try_from(wire).map_err(Into::into)
    }

    #[must_use]
    pub fn list(&self, filter: ListCalls) -> Pager<Call> {
        let client = self.client.clone();
        let inner = dialkit_core::pagination::Pager::new(move |continuation| {
            let client = client.clone();
            let filter = filter.clone();
            async move {
                let account = client.inner.account_sid();
                let initial = continuation.is_none();
                let path = continuation.unwrap_or_else(|| {
                    format!(
                        "2010-04-01/Accounts/{}/Calls.json",
                        dialkit_core::request::encode_path_segment(account)
                    )
                });
                let mut query = Vec::new();
                if initial {
                    if let Some(status) = filter.status {
                        query.push(("Status".into(), status));
                    }
                    if let Some(size) = filter.page_size {
                        query.push(("PageSize".into(), size.to_string()));
                    }
                }
                let spec = RequestSpec {
                    operation: "list_call",
                    method: Method::GET,
                    route_template: "/2010-04-01/Accounts/{AccountSid}/Calls.json",
                    path,
                    query,
                    form: Vec::new(),
                    safety: OperationSafety::Read,
                };
                let response: ListCallResponse = client.inner.execute_json(&spec).await?;
                let items = response
                    .calls
                    .unwrap_or_default()
                    .into_iter()
                    .map(Call::try_from)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(crate::pagination::api_2010_page(
                    items,
                    response.next_page_uri.flatten(),
                ))
            }
            .boxed()
        });
        Pager::new(inner)
    }
}
