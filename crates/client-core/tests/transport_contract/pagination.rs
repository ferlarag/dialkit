use dialkit_core::{
    error::Error,
    pagination::{Page, Pager, resolve_continuation},
};
use futures_util::{FutureExt as _, StreamExt as _};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};
use url::Url;

fn pager(pages: Vec<Result<Page<u32>, Error>>) -> Pager<u32> {
    let pages = Arc::new(Mutex::new(VecDeque::from(pages)));
    Pager::new(move |_| {
        let pages = Arc::clone(&pages);
        async move { pages.lock().unwrap().pop_front().unwrap() }.boxed()
    })
}

#[tokio::test]
async fn empty_single_and_multiple_pages_preserve_order() {
    let mut empty = pager(vec![Ok(Page {
        items: vec![],
        next_page_uri: None,
    })]);
    assert!(empty.next().await.is_none());
    let mut values = pager(vec![
        Ok(Page {
            items: vec![1, 2],
            next_page_uri: Some("?PageToken=opaque".into()),
        }),
        Ok(Page {
            items: vec![3],
            next_page_uri: None,
        }),
    ]);
    assert_eq!(
        values
            .by_ref()
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .unwrap(),
        vec![1, 2, 3]
    );
}

#[tokio::test]
async fn page_level_access_preserves_service_page_boundaries() {
    let pages = pager(vec![
        Ok(Page {
            items: vec![1, 2],
            next_page_uri: Some("?next".into()),
        }),
        Ok(Page {
            items: vec![3],
            next_page_uri: None,
        }),
    ])
    .pages()
    .collect::<Vec<_>>()
    .await
    .into_iter()
    .collect::<Result<Vec<_>, _>>()
    .unwrap();
    assert_eq!(pages.len(), 2);
    assert_eq!(pages[0].items, vec![1, 2]);
    assert_eq!(pages[1].items, vec![3]);
}

#[tokio::test]
async fn repeated_continuation_and_midstream_error_are_emitted_once() {
    let mut repeated = pager(vec![
        Ok(Page {
            items: vec![1],
            next_page_uri: Some("?same".into()),
        }),
        Ok(Page {
            items: vec![2],
            next_page_uri: Some("?same".into()),
        }),
    ]);
    assert!(repeated.next().await.unwrap().is_ok());
    assert!(repeated.next().await.unwrap().is_ok());
    assert!(repeated.next().await.unwrap().is_err());
    assert!(repeated.next().await.is_none());

    let mut failed = pager(vec![
        Ok(Page {
            items: vec![1],
            next_page_uri: Some("?next".into()),
        }),
        Err(Error::Transport {
            attempts: 1,
            message: "injected".into(),
        }),
    ]);
    assert!(failed.next().await.unwrap().is_ok());
    assert!(failed.next().await.unwrap().is_err());
    assert!(failed.next().await.is_none());
}

#[test]
fn continuation_must_remain_same_origin() {
    let base = Url::parse("https://api.twilio.test/").unwrap();
    assert!(resolve_continuation(&base, "/next?token=opaque").is_ok());
    assert!(resolve_continuation(&base, "https://evil.test/steal").is_err());
    assert!(resolve_continuation(&base, "http://api.twilio.test/downgrade").is_err());
}
