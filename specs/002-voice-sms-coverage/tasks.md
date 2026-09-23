---

description: "Execution tasks for complete pinned Twilio Voice and Messaging coverage"
---

# Tasks: Complete Voice and Messaging Coverage

**Input**: Design documents from `/specs/002-voice-sms-coverage/`

**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/`, `quickstart.md`

**Tests**: Required. FR-014 and FR-015 require deterministic per-operation, TwiML, webhook, compatibility, and regression evidence. Test tasks precede the implementation they prove.

**Organization**: Tasks are grouped by user story so each story can be implemented and validated as an independent increment after the shared foundation.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel because it uses different files and has no dependency on another incomplete task in the same group
- **[Story]**: Maps the task to a user story from `spec.md`
- Every task names the exact repository path it changes

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Add the immutable inputs and workspace skeleton required by the design without changing runtime behavior.

- [X] T001 Vendor the pinned `twilio_messaging_v1.json` bytes and verify SHA-256 `611c6fde586347615039a7d8c87a4b10d39c6d8566b7d1a691353ef995aaa473` in `codegen/spec/twilio_messaging_v1.json`
- [X] T002 [P] Preserve Twilio attribution for the second specification in `codegen/spec/LICENSE.twilio-oai` and `crates/messaging-generated/LICENSE.twilio-oai`
- [X] T003 Add the `dialkit-messaging-generated` workspace member and lockstep package metadata in `Cargo.toml` and `crates/messaging-generated/Cargo.toml`
- [X] T004 [P] Add Messaging generated-crate package documentation and provenance placeholders in `crates/messaging-generated/README.md` and `crates/messaging-generated/PROVENANCE.md`
- [X] T005 Split source-specific generator settings without changing API-2010 output in `codegen/config/api-2010.yaml` and `codegen/config/messaging-v1.yaml`
- [X] T006 Convert generation provenance to source-keyed pins and output inventories in `codegen/generation-manifest.toml`, `codegen/generated-api-2010-files.txt`, and `codegen/generated-messaging-v1-files.txt`
- [X] T007 [P] Define versioned schemas and contributor guidance for REST, TwiML, and webhook coverage manifests in `codegen/coverage/README.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Freeze compatibility and establish the shared routing, generation, pagination, byte-response, and redaction mechanisms required by every story.

**⚠️ CRITICAL**: No user story implementation begins until this phase passes its checkpoint.

- [X] T008 [P] Freeze the existing public symbol and generated-isolation baseline in `crates/client/tests/compatibility/public_api.rs` and `crates/client/tests/compile/generated_leak.rs`
- [X] T009 [P] Freeze existing Calls, Messages, pagination, error, TwiML, and webhook outputs in `crates/client/tests/compatibility/behavior.rs` and `crates/client/tests/fixtures/compatibility/`
- [X] T010 [P] Add failing endpoint-profile and credential-origin contracts for API 2010 and Messaging v1 in `crates/client-core/tests/transport_contract/endpoints.rs`
- [X] T011 Implement named endpoint profiles with trusted-origin validation in `crates/client-core/src/request.rs` and `crates/client-core/src/lib.rs`
- [X] T012 Preserve `ClientBuilder::base_url` semantics and add an independent Messaging endpoint override in `crates/client/src/client.rs`
- [X] T013 [P] Add failing continuation adapters for `next_page_uri` and `meta.next_page_url`, including loops and cross-origin cases, in `crates/client-core/tests/transport_contract/pagination.rs`
- [X] T014 Implement source-neutral opaque continuation handling in `crates/client-core/src/pagination.rs`
- [X] T015 [P] Add failing byte-stream, cancellation, empty-body, redirect, and non-JSON error contracts in `crates/client-core/tests/transport_contract/media.rs`
- [X] T016 Implement credential-safe streaming byte responses without exposing Reqwest in `crates/client-core/src/response.rs` and `crates/client-core/src/request.rs`
- [X] T017 [P] Extend sensitive-value canaries to generated errors, URLs, bodies, signatures, SIP, payment, and media fields in `crates/client-core/tests/transport_contract/redaction_canaries.rs`
- [X] T018 Harden generated error/debug templates and stable normalization in `codegen/templates/rust/reqwest/api_mod.mustache`, `crates/client-core/src/error.rs`, and `crates/client/src/error.rs`
- [X] T019 Add a failing atomic two-source regeneration/rollback test matrix in `codegen/tests/regenerate.bats`
- [X] T020 Make regeneration table-driven, clean-directory, and atomic across both generated crates in `codegen/regenerate.sh` and `codegen/normalize-generated.sh`

