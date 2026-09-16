# Data Model: Community Twilio SDK

This document describes the stable domain model and the internal records needed to satisfy the feature. Generated api_v2010 models are derived directly from the pinned specification and are not repeated here.

## Client

### ClientConfiguration

| Field | Type | Rules |
|-------|------|-------|
| `account_sid` | `AccountSid` | Required; non-empty; identifies the account used in api_v2010 paths. |
| `credentials` | `Credentials` | Required; secret-bearing; never serialized or printed. |
| `base_url` | URL | Defaults to Twilio's HTTPS REST origin; custom origin allowed for tests; must be HTTPS outside explicit test configuration. |
| `connect_timeout` | Duration | Positive and bounded; default documented by the facade. |
| `request_timeout` | Duration | Positive, greater than or equal to connect timeout. |
| `retry_policy` | `RetryPolicy` | Bounded attempts/delays; conservative default. |
| `tls_backend` | Feature selection | Rustls by default; native TLS opt-in and mutually exclusive at build configuration. |

Validation occurs when the client is built. Once built, the configuration is immutable and safe to share; per-operation inputs do not mutate global client state.

### Credentials

Variants:

- `AccountToken { auth_token }`: Basic-auth username is the account SID; secret is the auth token.
- `ApiKey { key_sid, key_secret }`: Basic-auth username is the API key SID; secret is the key secret; account SID remains the resource scope.

Rules:

- Secret values use protected containers, expose no plaintext accessor in public API, do not implement serialization, and display as `[REDACTED]`.
- Empty identifiers or secrets fail client construction.
- Authorization material exists only while constructing the outbound header.

### RetryPolicy

| Field | Type | Rules |
|-------|------|-------|
| `max_attempts` | Positive integer | Includes the initial attempt; default is bounded; zero is invalid. |
| `initial_delay` | Duration | Non-negative and no greater than maximum delay. |
| `max_delay` | Duration | Positive hard cap. |
| `multiplier` | Positive number | At least 1.0. |
| `jitter` | Strategy | Full jitter by default; deterministic injection available only for tests. |
| `mutation_mode` | Enum | `Never` by default or explicit `OptIn`; does not override cross-origin or validation failures. |

## Operations and resources

### ResourceService

A lightweight handle borrowing or cloning the shared `Client`, scoped to one resource family. Initial stable services are `Calls` and `Messages`; complete api_v2010 services exist in the generated companion.

Relationship: one `Client` creates any number of service handles, all sharing the same connection pool and immutable policy.

### OperationInput

Stable facade inputs use private fields and validated builders.

Common fields:

| Field | Cardinality | Rules |
|-------|-------------|-------|
| Resource identifiers | Per operation | Validate non-empty and known prefix/shape where Twilio specifies one. |
| `from` and `to` | Required for create operations unless Twilio defines an alternative | Preserve exact user value; never emit in traces. |
| Content/instructions | Operation-specific | Enforce mutually exclusive inputs such as inline TwiML versus callback URL. |
| Optional service fields | Zero or more | Omitted values are not serialized; builder methods follow task language. |

Initial stable inputs:

- `CreateCall`: account scope from client; required `from`, `to`, and exactly one call instruction source; optional callback/status settings supported as deliberately added facade fields.
- `CreateMessage`: required sender, recipient, and at least one supported content source; optional messaging service/callback/media settings with Twilio-defined mutual-exclusion rules.
- List filters for calls/messages: date/status/address filters and page size; continuation remains internal.

### ResourceResult

Stable resource snapshots with private or non-exhaustive fields and accessors. Identifiers, timestamps, addresses, status-like values, and resource URIs map explicitly from generated responses. Enum-like service values use `OpenValue`, below.

### OpenValue

A string-backed, equality/hashable value with:

- Known associated constants for values in the pinned specification.
- `as_str()` and conversion from owned/borrowed strings.
- Exact preservation of unknown values.
- No exhaustive public enum matching requirement.

Validation: empty values are retained if received from the service but rejected where the developer constructs an input requiring a non-empty value.

## Pagination

### Page<T>

| Field | Type | Rules |
|-------|------|-------|
| `items` | Ordered list of `T` | Preserve service order. |
| `page` | Optional non-negative integer | Service metadata; absence is allowed. |
| `page_size` | Optional positive integer | Service metadata; not assumed to equal item count. |
| `uri` | Optional relative/safe URL | Diagnostic metadata; credentials never embedded. |
| `next_page_uri` | Optional opaque URI | Internal continuation after same-origin validation. |
| `previous_page_uri` | Optional opaque URI | Exposed only as metadata; not needed for item streaming. |

### Pager<T>

State transitions:

```text
Ready(first request)
  -> Fetching
  -> Yielding(page items)
  -> Ready(valid next_page_uri)
  -> Complete(no continuation)
  -> Failed(error)

Any state -> Cancelled when dropped
```

Rules:

