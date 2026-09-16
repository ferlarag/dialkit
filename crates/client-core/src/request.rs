//! Request execution through one authenticated, pooled HTTP client.

use crate::{
    auth::Credentials,
    error::{ApiError, Error, ResponseMetadata},
    retry::{OperationSafety, RetryPolicy},
    trace,
};
use reqwest::{Method, StatusCode, header};
use serde::{Serialize, de::DeserializeOwned};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tracing::Instrument as _;
use url::Url;

#[must_use]
pub fn encode_path_segment(value: &str) -> String {
    url::form_urlencoded::byte_serialize(value.as_bytes()).collect()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TlsBackend {
    Rustls,
    Native,
}

/// Reports the single backend selected for new clients. If downstream feature
/// unification enables both backends, Rustls wins deterministically.
#[must_use]
pub const fn active_tls_backend() -> TlsBackend {
    #[cfg(feature = "rustls-tls")]
    {
        TlsBackend::Rustls
    }
    #[cfg(all(not(feature = "rustls-tls"), feature = "native-tls"))]
    {
        TlsBackend::Native
    }
    #[cfg(not(any(feature = "rustls-tls", feature = "native-tls")))]
    {
        compile_error!("dialkit-core requires rustls-tls or native-tls");
    }
}

#[derive(Clone, Debug)]
pub struct ClientConfiguration {
    pub credentials: Credentials,
    pub base_url: Url,
    pub connect_timeout: Duration,
    pub request_timeout: Duration,
    pub retry_policy: RetryPolicy,
    pub allow_http_for_tests: bool,
}

#[derive(Clone, Debug)]
pub struct RequestSpec {
    pub operation: &'static str,
    pub method: Method,
    pub route_template: &'static str,
    pub path: String,
    pub query: Vec<(String, String)>,
    pub form: Vec<(String, String)>,
    pub safety: OperationSafety,
}

#[derive(Clone)]
pub struct HttpClient {
    config: Arc<ClientConfiguration>,
    http: reqwest::Client,
}

impl HttpClient {
    pub fn new(config: ClientConfiguration) -> Result<Self, Error> {
        config
            .credentials
            .validate()
            .map_err(|message| Error::Authentication(message.into()))?;
        validate_endpoint(&config.base_url, config.allow_http_for_tests)?;
        if config.connect_timeout.is_zero() || config.request_timeout < config.connect_timeout {
            return Err(Error::Validation(
                "request timeout must be at least the positive connect timeout".into(),
            ));
        }
        let builder = reqwest::Client::builder()
            .connect_timeout(config.connect_timeout)
            .timeout(config.request_timeout);
        #[cfg(feature = "rustls-tls")]
        let builder = builder.use_rustls_tls();
        #[cfg(all(not(feature = "rustls-tls"), feature = "native-tls"))]
        let builder = builder.use_native_tls();
        let http = builder.build().map_err(|error| Error::Transport {
            attempts: 0,
            message: safe_transport(&error),
        })?;
        Ok(Self {
            config: Arc::new(config),
            http,
        })
    }

    #[must_use]
    pub fn account_sid(&self) -> &str {
        self.config.credentials.account_sid().as_str()
    }
    #[must_use]
    pub fn base_url(&self) -> &Url {
        &self.config.base_url
    }

    /// Creates an authenticated request for schema-generated bindings.
    pub fn request(&self, method: Method, url: &str) -> reqwest::RequestBuilder {
        self.http.request(method, url).header(
            header::AUTHORIZATION,
            self.config.credentials.authorization_value(),
        )
    }

    /// Executes a generated request through the same retry, timeout, error,
    /// and tracing policy as handwritten facade operations.
    pub async fn execute(&self, request: reqwest::Request) -> Result<reqwest::Response, Error> {
        let method = request.method().clone();
        let safety = if matches!(method, Method::GET | Method::HEAD | Method::OPTIONS) {
            OperationSafety::Read
        } else {
            OperationSafety::Mutation
        };
        let span = trace::request_span("generated_operation", method.as_str(), "/generated");
        self.execute_request_inner(request, safety)
            .instrument(span)
            .await
    }

    pub async fn execute_json<T>(&self, spec: &RequestSpec) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let span = trace::request_span(spec.operation, spec.method.as_str(), spec.route_template);
        self.execute_json_inner(spec).instrument(span).await
    }

    async fn execute_json_inner<T>(&self, spec: &RequestSpec) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        tracing::info!(operation = spec.operation, "request started");
        let url = if spec.path.starts_with("http://") || spec.path.starts_with("https://") {
            crate::pagination::resolve_continuation(&self.config.base_url, &spec.path)?
        } else {
            self.config
                .base_url
                .join(spec.path.trim_start_matches('/'))
                .map_err(|_| Error::Validation("request path could not be resolved".into()))?
        };
        let max_attempts = self.config.retry_policy.attempts();
        for attempt in 1..=max_attempts {
            tracing::Span::current().record("attempt", attempt);
            let started = Instant::now();
            let mut request = self.http.request(spec.method.clone(), url.clone()).header(
                header::AUTHORIZATION,
                self.config.credentials.authorization_value(),
            );
            if !spec.query.is_empty() {
                request = request.query(&spec.query);
            }
            if !spec.form.is_empty() {
                request = request.form(&spec.form);
            }
            let response = request.send().await;
            match response {
                Ok(response) => {
                    let status = response.status();
                    let metadata = metadata(&response, attempt);
                    trace::record_response(&tracing::Span::current(), &metadata);
                    if status.is_success() {
                        let decoded = response.json::<T>().await.map_err(|_| Error::Decode {
                            message: "success body did not match the pinned response model".into(),
                            metadata,
                        });
                        trace::record_outcome(
                            &tracing::Span::current(),
                            if decoded.is_ok() {
                                "success"
                            } else {
                                "decode_error"
                            },
                            started.elapsed(),
                        );
                        return decoded;
                    }
                    let retry_after = metadata.retry_after;
                    let request_id = metadata.request_id.clone();
                    let body = response.text().await.unwrap_or_default();
                    let sensitive = self.sensitive_values_for_spec(spec);
                    let api = decode_api_error(status, &body, request_id, &sensitive)
                        .with_metadata(metadata.clone());
                    let retryable = status == StatusCode::TOO_MANY_REQUESTS
                        || (matches!(status.as_u16(), 500 | 502 | 503 | 504)
                            && self.config.retry_policy.retries_ambiguous(spec.safety));
                    if retryable && attempt < max_attempts {
                        trace::record_outcome(
                            &tracing::Span::current(),
                            "retry",
                            started.elapsed(),
                        );
                        tokio::time::sleep(self.config.retry_policy.delay(attempt, retry_after))
                            .await;
                        continue;
                    }
                    trace::record_outcome(
                        &tracing::Span::current(),
                        if status == StatusCode::TOO_MANY_REQUESTS {
                            "rate_limited"
                        } else {
                            "api_error"
                        },
                        started.elapsed(),
                    );
                    return if status == StatusCode::TOO_MANY_REQUESTS {
                        Err(Error::RateLimited {
                            attempts: attempt,
                            metadata: Box::new(metadata),
                            api: Some(Box::new(api)),
                        })
                    } else {
                        Err(Error::Api(Box::new(api)))
                    };
                }
                Err(error) => {
                    let retryable = (error.is_connect() || error.is_timeout())
                        && self.config.retry_policy.retries_ambiguous(spec.safety);
                    if retryable && attempt < max_attempts {
                        trace::record_outcome(
                            &tracing::Span::current(),
                            "retry",
                            started.elapsed(),
                        );
                        tokio::time::sleep(self.config.retry_policy.delay(attempt, None)).await;
                        continue;
                    }
                    trace::record_outcome(
                        &tracing::Span::current(),
                        if error.is_timeout() {
                            "timeout"
                        } else {
                            "transport_error"
                        },
                        started.elapsed(),
                    );
                    return if error.is_timeout() {
                        Err(Error::Timeout { attempts: attempt })
                    } else {
                        Err(Error::Transport {
                            attempts: attempt,
                            message: safe_transport(&error),
                        })
                    };
                }
            }
        }
        unreachable!("retry policy always contains at least one attempt")
    }

    async fn execute_request_inner(
        &self,
        request: reqwest::Request,
        safety: OperationSafety,
    ) -> Result<reqwest::Response, Error> {
        tracing::info!(operation = "generated_operation", "request started");
        let max_attempts = self.config.retry_policy.attempts();
        for attempt in 1..=max_attempts {
            tracing::Span::current().record("attempt", attempt);
            let started = Instant::now();
            let attempt_request = request.try_clone().ok_or_else(|| {
                Error::Validation("generated request body cannot be retried safely".into())
            })?;
            match self.http.execute(attempt_request).await {
                Ok(response) => {
                    let status = response.status();
                    let metadata = metadata(&response, attempt);
                    trace::record_response(&tracing::Span::current(), &metadata);
                    if status.is_success() {
                        trace::record_outcome(
                            &tracing::Span::current(),
                            "success",
                            started.elapsed(),
                        );
                        return Ok(response);
                    }
                    let retryable = status == StatusCode::TOO_MANY_REQUESTS
                        || (matches!(status.as_u16(), 500 | 502 | 503 | 504)
                            && self.config.retry_policy.retries_ambiguous(safety));
                    if retryable && attempt < max_attempts {
                        trace::record_outcome(
                            &tracing::Span::current(),
                            "retry",
                            started.elapsed(),
                        );
                        tokio::time::sleep(
                            self.config
                                .retry_policy
                                .delay(attempt, metadata.retry_after),
                        )
                        .await;
                        continue;
                    }
                    let request_id = metadata.request_id.clone();
                    let body = response.text().await.unwrap_or_default();
                    let sensitive = self.sensitive_values_for_request(&request);
                    let api = decode_api_error(status, &body, request_id, &sensitive)
                        .with_metadata(metadata.clone());
                    trace::record_outcome(
                        &tracing::Span::current(),
                        if status == StatusCode::TOO_MANY_REQUESTS {
                            "rate_limited"
                        } else {
                            "api_error"
                        },
                        started.elapsed(),
                    );
                    return if status == StatusCode::TOO_MANY_REQUESTS {
                        Err(Error::RateLimited {
                            attempts: attempt,
                            metadata: Box::new(metadata),
                            api: Some(Box::new(api)),
                        })
                    } else {
                        Err(Error::Api(Box::new(api)))
                    };
                }
                Err(error) => {
                    let retryable = (error.is_connect() || error.is_timeout())
                        && self.config.retry_policy.retries_ambiguous(safety);
                    if retryable && attempt < max_attempts {
                        trace::record_outcome(
                            &tracing::Span::current(),
                            "retry",
                            started.elapsed(),
                        );
                        tokio::time::sleep(self.config.retry_policy.delay(attempt, None)).await;
                        continue;
                    }
                    trace::record_outcome(
                        &tracing::Span::current(),
                        if error.is_timeout() {
                            "timeout"
                        } else {
                            "transport_error"
                        },
                        started.elapsed(),
                    );
                    return if error.is_timeout() {
                        Err(Error::Timeout { attempts: attempt })
                    } else {
                        Err(Error::Transport {
                            attempts: attempt,
                            message: safe_transport(&error),
                        })
                    };
                }
            }
        }
        unreachable!("retry policy always contains at least one attempt")
    }

    fn sensitive_values_for_spec(&self, spec: &RequestSpec) -> Vec<String> {
        let mut values = self.config.credentials.sensitive_values();
        values.extend(spec.query.iter().map(|(_, value)| value.clone()));
        values.extend(spec.form.iter().map(|(_, value)| value.clone()));
        values.extend(
            spec.path
                .split(['/', '?', '&', '='])
                .filter(|value| !value.is_empty())
                .map(str::to_owned),
        );
        values
    }

    fn sensitive_values_for_request(&self, request: &reqwest::Request) -> Vec<String> {
        let mut values = self.config.credentials.sensitive_values();
        values.extend(
            request
                .url()
                .path_segments()
                .into_iter()
                .flatten()
                .filter(|value| !value.is_empty())
                .map(str::to_owned),
        );
        values.extend(
            request
                .url()
                .query_pairs()
                .map(|(_, value)| value.into_owned()),
        );
        if let Some(body) = request.body().and_then(reqwest::Body::as_bytes) {
            values.extend(url::form_urlencoded::parse(body).map(|(_, value)| value.into_owned()));
        }
        values
    }
}