**Checkpoint**: Both pinned inputs verify; current public/behavior fixtures are frozen; endpoint, pagination, byte, redaction, and atomic-regeneration foundation tests pass.

---

## Phase 3: User Story 1 - Use Every Pinned REST Operation (Priority: P1) 🎯 MVP

**Goal**: Expose all 197 source-qualified operations through generated companions, with typed results, safe errors, complete parameter behavior, pagination, binary-media separation, and thin stable facades for common workflows.

**Independent Test**: Run the generated contract fixtures for all 139 `API-2010` and 58 `MSG-V1` entries; every applicable method/path/auth/parameter/success/error/pagination dimension passes and every N/A dimension has a reason.

### Tests for User Story 1

- [X] T021 [P] [US1] Add a failing source-qualified 139/58/197 inventory test that rejects missing, duplicate, substituted, and unresolved operations in `crates/api-generated/tests/selected_coverage.rs`
- [X] T022 [P] [US1] Add failing Messaging generation regressions and exhaustive unknown-enum round trips in `crates/messaging-generated/tests/generation_regression.rs` and `crates/messaging-generated/tests/fixtures/unknown_enum_values.json`
- [X] T023 [P] [US1] Add failing API-2010 contracts for applications, calls, events, notifications, transcriptions, streams, tokens, and user-defined messages in `crates/api-generated/tests/contracts/voice_core.rs`
- [X] T024 [P] [US1] Add failing API-2010 contracts for conferences, participants, recordings, payments, queues, and members in `crates/api-generated/tests/contracts/voice_control.rs`
- [X] T025 [P] [US1] Add failing API-2010 contracts for SIP resources, caller IDs, validation requests, and incoming phone numbers in `crates/api-generated/tests/contracts/voice_configuration.rs`
- [X] T026 [P] [US1] Add failing API-2010 contracts for messages, feedback, media, and short codes in `crates/api-generated/tests/contracts/messaging_api2010.rs`
- [X] T027 [P] [US1] Add failing Messaging-v1 contracts for Services, alpha/channel/destination senders, phone numbers, and short codes in `crates/messaging-generated/tests/contracts/services_and_senders.rs`
- [X] T028 [P] [US1] Add failing Messaging-v1 contracts for A2P, toll-free, deactivation, and link-shortening resources in `crates/messaging-generated/tests/contracts/compliance_and_links.rs`
- [X] T029 [P] [US1] Add failing facade contracts for recording/MMS metadata versus byte download and redirect safety in `crates/client/tests/contracts/media.rs`
- [X] T030 [P] [US1] Add failing stable-facade compile contracts for all FR-004 domain entry points and generated-type isolation in `crates/client/tests/public_contract/domains.rs`

### Implementation for User Story 1

- [X] T031 [US1] Generate all 58 Messaging-v1 APIs/models/docs with preserved unknown enums in `crates/messaging-generated/src/` and `crates/messaging-generated/docs/`
- [X] T032 [US1] Wire Messaging-v1 generated configuration through the shared endpoint-aware core in `crates/messaging-generated/src/apis/configuration.rs`
- [X] T033 [US1] Implement deterministic typed contract-driver generation and fixture overrides in `codegen/contracts/generate.rs` and `codegen/contracts/overrides.toml`
- [X] T034 [P] [US1] Populate the 139-entry source-qualified API-2010 evidence manifest in `codegen/coverage/rest-api-2010.toml`
- [X] T035 [P] [US1] Populate the 58-entry source-qualified Messaging-v1 evidence manifest in `codegen/coverage/rest-messaging-v1.toml`
- [X] T036 [US1] Adapt both generated page shapes to the stable `Pager<T>` and one-page contract in `crates/client/src/pagination.rs`
- [X] T037 [P] [US1] Add typed applications, calls, call-event, and call-notification adapters while preserving existing Calls APIs in `crates/client/src/voice/calls.rs` and `crates/client/src/calls.rs`
- [X] T038 [P] [US1] Add conferences and participants stable handles with open statuses and typed identifiers in `crates/client/src/voice/conferences.rs`
- [X] T039 [P] [US1] Add recordings, transcriptions, add-on payload, and explicit byte-download handles in `crates/client/src/voice/recordings.rs` and `crates/client/src/media.rs`
- [X] T040 [P] [US1] Add queues and members stable handles in `crates/client/src/voice/queues.rs`
- [X] T041 [P] [US1] Add SIP domains, credentials, ACLs, mappings, SIPREC, and streams stable handles in `crates/client/src/voice/sip.rs`
- [X] T042 [P] [US1] Add caller-ID, validation, incoming-number, and application configuration handles in `crates/client/src/voice/configuration.rs`
- [X] T043 [P] [US1] Extend Messages with feedback, media metadata/download, MMS sender alternatives, and pinned validation in `crates/client/src/messages.rs` and `crates/client/src/messaging/media.rs`
- [X] T044 [P] [US1] Add Messaging Services and sender-management stable handles backed by Messaging v1 in `crates/client/src/messaging_services/mod.rs` and `crates/client/src/messaging_services/senders.rs`
- [X] T045 [US1] Add remaining Messaging-v1 compliance and link-shortening operations to the generated access documentation in `crates/messaging-generated/README.md`
- [X] T046 [US1] Register all additive domain accessors while keeping generated types private in `crates/client/src/client.rs` and `crates/client/src/lib.rs`
- [X] T047 [US1] Document REST discovery, generated-companion escape hatches, pagination, errors, and binary media in `crates/client/README.md` and `crates/api-generated/README.md`

