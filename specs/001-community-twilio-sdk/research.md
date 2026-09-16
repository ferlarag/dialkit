# Research: Community Twilio SDK

## 1. Workspace and publication boundary

**Decision**: Use a Rust 2024 virtual workspace with `dialkit-core`, `dialkit-api-generated`, and `dialkit`. Publish all three in dependency order at a lockstep version. Keep the dependency graph one-way: `dialkit -> {dialkit-core, dialkit-api-generated}` and `dialkit-api-generated -> dialkit-core`. Never re-export generated types from `dialkit`.

**Rationale**: The generated package must depend on the shared transport without allowing generator concerns into core. The facade can convert generated types into stable public types. crates.io packages cannot rely on an unpublished path-only dependency, so the two supporting crates must be publishable even though only `dialkit` is the intended entry point.

**Alternatives considered**: A non-published generated crate prevents publishing the facade; embedding generated source in the facade loses the requested ownership boundary; a self-contained generated client duplicates transport, authentication, retry, and error policy.

## 2. Rust version and dependency policy

**Decision**: Retain Edition 2024 and set `rust-version = "1.85"`, the edition floor. Test Rust 1.85 and stable. Use compatible direct dependency requirements in manifests and commit the exact resolved `Cargo.lock`.

**Rationale**: The repository already targets Edition 2024, and a new SDK gains a clear, supportable MSRV without a migration. Exact transitive resolution remains reproducible while normal compatible dependency ranges allow downstream resolution.

**Alternatives considered**: Edition 2021 would reach older toolchains but contradict the initialized project and adds maintenance for no established user base. Raising the MSRV to current stable would shorten compatibility unnecessarily.

