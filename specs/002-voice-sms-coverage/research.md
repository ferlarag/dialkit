# Research: Complete Voice and Messaging Coverage

## 1. Pinned scope and identity

**Decision**: Retain `twilio/twilio-oai` commit `5aa7f31977ce5812f7b7bc1f46a38555ebaa2888`. Keep the existing API 2010 file/hash and vendor `spec/json/twilio_messaging_v1.json` with SHA-256 `611c6fde586347615039a7d8c87a4b10d39c6d8566b7d1a691353ef995aaa473`. Record the 2026-09-17 TwiML/webhook snapshot and the feature matrices as normative inputs.

**Rationale**: The Messaging file was independently fetched from the commit-addressed official source; its hash matches the specification and it contains exactly 58 operation IDs. Retaining the adopted commit separates coverage expansion from upstream migration.

**Alternatives considered**: Generating from upstream `main` is not reproducible. Updating the commit during this feature combines compatibility risk with coverage work. Treating current online TwiML/webhook pages as moving truth silently changes scope.

## 2. Source-qualified coverage manifest

**Decision**: Store REST keys as `(source_id, operation_id)`, not operation ID alone. The manifest contains exactly 139 selected API-2010 rows and all 58 Messaging-v1 rows, with domain, method/path, generated symbol, phase, pagination flag, and each evidence dimension marked by fixture or `n/a` plus reason. Maintain analogous TwiML-node/relationship and webhook-family manifests.

**Rationale**: The API-2010 document itself has 197 operations, but only 139 are in this feature. A total-count check can therefore pass for the wrong inventory. Operation/model names also collide between sources (for example Short Code operations), so source identity is essential.

**Alternatives considered**: Parsing Markdown tables is brittle. Count-only auditing hides substitutions. Generator output cannot define support because it also contains explicitly excluded API-2010 operations.

## 3. Generated package boundary

**Decision**: Preserve the current `dialkit-api-generated` crate and all `dialkit_api_generated::{apis,models}` paths. Add `dialkit-messaging-generated` as a sibling generated companion using the same core, generator, templates, lints, and lockstep version. Neither companion is re-exported by `dialkit`.

**Rationale**: Dry-generation research found Messaging v1 model/operation collisions with API 2010, including Short Code list/fetch names. The specifications also have distinct default origins. A sibling crate avoids collision-prone post-processing and is the narrowest way to preserve advanced consumers' current generated paths.

**Alternatives considered**: Flattening both sources collides. Moving API 2010 under a new namespace is breaking. Nesting only Messaging v1 in the existing crate is possible but requires fragile rewriting of generated `crate::models` references. A wholly separate client stack would duplicate transport policy.

## 4. Reproducible two-spec generation

**Decision**: Make `generation-manifest.toml` source-keyed and make `regenerate.sh` table-driven over two `(spec, config, output, inventory)` records. Validate all hashes before work; generate both in clean temporary directories under fixed locale/timezone; normalize and format; run both generated test suites; compare or stage all owned outputs; update atomically or restore both. Keep generator 7.25.0 and template revision unless an evidenced blocker requires a separate reviewed change.

**Rationale**: The present pipeline already provides the correct verified-input and clean-tree foundation. One transaction prevents partial adoption where provenance and one generated tree disagree.

**Alternatives considered**: Independent scripts invite drift and partial updates. Fetching specs during CI prevents offline deterministic regeneration. Manual generated edits violate the constitution.

## 5. Endpoint-aware shared transport

**Decision**: Add internal endpoint profiles so API 2010 uses `https://api.twilio.com` and Messaging v1 uses `https://messaging.twilio.com`. Preserve `ClientBuilder::base_url` exactly for existing API-2010 behavior; add an additive Messaging endpoint override for tests/private deployment. Construct two `HttpClient` instances from the same credentials, retry, timeout, TLS, and redaction configuration. Pagination validates continuation origin against the profile that initiated it.

**Rationale**: One current `HttpClient` owns one base URL and cannot correctly route both official specs. Explicit profiles preserve authentication/origin safety and allow deterministic local contract servers.

**Alternatives considered**: Inferring hosts from generated URLs risks credential forwarding. Changing `base_url` to affect both services changes existing semantics. A separate transport implementation would split retry/error/security policy.

## 6. Generated-layer contract evidence

**Decision**: Generate or mechanically derive typed contract drivers for every selected REST operation. Each entry exercises its generated function against a local server and proves method/path, authentication, parameter placement/encoding, success decode, Twilio error decode, and pagination when applicable. Every missing dimension must be an explicit `n/a` with a reason. Schema-valid synthetic fixtures and a small reviewed overrides file supply values when upstream examples are absent.

**Rationale**: Current source-text auditing plus representative runtime tests do not meet the per-operation requirement. Generated drivers exercise compiled typed functions without hand-maintaining 197 tests.

**Alternatives considered**: Raw HTTP fixtures bypass generated APIs. One representative operation per domain cannot prove full coverage. Live Twilio tests are incomplete, credential-dependent, and potentially billable.

## 7. Compatibility-preserving stable facades

**Decision**: Preserve `Client::calls()`, `Client::messages()`, their request/resource types, `Pager`, `Error`, TwiML entry points, webhook validator methods, feature flags, and outputs. Add private-field/accessor-based handles only for common workflows: conferences/participants, recordings, queues/members, SIP, applications/phone configuration, message media, Messaging Services, and senders. Internally delegate to generated functions and map at the facade boundary.

