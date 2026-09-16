use dialkit_api_generated::{
    apis::{
        api20100401_call_api, api20100401_conference_api, api20100401_message_api,
        api20100401_recording_api, configuration::Configuration,
    },
    models::{CallEnumStatus, ConferenceEnumUpdateStatus},
};
use dialkit_core::{
    auth::{AccountSid, Credentials},
    request::{ClientConfiguration, HttpClient},
    retry::RetryPolicy,
};
use secrecy::SecretString;
use std::time::Duration;
use url::Url;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path},
};

fn configuration(server: &MockServer) -> Configuration {
    let client = HttpClient::new(ClientConfiguration {
        credentials: Credentials::account_token(
            AccountSid::new("ACaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap(),
            SecretString::from("secret".to_owned()),
        ),
        base_url: Url::parse(&server.uri()).unwrap(),
        connect_timeout: Duration::from_secs(1),
        request_timeout: Duration::from_secs(2),
        retry_policy: RetryPolicy::conservative(),
        allow_http_for_tests: true,
    })
    .unwrap();
    Configuration::new(client)
}

#[tokio::test]
async fn representative_resource_paths_are_encoded_and_responses_decode() {
    let server = MockServer::start().await;
    assert_eq!(
        dialkit_api_generated::apis::urlencode("segment/with space"),
        "segment%2Fwith+space"
    );
    let account = "ACaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    let cases = [
        "/2010-04-01/Accounts/ACaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa/Calls/CA11111111111111111111111111111111.json",
        "/2010-04-01/Accounts/ACaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa/Messages/SM22222222222222222222222222222222.json",
        "/2010-04-01/Accounts/ACaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa/Recordings/RE33333333333333333333333333333333.json",
        "/2010-04-01/Accounts/ACaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa/Conferences/CF44444444444444444444444444444444.json",
    ];
    for route in cases {
        Mock::given(method("GET"))
            .and(path(route))
            .respond_with(ResponseTemplate::new(200).set_body_raw("{}", "application/json"))
            .expect(1)
            .mount(&server)
            .await;
    }
    let config = configuration(&server);
    api20100401_call_api::fetch_call(
        &config,
        api20100401_call_api::FetchCallParams {
            account_sid: account.into(),
            sid: "CA11111111111111111111111111111111".into(),
        },
    )
    .await
    .unwrap();
    api20100401_message_api::fetch_message(
        &config,
        api20100401_message_api::FetchMessageParams {
            account_sid: account.into(),
            sid: "SM22222222222222222222222222222222".into(),
        },
    )
    .await
    .unwrap();
    api20100401_recording_api::fetch_recording(
        &config,
        api20100401_recording_api::FetchRecordingParams {
            account_sid: account.into(),
            sid: "RE33333333333333333333333333333333".into(),
            include_soft_deleted: None,
        },
    )
    .await
    .unwrap();
    api20100401_conference_api::fetch_conference(
        &config,
        api20100401_conference_api::FetchConferenceParams {
            account_sid: account.into(),
            sid: "CF44444444444444444444444444444444".into(),
        },
    )
    .await
    .unwrap();
}

#[test]
fn generated_configuration_redacts_credentials() {
    let server = futures_lite_server_uri();
    let client = HttpClient::new(ClientConfiguration {
        credentials: Credentials::account_token(
            AccountSid::new("ACdddddddddddddddddddddddddddddddd").unwrap(),
            SecretString::from("token-debug-canary".to_owned()),
        ),
        base_url: Url::parse(&server).unwrap(),
        connect_timeout: Duration::from_secs(1),
        request_timeout: Duration::from_secs(2),
        retry_policy: RetryPolicy::conservative(),
        allow_http_for_tests: true,
    })
    .unwrap();
    let rendered = format!("{:?}", Configuration::new(client));
    assert!(!rendered.contains("ACdddddddddddddddddddddddddddddddd"));
    assert!(!rendered.contains("token-debug-canary"));
    assert!(rendered.contains("[REDACTED]"));
}

fn futures_lite_server_uri() -> String {
    "http://127.0.0.1:9".to_owned()
}

#[test]
fn unknown_enum_values_round_trip_verbatim() {
    let status: CallEnumStatus = serde_json::from_str("\"future-status\"").unwrap();
    assert_eq!(status.as_str(), "future-status");
    assert_eq!(serde_json::to_string(&status).unwrap(), "\"future-status\"");
}

