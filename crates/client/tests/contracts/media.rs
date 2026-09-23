use dialkit::{AccountSid, Client, RecordingFormat, RecordingSid};
use futures_util::StreamExt as _;
use url::Url;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header_exists, method, path},
};

#[tokio::test]
async fn recording_metadata_and_byte_download_have_distinct_safe_paths() {
    let server = MockServer::start().await;
    let account = "AC00000000000000000000000000000000";
    let recording = RecordingSid::new("RE00000000000000000000000000000000").unwrap();
    Mock::given(method("GET"))
        .and(path(format!(
            "/2010-04-01/Accounts/{account}/Recordings/{}.wav",
            recording.as_str()
        )))
        .and(header_exists("authorization"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "audio/wav")
                .set_body_bytes([1_u8, 2, 3, 4]),
        )
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::builder(dialkit::Credentials::account_token(
        AccountSid::new(account).unwrap(),
        "synthetic-media-token",
    ))
    .base_url(Url::parse(&server.uri()).unwrap())
    .build()
    .unwrap();
    let body = client
        .recordings()
        .download(&recording, RecordingFormat::Wav)
        .await
        .unwrap();
    assert_eq!(body.content_type(), Some("audio/wav"));
    let bytes = body
        .map(|chunk| chunk.unwrap())
        .collect::<Vec<_>>()
        .await
        .concat();
    assert_eq!(bytes, [1, 2, 3, 4]);
}
