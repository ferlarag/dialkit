# Quickstart: Validate Complete Voice and Messaging Coverage

This guide describes the commands and observable results that validate the implementation. It uses only synthetic credentials and local fixtures.

## Prerequisites

- Rust 1.85 and current stable
- Java runtime suitable for the pinned OpenAPI Generator JAR
- Bash, Curl, SHA-256 tools, Git, and the repository's normal CI utilities
- No Twilio account, real credentials, or live Twilio traffic

From the repository root:

```bash
rustc --version
cargo --version
git status --short
```

Expected: the requested branch/worktree is active, and pre-existing user changes are understood before generation.

## 1. Verify pinned sources and clean generation

```bash
codegen/regenerate.sh --check
```

Expected:

- API 2010 and Messaging v1 hashes match the generation manifest;
- generator/config/template/output hashes match;
- both generated companion trees reproduce from clean temporary directories;
- generator regressions and unknown-enum round trips pass;
- no generated file is added, changed, or stale.

A hash mismatch or a diff blocks all later validation.

## 2. Audit the coverage inventory

Run the dedicated coverage audit target defined by the implementation, then inspect its machine-readable report.

Expected totals:

```text
API-2010 selected: 139
MSG-V1 selected:    58
REST total:        197
missing:             0
duplicate keys:      0
unresolved:          0
missing evidence:    0
```

The audit must also report complete TwiML node/relationship and webhook family/variant totals. It must reject operation-ID-only collisions and prove each REST row against the correct source.

## 3. Run REST operation contracts

```bash
cargo test -p dialkit-core --test transport_contract
cargo test -p dialkit-api-generated --tests
cargo test -p dialkit-messaging-generated --tests
```

Expected: every selected operation's generated typed function passes local method/path/auth/parameter/success/error checks, and every list operation passes pagination checks. Non-applicable dimensions appear in the report with reasons. No request reaches Twilio.

## 4. Prove pagination and origin safety

Run focused pagination contracts for both page shapes.

Expected:

- zero, one, and multiple pages preserve item order;
- filters appear only on the initial request;
- relative and same-origin absolute continuations succeed;
- reserved cursor characters are not reconstructed;
- absent/null continuation stops;
- repeated continuation fails once;
- cross-origin/downgrade continuation fails before authorization is attached.

## 5. Prove forward-compatible decoding

Run both generated crates' unknown-enum and additive-field fixtures plus stable-facade open-value tests.

Expected: an unknown service value such as `introduced-after-pin` decodes and serializes exactly. An added optional REST response field does not break existing decoding. Webhook unknown parameters/properties remain inspectable rather than merely ignored.

## 6. Prove metadata and binary media separation

Run the recording and MMS media contracts.

Expected:

- metadata methods decode JSON resource models;
- download methods stream exact bytes without JSON decoding;
- content metadata is exposed through transport-neutral accessors;
- empty content and cancellation are handled;
- unsafe redirects do not receive credentials;
- structured and malformed errors normalize without leaking content.

## 7. Validate complete TwiML

```bash
cargo test -p dialkit twiml
```

Expected:

- every node and legal parent-child row has canonical XML;
- all documented attributes/constraints are exercised;
- order, Unicode, repeated nodes, empty optional text, and XML escaping pass;
- prohibited nesting and invalid required/enum/range/mutually-exclusive values produce actionable errors;
- existing Voice and Messaging fixture bytes are unchanged;
- debug/error output contains no message, media, SIP, or payment canaries.

## 8. Validate webhook security and typed parsing

```bash
cargo test -p dialkit webhook
```

Expected:

- form GET/POST and JSON vectors validate with exact public URLs;
- changed URL, pair, raw body, digest, or signature is rejected;
- duplicates and empty values participate correctly in canonicalization;
- every explicit family/variant parses its known fields;
- unknown fields and future status/event values survive;
- payment-sensitive and other redaction canaries never appear in debug/errors/traces;
- fixtures demonstrate replay/out-of-order tolerance without claiming deduplication.

## 9. Freeze and verify compatibility

```bash
cargo test -p dialkit --test public_contract
cargo test --workspace --all-features
cargo semver-checks check-release -p dialkit
```

Also validate default features, each supported TLS backend, `--no-default-features`, Rust 1.85, and stable as configured in CI.

Expected: current Calls, Messages, pagination, TwiML, webhook, error, TLS, and generated-isolation fixtures compile and behave unchanged. Additive APIs do not expose generated/core/Reqwest types.

## 10. Compile the required workflows

Compile and, where fixture-backed, run examples for:

1. outbound call creation;
2. inbound-call TwiML;
3. call-status verification/parsing;
4. conference participation;
5. recording control and byte download;
6. queue/member control;
7. SIP domain and credential setup;
8. SMS send;
9. MMS media use;
10. inbound-message verification/parsing;
11. delivery-status verification/parsing;
12. Messaging Service sender management.

Expected: all examples use the stable facade for the common workflow, compile with synthetic values, and contain neither raw HTTP/auth code nor hand-authored XML.

## 11. Run repository quality and security gates

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo doc --workspace --all-features --no-deps
scripts/audit-sensitive-data.sh
```

Expected: all checks pass without external credentials. The sensitive-data audit finds no real credentials, message/user data, SIP passwords, payment data, or usable signatures.

## 12. Validate phased delivery

At the end of each implementation phase, run the matrix audit filtered to that phase and the full compatibility baseline.

| Phase | Exit evidence |
|-------|---------------|
| Foundation | Both pins verified; two-host routing safe; baseline frozen; manifest reports 139/58 keys. |
| Generated REST | All 197 generated functions compile and all required REST evidence passes. |
| Stable facades | Common FR-004/FR-018 handles compile and delegate without generated-type leakage. |
| TwiML/webhooks | Every manifest entry has positive, negative/forward-compatible, and redaction evidence. |
| Release proof | All 12 examples, docs, compatibility, semver, regeneration, security, and totals pass. |

An entry may not be marked complete with missing evidence, and a later phase may not waive a failed earlier gate.

## 13. Produce release evidence

Run the release-audit command defined by implementation.

Expected: within 10 minutes on the standard CI runner it emits both pins/hashes, generator/template/config/output identities, 139/58/197 REST totals, TwiML/webhook totals, compatibility and semver results, documentation/example results, security result, audit duration, and approved exceptions. This plan expects zero exceptions.

Optional live tests are separate, explicitly operator-triggered, and never part of ordinary completion. They may not create billable calls/messages or provisioning without an additional explicit action.
