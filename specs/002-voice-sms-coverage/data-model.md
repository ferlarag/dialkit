# Data Model: Complete Voice and Messaging Coverage

This feature has no database model. These entities describe version-controlled generation records and the public/runtime value model.

## PinnedSource

| Field | Type | Rules |
|-------|------|-------|
| `source_id` | `API-2010` or `MSG-V1` | Stable, unique key. |
| `repository` | String | `twilio/twilio-oai`. |
| `commit` | 40-character SHA | Exactly `5aa7f31977ce5812f7b7bc1f46a38555ebaa2888` for this release. |
| `path` | Repository-relative path | Exact upstream file. |
| `sha256` | 64 lowercase hex chars | Verified before generation/audit. |
| `default_origin` | HTTPS origin | API 2010: `api.twilio.com`; Messaging v1: `messaging.twilio.com`. |
| `selected_operations` | Integer | 139 or 58; union is 197. |

Relationships: one source owns many REST coverage entries and one generated companion. The same commit does not make the two files interchangeable.

## DocumentationSnapshot

| Field | Type | Rules |
|-------|------|-------|
| `reviewed_on` | Date | `2026-09-17`. |
| `source_urls` | Non-empty list | Official TwiML/webhook/security references. |
| `twiml_manifest_hash` | SHA-256 | Hash of normalized normative node/relationship manifest. |
| `webhook_manifest_hash` | SHA-256 | Hash of normalized family/variant manifest. |
| `approved_exclusions` | List | Must match the feature spec. |

## CoverageKey

Discriminated key:

- REST: `(source_id, operation_id)`
- TwiML: `(family, node, parent_context)`; root nodes use `Response` as context
- Webhook: `(family, variant, encoding)`

Keys are unique. Operation IDs alone are not unique across sources.

## RestCoverageEntry

| Field | Type | Rules |
|-------|------|-------|
| `key` | REST `CoverageKey` | Resolves exactly once in its pinned source. |
| `domain` | Stable label | Matches one feature-matrix row. |
| `method`, `path` | HTTP contract | Must equal pinned OpenAPI. |
| `generated_crate`, `generated_symbol` | String | Compiles and maps to the source operation. |
| `phase` | Delivery phase | Cannot regress after becoming complete. |
| `pagination` | Boolean | True only for list/page operations. |
| `evidence` | Evidence matrix | Every required dimension is fixture-backed or explicitly N/A. |

Required evidence dimensions: method/path, authentication, path/query/header/form/body encoding as applicable, success decoding, error decoding, and pagination.

State transitions:

```text
declared -> generated -> contract-covered -> compatibility-checked -> complete
    \----------------------------------------------------------> blocked
```

`complete` requires every applicable dimension to pass. A source/hash change resets the entry to `declared`.

## TwimlCoverageEntry

| Field | Type | Rules |
|-------|------|-------|
| `key` | TwiML `CoverageKey` | One row per legal parent relationship. |
| `kind` | Verb or noun | Exact case-sensitive XML name. |
| `attributes` | Attribute specifications | Name, type, requiredness, enum/range/cross-field rules. |
| `content` | Text/empty/children/mixed policy | Determines typed builder shape. |
| `control_flow` | Continue or transfer/terminal | Used for reachability validation/documentation. |
| `evidence` | Fixture references | Canonical XML, attributes, escaping, nesting, and invalid constraints. |

## WebhookCoverageEntry

| Field | Type | Rules |
|-------|------|-------|
| `key` | Webhook `CoverageKey` | Explicit family and transport; no heuristic inference. |
| `known_fields` | Field specifications | Typed name, cardinality, optionality, sensitivity. |
| `open_values` | Set of fields | Unknown strings must be retained. |
| `unknown_retention` | Form or JSON policy | Form retains original pairs; JSON retains structured properties. |
| `twiml_response` | Required/optional/none | Documents handler response behavior. |
| `evidence` | Fixture references | Valid/invalid signature, typed parse, unknown retention, and applicable negative case. |

## EvidenceReference

| Field | Type | Rules |
|-------|------|-------|
| `dimension` | Stable enum | One auditable contract dimension. |
| `applicability` | `required` or `not_applicable` | N/A requires a non-empty reason. |
| `fixture` | Repository-relative path | Required when applicable; file must exist. |
| `test` | Compiled test identifier | Must be discoverable by the audit. |
| `result` | Pending/pass/fail | Release permits pass only for required entries. |

## EndpointProfile

