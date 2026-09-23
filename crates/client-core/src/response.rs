//! Transport-neutral streaming byte responses.

use crate::error::Error;
use bytes::Bytes;
use futures_core::Stream;
use futures_util::StreamExt as _;
use std::{fmt, pin::Pin};

pub type ByteStream = Pin<Box<dyn Stream<Item = Result<Bytes, Error>> + Send>>;

pub struct ByteResponse {
    content_type: Option<String>,
    content_length: Option<u64>,
    stream: ByteStream,
}

impl ByteResponse {
    pub(crate) fn from_reqwest(response: reqwest::Response) -> Self {
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned);
        let content_length = response.content_length();
        let stream = response.bytes_stream().map(|result| {
            result.map_err(|error| {
                if error.is_timeout() {
                    Error::Timeout { attempts: 1 }
                } else {
                    Error::Transport {
                        attempts: 1,
                        message: "byte stream failed".into(),
                    }
                }
            })
        });
        Self {
            content_type,
            content_length,
            stream: Box::pin(stream),
        }
    }

    #[must_use]
    pub fn content_type(&self) -> Option<&str> {
        self.content_type.as_deref()
    }

    #[must_use]
    pub const fn content_length(&self) -> Option<u64> {
        self.content_length
    }

    #[must_use]
    pub fn into_stream(self) -> ByteStream {
        self.stream
    }

    pub fn as_stream_mut(&mut self) -> Pin<&mut (dyn Stream<Item = Result<Bytes, Error>> + Send)> {
        self.stream.as_mut()
    }
}

impl fmt::Debug for ByteResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ByteResponse")
            .field("content_type", &self.content_type)
            .field("content_length", &self.content_length)
            .field("stream", &"[STREAM]")
            .finish()
    }
}