#[test]
fn every_generated_string_enum_shape_round_trips_unknown_values() {
    macro_rules! check {
        ($($name:ident),+ $(,)?) => {$({
            let value: dialkit_api_generated::models::$name =
                serde_json::from_str("\"introduced-after-pin\"").unwrap();
            assert_eq!(value.as_str(), "introduced-after-pin", stringify!($name));
            assert_eq!(
                serde_json::to_string(&value).unwrap(),
                "\"introduced-after-pin\"",
                stringify!($name)
            );
        })+};
    }
    check!(
        AccountEnumStatus,
        AccountEnumType,
        AuthorizedConnectAppEnumPermission,
        CallEnumEvent,
        CallEnumStatus,
        CallEnumUpdateStatus,
        CallRecordingEnumSource,
        CallRecordingEnumStatus,
        ConferenceEnumReasonConferenceEnded,
        ConferenceEnumStatus,
        ConferenceEnumUpdateStatus,
        ConferenceRecordingEnumSource,
        ConferenceRecordingEnumStatus,
        ConnectAppEnumPermission,
        DependentPhoneNumberEnumAddressRequirement,
        DependentPhoneNumberEnumEmergencyStatus,
        IncomingPhoneNumberEnumAddressRequirement,
        IncomingPhoneNumberEnumEmergencyAddressStatus,
        IncomingPhoneNumberEnumEmergencyStatus,
        IncomingPhoneNumberEnumVoiceReceiveMode,
        IncomingPhoneNumberLocalEnumAddressRequirement,
        IncomingPhoneNumberLocalEnumEmergencyAddressStatus,
        IncomingPhoneNumberLocalEnumEmergencyStatus,
        IncomingPhoneNumberLocalEnumVoiceReceiveMode,
        IncomingPhoneNumberMobileEnumAddressRequirement,
        IncomingPhoneNumberMobileEnumEmergencyAddressStatus,
        IncomingPhoneNumberMobileEnumEmergencyStatus,
        IncomingPhoneNumberMobileEnumVoiceReceiveMode,
        IncomingPhoneNumberTollFreeEnumAddressRequirement,
        IncomingPhoneNumberTollFreeEnumEmergencyAddressStatus,
        IncomingPhoneNumberTollFreeEnumEmergencyStatus,
        IncomingPhoneNumberTollFreeEnumVoiceReceiveMode,
        MessageEnumAddressRetention,
        MessageEnumContentRetention,
        MessageEnumDirection,
        MessageEnumRiskCheck,
        MessageEnumScheduleType,
        MessageEnumStatus,
        MessageEnumTrafficType,
        MessageEnumUpdateStatus,
        MessageFeedbackEnumOutcome,
        ParticipantEnumStatus,
        PaymentsEnumBankAccountType,
        PaymentsEnumCapture,
        PaymentsEnumPaymentMethod,
        PaymentsEnumStatus,
        PaymentsEnumTokenType,
        RealtimeTranscriptionEnumStatus,
        RealtimeTranscriptionEnumTrack,
        RealtimeTranscriptionEnumUpdateStatus,
        RecordingAddOnResultEnumStatus,
        RecordingEnumSource,
        RecordingEnumStatus,
        RecordingTranscriptionEnumStatus,
        SiprecEnumStatus,
        SiprecEnumTrack,
        SiprecEnumUpdateStatus,
        SmsFeedbackEnumOutcome,
        SmsMessageEnumDirection,
        SmsMessageEnumStatus,
        SmsMessageEnumUpdateStatus,
        StreamEnumStatus,
        StreamEnumTrack,
        StreamEnumUpdateStatus,
        TranscriptionEnumStatus,
        UsageTriggerEnumRecurring,
        UsageTriggerEnumTriggerField,
    );

    macro_rules! check_nested {
        ($($name:path),+ $(,)?) => {$({
            let value: $name = serde_json::from_str("\"introduced-after-pin\"").unwrap();
            assert_eq!(value.as_str(), "introduced-after-pin", stringify!($name));
            assert_eq!(
                serde_json::to_string(&value).unwrap(),
                "\"introduced-after-pin\"",
                stringify!($name)
            );
        })+};
    }
    use dialkit_api_generated::models;
    check_nested!(
        models::api_v2010_account_address_dependent_phone_number::SmsFallbackMethod,
        models::api_v2010_account_address_dependent_phone_number::SmsMethod,
        models::api_v2010_account_address_dependent_phone_number::StatusCallbackMethod,
        models::api_v2010_account_address_dependent_phone_number::VoiceFallbackMethod,
        models::api_v2010_account_address_dependent_phone_number::VoiceMethod,
        models::api_v2010_account_application::SmsFallbackMethod,
        models::api_v2010_account_application::SmsMethod,
        models::api_v2010_account_application::StatusCallbackMethod,
        models::api_v2010_account_application::VoiceFallbackMethod,
        models::api_v2010_account_application::VoiceMethod,
        models::api_v2010_account_call_call_notification::RequestMethod,
        models::api_v2010_account_call_call_notification_instance::RequestMethod,
        models::api_v2010_account_connect_app::DeauthorizeCallbackMethod,
        models::api_v2010_account_incoming_phone_number::SmsFallbackMethod,
        models::api_v2010_account_incoming_phone_number::SmsMethod,
        models::api_v2010_account_incoming_phone_number::StatusCallbackMethod,
        models::api_v2010_account_incoming_phone_number::VoiceFallbackMethod,
        models::api_v2010_account_incoming_phone_number::VoiceMethod,
        models::api_v2010_account_incoming_phone_number_incoming_phone_number_local::SmsFallbackMethod,
        models::api_v2010_account_incoming_phone_number_incoming_phone_number_local::SmsMethod,
        models::api_v2010_account_incoming_phone_number_incoming_phone_number_local::StatusCallbackMethod,
        models::api_v2010_account_incoming_phone_number_incoming_phone_number_local::VoiceFallbackMethod,
        models::api_v2010_account_incoming_phone_number_incoming_phone_number_local::VoiceMethod,
        models::api_v2010_account_incoming_phone_number_incoming_phone_number_mobile::SmsFallbackMethod,
        models::api_v2010_account_incoming_phone_number_incoming_phone_number_mobile::SmsMethod,
        models::api_v2010_account_incoming_phone_number_incoming_phone_number_mobile::StatusCallbackMethod,
        models::api_v2010_account_incoming_phone_number_incoming_phone_number_mobile::VoiceFallbackMethod,
        models::api_v2010_account_incoming_phone_number_incoming_phone_number_mobile::VoiceMethod,
        models::api_v2010_account_incoming_phone_number_incoming_phone_number_toll_free::SmsFallbackMethod,
        models::api_v2010_account_incoming_phone_number_incoming_phone_number_toll_free::SmsMethod,
        models::api_v2010_account_incoming_phone_number_incoming_phone_number_toll_free::StatusCallbackMethod,
        models::api_v2010_account_incoming_phone_number_incoming_phone_number_toll_free::VoiceFallbackMethod,
        models::api_v2010_account_incoming_phone_number_incoming_phone_number_toll_free::VoiceMethod,
        models::api_v2010_account_notification::RequestMethod,
        models::api_v2010_account_notification_instance::RequestMethod,
        models::api_v2010_account_short_code::SmsFallbackMethod,
        models::api_v2010_account_short_code::SmsMethod,
        models::api_v2010_account_sip_sip_domain::VoiceFallbackMethod,
        models::api_v2010_account_sip_sip_domain::VoiceMethod,
        models::api_v2010_account_sip_sip_domain::VoiceStatusCallbackMethod,
        models::api_v2010_account_usage_usage_trigger::CallbackMethod,
    );
}

