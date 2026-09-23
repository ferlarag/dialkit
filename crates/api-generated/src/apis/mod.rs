use std::error;
use std::fmt;

#[derive(Default)]
pub(crate) struct FormParams(Vec<(&'static str, String)>);

impl FormParams {
    pub(crate) fn insert(&mut self, name: &'static str, value: String) {
        self.0.push((name, value));
    }
}

impl serde::Serialize for FormParams {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serde::Serialize::serialize(&self.0, serializer)
    }
}

#[derive(Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

pub enum Error<T> {
    Core(dialkit_core::error::Error),
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl<T> fmt::Debug for ResponseContent<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ResponseContent")
            .field("status", &self.status)
            .field("content", &"[REDACTED]")
            .field("entity", &self.entity.as_ref().map(|_| "[REDACTED]"))
            .finish()
    }
}

impl<T> fmt::Debug for Error<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Core(error) => formatter.debug_tuple("Core").field(error).finish(),
            Self::Reqwest(_) => formatter.write_str("Reqwest([REDACTED])"),
            Self::Serde(_) => formatter.write_str("Serde([REDACTED])"),
            Self::Io(_) => formatter.write_str("Io([REDACTED])"),
            Self::ResponseError(content) => formatter
                .debug_tuple("ResponseError")
                .field(content)
                .finish(),
        }
    }
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Core(error) => write!(formatter, "dialkit transport: {error}"),
            Self::Reqwest(_) => formatter.write_str("request construction failed"),
            Self::Serde(_) => formatter.write_str("response decoding failed"),
            Self::Io(_) => formatter.write_str("I/O failed"),
            Self::ResponseError(content) => {
                write!(formatter, "response returned status {}", content.status)
            }
        }
    }
}

impl<T> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Core(error) => Some(error),
            Self::Reqwest(error) => Some(error),
            Self::Serde(error) => Some(error),
            Self::Io(error) => Some(error),
            Self::ResponseError(_) => None,
        }
    }
}

impl<T> From<dialkit_core::error::Error> for Error<T> {
    fn from(error: dialkit_core::error::Error) -> Self {
        Self::Core(error)
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(error: reqwest::Error) -> Self {
        Self::Reqwest(error)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(error: serde_json::Error) -> Self {
        Self::Serde(error)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

pub fn urlencode<T: AsRef<str>>(value: T) -> String {
    ::url::form_urlencoded::byte_serialize(value.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];
        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => {
                    params.append(&mut parse_deep_object(&format!("{prefix}[{key}]"), value));
                }
                serde_json::Value::Array(array) => {
                    for (index, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{prefix}[{key}][{index}]"),
                            value,
                        ));
                    }
                }
                serde_json::Value::String(value) => {
                    params.push((format!("{prefix}[{key}]"), value.clone()));
                }
                _ => params.push((format!("{prefix}[{key}]"), value.to_string())),
            }
        }
        return params;
    }
    Vec::new()
}

#[allow(dead_code)]
enum ContentType {
    Json,
    Text,
    Unsupported(String),
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        if content_type.starts_with("application") && content_type.contains("json") {
            Self::Json
        } else if content_type.starts_with("text/plain") {
            Self::Text
        } else {
            Self::Unsupported(content_type.to_owned())
        }
    }
}

pub mod api20100401_account_api;
pub mod api20100401_add_on_result_api;
pub mod api20100401_address_api;
pub mod api20100401_all_time_api;
pub mod api20100401_application_api;
pub mod api20100401_assigned_add_on_api;
pub mod api20100401_assigned_add_on_extension_api;
pub mod api20100401_auth_calls_credential_list_mapping_api;
pub mod api20100401_auth_calls_ip_access_control_list_mapping_api;
pub mod api20100401_auth_registrations_credential_list_mapping_api;
pub mod api20100401_authorized_connect_app_api;
pub mod api20100401_available_phone_number_country_api;
pub mod api20100401_balance_api;
pub mod api20100401_call_api;
pub mod api20100401_call_notification_api;
pub mod api20100401_call_recording_api;
pub mod api20100401_call_transcription_api;
pub mod api20100401_conference_api;
pub mod api20100401_conference_recording_api;
pub mod api20100401_connect_app_api;
pub mod api20100401_credential_api;
pub mod api20100401_credential_list_api;
pub mod api20100401_credential_list_mapping_api;
pub mod api20100401_daily_api;
pub mod api20100401_data_api;
pub mod api20100401_dependent_phone_number_api;
pub mod api20100401_domain_api;
pub mod api20100401_event_api;
pub mod api20100401_feedback_api;
pub mod api20100401_incoming_phone_number_api;
pub mod api20100401_incoming_phone_number_local_api;
pub mod api20100401_incoming_phone_number_mobile_api;
pub mod api20100401_incoming_phone_number_toll_free_api;
pub mod api20100401_ip_access_control_list_api;
pub mod api20100401_ip_access_control_list_mapping_api;
pub mod api20100401_key_api;
pub mod api20100401_last_month_api;
pub mod api20100401_local_api;
pub mod api20100401_machine_to_machine_api;
pub mod api20100401_media_api;
pub mod api20100401_media_instance_api;
pub mod api20100401_member_api;
pub mod api20100401_message_api;
pub mod api20100401_mobile_api;
pub mod api20100401_monthly_api;
pub mod api20100401_national_api;
pub mod api20100401_new_key_api;
pub mod api20100401_new_signing_key_api;
pub mod api20100401_notification_api;
pub mod api20100401_outgoing_caller_id_api;
pub mod api20100401_participant_api;
pub mod api20100401_payload_api;
pub mod api20100401_payment_api;
pub mod api20100401_queue_api;
pub mod api20100401_record_api;
pub mod api20100401_recording_api;
pub mod api20100401_recording_transcription_api;
pub mod api20100401_shared_cost_api;
pub mod api20100401_short_code_api;
pub mod api20100401_signing_key_api;
pub mod api20100401_sip_ip_address_api;
pub mod api20100401_siprec_api;
pub mod api20100401_stream_api;
pub mod api20100401_this_month_api;
pub mod api20100401_today_api;
pub mod api20100401_token_api;
pub mod api20100401_toll_free_api;
pub mod api20100401_transcription_api;
pub mod api20100401_trigger_api;
pub mod api20100401_user_defined_message_api;
pub mod api20100401_user_defined_message_subscription_api;
pub mod api20100401_validation_request_api;
pub mod api20100401_voip_api;
pub mod api20100401_yearly_api;
pub mod api20100401_yesterday_api;

pub mod configuration;
