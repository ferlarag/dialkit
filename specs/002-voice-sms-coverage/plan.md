# Implementation Plan: Complete Voice and Messaging Coverage

**Branch**: `sms-voice-coverage` | **Spec Kit feature**: `002-voice-sms-coverage` | **Date**: 2026-09-17 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/002-voice-sms-coverage/spec.md`

## Summary

Extend the existing generated-layer architecture schema-first. Keep `dialkit-api-generated` and all of its public paths unchanged for API 2010, add a sibling `dialkit-messaging-generated` crate for the pinned Messaging v1 specification, and generate both reproducibly from OpenAPI Generator 7.25.0. Drive the supported release claim from a checked-in coverage manifest containing exactly the 197 operations in the feature matrices rather than from whatever a generator happens to emit. Preserve every existing `dialkit` symbol and behavior while adding thin domain handles for common Voice, Messaging, media, and Messaging Service workflows; all other in-scope operations remain directly available through the generated companions.

Complete the handwritten stable layer with typed, ordered TwiML builders and verified webhook envelopes. Forward-compatible generated enums retain unknown raw values, webhook models retain unknown fields, pagination follows opaque continuation URLs on the correct allowlisted origin, and error/debug/tracing paths redact sensitive content. Deterministic per-entry contract evidence, golden XML/webhook fixtures, compatibility tests, clean regeneration, coverage auditing, and release evidence gate each delivery phase.

## Technical Context

**Language/Version**: Rust 2024 Edition; minimum supported Rust 1.85 (unchanged)

**Primary Dependencies**: Existing Reqwest 0.12.25, Tokio 1.47.1, Serde 1.0.228, serde_json 1.0.145, secrecy 0.10.3, thiserror 2.0.20, tracing 0.1.44, futures-core/futures-util 0.3.31, async-stream 0.3.6, quick-xml 0.38.3, HMAC/SHA-1/SHA-256/Base64, chrono 0.4.42, and url 2.5.7. OpenAPI Generator 7.25.0 remains the pinned build tool.

**Storage**: No runtime persistence. Version-controlled storage adds the Messaging v1 OpenAPI document, a declarative coverage manifest, TwiML/webhook snapshot metadata, generator inputs, deterministic fixtures, and release evidence.

**Testing**: Rust unit/doc/compile tests; Wiremock contracts; Insta golden fixtures; Proptest escaping/canonicalization properties; Trybuild compatibility boundaries; cargo-semver-checks; generator regression and clean-tree checks; a coverage audit that joins every supported entry to applicable evidence.

**Target Platform**: Server-side Linux, macOS, and Windows supported by Rust 1.85; native async applications. Browser/WASM remains out of scope.

**Project Type**: Published Rust library workspace with one shared core, two pinned generated companions, and the stable `dialkit` facade.

**Performance Goals**: Preserve existing request-overhead and bounded-page-memory goals; validate or parse a 64 KiB webhook and render a 64 KiB TwiML document under 10 ms p95 on the CI reference runner; complete the full release coverage audit in under 10 minutes.

**Constraints**: Existing public source and observable behavior are frozen; generated files are never edited manually; the two OpenAPI files and documentation matrices are immutable release inputs; API 2010 and Messaging v1 require distinct trusted origins; unknown service values/fields must remain inspectable; binary media is never decoded as JSON; credentials, signatures, message bodies, phone/SIP/payment data, and concrete URLs must not enter errors or traces; ordinary CI is credential-free and non-billable.

**Scale/Scope**: Exactly 197 supported REST operations (139 API 2010 and 58 Messaging v1), all TwiML nodes and webhook families in the specification matrices, 12 compiling representative workflows, and phased delivery ordered by the coverage manifest.

## Constitution Check

*GATE: Passed before Phase 0 and re-checked after Phase 1 design.*

| Principle or Gate | Pre-Research Evaluation | Post-Design Evidence |
|-------------------|-------------------------|----------------------|
| I. Twilio specification is source of truth | PASS: the feature retains one deliberately adopted commit and names two exact files/hashes plus dated documentation matrices. | [generation-and-coverage.md](contracts/generation-and-coverage.md) makes both byte hashes, operation inventory, documentation snapshot, and exclusions release inputs. |
| II. Reproducible generation | PASS: the existing pinned generator/template pipeline can be generalized to two specs without changing pins. | Generation is clean-directory, per-spec/per-crate, normalized, formatted, staged together, inventoried, and compared as one declared output set; no generated edit is permitted. |
| III. Rust-native developer experience | PASS: the generated layer remains available while common tasks gain additive stable handles and typed builders. | [public-api.md](contracts/public-api.md) preserves existing calls/messages APIs and defines thin domain facades, typed TwiML, webhook envelopes, bytes, and open values. |
| IV. Contract confidence | PASS: the feature requires evidence for every matrix entry, not representative schema checks alone. | [behavior-and-evidence.md](contracts/behavior-and-evidence.md) defines the evidence ledger, REST dimensions, XML goldens, webhook vectors, pagination cases, redaction canaries, and N/A rules. |
| V. Compatibility and maintenance | PASS: generated schema churn is isolated; stable changes are additive and gated against a frozen baseline. | Compatibility compile/behavior fixtures and semver checks are release gates; later upstream changes require a separate pin/matrix review. |
| Toolchain and CI gates | PASS: MSRV and existing workspace remain unchanged. | [quickstart.md](quickstart.md) covers format, lint, feature/MSRV, docs, contracts, compatibility, regeneration, coverage, and release evidence. |
| Secrets, dependencies, attribution | PASS: no new runtime dependency is required by the proposed design. | Redaction rules cover errors, fixtures, docs, callbacks, SIP/payment fields, and media; both upstream specification licenses and provenance are retained. |

No constitution violations or exceptions are required.

## Project Structure

### Documentation (this feature)

```text
specs/002-voice-sms-coverage/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   ├── behavior-and-evidence.md
│   ├── generation-and-coverage.md
│   └── public-api.md
└── tasks.md                    # Created later by $speckit-tasks
```

### Source Code (repository root)

```text
Cargo.toml
crates/
├── client-core/
│   ├── src/
│   │   ├── error.rs
│   │   ├── pagination.rs
│   │   └── request.rs          # endpoint-aware execution and byte responses
│   └── tests/transport_contract/
├── api-generated/
│   ├── src/
│   │   ├── apis/               # existing API-2010 paths remain compatible
│   │   └── models/             # existing API-2010 paths remain compatible
│   ├── docs/
│   └── tests/
│       ├── contracts/           # generated operation fixtures/evidence
│       ├── coverage_audit.rs
│       └── generation_regression.rs
├── messaging-generated/        # generated Messaging v1 companion; shares core
│   ├── src/{apis,models}/
│   ├── docs/
│   └── tests/{contracts,coverage_audit.rs,generation_regression.rs}
└── client/
    ├── src/
    │   ├── calls.rs             # preserved public API, additive methods only
    │   ├── messages.rs          # preserved public API, additive methods only
    │   ├── voice/               # thin stable Voice resource handles
    │   ├── messaging_services/  # thin stable Messaging v1 handles
    │   ├── media.rs             # metadata vs byte retrieval boundary
    │   ├── pagination.rs
    │   ├── twiml/
    │   └── webhook/             # validation, envelopes, typed event families
    ├── examples/                # 12 required task-oriented workflows
    └── tests/
        ├── compatibility/
        ├── contracts/
        ├── fixtures/twiml/
        └── fixtures/webhooks/
