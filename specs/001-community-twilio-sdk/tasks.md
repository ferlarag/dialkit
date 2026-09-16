---

description: "Dependency-ordered implementation tasks for the Community Twilio SDK"
---

# Tasks: Community Twilio SDK

**Input**: Design documents from `/specs/001-community-twilio-sdk/`

**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/`, `quickstart.md`

**Tests**: Required by the feature specification and constitution. Story phases put focused tests before their implementation tasks.

**Organization**: Tasks are grouped by user story so each story can be implemented and validated as an independent increment after the shared foundation is complete.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel because it targets different files and has no dependency on an incomplete task in the same phase
- **[Story]**: Maps the task to a user story from `spec.md`
- Every task names the exact file or directory it changes or validates

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Convert the starter binary into the planned publishable Rust workspace and establish versioned generation inputs.

- [X] T001 Replace the starter package with a resolver-3 virtual workspace and shared Rust 1.85 package metadata in Cargo.toml
- [X] T002 [P] Create the dialkit-core package manifest with shared runtime and test dependencies in crates/client-core/Cargo.toml
- [X] T003 [P] Create the dialkit-api-generated package manifest with path-plus-version dependency metadata in crates/api-generated/Cargo.toml
- [X] T004 [P] Create the dialkit facade package manifest and rustls-tls, native-tls, webhooks, and twiml feature definitions in crates/client/Cargo.toml
- [X] T005 [P] Add package entry points and module declarations in crates/client-core/src/lib.rs and crates/client/src/lib.rs
- [X] T006 [P] Add workspace formatting, lint, generated-file, and secret exclusions in .gitignore and Cargo.toml
- [X] T007 Commit the pinned Twilio api_v2010 input and its MIT attribution in codegen/spec/twilio_api_v2010.json and codegen/spec/LICENSE.twilio-oai

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Establish the reproducible generated layer and shared transport primitives required by every user story.

**Critical**: No user story work begins until this phase is complete.

- [X] T008 Record the generator, artifact, specification, configuration, and template pins and checksums in codegen/generation-manifest.toml
- [X] T009 [P] Configure OpenAPI Generator 7.25.0 for async Reqwest, single request structs, and hidden timestamps in codegen/generator-config.yaml
- [X] T010 [P] Add minimal versioned Rust generator overrides with do-not-edit headers and raw-preserving unknown string enums in codegen/templates/rust/model.mustache
- [X] T011 Implement checksum validation, clean temporary generation, formatting, generated tests, full-tree comparison, check mode, and guarded update mode in codegen/regenerate.sh
- [X] T012 [P] Define redacted credentials, AccountSid, ApiKeySid, and Basic-auth construction in crates/client-core/src/auth.rs
- [X] T013 [P] Define the non-exhaustive stable error taxonomy, response metadata, bounded excerpts, and safe formatting in crates/client-core/src/error.rs
- [X] T014 [P] Define request descriptors, operation safety metadata, URL/path/query/form encoding boundaries, and response decoding interfaces in crates/client-core/src/request.rs
- [X] T015 [P] Implement bounded retry policy, Retry-After handling, full-jitter backoff, and mutation safety classification in crates/client-core/src/retry.rs
- [X] T016 [P] Implement allowlisted operation tracing and sensitive-value redaction helpers in crates/client-core/src/trace.rs
- [X] T017 Implement the pooled Reqwest executor with HTTPS enforcement, explicit loopback-test HTTP support, timeouts, authentication, retries, cancellation, and error normalization in crates/client-core/src/lib.rs
- [X] T018 Run the pinned generator through codegen/regenerate.sh and commit the complete generated companion tree under crates/api-generated/src/

**Checkpoint**: The workspace builds, all immutable generation inputs are recorded, and generated operations use one secure shared transport core.

---

## Phase 3: User Story 1 - Complete a Core Communication Task (Priority: P1) MVP

**Goal**: Let an application configure one client, create a call, send a message, and receive stable typed results and actionable errors without touching generated or transport types.

**Independent Test**: Against a loopback Twilio-compatible server, the documented minimum call and message requests authenticate and serialize correctly, return typed facade values, and turn a rejected request into a safe error containing the Twilio code and request ID.

### Tests for User Story 1

- [X] T019 [P] [US1] Add failing call-builder, call wire-contract, typed-result, and structured-error tests in crates/client/tests/public_contract/calls.rs
- [X] T020 [P] [US1] Add failing message-builder, message wire-contract, typed-result, and structured-error tests in crates/client/tests/public_contract/messages.rs
- [X] T021 [P] [US1] Add fake Calls and Messages request/response/error payloads with non-sensitive values in crates/client/tests/fixtures/calls.json and crates/client/tests/fixtures/messages.json
- [X] T022 [US1] Add compile-pass and compile-fail coverage proving stable facade signatures do not expose Reqwest or generated types in crates/client/tests/public_contract/compile_boundary.rs

### Implementation for User Story 1

- [X] T023 [P] [US1] Implement validated identifiers, PhoneEndpoint, CallInstructions, CreateCall, Call, CallStatus, and generated conversions in crates/client/src/calls.rs
- [X] T024 [P] [US1] Implement MessageSender, CreateMessage content invariants, Message, MessageStatus, and generated conversions in crates/client/src/messages.rs
- [X] T025 [US1] Implement Credentials, ClientBuilder, immutable shared Client, and Calls and Messages service accessors in crates/client/src/client.rs
- [X] T026 [US1] Map core and generated failures into accessor-based public Error and ApiError values in crates/client/src/error.rs
- [X] T027 [P] [US1] Add compiling client setup, call creation, and structured error examples in crates/client/examples/setup_client.rs, crates/client/examples/make_call.rs, and crates/client/examples/handle_error.rs
- [X] T028 [P] [US1] Add the compiling outbound message example and facade-first getting-started instructions in crates/client/examples/send_message.rs and crates/client/README.md

**Checkpoint**: User Story 1 passes independently and is the suggested MVP release slice.

---

## Phase 4: User Story 2 - Work Across the v2010 Resource Surface (Priority: P2)

**Goal**: Expose every pinned api_v2010 operation through the generated companion and provide safe incremental pagination with convenient facade listing for calls and messages.

**Independent Test**: A machine-readable audit maps every pinned operation to generated documentation, representative Calls, Messages, Recordings, and Conferences requests match fixtures, and multi-page iteration returns every item exactly once in order while reporting failures once.

### Tests for User Story 2

- [X] T029 [P] [US2] Add a failing 100-percent operation/model coverage and documentation audit against the pinned OpenAPI document in crates/api-generated/tests/coverage_audit.rs
- [X] T030 [P] [US2] Add failing Calls, Messages, Recordings, and Conferences request/response encoding contracts in crates/api-generated/tests/generated_contract.rs
- [X] T031 [P] [US2] Add unknown-enum round-trip fixtures covering every generated string-enum shape in crates/api-generated/tests/fixtures/unknown_enum_values.json
- [X] T032 [P] [US2] Add failing empty, single-page, multi-page, opaque-continuation, repeated-continuation, cross-origin, and mid-stream-failure tests in crates/client-core/tests/transport_contract/pagination.rs

### Implementation for User Story 2

- [X] T033 [US2] Extend generator templates and configuration until every pinned operation, model, and rustdoc page passes crates/api-generated/tests/coverage_audit.rs via codegen/templates/rust/ and codegen/generator-config.yaml
- [X] T034 [US2] Implement ordered Page and cancellable Pager state transitions with same-origin continuation validation and one-error termination in crates/client-core/src/pagination.rs
- [X] T035 [P] [US2] Implement Calls get/list filters, facade Page conversion, and Pager integration in crates/client/src/calls.rs
- [X] T036 [P] [US2] Implement Messages get/list filters, facade Page conversion, and Pager integration in crates/client/src/messages.rs
- [X] T037 [US2] Document same-version generated-companion construction and its stability boundary in crates/api-generated/README.md
- [X] T038 [P] [US2] Add the compiling call pagination example in crates/client/examples/paginate_calls.rs

**Checkpoint**: User Story 2 passes independently after the foundation, with complete pinned coverage and safe pagination.

---

## Phase 5: User Story 3 - Build Secure Twilio Workflows (Priority: P3)

**Goal**: Validate inbound webhooks, build safe TwiML, configure timeout/retry behavior, and observe operations without leaking secrets or request payloads.

**Independent Test**: Published examples and deterministic contracts distinguish valid, invalid, and malformed webhooks; render escaped valid voice and messaging TwiML; obey the retry matrix and timeouts; and find zero seeded canaries in traces and formatted errors.

### Tests for User Story 3

- [X] T039 [P] [US3] Add failing authentication, error-matrix, timeout, Retry-After, mutation-retry, and cancellation contracts in crates/client-core/tests/transport_contract/resilience.rs
- [X] T040 [P] [US3] Add failing trace and formatted-error canary tests for credentials, SIDs, phone numbers, URLs, queries, bodies, media, messages, and TwiML in crates/client-core/tests/transport_contract/redaction_canaries.rs
- [X] T041 [P] [US3] Add official-style form and JSON webhook vectors plus altered-input, repeated-key, Unicode, port-mode, and malformed-input tests in crates/client/tests/public_contract/webhook.rs and crates/client/tests/webhook_vectors/
- [X] T042 [P] [US3] Add voice and messaging golden fixtures plus escaping, Unicode, ordering, root-count, missing-value, and invalid-nesting property tests in crates/client/tests/public_contract/twiml.rs and crates/client/tests/fixtures/twiml/

### Implementation for User Story 3

- [X] T043 [US3] Complete executor integration for connect/total timeouts, retry attempt metadata, Retry-After, mutation opt-in, and cancellation in crates/client-core/src/request.rs
- [X] T044 [US3] Apply the trace allowlist at every request attempt and response outcome and ensure all error paths use bounded redaction in crates/client-core/src/trace.rs
- [X] T045 [US3] Implement strict form and raw-JSON HMAC validation, SHA-256 body verification, constant-time comparison, and opt-in port compatibility in crates/client/src/webhook.rs
- [X] T046 [P] [US3] Implement typed Say, Play, Gather, Dial, Record, Hangup, Pause, and voice Redirect builders in crates/client/src/twiml/voice.rs
- [X] T047 [P] [US3] Implement typed Message, Body, Media, and messaging Redirect builders in crates/client/src/twiml/messaging.rs
- [X] T048 [US3] Implement validated ordered TwimlResponse rendering with one Response root and escaped UTF-8 XML in crates/client/src/twiml/mod.rs
- [X] T049 [US3] Expose timeout and RetryPolicy configuration through stable facade types without leaking core or Reqwest types in crates/client/src/client.rs
- [X] T050 [US3] Gate webhook, TwiML, and TLS dependencies correctly and reject mutually exclusive TLS selections in crates/client/src/lib.rs
- [X] T051 [P] [US3] Add compiling webhook validation and TwiML construction examples in crates/client/examples/validate_webhook.rs and crates/client/examples/twiml_response.rs

**Checkpoint**: User Story 3 passes independently after the foundation, including zero-leak diagnostics.

---

## Phase 6: User Story 4 - Adopt Upstream Changes Safely (Priority: P4)

**Goal**: Give maintainers a reproducible, reviewable workflow for adopting pinned upstream changes and releasing compatible artifacts with provenance.

**Independent Test**: From a clean checkout, unchanged pins regenerate with zero differences; a changed input or generated path fails check mode without altering committed output; compatibility, packaging, and provenance evidence are produced before release.

### Tests for User Story 4

- [X] T052 [P] [US4] Add failing shell integration tests for checksum mismatch, clean-tree equality, changed/added/deleted path detection, and no-write check mode in codegen/tests/regenerate.bats
- [X] T053 [P] [US4] Add a fixture proving a generator correction is expressed in a template and survives clean regeneration in crates/api-generated/tests/generation_regression.rs

### Implementation for User Story 4

- [X] T054 [US4] Harden codegen/regenerate.sh until all codegen/tests/regenerate.bats cases pass and update mode replaces only crates/api-generated after validation succeeds
- [X] T055 [P] [US4] Add Rust 1.85/stable build, format, deny-warnings lint, workspace/all-feature test, doctest, example, TLS-matrix, and clean-regeneration gates in .github/workflows/ci.yml
- [X] T056 [P] [US4] Add a report-only scheduled workflow that fetches an immutable candidate revision and publishes an upstream diff artifact without committing or publishing in .github/workflows/upstream-spec-check.yml
- [X] T057 [US4] Document the upstream diff classification, generated-diff review, compatibility assessment, migration guidance, and rollback checklist in docs/maintainer/upstream-adoption.md
- [X] T058 [P] [US4] Add package order, cargo-semver-checks, provenance, attribution, and release-note evidence requirements in docs/maintainer/releasing.md
- [X] T059 [P] [US4] Add package metadata, generated stability warnings, license links, and provenance links in crates/client-core/README.md, crates/api-generated/README.md, and crates/client/README.md
- [X] T060 [US4] Run clean regeneration and record the initial complete operation coverage and generation-pin evidence in specs/001-community-twilio-sdk/release-evidence.md

**Checkpoint**: User Story 4 passes independently and an unexplained generated diff blocks the release workflow.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Close workspace-wide quality, performance, documentation, and release-readiness gaps after the desired stories are complete.

- [X] T061 [P] Add deterministic local benchmarks for request overhead, streaming memory, 64 KiB webhook validation, and 64 KiB TwiML rendering in crates/client/benches/performance.rs
- [X] T062 [P] Add workspace license, contribution, security-reporting, MSRV, feature, privacy, and community-status documentation in README.md, LICENSE, CONTRIBUTING.md, and SECURITY.md
- [X] T063 Audit all committed fixtures, examples, docs, snapshots, and generated output for secret or sensitive Twilio canaries and record the scan configuration in scripts/audit-sensitive-data.sh
- [X] T064 Run and reconcile every command and expected result in specs/001-community-twilio-sdk/quickstart.md
- [X] T065 Verify dependency direction, a single Reqwest 0.12 line, package contents, and dependency-order dry runs and record results in specs/001-community-twilio-sdk/release-evidence.md
- [X] T066 Run cargo-semver-checks for the facade or record first-release non-applicability, then complete the constitution compliance review in specs/001-community-twilio-sdk/release-evidence.md

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Starts immediately.
- **Foundational (Phase 2)**: Depends on Setup and blocks every user story.
- **User Story 1 (Phase 3)**: Starts after Foundational; this is the MVP.
- **User Story 2 (Phase 4)**: Starts after Foundational. It can proceed in parallel with US1, although T035 and T036 should be rebased after any concurrent US1 edits to the same facade files.
- **User Story 3 (Phase 5)**: Starts after Foundational. It can proceed in parallel with US1 and US2, although T049 follows the client shape established by T025 when both stories are selected.
- **User Story 4 (Phase 6)**: Starts after Foundational generation is operational. T053 and T060 consume the generated behavior completed for selected earlier stories, so run them after those selected story phases.
- **Polish (Phase 7)**: Runs after all stories selected for the release are complete.

### User Story Completion Order

```text
Setup -> Foundation -> US1 (MVP)
                    -> US2
                    -> US3
                    -> US4 release proof (after all stories selected for release)