**Sources**: [Rust 2024 Edition Guide](https://doc.rust-lang.org/edition-guide/rust-2024/index.html), [Rust 1.85 release](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/).

## 3. HTTP and asynchronous runtime

**Decision**: Use Reqwest 0.12.25 with default features disabled and Rustls/JSON enabled. Use Tokio 1.47.1 for time support in libraries and add macros plus multi-thread runtime only for development/tests. A reusable core client owns pooling, authentication, timeouts, retry, execution, response metadata, error decoding, and tracing. Generated operations only serialize operation-specific inputs and deserialize typed outputs through that core.

**Rationale**: OpenAPI Generator's current Rust Reqwest template aligns with Reqwest 0.12. One shared reusable client prevents diverging security and resilience behavior. Rustls avoids a system TLS dependency by default.

**Alternatives considered**: Reqwest 0.13 requires a coordinated custom-template migration; Hyper exposes unnecessary low-level detail; generated self-owned clients duplicate policy; a general retry middleware lacks endpoint semantics needed to avoid duplicate calls/messages.

**Sources**: [OpenAPI Generator Rust client](https://openapi-generator.tech/docs/generators/rust/), [Reqwest ClientBuilder](https://docs.rs/reqwest/latest/reqwest/struct.ClientBuilder.html).

## 4. Generator selection

**Decision**: Pin OpenAPI Generator 7.25.0, tag commit `ef964b04480889ef86b56cfae84ade8ad4c91c41`, and CLI JAR SHA-256 `41ce4f6b07f196676439d710759fa1ced7a08066d06ff1bf314681470289efae`. Select generator `rust`, library `reqwest`, asynchronous operations, single request-parameter structs, and hidden generation timestamps.

**Rationale**: The generator provides broad schema coverage, Basic authentication, and an async Reqwest client. Its output ergonomics and incomplete support for advanced schema composition are acceptable behind a boundary but require compile and contract tests.

**Alternatives considered**: `reqwest-trait` generates additional mocking/builder surface that belongs in dialkit; Progenitor is more idiomatic but targets OpenAPI 3.0.x and warns arbitrary documents may fail; a custom Java generator or Twilio generator fork adds a second substantial codebase.

**Sources**: [Rust generator documentation](https://openapi-generator.tech/docs/generators/rust/), [installation and version pinning](https://openapi-generator.tech/docs/installation/), [Progenitor](https://github.com/oxidecomputer/progenitor).

## 5. Twilio specification pin

**Decision**: Initially adopt `twilio/twilio-oai` commit `5aa7f31977ce5812f7b7bc1f46a38555ebaa2888`, file `spec/json/twilio_api_v2010.json`, SHA-256 `170b3ccd0f891416840083d72f1795b1499b14a18d4873fd2b39f47ef84642d6`. Vendor or checksum-verify the file obtained from the commit-addressed raw URL; never generate from `main`.

**Rationale**: Full commit and content hashes give immutable identity, offline review, and a stable generation input while preserving Twilio's specification as the source of truth.

**Alternatives considered**: A release tag is readable but still should resolve to a full SHA. Fetching a moving branch makes clean regeneration non-reproducible.

**Sources**: [Twilio OpenAPI repository](https://github.com/twilio/twilio-oai), [api_v2010 document](https://github.com/twilio/twilio-oai/blob/main/spec/json/twilio_api_v2010.json).

## 6. Template ownership and unknown enum values

**Decision**: Extract templates from generator 7.25.0 and commit only required overrides under `codegen/templates/rust`. Record template revision `dialkit-rust-v1` and a deterministic tree hash. Override model generation so string enums serialize known values and preserve unrecognized service values as their original string. Public facade enum-like values use string-backed newtypes with known associated constants and `as_str()`.

**Rationale**: The generator does not offer `enumUnknownDefaultCase` for Rust, and a closed generated enum fails during deserialization before a facade can convert it. Raw-value preservation supports forward compatibility and avoids forcing exhaustive matches that later become breaking.

**Alternatives considered**: `#[serde(other)]` loses the original value; representing every generated enum as an untyped string loses known-value discoverability; editing generated files is non-reproducible.

**Sources**: [OpenAPI template customization](https://openapi-generator.tech/docs/templating/), [Rust generator implementation](https://github.com/OpenAPITools/openapi-generator/blob/master/modules/openapi-generator/src/main/java/org/openapitools/codegen/languages/RustClientCodegen.java), [Rust model template](https://github.com/OpenAPITools/openapi-generator/blob/master/modules/openapi-generator/src/main/resources/rust/model.mustache).

## 7. Authentication and secret handling

**Decision**: Support preferred API-key Basic authentication (`AccountSid`, `ApiKeySid`, `ApiKeySecret`) and convenient account-token Basic authentication (`AccountSid`, `AuthToken`). Store secrets in `SecretString`, omit serialization/plaintext getters, implement redacted debug output, and expose plaintext only inside the narrow authorization-header construction path.

**Rationale**: Twilio recommends API keys for applications while some workflows and the requested sample use the account token. Supporting both enables rotation/scoping without blocking simple adoption. Explicit exposure and redacted output reduce accidental leaks.

**Alternatives considered**: SID/token only is simpler but weaker for production rotation and scoping; plain strings plus custom `Debug` remain easy to clone or expose accidentally.

**Sources**: [Twilio API requests and authentication](https://www.twilio.com/docs/usage/requests-to-twilio), [secrecy](https://docs.rs/secrecy/0.10.3/secrecy/).

## 8. Errors and response metadata

**Decision**: Expose a stable non-exhaustive error taxonomy for authentication, validation, rate limiting, transport, timeout, decoding, webhook validation, XML, and Twilio API failures. API failures preserve HTTP status, optional numeric Twilio code, message, optional `more_info`, request ID, safe response metadata, and a bounded/redacted raw body when structured decoding fails.

**Rationale**: Developers need the Twilio code and correlation ID even when the response is malformed or undocumented. Non-exhaustive stable types can grow without leaking Reqwest/generated errors.

**Alternatives considered**: Generated errors lose malformed-response context; exposing Reqwest types couples the facade to transport; an unstructured catch-all error is not actionable.

**Sources**: [Twilio API responses](https://www.twilio.com/docs/usage/twilios-response), [Twilio REST API best practices](https://www.twilio.com/docs/usage/rest-api-best-practices), [thiserror](https://docs.rs/thiserror/latest/thiserror/).

## 9. Pagination and continuation safety

**Decision**: Expose a concrete `Pager<T>` stream and page-level access. Follow `next_page_uri` as an opaque value, preserve order, stop when absent/null, and reject an absolute or resolved continuation whose origin differs from the configured Twilio endpoint before attaching credentials.

**Rationale**: Twilio instructs clients to follow the returned URI, which may contain evolving cursor parameters. Streaming bounds memory and cancellation occurs by dropping the pager. Origin validation prevents a malicious or malformed response from exfiltrating credentials.

**Alternatives considered**: Reconstructing cursors is fragile; an eager vector is unbounded; requiring manual continuation harms ergonomics.

**Sources**: [Twilio pagination](https://www.twilio.com/docs/usage/twilios-response), [futures-core Stream](https://docs.rs/futures-core/latest/futures_core/stream/).

## 10. Retry and timeout policy

**Decision**: Apply connect and total request timeouts. Provide bounded exponential backoff with full jitter, a delay cap, maximum attempts, and `Retry-After` support. By default retry 429 responses and safe read/list operations on selected transient failures. Do not retry mutating api_v2010 requests after ambiguous timeout/5xx unless endpoint metadata proves safety or the caller explicitly opts in.

**Rationale**: Twilio states a 429 request was not processed and recommends backoff, but api_v2010 has no universal outbound idempotency key. Blanket POST retries can duplicate calls and messages.

**Alternatives considered**: Retrying every transient status risks duplicate side effects; no retries pushes routine rate-limit recovery onto every user.

**Sources**: [Twilio REST API best practices](https://www.twilio.com/docs/usage/rest-api-best-practices), [Twilio retry guidance](https://help.twilio.com/articles/48916449686299).

## 11. Tracing and redaction

**Decision**: Emit opt-in `tracing` spans/events using an allowlist: HTTP method, route template, response status, Twilio request ID, attempt, and elapsed time. Skip all function arguments. Never record authorization, secrets, complete URLs/query strings, form/body content, phone numbers, or message content. The application owns subscriber configuration.

**Rationale**: Attribute instrumentation records debug arguments unless skipped. Allowlisting is safer than trying to enumerate every possible sensitive field after logging.

**Alternatives considered**: Verbose HTTP logging and denylist redaction are prone to secret/PII leakage; owning a subscriber would impose global application policy.

**Source**: [`tracing::instrument`](https://docs.rs/tracing/latest/tracing/attr.instrument.html).

## 12. Webhook validation

**Decision**: Provide distinct form and JSON validators that accept the exact externally visible URL. Form validation preserves all pairs, including repeated keys, and applies Twilio's sorting/canonicalization rules. JSON validation verifies raw-body SHA-256 against `bodySHA256` and signs the exact URL. Require `X-Twilio-Signature`, use HMAC-SHA1/Base64, and verify in constant time. Add an explicit opt-in compatibility mode for Twilio's historical with/without-port behavior; never silently reconstruct proxy URLs.

**Rationale**: Twilio signs the externally visible URL and evolving request parameters. Raw JSON bytes and repeated form values cannot be recovered reliably after framework parsing. Explicit URL ownership works behind reverse proxies.

**Alternatives considered**: Generic middleware reconstruction is convenient but unreliable behind TLS termination/proxies; a single map loses repeated keys; normal string equality leaks timing information. Shared-key webhook signing is outside this api_v2010 MVP.

**Sources**: [Twilio webhook security](https://www.twilio.com/docs/usage/webhooks/webhooks-security), [Twilio security algorithm](https://www.twilio.com/docs/usage/security), [Twilio Python validator](https://github.com/twilio/twilio-python/blob/main/twilio/request_validator.py).

## 13. TwiML representation

**Decision**: Handwrite a typed, ordered TwiML AST/builder and render it with `quick_xml::Writer`. Emit UTF-8 XML with one `<Response>` root, exact case-sensitive names, escaped text/attributes, and construction-time nesting validation where practical. Initially support Voice `Say`, `Play`, `Gather`, `Dial`, `Record`, `Hangup`, `Pause`, and `Redirect`, and Messaging `Message`, `Body`, `Media`, and `Redirect`.

**Rationale**: TwiML is ordered XML with nested mixed content. Typed construction prevents common invalid structures without exposing string concatenation.

**Alternatives considered**: Raw XML strings are unsafe and error-prone; a generic Serde XML model is awkward for ordered mixed content; generating all TwiML creates another large unstable surface.

**Sources**: [Twilio Voice TwiML](https://www.twilio.com/docs/voice/twiml), [quick-xml Writer](https://docs.rs/quick-xml/latest/quick_xml/writer/struct.Writer.html).

## 14. Testing and generation drift

**Decision**: Layer tests: unit/doc/compile tests; Wiremock contracts for auth, encoding, errors, pagination, timeouts, retries, and redaction; golden/property tests for webhook canonicalization and TwiML; generated-layer compilation and representative fixtures; clean temporary regeneration plus complete tree comparison. Keep ordinary CI credential-free. Allow opt-in live smoke tests only with Twilio test credentials and supported magic values.

**Rationale**: Schema validity cannot prove wire behavior or ergonomics. Clean-output comparison catches deleted stale files. Twilio test credentials are non-billing but cover only a subset, so they supplement rather than replace local contracts.

**Alternatives considered**: Regenerating in place may leave stale files; live-only tests are incomplete, flaky, and credential-dependent; schema-only tests miss behavioral defects.

**Source**: [Twilio test credentials](https://www.twilio.com/docs/iam/test-credentials).