**Checkpoint**: All 197 operations are invocable with passing deterministic contracts; common Voice/Messaging workflows are discoverable through stable facades; the pre-feature compatibility baseline still passes.

---

## Phase 4: User Story 2 - Build Complete Voice and Messaging TwiML (Priority: P1)

**Goal**: Build every TwiML node and legal relationship in the pinned matrix with typed attributes, safe escaping, deterministic XML, and actionable validation.

**Independent Test**: Render a valid document for every node and allowed relationship, compare every canonical golden, and reject each prohibited relationship or invalid constrained value while keeping existing TwiML outputs byte-equivalent.

### Tests for User Story 2

- [X] T048 [P] [US2] Add the complete node, attribute, parent-child, and control-flow inventory in `codegen/coverage/twiml-nodes.toml`
- [X] T049 [P] [US2] Add canonical Voice goldens for every verb/noun and legal relationship in `crates/client/tests/fixtures/twiml/voice/`
- [X] T050 [P] [US2] Add canonical Messaging goldens and repeated-media/order cases in `crates/client/tests/fixtures/twiml/messaging/`
- [X] T051 [P] [US2] Add failing invalid-constraint, forbidden-nesting, escaping, Unicode, repetition, and control-transfer tests in `crates/client/tests/contracts/twiml_validation.rs`

### Implementation for User Story 2

- [X] T052 [US2] Refactor the private ordered AST into typed attributes and redacted debug output without changing existing XML in `crates/client/src/twiml/mod.rs`
- [X] T053 [P] [US2] Implement `Connect` and ConversationRelay/Room/Stream/VirtualAgent noun builders in `crates/client/src/twiml/voice/connect.rs`
- [X] T054 [P] [US2] Implement complete `Dial` and Application/Client/Conference/Number/Queue/Sip noun builders in `crates/client/src/twiml/voice/dial.rs`
- [X] T055 [P] [US2] Implement complete `Gather`, `Say`, `Play`, and `Pause` attributes and child rules in `crates/client/src/twiml/voice/gather.rs`
- [X] T056 [P] [US2] Implement `Pay`, `Prompt`, and payment `Parameter` builders with redaction-safe validation in `crates/client/src/twiml/voice/pay.rs`
- [X] T057 [P] [US2] Implement `Start`/`Stop` Recording, Siprec, Stream, and Transcription builders in `crates/client/src/twiml/voice/start_stop.rs`
- [X] T058 [P] [US2] Implement remaining Enqueue/Echo/Hangup/Leave/Record/Redirect/Refer/Reject verbs and SIP/Parameter children in `crates/client/src/twiml/voice/control.rs`
- [X] T059 [P] [US2] Complete Message/Body/Media/Redirect attributes and nesting in `crates/client/src/twiml/messaging.rs`
- [X] T060 [US2] Document the complete nesting grammar and add the inbound-call TwiML example in `crates/client/README.md` and `crates/client/examples/inbound_call_twiml.rs`

