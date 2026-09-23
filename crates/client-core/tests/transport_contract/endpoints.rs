use dialkit_core::{
    auth::{AccountSid, Credentials},
    request::{EndpointProfile, EndpointService, HttpClient},
    retry::RetryPolicy,
};
use http::Method;
use secrecy::SecretString;
use std::time::Duration;
use url::Url;

fn credentials() -> Credentials {
    Credentials::account_token(
        AccountSid::new("AC00000000000000000000000000000000").unwrap(),
        SecretString::from("synthetic-endpoint-token".to_owned()),
    )
}

fn client(profile: EndpointProfile) -> HttpClient {
    HttpClient::for_profile(
        credentials(),
        profile,
        Duration::from_millis(100),
        Duration::from_secs(1),
        RetryPolicy::conservative(),
    )
    .unwrap()
}

#[test]
fn named_profiles_use_distinct_service_origins() {
    let api = EndpointProfile::api_2010();
    let messaging = EndpointProfile::messaging_v1();
    assert_eq!(api.service(), EndpointService::Api2010);
    assert_eq!(api.base_url().as_str(), "https://api.twilio.com/");
    assert_eq!(messaging.service(), EndpointService::MessagingV1);
    assert_eq!(
        messaging.base_url().as_str(),
        "https://messaging.twilio.com/"
    );
}

#[test]
fn credentials_are_attached_only_to_the_profile_origin() {
    let client = client(EndpointProfile::api_2010());
    let trusted = client
        .request(
            Method::GET,
            "https://api.twilio.com/2010-04-01/Accounts.json",
        )
        .build()
        .unwrap();
    let untrusted = client
        .request(Method::GET, "https://example.test/steal")
        .build()
        .unwrap();
    assert!(trusted.headers().contains_key("authorization"));
    assert!(!untrusted.headers().contains_key("authorization"));
}

#[test]
fn explicit_loopback_profiles_are_allowed_but_downgrades_are_not() {
    let profile = EndpointProfile::new(
        EndpointService::MessagingV1,
        Url::parse("http://127.0.0.1:4321/").unwrap(),
        true,
    );
    let client = client(profile);
    assert!(client.endpoint_profile().resolve("/v1/Services").is_ok());
    assert!(
        client
            .endpoint_profile()
            .resolve("https://messaging.twilio.com/v1/Services")
            .is_err()
    );
}
