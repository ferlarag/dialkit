# Contract: Public Rust API

## Package boundaries

| Package | Stability contract | Intended consumer |
|---------|--------------------|-------------------|
| `dialkit` | Stable Semantic Versioning; generated and transport types never appear in public signatures. | Application developers. |
| `dialkit-core` | Published for workspace dependency resolution; public only where required by generated and facade packages. | dialkit packages and advanced integrators. |
| `dialkit-api-generated` | Complete pinned api_v2010 surface; regenerated names/models may change only with an explicit versioned release. Not re-exported by `dialkit`. | Developers needing operations not yet wrapped by the facade. |

All packages use the same release version. Publication order is core, generated, facade. Package metadata clearly directs ordinary users to `cargo add dialkit`.

## Stable facade entry points

The following signatures are contract sketches. Exact generic bounds may be narrowed during implementation without adding generated or transport types to the public surface.

```rust
pub struct Client { /* private */ }

impl Client {
    pub fn new(
        account_sid: impl Into<AccountSid>,
        auth_token: impl IntoSecret,
    ) -> Result<Self, Error>;

    pub fn builder(credentials: Credentials) -> ClientBuilder;
    pub fn calls(&self) -> Calls;
    pub fn messages(&self) -> Messages;
}

pub struct ClientBuilder { /* private */ }

impl ClientBuilder {
    pub fn base_url(self, url: Url) -> Self;
    pub fn connect_timeout(self, timeout: Duration) -> Self;
    pub fn request_timeout(self, timeout: Duration) -> Self;
    pub fn retry_policy(self, policy: RetryPolicy) -> Self;
    pub fn build(self) -> Result<Client, Error>;
}
```

`Client`, service handles, immutable resource results, and safe configuration values are `Send + Sync`. No global client or runtime is created.

## Credentials

```rust
pub struct Credentials { /* secret-bearing private fields */ }

impl Credentials {
    pub fn account_token(account_sid: AccountSid, auth_token: impl IntoSecret) -> Self;
    pub fn api_key(
        account_sid: AccountSid,
        key_sid: ApiKeySid,
        key_secret: impl IntoSecret,
    ) -> Self;
}
```

Credential `Debug` output is redacted. Credentials implement neither `Display` nor `Serialize`, and the facade exposes no plaintext-secret getter.

## Calls

```rust
pub struct Calls { /* private */ }

impl Calls {
    pub async fn create(&self, request: CreateCall) -> Result<Call, Error>;
    pub async fn get(&self, sid: &CallSid) -> Result<Call, Error>;
    pub fn list(&self, filter: ListCalls) -> Pager<Call>;
}

pub struct CreateCall { /* private */ }

impl CreateCall {
    pub fn new(from: PhoneEndpoint, to: PhoneEndpoint, instructions: CallInstructions)
        -> Result<Self, Error>;
    // Optional task-focused setters are chainable and preserve invariants.
}

#[non_exhaustive]
pub enum CallInstructions {
    Twiml(TwimlResponse),
    Url(Url),
}
```

`CreateCall` cannot represent both inline TwiML and an instruction URL. Optional setters are added only when backed by the pinned schema and mapped by contract tests.

## Messages

```rust
pub struct Messages { /* private */ }

impl Messages {
    pub async fn create(&self, request: CreateMessage) -> Result<Message, Error>;
    pub async fn get(&self, sid: &MessageSid) -> Result<Message, Error>;
    pub fn list(&self, filter: ListMessages) -> Pager<Message>;
}

pub struct CreateMessage { /* private */ }

impl CreateMessage {
    pub fn new(from: MessageSender, to: PhoneEndpoint) -> Self;
    pub fn body(self, body: impl Into<String>) -> Self;
    pub fn media_url(self, url: Url) -> Self;
    pub fn build(self) -> Result<Self, Error>;
}
```

Construction enforces the pinned schema's sender and content alternatives. Phone numbers, message content, and media URLs never enter library-generated traces.

## Open service values

Service-controlled values such as call/message status are string-backed newtypes:

```rust
pub struct CallStatus(String);

impl CallStatus {
    pub const QUEUED: Self = /* known value */;
    pub const IN_PROGRESS: Self = /* known value */;
    pub fn as_str(&self) -> &str;
}
```

Deserialization and generated-to-facade conversion preserve unknown strings exactly. Consumers compare known constants but cannot exhaustively match future service values.

## Pagination

