use dialkit_core::{
    auth::{AccountSid, Credentials},
    error::Error,
    request::{EndpointProfile, EndpointService, HttpClient, RequestSpec},
    retry::{OperationSafety, RetryPolicy},
};
use futures_util::StreamExt as _;
use http::Method;
use secrecy::SecretString;
use std::time::Duration;
use url::Url;
use wiremock::{Mock, MockServer, ResponseTemplate, matchers::method};

fn client(server: &MockServer) -> HttpClient {
    HttpClient::for_profile(
        Credentials::account_token(
            AccountSid::new("AC00000000000000000000000000000000").unwrap(),
            SecretString::from("synthetic-media-token".to_owned()),
        ),
        EndpointProfile::new(
            EndpointService::Api2010,
            Url::parse(&server.uri()).unwrap(),
            true,
        ),
        Duration::from_millis(100),
        Duration::from_secs(1),
        RetryPolicy::conservative(),
    )
    .unwrap()
}

fn spec(path: &str) -> RequestSpec {
    RequestSpec {
        operation: "download_media",
        method: Method::GET,
        route_template: "/Media/{Sid}",
        path: path.to_owned(),
        query: vec![],
        form: vec![],
        safety: OperationSafety::Read,
    }
}

#[tokio::test]
async fn streams_exact_bytes_and_exposes_safe_metadata() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "audio/wav")
                .insert_header("content-length", "5")
                .set_body_bytes([0_u8, 1, 2, 254, 255]),
        )
        .mount(&server)
        .await;
    let response = client(&server).execute_bytes(&spec("media")).await.unwrap();
    assert_eq!(response.content_type(), Some("audio/wav"));
    assert_eq!(response.content_length(), Some(5));
    let chunks = response.into_stream().collect::<Vec<_>>().await;
    let bytes = chunks
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
        .concat();
    assert_eq!(bytes, [0, 1, 2, 254, 255]);
}

#[tokio::test]
async fn empty_bodies_stream_cleanly_and_can_be_cancelled_by_drop() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;
    let response = client(&server).execute_bytes(&spec("empty")).await.unwrap();
    let mut stream = response.into_stream();
    assert!(stream.next().await.is_none());
    drop(stream);
}

#[tokio::test]
async fn redirects_and_non_json_failures_are_safe_errors() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(302)
                .insert_header("location", "https://example.test/credential-canary"),
        )
        .mount(&server)
        .await;
    let error = client(&server)
        .execute_bytes(&spec("redirect"))
        .await
        .unwrap_err();
    assert!(matches!(error, Error::Api(_)));
    assert!(!format!("{error:?} {error}").contains("credential-canary"));

    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(500).set_body_string("secret-body-canary"))
        .mount(&server)
        .await;
    let error = client(&server)
        .execute_bytes(&spec("failure"))
        .await
        .unwrap_err();
    assert!(!format!("{error:?} {error}").contains("secret-body-canary"));
}