- At most one page request is in flight for sequential iteration.
- A continuation is resolved against the configured base URL and must retain the same scheme, host, and effective port before authentication is attached.
- A repeated continuation URI is detected and fails rather than looping indefinitely.
- A mid-stream failure yields one error and then terminates.

## Errors and response metadata

### Error

A stable, non-exhaustive error category with variants/accessors for:

- `Authentication`: invalid local credential configuration.
- `Validation`: invalid operation input before a request.
- `RateLimited`: response status, retry timing, request ID, and optional Twilio error.
- `Api`: `ApiError` returned by Twilio.
- `Transport`: connection/protocol failure with secrets removed.
- `Timeout`: connect or total request timeout and attempt count.
- `Decode`: unexpected response format plus bounded safe context.
- `WebhookValidation`: malformed or incomplete validation input; a validly formed signature mismatch is normally a `ValidationResult::Invalid`, not an operational error.
- `Xml`: invalid TwiML structure or rendering failure.

The error type never exposes underlying Reqwest or generated errors as public fields.

### ApiError

| Field | Type | Rules |
|-------|------|-------|
| `status` | HTTP status | Always present from response. |
| `code` | Optional unsigned integer | Preserve Twilio code when parseable. |
| `message` | String | Redacted and bounded. |
| `more_info` | Optional URL/string | Preserve when safe and parseable. |
| `details` | Optional structured value | Bounded and redacted; forward-compatible. |
| `request_id` | Optional string | From Twilio response header. |
| `raw_excerpt` | Optional bytes/string | Only for malformed/nonstandard errors; bounded and redacted. |

### ResponseMetadata

Optional values include Twilio request ID, concurrent-request count, request duration, retry timing, HTTP status, and attempt count. It excludes complete request URLs, authorization, and payloads.

## Webhook validation

### FormWebhookInput

| Field | Type | Rules |
|-------|------|-------|
| `external_url` | Exact URL string | Required; supplied by caller; not normalized or reconstructed. |
| `parameters` | Ordered multimap/pairs | Preserve every name/value, including duplicates. |
| `signature` | Base64 string | Required from `X-Twilio-Signature`. |
| `port_compatibility` | Enum | Strict by default; explicit compatibility mode only. |

### JsonWebhookInput

| Field | Type | Rules |
|-------|------|-------|
| `external_url` | Exact URL string including query | Required; includes `bodySHA256` exactly as received. |
| `raw_body` | Bytes | Required; no reserialization. |
| `signature` | Base64 string | Required. |
| `port_compatibility` | Enum | Strict by default. |

### ValidationResult

States:

- `Valid`: digest and signature requirements pass.
- `Invalid(SignatureMismatch)`: well-formed signature does not match.
- `Invalid(BodyDigestMismatch)`: JSON raw body does not match `bodySHA256`.

Malformed Base64, missing signature/digest, or unusable URL is an input error rather than `Invalid`, allowing developers to distinguish malformed delivery from a genuine negative validation result.

## TwiML

### TwimlResponse

An ordered collection of top-level voice or messaging nodes. Rendering always produces one case-sensitive `<Response>` root and UTF-8 escaped XML.

### TwimlNode

Initial typed nodes:

- Voice: `Say`, `Play`, `Gather`, `Dial`, `Record`, `Hangup`, `Pause`, `Redirect`.
- Messaging: `Message` containing ordered `Body` and `Media` nouns, plus messaging `Redirect`.

Each node owns known attributes as typed/private fields, validates required values, and controls which children can be nested. Unknown raw XML is not part of the initial stable contract.

State transitions:

```text
Builder(empty) -> Builder(valid partial) -> TwimlResponse(validated) -> XML(rendered)
                         \-> Validation error
```

## Generation and release records

### GenerationProvenance

| Field | Initial value | Rule |
|-------|---------------|------|
| `generator_name` | `rust` | Fixed for a generation run. |
| `generator_version` | `7.25.0` | Exact semantic version. |
| `generator_artifact_sha256` | `41ce4f6b07f196676439d710759fa1ced7a08066d06ff1bf314681470289efae` | Verified before execution. |
| `twilio_spec_commit` | `5aa7f31977ce5812f7b7bc1f46a38555ebaa2888` | Full SHA only. |
| `twilio_spec_path` | `spec/json/twilio_api_v2010.json` | Repository-relative upstream source. |
| `twilio_spec_sha256` | `170b3ccd0f891416840083d72f1795b1499b14a18d4873fd2b39f47ef84642d6` | Verified against vendored/fetched bytes. |
| `template_revision` | `dialkit-rust-v1` | Human-readable immutable revision for a release. |
| `template_tree_sha256` | Computed | Deterministic hash of sorted template paths and contents. |
| `configuration_sha256` | Computed | Hash of generation configuration. |

Lifecycle:

```text
Proposed upstream revision
  -> Diff reviewed
  -> Pins updated
  -> Generated in clean directory
  -> Contract/compatibility checks passed
  -> Committed candidate
  -> Released with provenance
```

Any changed pin resets the candidate to the review stage. A clean run with unchanged pins must yield a byte-for-byte equivalent tree after the documented formatter step.