**Checkpoint**: The TwiML manifest is fully green; existing programs compile and retain byte-equivalent XML; all new nodes, attributes, relationships, and invalid cases have evidence.

---

## Phase 5: User Story 3 - Receive and Verify Webhooks and Callbacks (Priority: P1)

**Goal**: Verify exact Twilio signatures and parse every pinned Voice/Messaging webhook family into typed known fields while retaining unknown fields and values.

**Independent Test**: Replay canonical GET, form POST, and JSON fixtures for every webhook row; valid signatures parse, changed inputs fail, known values are typed, and unknown fields/values remain inspectable without leaking sensitive data.

### Tests for User Story 3

- [X] T061 [P] [US3] Add the complete family, variant, encoding, known-field, open-value, and response inventory in `codegen/coverage/webhook-families.toml`
- [X] T062 [P] [US3] Add synthetic signed Voice instruction/progress/action callback fixtures in `crates/client/tests/fixtures/webhooks/voice/`
- [X] T063 [P] [US3] Add synthetic signed recording/conference/queue/Gather/stream/SIPREC/transcription/payment/UDM fixtures in `crates/client/tests/fixtures/webhooks/callbacks/`
- [X] T064 [P] [US3] Add synthetic signed inbound SMS/MMS, delivery-status, Messaging Service routing, and JSON fixtures in `crates/client/tests/fixtures/webhooks/messaging/`
- [X] T065 [P] [US3] Add failing verification, unknown-retention, duplicate/empty pair, replay/order, and redaction contracts in `crates/client/tests/contracts/webhook_events.rs`

### Implementation for User Story 3

- [X] T066 [US3] Split raw inputs, verified-family envelopes, open values, ordered form extras, and JSON extras into `crates/client/src/webhook/model.rs`
- [X] T067 [US3] Preserve low-level validator behavior and add explicit verify-then-parse entry points in `crates/client/src/webhook/validator.rs` and `crates/client/src/webhook/mod.rs`
- [X] T068 [P] [US3] Implement Voice instruction, call-progress, answering-machine, and TwiML action parsers in `crates/client/src/webhook/voice.rs`
- [X] T069 [P] [US3] Implement recording, conference, queue, and Gather callback parsers in `crates/client/src/webhook/voice_control.rs`
- [X] T070 [P] [US3] Implement stream, SIPREC, transcription, payment, and user-defined-message parsers in `crates/client/src/webhook/voice_realtime.rs`
- [X] T071 [P] [US3] Implement inbound SMS/MMS, indexed media, delivery-status, and Messaging Service routing parsers in `crates/client/src/webhook/messaging.rs`
- [X] T072 [US3] Add redacted `Debug` and field/rule-only parse errors across typed webhook models in `crates/client/src/webhook/error.rs`
- [X] T073 [P] [US3] Add the call-status verification example in `crates/client/examples/verify_call_status.rs`
- [X] T074 [P] [US3] Add inbound-message and delivery-status verification examples in `crates/client/examples/parse_inbound_message.rs` and `crates/client/examples/verify_delivery_status.rs`
- [X] T075 [US3] Document exact public URL handling, proxy safety, family selection, extras, and no-deduplication guarantees in `crates/client/README.md`

**Checkpoint**: Every webhook family/variant has valid, invalid, typed-parse, forward-compatible, and redaction evidence; existing `WebhookValidator` behavior is unchanged.

---

## Phase 6: User Story 4 - Upgrade Without an API Break (Priority: P1)

**Goal**: Demonstrate that existing Dialkit consumers compile and observe unchanged Calls, Messages, TwiML, webhook, error, pagination, feature, and generated-companion behavior.

**Independent Test**: Compile the frozen public suite and compare all baseline fixtures against the candidate; cargo-semver-checks and generated-path checks report no unapproved break.

### Tests for User Story 4

- [X] T076 [P] [US4] Expand public compile contracts for all pre-feature facade symbols and feature combinations in `crates/client/tests/compatibility/public_api.rs`
- [X] T077 [P] [US4] Add byte-for-byte baseline assertions for Calls, Messages, TwiML, webhook results, errors, and pagination in `crates/client/tests/compatibility/behavior.rs`
- [X] T078 [P] [US4] Add generated API-2010 path/name compatibility checks in `crates/api-generated/tests/public_compatibility.rs`
- [X] T079 [P] [US4] Add default/rustls/native-tls/no-default-features compile jobs and MSRV/stable gates in `.github/workflows/ci.yml`

