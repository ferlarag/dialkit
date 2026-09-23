use super::sid_type;
use crate::{CallSid, Client, Error, MediaBody};
use dialkit_core::{request::RequestSpec, retry::OperationSafety};
use http::Method;
use serde::Deserialize;

sid_type!(RecordingSid, "RE", "recording SID");
sid_type!(TranscriptionSid, "TR", "transcription SID");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecordingFormat {
    Mp3,
    Wav,
}

impl RecordingFormat {
    fn extension(self) -> &'static str {
        match self {
            Self::Mp3 => "mp3",
            Self::Wav => "wav",
        }
    }
}

#[derive(Clone)]
pub struct Recordings {
    client: Client,
}

#[derive(Deserialize)]
struct RecordingWire {
    sid: String,
}

impl Recordings {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn start(&self, call: &CallSid) -> Result<RecordingSid, Error> {
        let account = self.client.inner.account_sid();
        let spec = RequestSpec {
            operation: "start_call_recording",
            method: Method::POST,
            route_template: "/2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Recordings.json",
            path: format!(
                "2010-04-01/Accounts/{}/Calls/{}/Recordings.json",
                dialkit_core::request::encode_path_segment(account),
                dialkit_core::request::encode_path_segment(call.as_str()),
            ),
            query: vec![],
            form: vec![],
            safety: OperationSafety::Mutation,
        };
        let wire: RecordingWire = self.client.inner.execute_json(&spec).await?;
        RecordingSid::new(wire.sid)
    }

    pub async fn download(
        &self,
        recording: &RecordingSid,
        format: RecordingFormat,
    ) -> Result<MediaBody, Error> {
        let account = self.client.inner.account_sid();
        let spec = RequestSpec {
            operation: "download_recording",
            method: Method::GET,
            route_template: "/2010-04-01/Accounts/{AccountSid}/Recordings/{RecordingSid}.{Format}",
            path: format!(
                "2010-04-01/Accounts/{}/Recordings/{}.{}",
                dialkit_core::request::encode_path_segment(account),
                dialkit_core::request::encode_path_segment(recording.as_str()),
                format.extension(),
            ),
            query: vec![],
            form: vec![],
            safety: OperationSafety::Read,
        };
        self.client
            .inner
            .execute_bytes(&spec)
            .await
            .map(MediaBody::new)
            .map_err(Into::into)
    }
}
