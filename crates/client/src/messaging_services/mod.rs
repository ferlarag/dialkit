//! Stable Messaging Services facade.

mod senders;
pub use senders::*;

use crate::{Client, Error};
use dialkit_core::{request::RequestSpec, retry::OperationSafety};
use http::Method;
use serde::Deserialize;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MessagingServiceSid(String);

impl MessagingServiceSid {
    pub fn new(value: impl Into<String>) -> Result<Self, Error> {
        let value = value.into();
        if value.len() != 34
            || !value.starts_with("MG")
            || !value[2..].bytes().all(|b| b.is_ascii_hexdigit())
        {
            return Err(Error::Validation(
                "Messaging Service SID must be MG followed by 32 hexadecimal characters".into(),
            ));
        }
        Ok(Self(value))
    }
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone)]
pub struct MessagingServices {
    client: Client,
}

impl MessagingServices {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }
    #[must_use]
    pub fn senders(&self, service: MessagingServiceSid) -> ServiceSenders {
        ServiceSenders::new(self.client.clone(), service)
    }

    pub async fn get(&self, service: &MessagingServiceSid) -> Result<MessagingService, Error> {
        let spec = RequestSpec {
            operation: "fetch_service",
            method: Method::GET,
            route_template: "/v1/Services/{Sid}",
            path: format!(
                "v1/Services/{}",
                dialkit_core::request::encode_path_segment(service.as_str())
            ),
            query: vec![],
            form: vec![],
            safety: OperationSafety::Read,
        };
        let wire: ServiceWire = self.client.messaging.execute_json(&spec).await?;
        let sid = wire
            .sid
            .ok_or_else(|| Error::Decode {
                attempts: 1,
                message: "Messaging Service response omitted sid".into(),
            })
            .and_then(MessagingServiceSid::new)?;
        Ok(MessagingService {
            sid,
            friendly_name: wire.friendly_name,
        })
    }
}

#[derive(Deserialize)]
struct ServiceWire {
    sid: Option<String>,
    friendly_name: Option<String>,
}

#[derive(Clone, Debug)]
pub struct MessagingService {
    sid: MessagingServiceSid,
    friendly_name: Option<String>,
}

impl MessagingService {
    #[must_use]
    pub fn sid(&self) -> &MessagingServiceSid {
        &self.sid
    }
    #[must_use]
    pub fn friendly_name(&self) -> Option<&str> {
        self.friendly_name.as_deref()
    }
}