### Implementation for User Story 4

- [X] T080 [US4] Route existing Calls and Messages through generated adapters only where baseline requests/results remain identical in `crates/client/src/calls.rs` and `crates/client/src/messages.rs`
- [X] T081 [US4] Preserve existing TwiML validation/output and webhook malformed-versus-mismatch semantics through compatibility shims in `crates/client/src/twiml/mod.rs` and `crates/client/src/webhook/mod.rs`
- [X] T082 [US4] Preserve existing `Error`, `ApiError`, `Pager`, `Page`, and `ClientBuilder::base_url` public shapes in `crates/client/src/error.rs`, `crates/client/src/pagination.rs`, and `crates/client/src/client.rs`
- [X] T083 [US4] Configure and document cargo-semver-checks against the previous release baseline in `.github/workflows/ci.yml` and `docs/maintainer/releasing.md`
- [X] T084 [US4] Record the additive compatibility assessment and any security-motivated debug-format correction in `specs/002-voice-sms-coverage/compatibility-report.md`

**Checkpoint**: Frozen source and behavior suites, feature matrix, generated path checks, and semver checks pass with zero unapproved regressions.

---

## Phase 7: User Story 5 - Audit Coverage and Future Scope Changes (Priority: P2)

**Goal**: Give maintainers a reproducible audit that proves the pinned scope/evidence and reports future upstream drift without silently adopting it.

**Independent Test**: Regenerate inventories from both pinned sources, compare them with all coverage manifests, inject missing/duplicate/renamed/wrong-source entries, and verify the audit fails; then produce complete release evidence from a clean candidate.

### Tests for User Story 5

- [X] T085 [P] [US5] Add mutation fixtures for missing, duplicate, renamed, wrong-source, later-source, and missing-evidence entries in `codegen/tests/fixtures/coverage/`
- [X] T086 [P] [US5] Add failing union/TwiML/webhook audit tests and under-ten-minute report assertions in `codegen/tests/coverage_audit.bats`
- [X] T087 [P] [US5] Add an upstream candidate-diff test proving report-only behavior in `codegen/tests/upstream_diff.bats`

### Implementation for User Story 5

- [X] T088 [US5] Implement source-qualified REST, TwiML, webhook, evidence-file, and generated-symbol audits in `codegen/audit-coverage.sh`
- [X] T089 [US5] Generate human-readable and machine-readable pass/fail coverage reports in `codegen/report-coverage.sh` and `target/coverage/README.md`
- [X] T090 [US5] Extend the upstream workflow to diff both pinned specifications and documentation-manifest inputs without regeneration or publication in `.github/workflows/upstream-spec-check.yml`
- [X] T091 [US5] Extend clean-regeneration and coverage gates for both generated crates in `.github/workflows/ci.yml`
- [X] T092 [US5] Generate release evidence containing pins, hashes, totals, compatibility, timing, security, and exceptions in `scripts/generate-release-evidence.sh` and `specs/002-voice-sms-coverage/release-evidence.md`
- [X] T093 [US5] Document reviewed pin adoption, matrix reconciliation, compatibility assessment, and rollback in `docs/maintainer/upstream-adoption.md` and `docs/maintainer/releasing.md`

**Checkpoint**: The audit reports 139 + 58 = 197 with complete TwiML/webhook evidence, detects all injected drift, and produces traceable release evidence without changing the adopted scope.

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Complete required examples, documentation, performance, security, and release-quality checks spanning all stories.

