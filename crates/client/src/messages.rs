//! Stable Messages facade.

use crate::{Client, Error, MessageMedia, Pager, PhoneEndpoint};
use dialkit_api_generated::models::{ApiV2010AccountMessage, ListMessageResponse};
use dialkit_core::{error::Error as CoreError, request::RequestSpec, retry::OperationSafety};
use futures_util::FutureExt as _;
use http::Method;
use std::fmt;
use url::Url;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MessageSid(String);
impl MessageSid {
    pub fn new(value: impl Into<String>) -> Result<Self, Error> {
        let value = value.into();
        if value.len() != 34
            || !value.starts_with("SM")
            || !value[2..].bytes().all(|byte| byte.is_ascii_hexdigit())
        {
            return Err(Error::Validation(
                "message SID must be SM followed by 32 hexadecimal characters".into(),
            ));
        }
        Ok(Self(value))
    }
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MessageStatus(String);
impl MessageStatus {
    pub const QUEUED: &'static str = "queued";
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub enum MessageSender {
    Phone(PhoneEndpoint),
    MessagingService(String),
}
impl MessageSender {
    #[must_use]
    pub const fn phone(value: PhoneEndpoint) -> Self {
        Self::Phone(value)
    }
    pub fn messaging_service(value: impl Into<String>) -> Result<Self, Error> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(Error::Validation(
                "messaging service SID cannot be empty".into(),
            ));
        }
        Ok(Self::MessagingService(value))
    }
}

pub struct CreateMessage {
    sender: MessageSender,
    to: PhoneEndpoint,
    body: Option<String>,
    media_urls: Vec<Url>,
}
impl CreateMessage {
    #[must_use]
    pub fn new(sender: MessageSender, to: PhoneEndpoint) -> Self {
        Self {
            sender,
            to,
            body: None,
            media_urls: Vec::new(),
        }
    }
    #[must_use]
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }
    #[must_use]
    pub fn media_url(mut self, url: Url) -> Self {
        self.media_urls.push(url);
        self
    }
    pub fn build(self) -> Result<Self, Error> {
        if self.body.as_deref().is_none_or(str::is_empty) && self.media_urls.is_empty() {
            return Err(Error::Validation(
                "message requires a body or media URL".into(),
            ));
        }
        Ok(self)
    }
}

#[derive(Clone, Debug)]
pub struct Message {
    sid: MessageSid,
    status: MessageStatus,
}
impl Message {
    #[must_use]
    pub fn sid(&self) -> &MessageSid {
        &self.sid
    }
    #[must_use]
    pub fn status(&self) -> &MessageStatus {
        &self.status
    }
}

impl TryFrom<ApiV2010AccountMessage> for Message {
    type Error = CoreError;
    fn try_from(value: ApiV2010AccountMessage) -> Result<Self, Self::Error> {
        let sid = value.sid.flatten().ok_or_else(|| CoreError::Decode {
            message: "message response omitted sid".into(),
            metadata: Default::default(),
        })?;
        let status = value
            .status
            .map_or_else(|| "unknown".into(), |status| status.to_string());
        let sid = MessageSid::new(sid).map_err(|_| CoreError::Decode {
            message: "message response contained an invalid sid".into(),
            metadata: Default::default(),
        })?;
        Ok(Self {
            sid,
            status: MessageStatus(status),
        })
    }
}

#[derive(Clone, Debug, Default)]
pub struct ListMessages {
    page_size: Option<u32>,
}
impl ListMessages {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn page_size(mut self, value: u32) -> Self {
        self.page_size = Some(value);
        self
    }
}

#[derive(Clone)]
pub struct Messages {
    client: Client,
}
impl Messages {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }
    #[must_use]
    pub fn media(&self, message: MessageSid) -> MessageMedia {
        MessageMedia::new(self.client.clone(), message)
    }
    pub async fn create(&self, request: CreateMessage) -> Result<Message, Error> {
        let mut form = vec![("To".into(), request.to.to_string())];
        match request.sender {
            MessageSender::Phone(value) => form.push(("From".into(), value.to_string())),
            MessageSender::MessagingService(value) => {
                form.push(("MessagingServiceSid".into(), value))
            }
        }
        if let Some(body) = request.body {
            form.push(("Body".into(), body));
        }
        form.extend(
            request
                .media_urls
                .into_iter()
                .map(|url| ("MediaUrl".into(), url.to_string())),
        );
        let account = self.client.inner.account_sid();
        let spec = RequestSpec {
            operation: "create_message",
            method: Method::POST,
            route_template: "/2010-04-01/Accounts/{AccountSid}/Messages.json",
            path: format!(
                "2010-04-01/Accounts/{}/Messages.json",
                dialkit_core::request::encode_path_segment(account)
            ),
            query: Vec::new(),
            form,
            safety: OperationSafety::Mutation,
        };
        let wire: ApiV2010AccountMessage = self.client.inner.execute_json(&spec).await?;
        Message::try_from(wire).map_err(Into::into)
    }

    pub async fn get(&self, sid: &MessageSid) -> Result<Message, Error> {
        let account = self.client.inner.account_sid();
        let spec = RequestSpec {
            operation: "fetch_message",
            method: Method::GET,
            route_template: "/2010-04-01/Accounts/{AccountSid}/Messages/{Sid}.json",
            path: format!(
                "2010-04-01/Accounts/{}/Messages/{}.json",
                dialkit_core::request::encode_path_segment(account),
                dialkit_core::request::encode_path_segment(sid.as_str())
            ),
            query: Vec::new(),
            form: Vec::new(),
            safety: OperationSafety::Read,
        };
        let wire: ApiV2010AccountMessage = self.client.inner.execute_json(&spec).await?;
        Message::try_from(wire).map_err(Into::into)
    }

    #[must_use]
    pub fn list(&self, filter: ListMessages) -> Pager<Message> {
        let client = self.client.clone();
        let inner = dialkit_core::pagination::Pager::new(move |continuation| {
            let client = client.clone();
            let filter = filter.clone();
            async move {
                let account = client.inner.account_sid();
                let initial = continuation.is_none();
                let path = continuation.unwrap_or_else(|| {
                    format!(
                        "2010-04-01/Accounts/{}/Messages.json",
                        dialkit_core::request::encode_path_segment(account)
                    )
                });
                let query = if initial {
                    filter
                        .page_size
                        .map(|size| vec![("PageSize".into(), size.to_string())])
                        .unwrap_or_default()
                } else {
                    Vec::new()
                };
                let spec = RequestSpec {
                    operation: "list_message",
                    method: Method::GET,
                    route_template: "/2010-04-01/Accounts/{AccountSid}/Messages.json",
                    path,
                    query,
                    form: Vec::new(),
                    safety: OperationSafety::Read,
                };
                let response: ListMessageResponse = client.inner.execute_json(&spec).await?;
                let items = response
                    .messages
                    .unwrap_or_default()
                    .into_iter()
                    .map(Message::try_from)
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

impl fmt::Display for MessageSid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