| Field | Type | Rules |
|-------|------|-------|
| `service` | API 2010 or Messaging v1 | Selects generated companion. |
| `base_url` | URL | HTTPS in production; HTTP only for explicit local test mode. |
| `allowed_origin` | Scheme/host/effective-port tuple | Continuations and authenticated redirects must match unless a documented no-credential redirect path applies. |
| `credentials` | Shared secret-bearing handle | Never serialized or displayed. |
| `timeouts`, `retry_policy`, `tls` | Existing core settings | Same policy surface for both services. |

Existing `ClientBuilder::base_url` configures API 2010 only. A new Messaging override is additive.

## StableResource and OpenValue

Stable resource structs have private fields and accessors. Common fields include typed SID/newtype identifiers, open status values, timestamps, URLs, and related resource identifiers. Generated models never appear in stable signatures.

`OpenValue` stores the exact service string and offers known associated constants plus `as_str()`. Equality and serialization operate on the raw value; unknown values are valid.

## Page and Pager

| Field | Type | Rules |
|-------|------|-------|
| `items` | Ordered list | Preserves Twilio response order. |
| `continuation` | Opaque optional value | Never reconstructed by facade code. |
| `origin` | Endpoint origin | Must remain trusted before credentials attach. |
| `seen` | Internal set | Repeated continuation fails once, preventing cycles. |

Item streaming holds at most one active page plus consumer-held items. A fetch error is yielded once and terminates traversal.

## MediaMetadata and MediaBody

`MediaMetadata` is a normal typed JSON resource. `MediaBody` is a transport-neutral byte stream/sink result with safe metadata such as media type and optional length. It contains no Reqwest type and does not buffer the entire object by contract. Metadata fetch and byte download are distinct methods/types.

State transitions:

```text
metadata requested -> JSON resource
download requested -> headers validated -> bytes streamed -> complete/cancelled/error
```

An authenticated redirect is followed only under the declared safe redirect policy; credentials are never forwarded to an untrusted origin.

## TwimlResponse, TwimlNode, and Attribute

`TwimlResponse` contains an ordered non-empty sequence of voice or messaging nodes under exactly one `Response` root. A node has a case-sensitive name, typed attributes, one declared content form, and children allowed by its parent-specific grammar.

Builders progress as follows:

```text
empty builder -> typed partial builder -> validated response -> escaped deterministic XML
                         \-> validation error (field/rule, never sensitive value)
```

Existing public builders and their XML remain valid. No raw unescaped node exists.

## RawWebhookInput

Variants:

- Form: exact external URL, method, original ordered/repeated name-value pairs, signature header supplied only to validation.
- JSON: exact external URL including `bodySHA256`, method, raw bytes, signature header supplied only to validation.

The input is sensitive and uses redacted `Debug`. It is not stored in a parsed event.

## VerifiedWebhook and WebhookEvent

`VerifiedWebhook<F>` is constructed only after signature/body-digest validation and records the explicitly selected family `F`. Parsing yields shared correlation fields, a family-specific payload, and extras:

- form extras: ordered/repeated pairs including empty values;
- JSON extras: string-keyed structured JSON values.

Status/event fields use `OpenValue`. Indexed MMS fields become an ordered media collection while unmatched original fields remain in extras. Events do not promise deduplication or ordering.

State transitions:

```text
raw input -> valid signature/body -> verified family -> typed event + extras
          -> invalid result
          -> malformed-input error
```

## Error and SafeErrorContext

The existing non-exhaustive `Error` taxonomy remains. `ApiError` retains HTTP status, optional Twilio code, safe message, `more_info`, request ID, attempt count, retry-after, and selected concurrency/timing headers. Generated, Reqwest, raw body, signature, URL/query, phone/SIP/payment, message/TwiML, and unknown-field values never appear in stable errors or debug output.

Malformed body excerpts are bounded/redacted internally and omitted from public `Debug`/`Display` unless proven safe.

## CompatibilityBaseline

| Field | Type | Rules |
|-------|------|-------|
| `public_symbols` | Compile fixtures/rustdoc snapshot | Existing symbols must compile unchanged. |
| `wire_fixtures` | Request/response goldens | Existing serialization/decoding must be equivalent. |
| `twiml_goldens` | XML fixtures | Existing programs remain byte-equivalent. |
| `webhook_vectors` | Validation fixtures | Existing results and error distinctions remain equivalent. |
| `error_fixtures` | Safe formatted output | No unapproved message/variant changes or leakage. |

Baseline changes require an additive compatibility explanation; breaking changes are deferred to a major version.

## ReleaseEvidence

Release evidence records both source pins/hashes, generator artifact/template/config/output hashes, selected totals 139/58/197, TwiML and webhook totals, per-entry pass summaries, compatibility/semver results, documentation examples, security scan, timing, and approved exceptions. Any exception has an owner and expiry; this plan requires none.
