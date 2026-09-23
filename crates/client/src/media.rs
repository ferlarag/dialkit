use crate::{Client, Error, MessageMediaRef, MessageMediaSid, MessageSid};
use bytes::Bytes;
use dialkit_core::{request::RequestSpec, retry::OperationSafety};
use futures_core::Stream;
use http::Method;
use serde::Deserialize;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

/// Fetches JSON metadata for an MMS attachment through [`Client::media`].
/// Binary attachment bytes are retrieved separately with
/// [`crate::messaging::media::MessageMedia::download`].
///
/// # Example
///
/// ```no_run
/// use dialkit::{AccountSid, Client, MessageMediaRef, MessageMediaSid, MessageSid};
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new(
///     AccountSid::new("AC00000000000000000000000000000000")?,
///     "synthetic-token",
/// )?;
/// let message = MessageSid::new("SM00000000000000000000000000000000")?;
/// let attachment = MessageMediaSid::new("ME00000000000000000000000000000000")?;
/// let reference = MessageMediaRef::new(message.clone(), attachment.clone());
/// let metadata = client.media().metadata(&reference).await?;
/// assert_eq!(metadata.sid(), &attachment);
/// let _content_type = metadata.content_type();
/// // Download the bytes only when needed; this call does not decode them as JSON.
/// let _body = client.messages().media(message).download(&attachment).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct Media {
    client: Client,
}

/// Typed JSON metadata for one MMS attachment; it does not contain its bytes.
#[derive(Clone)]
pub struct MediaMetadata {
    sid: MessageMediaSid,
    message_sid: MessageSid,
    content_type: String,
}
impl MediaMetadata {
    #[must_use]
    pub fn sid(&self) -> &MessageMediaSid {
        &self.sid
    }
    #[must_use]
    pub fn message_sid(&self) -> &MessageSid {
        &self.message_sid
    }
    #[must_use]
    pub fn content_type(&self) -> &str {
        &self.content_type
    }
}
impl std::fmt::Debug for MediaMetadata {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("MediaMetadata([REDACTED])")
    }
}

#[derive(Deserialize)]
struct MediaWire {
    sid: String,
    parent_sid: String,
    content_type: String,
}

impl Media {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }
    #[must_use]
    pub fn account_sid(&self) -> &str {
        self.client.inner.account_sid()
    }

    /// Fetches metadata for the message and attachment SIDs in `media`.
    pub async fn metadata(&self, media: &MessageMediaRef) -> Result<MediaMetadata, Error> {
        let account = self.client.inner.account_sid();
        let spec = RequestSpec {
            operation: "fetch_media",
            method: Method::GET,
            route_template: "/2010-04-01/Accounts/{AccountSid}/Messages/{MessageSid}/Media/{Sid}.json",
            path: format!(
                "2010-04-01/Accounts/{}/Messages/{}/Media/{}.json",
                dialkit_core::request::encode_path_segment(account),
                dialkit_core::request::encode_path_segment(media.message_sid().as_str()),
                dialkit_core::request::encode_path_segment(media.media_sid().as_str())
            ),
            query: vec![],
            form: vec![],
            safety: OperationSafety::Read,
        };
        let wire: MediaWire = self.client.inner.execute_json(&spec).await?;
        Ok(MediaMetadata {
            sid: MessageMediaSid::new(wire.sid)?,
            message_sid: MessageSid::new(wire.parent_sid)?,
            content_type: wire.content_type,
        })
    }
}

pub struct MediaBody {
    inner: dialkit_core::response::ByteResponse,
}

impl MediaBody {
    pub(crate) fn new(inner: dialkit_core::response::ByteResponse) -> Self {
        Self { inner }
    }
    #[must_use]
    pub fn content_type(&self) -> Option<&str> {
        self.inner.content_type()
    }
    #[must_use]
    pub const fn content_length(&self) -> Option<u64> {
        self.inner.content_length()
    }
}

impl Stream for MediaBody {
    type Item = Result<Bytes, crate::Error>;

    fn poll_next(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        match this.inner.as_stream_mut().poll_next(context) {
            Poll::Ready(Some(result)) => Poll::Ready(Some(result.map_err(Into::into))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