- [X] T094 [P] Add conference, recording control/download, queue/member, and SIP setup examples in `crates/client/examples/conference_participant.rs`, `crates/client/examples/recording_download.rs`, `crates/client/examples/queue_member.rs`, and `crates/client/examples/sip_setup.rs`
- [X] T095 [P] Add MMS media and Messaging Service sender examples in `crates/client/examples/send_mms.rs` and `crates/client/examples/manage_service_sender.rs`
- [X] T096 Compile all 12 FR-018 workflows and doctests in the example matrix in `.github/workflows/ci.yml`
- [X] T097 [P] Update the user-facing coverage index, generated-companion guidance, TwiML grammar, webhook security, and media safety documentation in `README.md` and `crates/client/README.md`
- [X] T098 Run and tune the 64 KiB TwiML/webhook and coverage-audit performance gates in `crates/client/benches/performance.rs` and `codegen/tests/coverage_audit.bats`
- [X] T099 Extend the sensitive-data audit to generated docs, fixtures, examples, release evidence, SIP/payment fields, and usable signatures in `scripts/audit-sensitive-data.sh`
- [X] T100 Execute every command and expected result in `specs/002-voice-sms-coverage/quickstart.md`, then record the final zero-exception validation summary in `specs/002-voice-sms-coverage/release-evidence.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Starts immediately; T003 depends on T001/T002 for complete package provenance, and T006 depends on T005.
- **Foundational (Phase 2)**: Depends on Setup; blocks all user-story implementation.
- **US1 REST (Phase 3)**: Depends on Foundation and is the suggested MVP because generated REST coverage is the base product capability.
- **US2 TwiML (Phase 4)**: Depends only on Foundation; can proceed alongside US1 after the private AST baseline is frozen.
- **US3 Webhooks (Phase 5)**: Depends only on Foundation; can proceed alongside US1/US2 after validator compatibility is frozen.
- **US4 Compatibility (Phase 6)**: Baseline tests start after Foundation; final candidate assessment depends on whichever of US1-US3 are included in the release.
- **US5 Audit (Phase 7)**: Audit mechanics start after Foundation; final 197/TwiML/webhook release report depends on US1-US4 completion.
- **Polish (Phase 8)**: Depends on all five stories targeted for release.

### User Story Dependencies

- **US1 (P1)**: No story dependency; owns exhaustive REST coverage and the MVP.
- **US2 (P1)**: No dependency on US1; reuses only the shared compatibility/redaction foundation.
- **US3 (P1)**: No dependency on US1/US2; reuses only validator and redaction foundations.
- **US4 (P1)**: Independently proves the frozen baseline, then validates all completed additive stories before release.
- **US5 (P2)**: Inventory and drift detection are independent; its final completeness claim consumes evidence from US1-US4.

### Within Each User Story

- Write the listed failing tests/fixtures before implementation.
- Establish source/manifests/models before adapters and facade integration.
- Keep generated code changes reproducible through templates/scripts; never hand-edit unexplained generated output.
- Complete the story checkpoint before marking its manifest entries complete.
- Rerun the US4 compatibility baseline after every story phase.

### Parallel Opportunities

- Setup T002, T004, and T007 can proceed independently after T001 is understood.
- Foundation endpoint, pagination, media, redaction, and compatibility test work is split across different files.
- US1 contract groups T023-T029 and domain adapters T037-T044 are parallel after their shared prerequisites.
- US2 node families T053-T059 are parallel after T052 establishes the AST contract.
- US3 fixture groups T062-T064 and parser families T068-T071 are parallel after T066-T067 establish the envelope.
- US4 compile, behavior, generated-path, and CI matrix tests T076-T079 are parallel.
- US5 mutation, audit, and upstream-diff tests T085-T087 are parallel.

---

## Parallel Example: User Story 1

```text
Task T023: API-2010 core Voice contracts
Task T024: API-2010 Voice control contracts
Task T025: API-2010 Voice configuration contracts
Task T026: API-2010 Messaging contracts
Task T027: Messaging-v1 Services/senders contracts
Task T028: Messaging-v1 compliance/link contracts

