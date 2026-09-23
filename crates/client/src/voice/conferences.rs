use super::sid_type;
use crate::{Client, Error};
use dialkit_core::{request::RequestSpec, retry::OperationSafety};
use http::Method;
use serde::Deserialize;

sid_type!(ConferenceSid, "CF", "conference SID");
sid_type!(ParticipantCallSid, "CA", "participant call SID");

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ConferenceStatus(String);

impl ConferenceStatus {
    pub const IN_PROGRESS: &'static str = "in-progress";
    #[must_use]
    pub fn from_service(value: impl Into<String>) -> Self {
        Self(value.into())
    }
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone)]
pub struct Conferences {
    client: Client,
}

impl Conferences {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }
    #[must_use]
    pub fn participants(&self, conference: ConferenceSid) -> Participants {
        Participants {
            client: self.client.clone(),
            conference,
        }
    }
}

#[derive(Clone)]
pub struct Participants {
    client: Client,
    conference: ConferenceSid,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ParticipantUpdate {
    muted: Option<bool>,
    hold: Option<bool>,
}

impl ParticipantUpdate {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            muted: None,
            hold: None,
        }
    }

    #[must_use]
    pub const fn muted(mut self, value: bool) -> Self {
        self.muted = Some(value);
        self
    }

    #[must_use]
    pub const fn hold(mut self, value: bool) -> Self {
        self.hold = Some(value);
        self
    }
}

#[derive(Deserialize)]
struct ParticipantWire {
    #[serde(rename = "call_sid")]
    _call_sid: Option<String>,
}

impl Participants {
    pub async fn update(
        &self,
        call: &ParticipantCallSid,
        update: ParticipantUpdate,
    ) -> Result<(), Error> {
        let mut form = Vec::new();
        if let Some(muted) = update.muted {
            form.push(("Muted".into(), muted.to_string()));
        }
        if let Some(hold) = update.hold {
            form.push(("Hold".into(), hold.to_string()));
        }
        if form.is_empty() {
            return Err(Error::Validation(
                "participant update must change muted or hold".into(),
            ));
        }

        let account = self.client.inner.account_sid();
        let spec = RequestSpec {
            operation: "update_conference_participant",
            method: Method::POST,
            route_template: "/2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Participants/{CallSid}.json",
            path: format!(
                "2010-04-01/Accounts/{}/Conferences/{}/Participants/{}.json",
                dialkit_core::request::encode_path_segment(account),
                dialkit_core::request::encode_path_segment(self.conference.as_str()),
                dialkit_core::request::encode_path_segment(call.as_str()),
            ),
            query: vec![],
            form,
            safety: OperationSafety::Mutation,
        };
        let _: ParticipantWire = self.client.inner.execute_json(&spec).await?;
        Ok(())
    }

    #[must_use]
    pub fn conference_sid(&self) -> &ConferenceSid {
        &self.conference
    }
    #[must_use]
    pub fn account_sid(&self) -> &str {
        self.client.inner.account_sid()
    }
}
