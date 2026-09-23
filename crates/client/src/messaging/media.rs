use crate::{Client, Error, MediaBody, MessageSid};
use dialkit_core::{request::RequestSpec, retry::OperationSafety};
use http::Method;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MessageMediaSid(String);

impl MessageMediaSid {
    pub fn new(value: impl Into<String>) -> Result<Self, Error> {
        let value = value.into();
        if value.len() != 34
            || !value.starts_with("ME")
            || !value[2..].bytes().all(|b| b.is_ascii_hexdigit())
        {
            return Err(Error::Validation(
                "message media SID must be ME followed by 32 hexadecimal characters".into(),
            ));
        }
        Ok(Self(value))
    }
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub struct MessageMediaRef {
    message: MessageSid,
    media: MessageMediaSid,
}

impl MessageMediaRef {
    #[must_use]
    pub const fn new(message: MessageSid, media: MessageMediaSid) -> Self {
        Self { message, media }
    }
    #[must_use]
    pub fn message_sid(&self) -> &MessageSid {
        &self.message
    }
    #[must_use]
    pub fn media_sid(&self) -> &MessageMediaSid {
        &self.media
    }
}

#[derive(Clone)]
pub struct MessageMedia {
    client: Client,
    message: MessageSid,
}

impl MessageMedia {
    pub(crate) fn new(client: Client, message: MessageSid) -> Self {
        Self { client, message }
    }

    pub async fn download(&self, media: &MessageMediaSid) -> Result<MediaBody, Error> {
        let account = self.client.inner.account_sid();
        let spec = RequestSpec {
            operation: "download_message_media",
            method: Method::GET,
            route_template: "/2010-04-01/Accounts/{AccountSid}/Messages/{MessageSid}/Media/{MediaSid}",
            path: format!(
                "2010-04-01/Accounts/{}/Messages/{}/Media/{}",
                dialkit_core::request::encode_path_segment(account),
                dialkit_core::request::encode_path_segment(self.message.as_str()),
                dialkit_core::request::encode_path_segment(media.as_str()),
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
