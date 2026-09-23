//! Incremental, ordered pagination.

use crate::error::Error;
use futures_core::Stream;
use futures_util::{StreamExt, future::BoxFuture};
use std::{
    collections::HashSet,
    pin::Pin,
    task::{Context, Poll},
};
use url::Url;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct OpaqueContinuation(String);

impl OpaqueContinuation {
    #[must_use]
    pub fn new(value: impl Into<String>) -> Option<Self> {
        let value = value.into();
        (!value.is_empty()).then_some(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

pub fn api_2010_continuation(next_page_uri: Option<String>) -> Option<OpaqueContinuation> {
    next_page_uri.and_then(OpaqueContinuation::new)
}

pub fn messaging_v1_continuation(next_page_url: Option<String>) -> Option<OpaqueContinuation> {
    next_page_url.and_then(OpaqueContinuation::new)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub next_page_uri: Option<String>,
}

impl<T> Page<T> {
    #[must_use]
    pub fn from_continuation(items: Vec<T>, continuation: Option<OpaqueContinuation>) -> Self {
        Self {
            items,
            next_page_uri: continuation.map(OpaqueContinuation::into_string),
        }
    }

    #[must_use]
    pub fn continuation(&self) -> Option<OpaqueContinuation> {
        self.next_page_uri.clone().and_then(OpaqueContinuation::new)
    }
}

pub type PageStream<T> = Pin<Box<dyn Stream<Item = Result<Page<T>, Error>> + Send>>;

pub struct Pager<T> {
    pages: PageStream<T>,
    current: std::vec::IntoIter<T>,
}

impl<T: Send + Unpin + 'static> Pager<T> {
    pub fn new<F>(mut fetch: F) -> Self
    where
        F: FnMut(Option<String>) -> BoxFuture<'static, Result<Page<T>, Error>> + Send + 'static,
    {
        let pages = async_stream::stream! {
            let mut cursor = None;
            let mut seen = HashSet::new();
            loop {
                let page = match fetch(cursor.take()).await {
                    Ok(page) => page,
                    Err(error) => { yield Err(error); break; }
                };
                let next = page.next_page_uri.clone();
                yield Ok(page);
                match next {
                    None => break,
                    Some(uri) if !seen.insert(uri.clone()) => {
                        yield Err(Error::Validation("repeated pagination continuation URI".into()));
                        break;
                    }
                    Some(uri) => cursor = Some(uri),
                }
            }
        };
        Self {
            pages: Box::pin(pages),
            current: Vec::new().into_iter(),
        }
    }

    pub async fn next(&mut self) -> Option<Result<T, Error>> {
        StreamExt::next(self).await
    }

    #[must_use]
    pub fn pages(self) -> PageStream<T> {
        let mut pages = self.pages;
        let remaining = self.current.collect::<Vec<_>>();
        Box::pin(async_stream::stream! {
            if !remaining.is_empty() {
                yield Ok(Page { items: remaining, next_page_uri: None });
            }
            while let Some(page) = pages.next().await {
                yield page;
            }
        })
    }
}

impl<T: Unpin> Stream for Pager<T> {
    type Item = Result<T, Error>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            if let Some(item) = self.current.next() {
                return Poll::Ready(Some(Ok(item)));
            }
            match self.pages.as_mut().poll_next(cx) {
                Poll::Ready(Some(Ok(page))) => self.current = page.items.into_iter(),
                Poll::Ready(Some(Err(error))) => return Poll::Ready(Some(Err(error))),
                Poll::Ready(None) => return Poll::Ready(None),
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}

pub fn resolve_continuation(base: &Url, continuation: &str) -> Result<Url, Error> {
    let resolved = base
        .join(continuation)
        .map_err(|_| Error::Validation("invalid pagination continuation URI".into()))?;
    if resolved.scheme() != base.scheme()
        || resolved.host_str() != base.host_str()
        || resolved.port_or_known_default() != base.port_or_known_default()
    {
        return Err(Error::Validation(
            "cross-origin pagination continuation rejected".into(),
        ));
    }
    Ok(resolved)
}
