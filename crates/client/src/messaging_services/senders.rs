use super::MessagingServiceSid;
use crate::{Client, Error, IncomingPhoneNumberSid};
use dialkit_core::{request::RequestSpec, retry::OperationSafety};
use http::Method;
use serde::Deserialize;

#[derive(Clone)]
pub struct ServiceSenders {
    client: Client,
    service: MessagingServiceSid,
}

#[derive(Deserialize)]
struct PhoneNumberWire {
    #[serde(rename = "phone_number_sid")]
    _phone_number_sid: Option<String>,
}

impl ServiceSenders {
    pub(crate) fn new(client: Client, service: MessagingServiceSid) -> Self {
        Self { client, service }
    }

    pub async fn add_phone_number(
        &self,
        phone_number: &IncomingPhoneNumberSid,
    ) -> Result<(), Error> {
        let spec = RequestSpec {
            operation: "add_messaging_service_phone_number",
            method: Method::POST,
            route_template: "/v1/Services/{ServiceSid}/PhoneNumbers",
            path: format!(
                "v1/Services/{}/PhoneNumbers",
                dialkit_core::request::encode_path_segment(self.service.as_str()),
            ),
            query: vec![],
            form: vec![("PhoneNumberSid".into(), phone_number.as_str().to_owned())],
            safety: OperationSafety::Mutation,
        };
        let _: PhoneNumberWire = self.client.messaging.execute_json(&spec).await?;
        Ok(())
    }
    #[must_use]
    pub fn service_sid(&self) -> &MessagingServiceSid {
        &self.service
    }
    #[must_use]
    pub fn account_sid(&self) -> &str {
        self.client.messaging.account_sid()
    }
}
