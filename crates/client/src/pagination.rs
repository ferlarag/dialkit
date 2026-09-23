//! Stable facade pagination.

use crate::Error;
use futures_core::Stream;
use futures_util::StreamExt as _;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

pub(crate) fn api_2010_page<T>(
    items: Vec<T>,
    next_page_uri: Option<String>,
) -> dialkit_core::pagination::Page<T> {
    dialkit_core::pagination::Page::from_continuation(
        items,
        dialkit_core::pagination::api_2010_continuation(next_page_uri),
    )
}

#[allow(dead_code)]
pub(crate) fn messaging_v1_page<T>(
    items: Vec<T>,
    next_page_url: Option<String>,
) -> dialkit_core::pagination::Page<T> {
    dialkit_core::pagination::Page::from_continuation(
        items,
        dialkit_core::pagination::messaging_v1_continuation(next_page_url),
    )
}

pub struct Pager<T> {
    pub(crate) inner: dialkit_core::pagination::Pager<T>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Page<T> {
    items: Vec<T>,
}

impl<T> Page<T> {
    #[must_use]
    pub fn items(&self) -> &[T] {
        &self.items
    }

    #[must_use]
    pub fn into_items(self) -> Vec<T> {
        self.items
    }
}

pub type PageStream<T> = Pin<Box<dyn Stream<Item = Result<Page<T>, Error>> + Send>>;

impl<T: Send + Unpin + 'static> Pager<T> {
    pub(crate) fn new(inner: dialkit_core::pagination::Pager<T>) -> Self {
        Self { inner }
    }
    pub async fn next(&mut self) -> Option<Result<T, Error>> {
        self.inner.next().await.map(|item| item.map_err(Into::into))
    }

    #[must_use]
    pub fn pages(self) -> PageStream<T> {
        Box::pin(self.inner.pages().map(|page| {
            page.map(|page| Page { items: page.items })
                .map_err(Into::into)
        }))
    }
}

impl<T: Unpin> Stream for Pager<T> {
    type Item = Result<T, Error>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.inner).poll_next(cx) {
            Poll::Ready(Some(item)) => Poll::Ready(Some(item.map_err(Into::into))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
