# Initial Release Evidence

## Generation provenance

- Twilio specification commit: `5aa7f31977ce5812f7b7bc1f46a38555ebaa2888`
- Specification SHA-256: `170b3ccd0f891416840083d72f1795b1499b14a18d4873fd2b39f47ef84642d6`
- OpenAPI Generator: `7.25.0`
- Generator JAR SHA-256: `41ce4f6b07f196676439d710759fa1ced7a08066d06ff1bf314681470289efae`
- Template revision: `dialkit-rust-v1`

## Verified evidence

- Generated coverage audit maps every pinned `operationId` to its compiled function, request input,
  response signature, and operation documentation page.
- Every generated string-enum shape, including enums nested in generated models, preserves and
  round-trips unknown raw values.
- Clean regeneration validates all input hashes and compares added, changed, and deleted paths.
- Deterministic Calls, Messages, Recordings, Conferences, pagination, webhook, TwiML, resilience,
  authentication, encoding, error, and redaction contracts require no Twilio credentials.
- `codegen/regenerate.sh --check` completed with zero differences on 2026-09-16.
- All four regeneration integration cases passed: clean equality, checksum failure/no-write,
  changed-added-deleted detection, and update isolation.
- Rust 1.85.0 and stable Rust 1.98.1 both completed `cargo check --workspace --all-features`.
- Formatting, warnings-denied Clippy, workspace tests, facade doctests, and all seven examples pass.
- Rustls-only, native-TLS-only, and all-feature configurations pass; feature forwarding prevents
  the generated companion from silently enabling the other backend.
- The sensitive-data audit completed with zero Twilio credential or production-identifier canaries.
- Generated and facade operations share the protected core client, repeated form keys are retained,
  and real error/trace contracts prove credential and request-value redaction.

## Performance evidence

The deterministic local release benchmark completed on 2026-09-16 with these p95 observations:

- request construction: 271 ns (target: under 5 ms)
- 100,000-item page-at-a-time traversal: 15.259 µs, with 1,000 peak active items
- 64 KiB webhook canonicalization and valid signature verification: 52.308 µs (target: under 10 ms)
- 64 KiB escaped TwiML rendering: 477.979 µs (target: under 10 ms)

These are local regression thresholds, not cross-machine throughput claims.

## Package and dependency review

- Dependency direction is `dialkit` -> `dialkit-api-generated` -> `dialkit-core`, with a direct
  facade-to-core edge; core does not depend on either higher layer and generated does not depend on
  the facade.
- The resolved workspace contains one Reqwest line: 0.12.28.
- `dialkit-core` produced and verified its crate archive. Package manifests for all three crates
  include the README, package license, and exact generation provenance; the generated archive also
  includes Twilio's upstream MIT notice.
- The generated and facade package file lists were audited successfully. Their archive dry runs
  correctly stop until their unpublished path-and-version dependencies exist in crates.io. Initial
  publication order is therefore `dialkit-core`, `dialkit-api-generated`, then `dialkit`; each next
  archive must be rerun and verified after the preceding crate is published.

## Compatibility

This is the first unpublished release, so `cargo-semver-checks` has no published baseline and is
recorded as not applicable. The `dialkit` facade is the stable Semantic Versioning boundary;
`dialkit-api-generated` remains explicitly generator-driven and is not re-exported.

## Constitution compliance

- **Specification source of truth:** the adopted upstream commit and digest are pinned and the
  generated coverage audit accounts for every operation.
- **Reproducible generation:** tool, input, configuration, and template hashes are enforced; clean
  regeneration and drift integration tests pass.
- **Rust-native developer experience:** generated internals stay behind the facade, examples and
  rustdoc compile, credentials are redacted, and common builders validate before I/O.
- **Contract confidence:** deterministic local contracts cover authentication, encoding, endpoint
  models, errors, retries, pagination, webhook validation, and TwiML.
- **Compatibility and maintenance:** the facade is the declared SemVer boundary, first-release
  semver comparison is recorded as not applicable, upstream changes require review, and no workflow
  automatically publishes generated changes.

No constitution exception is required for this release candidate. The optional live Twilio test-
credential smoke test was not run because credentials were not supplied; it is explicitly outside
the mandatory deterministic acceptance gates.