codegen/
├── generation-manifest.toml
├── generator-config.yaml
├── coverage/
│   ├── rest-operations.toml     # exactly 197 entries and evidence dimensions
│   ├── twiml-nodes.toml
│   └── webhook-families.toml
├── spec/
│   ├── twilio_api_v2010.json
│   └── twilio_messaging_v1.json
├── templates/rust/
├── regenerate.sh
└── generated-files.txt
docs/maintainer/
├── releasing.md
└── upstream-adoption.md
```

**Structure Decision**: Preserve the established dependency direction and add one narrowly scoped generated sibling: `dialkit -> {dialkit-core, dialkit-api-generated, dialkit-messaging-generated}` and both generated crates depend only on core. A dry generation found colliding operation/model names (including Short Code list/fetch surfaces) and distinct service origins, so a sibling crate is safer than rewriting the existing companion namespace. This leaves every current `dialkit_api_generated::{apis,models}` path intact while still reusing the same transport, templates, generator, error policy, version, and publication process. `dialkit` does not re-export either generated crate. Existing facade modules and signatures stay in place; new domain handles are additive adapters. A source-qualified declarative manifest defines the supported subset and delivery order.

## Phase 0: Research Outcome

All decisions and rejected alternatives are recorded in [research.md](research.md). Important outcomes are dual-spec generated companions, endpoint-aware shared transport, manifest-driven evidence, additive facade growth, open-value/unknown-field preservation, explicit byte responses, a typed TwiML grammar, verify-before-parse webhooks, and vertical delivery slices. All technical questions are resolved.

## Phase 1: Design Outcome

- [data-model.md](data-model.md) defines pins, coverage entries/evidence, endpoint profiles, generated/stable resources, pages, bytes, TwiML grammar, webhook envelopes/events, errors, compatibility baselines, and phase gates.
- [generation-and-coverage.md](contracts/generation-and-coverage.md) defines immutable inputs, two-spec generation, namespace/output ownership, exact 197-entry auditing, drift review, and release evidence.
- [public-api.md](contracts/public-api.md) defines preserved APIs and additive stable handles for common Voice, Messaging, TwiML, webhook, pagination, and media workflows.
- [behavior-and-evidence.md](contracts/behavior-and-evidence.md) defines wire behavior, endpoint/origin safety, forward compatibility, secure errors, and per-entry deterministic evidence.
- [quickstart.md](quickstart.md) defines runnable, credential-free validation scenarios and the phased release gates.

The post-design constitution re-check passes with no exceptions.

## Delivery Phases

1. **Foundation and frozen baseline**: vendor/verify Messaging v1, formalize manifests, snapshot existing public/behavior contracts, generalize generation and endpoint routing, and make the audit report 197 entries before claiming any new facade coverage.
2. **P1 REST generated coverage**: generate/contract-test all matrix operations, shared pagination, errors, open values, and byte responses; close rows in manifest order by resource dependency.
3. **P1 stable workflow facades**: add only the common handles required by FR-004/FR-018, prioritizing calls/conferences/recordings/queues/SIP and messages/media/Messaging Services/senders.
4. **P1 TwiML and webhooks**: complete the grammar and callback families with canonical goldens, negative nesting/constraint fixtures, signature vectors, and unknown retention.
5. **P2 audit, docs, and release proof**: compile all 12 examples, run compatibility/semver and security scans, generate release evidence, and perform the usability review. Each phase closes only when its coverage-manifest entries contain passing evidence; later phases may not waive earlier gaps.
