use super::sid_type;
use crate::{Client, Error};
use dialkit_core::{request::RequestSpec, retry::OperationSafety};
use http::Method;
use serde::Deserialize;

sid_type!(SipDomainSid, "SD", "SIP domain SID");
sid_type!(CredentialListSid, "CL", "credential list SID");
sid_type!(IpAccessControlListSid, "AL", "IP access-control-list SID");
sid_type!(SipCredentialSid, "CR", "SIP credential SID");

#[derive(Clone)]
pub struct Sip {
    client: Client,
}

#[derive(Deserialize)]
struct SidWire {
    sid: String,
}

impl Sip {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create_domain(
        &self,
        domain_name: impl Into<String>,
    ) -> Result<SipDomainSid, Error> {
        let domain_name = nonempty("domain name", domain_name.into())?;
        let wire = self
            .create(
                "create_sip_domain",
                "SIP/Domains.json".into(),
                vec![("DomainName".into(), domain_name)],
            )
            .await?;
        SipDomainSid::new(wire.sid)
    }

    pub async fn create_credential_list(
        &self,
        friendly_name: impl Into<String>,
    ) -> Result<CredentialListSid, Error> {
        let friendly_name = nonempty("credential-list friendly name", friendly_name.into())?;
        let wire = self
            .create(
                "create_sip_credential_list",
                "SIP/CredentialLists.json".into(),
                vec![("FriendlyName".into(), friendly_name)],
            )
            .await?;
        CredentialListSid::new(wire.sid)
    }

    pub async fn create_credential(
        &self,
        list: &CredentialListSid,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Result<SipCredentialSid, Error> {
        let username = nonempty("SIP username", username.into())?;
        let password = nonempty("SIP password", password.into())?;
        let wire = self
            .create(
                "create_sip_credential",
                format!(
                    "SIP/CredentialLists/{}/Credentials.json",
                    dialkit_core::request::encode_path_segment(list.as_str())
                ),
                vec![("Username".into(), username), ("Password".into(), password)],
            )
            .await?;
        SipCredentialSid::new(wire.sid)
    }

    async fn create(
        &self,
        operation: &'static str,
        suffix: String,
        form: Vec<(String, String)>,
    ) -> Result<SidWire, Error> {
        let account = self.client.inner.account_sid();
        let spec = RequestSpec {
            operation,
            method: Method::POST,
            route_template: "/2010-04-01/Accounts/{AccountSid}/SIP/{Resource}.json",
            path: format!(
                "2010-04-01/Accounts/{}/{suffix}",
                dialkit_core::request::encode_path_segment(account),
            ),
            query: vec![],
            form,
            safety: OperationSafety::Mutation,
        };
        self.client
            .inner
            .execute_json(&spec)
            .await
            .map_err(Into::into)
    }

    #[must_use]
    pub fn account_sid(&self) -> &str {
        self.client.inner.account_sid()
    }
}

fn nonempty(label: &str, value: String) -> Result<String, Error> {
    if value.trim().is_empty() {
        Err(Error::Validation(format!("{label} cannot be empty")))
    } else {
        Ok(value)
    }
}