**Rationale**: The public compile fixtures already freeze generated-type isolation. Selective facades make FR-004/FR-018 discoverable without duplicating all 197 schemas or making generator names stable API.

**Alternatives considered**: Wrapping every operation creates a second exhaustive SDK. Exposing generated types breaks the stable boundary. Replacing existing Calls/Messages designs violates minor-version compatibility.

## 8. Pagination

**Decision**: Retain `Pager<T>`, `Page<T>`, item order, cancellation, and first-error termination. Add shared adapters for both generated page shapes. Apply filters only to the initial request, follow returned continuation strings opaquely, stop on missing/null, reject loops, and require the same scheme/host/effective port before attaching credentials. Add one-page access only where useful, with an opaque continuation value.

**Rationale**: The core already implements bounded-memory traversal, same-origin validation, and loop detection. Messaging v1 uses `meta.next_page_url` while API 2010 commonly uses `next_page_uri`; an adapter can normalize both without changing the facade.

**Alternatives considered**: Reconstructing `Page`/`PageToken` loses reserved characters and future cursor formats. Eager collection is unbounded. Exposing raw authenticated URLs shifts a security concern to users.

## 9. Forward-compatible models

**Decision**: Continue the generated `Unknown(String)` enum template and exact round-trip tests in both companions. Existing generated structs continue ignoring additive JSON fields to avoid breaking public struct construction. Stable service-controlled values remain string-backed types with known constants and `as_str()`. Webhook parsers uniquely retain unconsumed form pairs or JSON properties because FR-010 requires inspection.

**Rationale**: Closed enums make a post-pin value a decode failure; `serde(other)` loses the value. Adding flattened maps to every existing generated struct changes public layout and struct literals. Webhook envelopes can guarantee retention from inception.

**Alternatives considered**: Closed public enums and exhaustive matching are not forward-compatible. Broad generated `additional_properties` retrofits are not compatibility-safe. Untyped maps everywhere discard discoverability.

## 10. Binary recording and MMS media

**Decision**: Keep metadata operations as typed JSON. Add a separate core byte-stream execution path and facade download APIs returning transport-neutral metadata plus a bounded stream or caller-provided sink. Never expose `reqwest::Response`. Validate redirects before forwarding credentials and contract-test bytes, empty bodies, redirects, and non-JSON Twilio errors.

**Rationale**: Metadata and media have different response semantics; buffering large recordings into a mandatory `Vec<u8>` is unsafe. The stable boundary must remain transport-independent.

**Alternatives considered**: Content-type overloading of `fetch` is ambiguous. Exposing Reqwest couples the public API. Automatic download/transcoding/storage exceeds scope.

## 11. Secure error and debug behavior

**Decision**: Preserve the existing non-exhaustive facade error variants and accessor-based `ApiError`. Normalize both generated companions through core, retaining status, Twilio code/message/more-info, request ID, retry metadata, and safe headers while bounding/redacting malformed bodies. Replace content-bearing derived `Debug` implementations for generated errors, TwiML, and webhook models with redacted output. Add unique canaries in every sensitive location.

**Rationale**: Existing transport/facade redaction is a strong base, but generated response content and current TwiML derived `Debug` can expose payloads. Structural redaction is safer than assuming those paths are unreachable.

**Alternatives considered**: Returning generated/Reqwest errors leaks unstable and sensitive detail. A single string loses actionable service metadata. Denylist-only scrubbing misses new fields.

## 12. Complete typed TwiML

**Decision**: Extend the handwritten ordered AST and `quick_xml::Writer`; OpenAPI does not define TwiML. Add typed public builders per matrix node, parent-specific child types, typed attributes, and build-time validation. Preserve existing builder names/methods and byte-equivalent XML. Make invalid nesting unrepresentable where practical and use actionable validation for cross-field, range, and control-transfer rules. Do not add raw XML insertion.

**Rationale**: The current private AST already provides one root, order, and escaping. Typed parent relationships scale safely across the complete matrix.

**Alternatives considered**: Raw XML weakens the safety claim. Serde XML is awkward for ordered mixed content. Scraping live docs creates an unpinned generator.

## 13. Verified typed webhooks

**Decision**: Keep `WebhookValidator::validate_form` and `validate_json` unchanged. Add explicit family parsers and verify-then-parse convenience methods; do not infer a family from overlapping fields. Accept the exact public URL and original pairs/raw bytes. Open status/event values preserve unknown strings; form extras preserve duplicates/empties/order, and JSON extras preserve structured values. Parsed events never store auth tokens or signatures.

**Rationale**: Existing validation already implements exact-URL signing, duplicate-pair canonicalization, raw JSON hashing, constant-time verification, and opt-in port compatibility. Family-specific parsing avoids ambiguous universal-envelope heuristics.

**Alternatives considered**: Framework middleware reconstructs URLs unreliably behind proxies. Parse-before-verify encourages accidental use of spoofed data. A single inferred event enum is ambiguous.

## 14. Matrix-driven delivery

**Decision**: Deliver foundation/baseline, exhaustive generated REST, common facade workflows, TwiML, webhooks, then documentation/release proof. An entry becomes complete only when every applicable evidence link is green. Later phases cannot waive earlier gaps, and no “complete” claim is made before the union audit reports 139 + 58 = 197 plus full TwiML/webhook totals.

**Rationale**: This makes partial delivery useful and auditable while keeping cross-cutting transport and generation fixes ahead of facade expansion.

**Alternatives considered**: Large domain-by-domain rewrites can duplicate infrastructure and obscure total gaps. Documentation-first claims without machine evidence are not sustainable.