After generated layers and contracts compile:
Task T038: Conferences/participants facade
Task T039: Recordings/media facade
Task T040: Queues/members facade
Task T041: SIP facade
Task T042: Phone/application configuration facade
Task T043: Messages/media facade
Task T044: Messaging Services/senders facade
```

## Parallel Example: User Story 2

```text
After T052:
Task T053: Connect family
Task T054: Dial family
Task T055: Gather family
Task T056: Pay family
Task T057: Start/Stop family
Task T058: Remaining Voice controls
Task T059: Messaging TwiML
```

## Parallel Example: User Story 3

```text
After T066-T067:
Task T068: Voice instruction/progress/action parsers
Task T069: Recording/conference/queue/Gather parsers
Task T070: Realtime/payment/UDM parsers
Task T071: Messaging webhook parsers
Task T073: Call-status example
Task T074: Inbound/delivery examples
```

## Parallel Example: User Story 4

```text
Task T076: Public compile baseline
Task T077: Behavioral baseline
Task T078: Generated-path baseline
Task T079: Feature/MSRV CI matrix
```

## Parallel Example: User Story 5

```text
Task T085: Coverage mutation fixtures
Task T086: Audit failure/report tests
Task T087: Upstream report-only test
```

---

## Implementation Strategy

### MVP First: User Story 1

1. Complete Setup and Foundational phases.
2. Complete US1 tests before implementation.
3. Generate and contract-test the 139 + 58 operation union.
4. Add only the common REST facades required by FR-004/FR-018.
5. Stop and validate the US1 checkpoint plus the frozen compatibility baseline.

This MVP removes the principal coverage gap while leaving TwiML, webhooks, and audit work independently deliverable.

### Incremental Delivery

1. **Foundation**: immutable dual pins, compatibility baseline, secure two-origin transport, atomic regeneration.
2. **US1**: complete REST generated surface and common stable facades.
3. **US2**: complete typed TwiML with golden/negative evidence.
4. **US3**: complete verified typed webhooks with unknown retention.
5. **US4**: certify source and behavior compatibility for the assembled candidate.
6. **US5**: certify immutable coverage, drift detection, and release traceability.
7. **Polish**: compile all required workflows and close performance/security/documentation gates.

### Parallel Team Strategy

After Foundation:

- Track A: US1 generated REST contracts and facades.
- Track B: US2 TwiML grammar, builders, and goldens.
- Track C: US3 webhook envelopes, parsers, and fixtures.
- Compatibility baseline runs continuously; audit tooling can begin once manifest schemas stabilize.

## Notes

- `[P]` means different files and no dependency on another unfinished task in that group.
- Every user-story task includes its `[USn]` traceability label; Setup, Foundation, and Polish tasks intentionally do not.
- Generated outputs change only through pinned templates/configuration/scripts.
- All fixtures use synthetic non-secret values and local servers; ordinary CI never contacts Twilio.
- Commit after each task or coherent test/implementation pair, and rerun the compatibility baseline at every checkpoint.

## Phase 9: Convergence

- [X] T101 CRITICAL isolate copied-checkout regeneration builds and make generated tests independent of deleted checkout paths so the clean-regeneration CI gate passes reliably per Constitution II / quality gate 4 (contradicts)
- [X] T102 Generate and execute deterministic method/path, authentication, parameter, success, error, and pagination contracts for every applicable dimension of all 197 operations, with explicit N/A reasons per FR-014 and SC-002 (missing)
- [X] T103 Complete every pinned TwiML node's documented attributes, child forms, parent-child rules, control-flow behavior, and actionable constraint validation per FR-006 and FR-008 (partial)
- [X] T104 Add traceable canonical, attribute, nesting, invalid-input, and forward-compatibility evidence for every TwiML node and legal relationship per FR-015 and SC-003 (partial)
- [X] T105 Replace generic webhook-family projections with family-specific typed known fields and event/status variants while preserving unknown fields and values per FR-009 and FR-010 (partial)
- [X] T106 Add valid-signature, invalid-signature, known-field, unknown-field/value, and applicable transport fixtures for every webhook family and variant per FR-015 and SC-004 (missing)
- [X] T107 Make the coverage audit resolve every REST dimension, TwiML entry, and webhook family to executable evidence and reject generic labels, shared-file placeholders, and absent applicable evidence per FR-016 (partial)
- [X] T108 Implement and prove all 12 documented workflows through operational stable-facade examples, including conference participation, recording control/download, queue/member control, SIP setup, and Messaging Service sender management per FR-018 and SC-006 (partial)
- [X] T109 Make release evidence execute or consume authoritative results for every claimed gate, fail when any result is missing or failed, and stop emitting hard-coded PASS claims per FR-017 (contradicts)

## Phase 10: Convergence

- [X] T110 CRITICAL replace static generated-source inspection with executable local-server contracts for every applicable method/path, authentication, parameter location/requiredness/multiplicity/constraint/media-type, success, error, and pagination dimension of all 197 operations per FR-002, FR-014, SC-002, and Constitution IV (missing)
- [X] T111 CRITICAL add a consistent one-page and automatic all-page traversal API for all 43 selected list operations, preserving filters and opaque Twilio continuations with zero/multi-page, reserved-token, loop, and cross-origin contracts per FR-003 and US1/AC2 (missing)
- [X] T112 Render and validate every TwiML manifest node, documented attribute, legal parent-child edge, repeated/content form, control-flow rule, enum/range, mutual exclusion, and relevant invalid case with selector-addressable canonical fixtures per FR-006, FR-007, FR-008, FR-015, and SC-003 (partial)
- [X] T113 Replace generic webhook field maps and grouped projections with family-specific typed fields and event/status variants, then execute valid/invalid signature, known/unknown value, malformed, GET/form/JSON, and parser evidence for every applicable matrix variant per FR-009, FR-010, FR-015, and SC-004 (partial)
- [X] T114 Make coverage audit, CI, and release evidence consume actual per-entry executable results, reject shared test-name/static-source placeholders, and run `coverage_audit.bats` plus `upstream_diff.bats` so no report can pass while T110-T113 evidence is absent per FR-016, FR-017, SC-007, and Constitution IV (contradicts)
- [X] T115 Publish task-oriented documentation for all 12 representative workflows with stable-facade links, synthetic inputs, and no raw HTTP or hand-authored XML per FR-018 and SC-006 (partial)
- [X] T116 Conduct and record the representative-task usability review, including first-attempt outcomes proving at least 9 of 10 tasks are completed from public API names and documentation alone per SC-010 and plan delivery phase 5 (missing)

## Phase 11: Convergence

- [X] T117 CRITICAL extend all 197 local-server REST contracts and their audit ledger to prove exact authentication, pinned header/query/path/form/body placement, requiredness, multiplicity, documented constraints, media types, representative response fields, error shape, and explicit per-operation N/A reasons, including the four Messaging-v1 header parameters, per FR-002, FR-014, SC-002, and Constitution IV (partial)
- [X] T118 Add selector-addressable TwiML invalid fixtures for each documented enum/range, required-value, mutual-exclusion, and parent-child constraint while retaining every row's canonical render evidence per FR-008, FR-015, and SC-003 (partial)
- [X] T119 Add family-specific typed JSON webhook parsing for every JSON-capable matrix family, preserve structured unknown properties and unknown event/status values, and exercise those projections in each JSON variant's valid/invalid/malformed contract per FR-009, FR-010, and SC-004 (partial)

## Phase 12: Convergence

- [X] T120 Add usable typed stable-facade operations for applications, incoming phone-number configuration, and media metadata instead of account-SID-only handles, with synthetic local-server contracts and discoverable examples per FR-004, T037, T042, and T043 (partial)
- [X] T121 Add method-aware GET/query webhook verification and signed GET fixtures that validate the exact public URL, query ordering/encoding, tampering, and known/unknown field parsing independently of form POST per FR-011 and SC-004 (partial)
- [X] T122 Generate and verify supported-operation totals for every pinned REST matrix row in release evidence, alongside the existing 139/58/197 totals, per FR-017 and US5/AC3 (partial)
- [X] T123 Conduct a ten-task first-attempt usability review from public API names and documentation alone, recording actual task construction and outcomes rather than only compiling prewritten examples, with at least nine successful attempts per SC-010 and plan delivery phase 5 (partial)

## Phase 13: Convergence

- [X] T124 CRITICAL add complete compiling rustdoc examples for the new application, incoming phone-number, and media metadata public APIs, and run the documentation gate per Constitution III (partial)
- [X] T125 Inventory the pinned TwiML enum, range, required-value, and mutually-exclusive attribute rules beyond the currently validated method/input/payment/reason/positive-number cases; enforce each rule and add selector-addressable negative fixtures per FR-008 and US2/AC2 (partial)
- [X] T126 Preserve and expose structured known JSON webhook fields such as Message Status `ChannelData` through family-specific typed accessors, with malformed-type and unknown-value contracts rather than moving known fields into extras per FR-010 and US3/AC3 (partial)
- [X] T127 Implement and contract-test method-aware signed GET/query webhook verification independently of form POST, including exact URL, query ordering/encoding, duplicate/empty values, and tampering per FR-011 and US3/AC1-2 (partial)
- [X] T128 Derive release-evidence operation counts for every pinned REST matrix row, verify their sum against 139/58/197, and fail the gate on missing or mismatched rows per FR-017 and US5/AC3 (partial)
- [X] T129 Finish T120 by compiling the new application/phone/media example and passing full relevant facade contracts, formatting, lint, and compatibility gates before marking its stable operations complete per FR-004 and Constitution III-IV (partial)
- [X] T130 Replace the prewritten-example-only usability review with ten recorded first-attempt task constructions using public names and documentation, scoring at least nine successful attempts per SC-010 and plan delivery phase 5 (partial)