fn validate_endpoint(url: &Url, allow_http_for_tests: bool) -> Result<(), Error> {
    if url.scheme() == "https" {
        return Ok(());
    }
    let loopback = matches!(url.host_str(), Some("localhost" | "127.0.0.1" | "::1"));
    if url.scheme() == "http" && allow_http_for_tests && loopback {
        return Ok(());
    }
    Err(Error::Validation(
        "base URL must use HTTPS; HTTP is restricted to explicit loopback tests".into(),
    ))
}

fn metadata(response: &reqwest::Response, attempts: u32) -> ResponseMetadata {
    let string_header = |name: &str| {
        response
            .headers()
            .get(name)
            .and_then(|v| v.to_str().ok())
            .map(str::to_owned)
    };
    ResponseMetadata {
        status: Some(response.status().as_u16()),
        request_id: string_header("twilio-request-id"),
        concurrent_requests: string_header("twilio-concurrent-requests"),
        request_duration: string_header("twilio-request-duration"),
        retry_after: string_header("retry-after")
            .and_then(|v| v.parse::<u64>().ok())
            .map(Duration::from_secs),
        attempts,
    }
}

#[derive(serde::Deserialize)]
struct TwilioErrorBody {
    code: Option<u32>,
    message: Option<String>,
    more_info: Option<String>,
}

fn decode_api_error(
    status: StatusCode,
    body: &str,
    request_id: Option<String>,
    sensitive_values: &[String],
) -> ApiError {
    match serde_json::from_str::<TwilioErrorBody>(body) {
        Ok(value) => ApiError::new_redacted(
            status.as_u16(),
            value.code,
            value
                .message
                .unwrap_or_else(|| "Twilio rejected the request".into()),
            value.more_info,
            request_id,
            None,
            sensitive_values,
        ),
        Err(_) => ApiError::new_redacted(
            status.as_u16(),
            None,
            "Twilio returned an undocumented error response",
            None,
            request_id,
            Some(body),
            sensitive_values,
        ),
    }
}

fn safe_transport(error: &reqwest::Error) -> String {
    if error.is_connect() {
        "connection failed".into()
    } else if error.is_timeout() {
        "request timed out".into()
    } else {
        "HTTP transport failed".into()
    }
}

#[allow(dead_code)]
fn _assert_serialize<T: Serialize>(_value: &T) {}