#[tokio::test]
async fn representative_mutations_authenticate_and_preserve_form_values() {
    let server = MockServer::start().await;
    let account = "ACaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    let auth = format!(
        "Basic {}",
        base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            format!("{account}:secret")
        )
    );
    for (verb, route, status, body) in [
        (
            "POST",
            format!("/2010-04-01/Accounts/{account}/Calls.json"),
            201,
            "{}",
        ),
        (
            "POST",
            format!("/2010-04-01/Accounts/{account}/Messages.json"),
            201,
            "{}",
        ),
        (
            "DELETE",
            format!(
                "/2010-04-01/Accounts/{account}/Recordings/RE33333333333333333333333333333333.json"
            ),
            204,
            "",
        ),
        (
            "POST",
            format!(
                "/2010-04-01/Accounts/{account}/Conferences/CF44444444444444444444444444444444.json"
            ),
            200,
            "{}",
        ),
    ] {
        Mock::given(method(verb))
            .and(path(route))
            .and(header("authorization", auth.as_str()))
            .respond_with(ResponseTemplate::new(status).set_body_raw(body, "application/json"))
            .expect(1)
            .mount(&server)
            .await;
    }
    let config = configuration(&server);
    api20100401_call_api::create_call(
        &config,
        api20100401_call_api::CreateCallParams {
            account_sid: account.into(),
            to: "+15005550009".into(),
            from: "+15005550006".into(),
            method: None,
            fallback_url: None,
            fallback_method: None,
            status_callback: Some("https://example.test/calls".into()),
            status_callback_event: Some(vec!["initiated".into(), "completed".into()]),
            status_callback_method: Some("POST".into()),
            send_digits: None,
            timeout: None,
            record: None,
            recording_channels: None,
            recording_status_callback: None,
            recording_status_callback_method: None,
            recording_configuration_id: None,
            sip_auth_username: None,
            sip_auth_password: None,
            machine_detection: None,
            machine_detection_timeout: None,
            recording_status_callback_event: None,
            trim: None,
            caller_id: None,
            machine_detection_speech_threshold: None,
            machine_detection_speech_end_threshold: None,
            machine_detection_silence_timeout: None,
            async_amd: None,
            async_amd_status_callback: None,
            async_amd_status_callback_method: None,
            passports: None,
            byoc: None,
            call_reason: None,
            call_token: None,
            recording_track: None,
            time_limit: None,
            client_notification_url: None,
            url: Some("https://example.test/voice".into()),
            twiml: None,
            application_sid: None,
        },
    )
    .await
    .unwrap();
    api20100401_message_api::create_message(
        &config,
        api20100401_message_api::CreateMessageParams {
            account_sid: account.into(),
            to: "+15005550009".into(),
            status_callback: None,
            application_sid: None,
            max_price: None,
            provide_feedback: None,
            attempt: None,
            validity_period: None,
            force_delivery: None,
            content_retention: None,
            address_retention: None,
            smart_encoded: None,
            persistent_action: None,
            traffic_type: None,
            shorten_urls: None,
            schedule_type: None,
            send_at: None,
            send_as_mms: None,
            content_variables: None,
            message_intent: None,
            risk_check: None,
            from: Some("+15005550006".into()),
            fallback_from: None,
            messaging_service_sid: None,
            body: Some("hello".into()),
            media_url: Some(vec![
                "https://example.test/one.png".into(),
                "https://example.test/two.png".into(),
            ]),
            content_sid: None,
        },
    )
    .await
    .unwrap();
    api20100401_recording_api::delete_recording(
        &config,
        api20100401_recording_api::DeleteRecordingParams {
            account_sid: account.into(),
            sid: "RE33333333333333333333333333333333".into(),
        },
    )
    .await
    .unwrap();
    api20100401_conference_api::update_conference(
        &config,
        api20100401_conference_api::UpdateConferenceParams {
            account_sid: account.into(),
            sid: "CF44444444444444444444444444444444".into(),
            status: Some(ConferenceEnumUpdateStatus::Completed),
            announce_url: Some("https://example.test/announce".into()),
            announce_method: Some("POST".into()),
        },
    )
    .await
    .unwrap();

    let requests = server.received_requests().await.unwrap();
    assert_eq!(requests.len(), 4);
    let call_form = url::form_urlencoded::parse(requests[0].body.as_slice()).collect::<Vec<_>>();
    assert!(
        call_form.contains(&("StatusCallbackEvent".into(), "initiated".into())),
        "unexpected call form: {call_form:?}"
    );
    assert!(
        call_form.contains(&("StatusCallbackEvent".into(), "completed".into())),
        "unexpected call form: {call_form:?}"
    );
    let message_form = url::form_urlencoded::parse(requests[1].body.as_slice()).collect::<Vec<_>>();
    assert_eq!(
        message_form
            .iter()
            .filter(|(name, _)| name == "MediaUrl")
            .count(),
        2
    );
}
