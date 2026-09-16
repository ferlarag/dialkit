# Quickstart Validation: Community Twilio SDK

This guide is the acceptance runbook for the implementation described by [plan.md](plan.md). Commands assume the repository root and a completed implementation. Default validation uses only fake values and local servers; no Twilio account or paid request is required.

## Prerequisites

- Rust 1.85 and the current stable toolchain.
- Bash for the generation check.
- Java runtime capable of running the pinned OpenAPI Generator CLI JAR, unless the repository's verified immutable container path is selected.
- Network access only for initially acquiring checksum-verified build inputs; normal generation may use cached inputs.

Confirm toolchains:

```bash
rustup toolchain install 1.85 stable
rustc +1.85 --version
rustc +stable --version
```

Expected: the first compiler is Rust 1.85.x and both commands succeed.

## 1. Workspace and dependency boundaries

```bash
cargo +1.85 check --workspace --all-features
cargo +stable check --workspace --all-features
cargo tree --workspace
```

Expected:

- `dialkit-core`, `dialkit-api-generated`, and `dialkit` compile on both toolchains.
- The graph has no dependency from core to generated/facade and no dependency from generated to facade.
- Only one Reqwest major/minor line is used by SDK packages.

Run compile-boundary tests:

```bash
cargo test -p dialkit --test public_contract compile_boundary
```

Expected: documented facade examples compile, while compile-fail cases prove generated and Reqwest types do not leak through `dialkit` signatures.

## 2. Reproducible generation

Review [generation-provenance.md](contracts/generation-provenance.md), then run:

```bash
bash codegen/regenerate.sh --check
```

Expected:

- Generator artifact, spec, configuration, and template hashes match the manifest.
- Generation occurs in a clean temporary directory.
- The generated crate compiles and its fixtures pass.
- The full generated tree has zero differences, including deleted paths.
- The command exits nonzero before modifying committed output if any checksum is wrong.

To demonstrate drift detection in an isolated working copy, change one generation input or generated file and rerun check mode. Expected: a nonzero exit and a concise changed-path summary; no automatic publication or unrelated file modification.

## 3. Formatting, lint, tests, and documentation

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo test -p dialkit --doc
cargo test -p dialkit --examples
```

Expected: all commands pass. The example build includes client setup, call, message, pagination, structured error handling, webhook validation, and TwiML construction.

## 4. Authentication and request encoding contracts

```bash
cargo test -p dialkit-core --test transport_contract authentication
cargo test -p dialkit-api-generated --test generated_contract request_encoding
cargo test -p dialkit --test public_contract calls
cargo test -p dialkit --test public_contract messages
```

Expected:

- Both account-token and API-key Basic authentication reach the local server with the correct fake username/password pairing.
- Call/message path and form fields match pinned fixtures; absent options are omitted.
- Common facade builders require the minimum values and reject conflicting instruction/content choices before sending.
- Calls, Messages, Recordings, and Conferences representative generated operations match the pinned wire contract.

## 5. Error, timeout, retry, and redaction contracts

```bash
cargo test -p dialkit-core --test transport_contract error_matrix
cargo test -p dialkit-core --test transport_contract retry_matrix
cargo test -p dialkit-core --test transport_contract redaction_canaries
```

Expected:

- Structured and malformed Twilio errors preserve status, optional code, safe message, and request ID.
- 429 respects bounded retry timing; safe reads retry selected transient failures.
- Mutating calls/messages are not retried after ambiguous timeout/5xx unless explicitly enabled.
- Connect/total timeouts terminate within test tolerance and report attempt count.
- Unique canaries placed in tokens, authorization, phone numbers, query strings, bodies, message content, media URLs, and TwiML appear zero times in traces and formatted errors.

See [protocol-behavior.md](contracts/protocol-behavior.md) for the complete retry matrix and allowlist.

## 6. Pagination contracts

```bash
cargo test -p dialkit-core --test transport_contract pagination
cargo test -p dialkit --test public_contract paginate_calls
```

Expected:

- Empty, single-page, and multi-page fixtures terminate correctly.
- All multi-page items arrive exactly once and in service order.
- Returned `next_page_uri` is followed opaquely and preserves filters/cursors.
- Cross-origin and repeated continuations fail before credentials are attached.
- A mid-stream server failure is yielded once and not confused with completion.

## 7. Unknown service values

```bash
cargo test -p dialkit-api-generated --test generated_contract unknown_enum_values
cargo test -p dialkit --test public_contract unknown_service_values
```

Expected: every fixture containing a service value absent from the pinned schema deserializes successfully, exposes the original string through the facade, and serializes back unchanged where serialization is supported.

## 8. Webhook signature validation

```bash
cargo test -p dialkit --test public_contract webhook
```

Expected:

- Official form and JSON vectors validate.
- Altered URL, body, parameters, signature, or JSON digest returns invalid.
- Repeated form keys/values participate in canonicalization.
- Strict mode uses exactly the supplied external URL; port compatibility runs only when selected.
- Malformed/missing signature or digest is distinguished from a validly formed mismatch.
- Property tests cover ordering, repeated values, Unicode, and raw JSON bytes without panics.

## 9. TwiML construction

```bash
cargo test -p dialkit --test public_contract twiml
cargo run -p dialkit --example twiml_response
```

Expected:

- Golden XML covers the initial Voice and Messaging nodes in [public-api.md](contracts/public-api.md).
- Output has exactly one `<Response>` root, correct case-sensitive names, stable node order, and escaped Unicode/text/attributes.
- Invalid nesting or missing required values is rejected before serialization.
- Example output is valid TwiML and contains no raw unescaped user value.

## 10. Package and compatibility checks

```bash
cargo package -p dialkit-core
cargo package -p dialkit-api-generated
cargo package -p dialkit
cargo semver-checks -p dialkit
```

Expected:

- Each package includes required license, README, provenance, and source inputs without secrets.
- Dependency order and path-plus-version metadata permit publication.
- The stable facade introduces no undeclared breaking change against the latest published release.

For the first unpublished release, record `cargo-semver-checks` as not applicable; it becomes mandatory from the second release onward.

## 11. Optional Twilio test-credential smoke tests

This step is excluded from ordinary pull-request CI. Use Twilio test credentials and documented magic values only; never use production credentials or expect callbacks/real delivery.

```bash
export TWILIO_TEST_ACCOUNT_SID='test account SID'
export TWILIO_TEST_AUTH_TOKEN='test auth token'
cargo test -p dialkit --test public_contract live_test_credentials -- --ignored
```

Expected: only the Calls/Messages scenarios documented as supported by Twilio test credentials run. Logs do not display either environment value. Unset the variables after the run.

## Acceptance summary

The feature is ready for release only when Sections 1–10 pass, generation produces zero unexplained differences, operation coverage is 100% for the pinned specification, all seven examples compile, redaction canaries have zero leaks, and the provenance/compatibility review is attached to the release candidate. Optional live smoke tests provide extra confidence but cannot replace deterministic contracts.