```rust
pub struct Pager<T> { /* private */ }
pub struct Page<T> { /* private */ }

impl<T> Pager<T> {
    pub async fn next(&mut self) -> Option<Result<T, Error>>;
    pub fn pages(self) -> PageStream<T>;
}

impl<T> Stream for Pager<T> {
    type Item = Result<T, Error>;
}
```

Item iteration preserves service order, returns every item at most once, and terminates after completion or the first error. Dropping the pager cancels future fetches. Continuation tokens/URIs are not required from facade users.

## Errors

```rust
#[non_exhaustive]
pub enum Error {
    Authentication(AuthenticationError),
    Validation(ValidationError),
    RateLimited(RateLimitError),
    Api(ApiError),
    Transport(TransportError),
    Timeout(TimeoutError),
    Decode(DecodeError),
    WebhookValidation(WebhookInputError),
    Xml(TwimlError),
}

pub struct ApiError { /* private, accessor-based */ }

impl ApiError {
    pub fn status(&self) -> StatusCode;
    pub fn code(&self) -> Option<u32>;
    pub fn message(&self) -> &str;
    pub fn more_info(&self) -> Option<&str>;
    pub fn request_id(&self) -> Option<&str>;
}
```

Public errors do not contain Reqwest or generated error types. Error and debug formatting honor the redaction contract.

## Retry policy

```rust
pub struct RetryPolicy { /* private */ }

impl RetryPolicy {
    pub fn conservative() -> Self;
    pub fn max_attempts(self, attempts: NonZeroU32) -> Self;
    pub fn initial_delay(self, delay: Duration) -> Self;
    pub fn max_delay(self, delay: Duration) -> Self;
    pub fn retry_mutations(self, enabled: bool) -> Self;
}
```

Mutation retry remains disabled unless explicitly selected. The method documentation warns about duplicate side effects.

## Webhooks

```rust
pub struct WebhookValidator { /* secret-bearing */ }

impl WebhookValidator {
    pub fn new(auth_token: impl IntoSecret) -> Self;
    pub fn validate_form(
        &self,
        external_url: &str,
        parameters: &[(String, String)],
        signature: &str,
    ) -> Result<ValidationResult, Error>;
    pub fn validate_json(
        &self,
        external_url: &str,
        raw_body: &[u8],
        signature: &str,
    ) -> Result<ValidationResult, Error>;
    pub fn port_compatibility(self, mode: PortCompatibility) -> Self;
}

#[non_exhaustive]
pub enum ValidationResult {
    Valid,
    Invalid(InvalidReason),
}
```

The caller supplies the exact externally visible URL and unmodified form pairs/raw body. Strict URL behavior is the default.

## TwiML

```rust
pub struct TwimlResponse { /* validated ordered nodes */ }

impl TwimlResponse {
    pub fn voice() -> VoiceResponseBuilder;
    pub fn messaging() -> MessagingResponseBuilder;
    pub fn to_xml(&self) -> Result<String, Error>;
}
```

Initial voice verbs are `Say`, `Play`, `Gather`, `Dial`, `Record`, `Hangup`, `Pause`, and `Redirect`. Initial messaging nodes are `Message`, `Body`, `Media`, and `Redirect`. Builders expose only valid child relationships where practical, escape all user values, preserve order, and create exactly one `<Response>` root. No raw-XML escape hatch is included initially.

## Generated companion access

The `dialkit` facade intentionally covers Calls and Messages first. A user needing any other pinned api_v2010 operation adds `dialkit-api-generated` at the same version and constructs its service client from the documented `dialkit-core` configuration. The generated companion:

- exposes every operation and model in the pinned specification;
- preserves unknown string enum values;
- uses the shared core for auth, execution, errors, retry, timeout, pagination primitives, and tracing;
- is not imported or re-exported from the stable `dialkit` namespace;
- carries prominent documentation that generator-driven API changes are reviewed and versioned but not part of the facade's long-term ergonomic guarantee.

## Feature flags

| Feature | Default | Contract |
|---------|---------|----------|
| `rustls-tls` | Yes | Enables Rustls transport. |
| `native-tls` | No | Enables native TLS; CI verifies it separately. |
| `webhooks` | Yes | Enables webhook validation and crypto dependencies. |
| `twiml` | Yes | Enables typed TwiML and XML dependency. |

TLS selections that cannot safely coexist must produce a clear compile-time error. Disabling optional features must not change unrelated public behavior.
