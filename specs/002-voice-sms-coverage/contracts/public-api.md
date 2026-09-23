# Contract: Stable Public API

## Compatibility rule

All existing `dialkit` symbols, method signatures, feature defaults, validation meanings, serialization, XML output, and webhook validation results remain available and equivalent. Additions are permitted; generated, core, and Reqwest types never appear in stable signatures. The generated companions are separately consumable low-level APIs at the same release version.

## Client entry points

Existing entry points remain:

```rust
Client::new(account_sid, auth_token)
Client::builder(credentials)
Client::calls()
Client::messages()
```

Additive task-oriented handles may include:

```rust
client.conferences()
client.recordings()
client.queues()
client.sip()
client.applications()
client.phone_numbers()
client.media()
client.messaging_services()
```

Handles are cheap clones over internal clients. Common child resources are navigable from their parent, for example conferences to participants, queues to members, SIP domains to mappings, and Messaging Services to senders. Exact generated operation names remain available only in the companions.

`ClientBuilder::base_url` retains its API-2010 meaning. An additive `messaging_base_url` configures Messaging v1, primarily for local tests. Both inherit credentials, timeouts, retry, TLS, and redaction policy.

## Requests and resources

- Existing Calls/Messages types and constructors are unchanged.
- New requests use private fields and fluent builders; construction rejects only unambiguous pinned constraints.
- Mutually exclusive inputs are represented by enums or validation, not last-write-wins setters.
- Resource SIDs use typed validated newtypes where stable prefixes exist.
- Statuses/events are string-backed open values with known constants and `as_str()`.
- Resources are immutable views with accessors; no generated model is exposed.

Common facade coverage is selective. Every one of the 197 operations is available in a generated companion, but stable convenience methods are required only for FR-004/FR-018 workflows.

## Pagination

Existing `Pager<T>`, `Page<T>`, `Pager::next`, `Stream`, and `Pager::pages` behavior remains. New list methods return the same abstraction. Optional one-page methods may return a stable page with an opaque continuation; users never assemble page URLs/tokens.

## Media download

Metadata fetch and byte download are different methods. Download returns a transport-neutral streaming body or writes to a caller-provided async sink, with safe content type/length access. It does not return `reqwest::Response` or automatically persist/transcode content. Recording format selection is constrained and explicit.

## TwiML

Existing entry points and methods remain byte-compatible:

```rust
TwimlResponse::voice()
TwimlResponse::messaging()
VoiceResponseBuilder::{say,play,gather,dial,record,hangup,pause,redirect}
MessagingResponseBuilder::{message,redirect}
```

New typed verbs/nouns and attribute setters cover the complete matrix. Parent-specific types expose only legal children where practical. `build()` performs cross-field/range/control-flow validation; `to_xml()` emits one `Response` root with deterministic order and escaping. There is no raw-XML escape hatch. `Debug` is redacted for content-bearing builders/responses.

## Webhooks

The existing validator methods and meanings remain:

```rust
WebhookValidator::validate_form(...)
WebhookValidator::validate_json(...)
PortCompatibility::Strict // default
```

New APIs require the caller to choose an event family and support either:

1. validate raw input, then parse through a verified-family value; or
2. an equivalent `verify_and_parse_<family>` convenience method.

They never infer event type from overlapping fields. Typed events expose known fields and correlation identifiers plus inspectable extras. Unknown status/event strings remain exact. Form extras retain duplicates and empty values; JSON extras retain structured values. Parsed events do not retain signatures or credentials.

## Errors and redaction

The existing non-exhaustive `Error` and `ApiError` accessors remain. New failures map into existing categories unless a new additive non-exhaustive variant is necessary. Errors identify operation/family and field/rule but do not echo sensitive values. Debug/display/traces omit credentials, authorization, signatures, raw bodies, message/TwiML content, media URLs, phone/SIP identifiers or secrets, payment values, concrete authenticated URLs, and unknown-field bags.

## Generated companions

- `dialkit-api-generated`: preserves existing API-2010 public paths and may continue exposing operations outside this feature for compatibility.
- `dialkit-messaging-generated`: exposes all 58 pinned Messaging-v1 operations and models.
- Both preserve unknown enum raw strings, use the shared core, and document their generator-driven stability separately.
- Neither is re-exported by `dialkit`; compile-fail tests enforce the boundary.

## Documentation contract

Public documentation includes compiling synthetic examples for the 12 FR-018 workflows, links to exhaustive generated docs and coverage reports, a TwiML nesting reference, webhook verify-before-use guidance, media download safety, and compatibility notes. Examples require no real credentials and make no network call to Twilio in CI.
