use super::sid_type;
use crate::{CallSid, Client, Error};
use dialkit_core::{request::RequestSpec, retry::OperationSafety};
use http::Method;
use serde::Deserialize;
use url::Url;

sid_type!(QueueSid, "QU", "queue SID");

#[derive(Clone)]
pub struct Queues {
    client: Client,
}

impl Queues {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }
    #[must_use]
    pub fn members(&self, queue: QueueSid) -> QueueMembers {
        QueueMembers {
            client: self.client.clone(),
            queue,
        }
    }
}

#[derive(Clone)]
pub struct QueueMembers {
    client: Client,
    queue: QueueSid,
}

#[derive(Deserialize)]
struct QueueMemberWire {
    #[serde(rename = "call_sid")]
    _call_sid: Option<String>,
}

impl QueueMembers {
    pub async fn redirect(&self, call: &CallSid, url: &Url) -> Result<(), Error> {
        let account = self.client.inner.account_sid();
        let spec = RequestSpec {
            operation: "redirect_queue_member",
            method: Method::POST,
            route_template: "/2010-04-01/Accounts/{AccountSid}/Queues/{QueueSid}/Members/{CallSid}.json",
            path: format!(
                "2010-04-01/Accounts/{}/Queues/{}/Members/{}.json",
                dialkit_core::request::encode_path_segment(account),
                dialkit_core::request::encode_path_segment(self.queue.as_str()),
                dialkit_core::request::encode_path_segment(call.as_str()),
            ),
            query: vec![],
            form: vec![
                ("Url".into(), url.as_str().to_owned()),
                ("Method".into(), "POST".into()),
            ],
            safety: OperationSafety::Mutation,
        };
        let _: QueueMemberWire = self.client.inner.execute_json(&spec).await?;
        Ok(())
    }

    #[must_use]
    pub fn queue_sid(&self) -> &QueueSid {
        &self.queue
    }
    #[must_use]
    pub fn account_sid(&self) -> &str {
        self.client.inner.account_sid()
    }
}