Selected stories -> Polish
```

- **US1 (P1)**: No functional dependency on another story after Foundation.
- **US2 (P2)**: No functional dependency on US1; its call/message list conveniences share facade files with US1.
- **US3 (P3)**: No functional dependency on US1 or US2; its client configuration extension builds on the foundational client and should be reconciled with US1 when both are implemented.
- **US4 (P4)**: The workflow itself is independently testable after Foundation; final release evidence depends on whichever earlier stories are included in the release.

### Within Each User Story

- Write the story's tests and confirm they fail for the intended missing behavior.
- Implement domain types before service or executor integration.
- Implement services/execution before examples and documentation.
- Run the independent test and stop at the checkpoint before expanding scope.

## Parallel Execution Examples

### User Story 1

```text
Parallel: T019 call contracts, T020 message contracts, T021 fixtures
Then parallel: T023 call facade, T024 message facade
Then: T025 client integration -> T026 error mapping
Parallel after implementation: T027 call/setup/error examples, T028 message example/docs
```

### User Story 2

```text
Parallel: T029 coverage audit, T030 wire contracts, T031 enum fixtures, T032 pagination contracts
Then: T033 generated coverage and T034 pagination core
Then parallel: T035 calls listing, T036 messages listing, T037 companion docs
Then: T038 pagination example
```

### User Story 3

```text
Parallel: T039 resilience contracts, T040 redaction contracts, T041 webhook contracts, T042 TwiML contracts
Then parallel: T043-T044 transport hardening, T045 webhook validation, T046-T047 TwiML builders
Then: T048 TwiML renderer, T049 client policy facade, T050 feature gates
Then: T051 examples
```

### User Story 4

```text
Parallel: T052 regeneration integration tests, T053 persistent regression fixture
Then: T054 regeneration hardening
Parallel: T055 CI gates, T056 drift reporting, T057 adoption guide, T058 release guide, T059 package docs
Then: T060 release evidence
```

## Implementation Strategy

### MVP First: User Story 1

1. Complete Setup.
2. Complete Foundational generation and transport work.
3. Complete User Story 1 tests and implementation.
4. Stop and run the US1 independent test against the loopback server.
5. Demo the call and message examples before selecting additional stories.

### Incremental Delivery

1. Deliver Setup + Foundation as an internal generation/transport milestone.
2. Deliver US1 as the usable Calls/Messages MVP.
3. Add US2 for complete api_v2010 reach and pagination.
4. Add US3 for secure inbound and operational workflows.
5. Add US4 and Polish to make the selected feature set release-ready.

### Parallel Team Strategy

After Foundation is complete, separate owners can take US1, US2, and US3. Assign one owner to reconcile the shared `calls.rs`, `messages.rs`, and `client.rs` edits. US4 generation tests and maintainer documentation can start in parallel, while its final evidence task waits for the selected release scope.

## Notes

- Generated source under `crates/api-generated/src/` is changed only by `codegen/regenerate.sh`; fixes belong in the specification input, generator configuration, templates, or handwritten conversion layers.
- All ordinary tests use fake values and loopback servers; the optional credential smoke test remains outside pull-request CI.
- `[P]` marks file-independent work, not permission to ignore the phase or task dependencies above.
- Commit after each task or coherent task group, and preserve focused regression coverage for every corrected defect.

## Phase 8: Convergence

- [X] T067 CRITICAL replace plaintext generated credential fields and derived secret-bearing Debug output with protected credential configuration, redacted formatting, and leak regression tests in codegen/templates/rust/, crates/api-generated/src/apis/configuration.rs, and crates/api-generated/tests/ per Constitution Technical and Release Constraints (contradicts)
- [X] T068 CRITICAL redact configured secrets and sensitive request values from structured and malformed service error messages, details, and stored excerpts before formatting, then exercise real request/error/trace paths and run the audit in CI in crates/client-core/src/error.rs, crates/client-core/src/request.rs, crates/client-core/tests/transport_contract/redaction_canaries.rs, scripts/audit-sensitive-data.sh, and .github/workflows/ci.yml per FR-007 (contradicts)
- [X] T069 CRITICAL add focused generation-regression fixtures for the duplicated model namespace, misqualified serde_json value, and required account_sid corrections performed by codegen/normalize-generated.sh in crates/api-generated/tests/generation_regression.rs per Constitution IV (missing)
- [X] T070 route generated operation authentication, HTTPS enforcement, pooled execution, timeout, retry, response normalization, and tracing through dialkit-core instead of an independent reqwest client by adding reproducible generator overrides and companion construction documentation in codegen/templates/rust/, crates/api-generated/src/, and crates/api-generated/README.md per plan: shared transport decision (contradicts)
- [X] T071 record outcome category, Twilio request ID, attempt, status, and elapsed duration on success, retry, rate-limit, API, timeout, decode, and transport paths, with capture-layer assertions in crates/client-core/src/request.rs, crates/client-core/src/trace.rs, and crates/client-core/tests/transport_contract/redaction_canaries.rs per FR-019 (partial)
- [X] T072 preserve safe response metadata, Retry-After, request ID, and final attempt count through core and facade rate-limit, API, decode, timeout, and transport errors with accessor and formatting contracts in crates/client-core/src/error.rs, crates/client/src/error.rs, and their contract tests per FR-008 (partial)
- [X] T073 implement facade Page<T>, PageStream<T>, and Pager::pages() while preserving order, active-page memory bounds, cancellation, opaque continuations, and one-error termination in crates/client-core/src/pagination.rs, crates/client/src/pagination.rs, and pagination contracts per plan: pagination decision (missing)
- [X] T074 validate documented SID prefixes/shapes and percent-encode every facade path segment before URL resolution, with traversal, slash, whitespace, and Unicode regression cases in crates/client-core/src/auth.rs, crates/client/src/calls.rs, crates/client/src/messages.rs, and facade wire contracts per plan: protocol encoding (partial)
- [X] T075 validate required text, destinations, URLs, pause ranges, and structural constraints for every initial voice and messaging TwiML node and add missing-value/invalid-nesting coverage in crates/client/src/twiml/ and crates/client/tests/public_contract/twiml.rs per FR-018 (partial)
- [X] T076 replace count-based documentation coverage with an exact audit mapping every pinned operationId to its compiled generated function, request input, response type, and operation documentation page in crates/api-generated/tests/coverage_audit.rs per SC-002 (partial)
- [X] T077 add authenticated request-serialization and typed-response contracts for representative create/update Calls, Messages, Recordings, and Conferences operations, including repeated/optional form values and headers, in crates/api-generated/tests/generated_contract.rs per FR-024 (partial)
- [X] T078 complete deterministic authentication, structured/malformed error, rate-limit exhaustion, connect/total timeout, retry-attempt, mutation opt-in, and cancellation contracts in crates/client-core/tests/transport_contract/resilience.rs per FR-024 (partial)
- [X] T079 declare the complete generator-owned artifact inventory and make check/update modes plus changed/added/deleted integration tests compare and replace that entire inventory without touching handwritten package files in codegen/regenerate.sh and codegen/tests/regenerate.bats per FR-022 (partial)
- [X] T080 generate or maintain value fixtures that deserialize, expose, and serialize an unknown raw value through every generated string-enum shape rather than only listing type names in crates/api-generated/tests/fixtures/unknown_enum_values.json and crates/api-generated/tests/generated_contract.rs per FR-015 (partial)
- [X] T081 enforce and test unambiguous mutually exclusive rustls-tls/native-tls selection semantics while retaining separate backend CI coverage and a workable all-feature quality gate in crates/client/Cargo.toml, crates/client-core/Cargo.toml, crates/client/src/lib.rs, and .github/workflows/ci.yml per T050 (contradicts)
