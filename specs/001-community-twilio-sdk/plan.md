# Implementation Plan: Community Twilio SDK

**Branch**: `001-community-twilio-sdk` | **Date**: 2026-09-15 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/001-community-twilio-sdk/spec.md`

## Summary

Build dialkit as a Rust 2024 workspace with a stable handwritten `dialkit` facade over a complete, reproducibly generated Twilio api_v2010 companion crate and a shared transport/core crate. The facade initially makes calls, messages, pagination, errors, webhook validation, and a bounded TwiML vocabulary pleasant and stable; the generated companion preserves full pinned-schema coverage. A single core owns authentication, HTTP execution, conservative retries, timeouts, error normalization, and redacted tracing. Exact generator, specification, and template revisions plus deterministic mock-server contract tests make regeneration and releases auditable.

## Technical Context

**Language/Version**: Rust 2024 Edition; minimum supported Rust 1.85

**Primary Dependencies**: Reqwest 0.12.25 with Rustls; Tokio 1.47.1; Serde 1.0.228; secrecy 0.10.3; thiserror 2.0.20; tracing 0.1.44; futures-core/futures-util 0.3.31; async-stream 0.3.6; quick-xml 0.42.0; hmac 0.12.1, sha1 0.10.6, sha2 0.10.9, and base64 0.22.1. OpenAPI Generator 7.25.0 is a pinned build tool, not a runtime dependency.

**Storage**: No runtime persistence. Version-controlled storage contains the pinned Twilio specification, generator configuration, template overrides, generation manifest, fixtures, and generated source.

**Testing**: Built-in Rust test and documentation harness with Tokio; Wiremock 0.6.5 for HTTP contracts; Insta 1.43.2 for representative form/TwiML fixtures; Proptest 1.7.0 for canonicalization and escaping properties; Trybuild 1.0.111 for compile-time ergonomics and boundary checks; cargo-semver-checks before release.

**Target Platform**: Server-side Linux, macOS, and Windows targets supported by Rust 1.85; native asynchronous applications. Browser/WASM is outside this feature.

**Project Type**: Published library workspace with three lockstep-versioned crates: stable facade, generated API companion, and shared client core.

**Performance Goals**: Reuse HTTP connection pools; add no more than 5 ms p95 local SDK overhead to a request in contract benchmarks; process pagination with memory bounded to the active page plus consumer-held items; validate a 64 KiB webhook or render a 64 KiB TwiML document in under 10 ms p95 on CI reference hardware.

**Constraints**: No credentials, authorization values, phone numbers, or request bodies in library-generated traces; no blanket retry of mutating operations after ambiguous failures; pagination continuation must remain same-origin before authentication is attached; generated artifacts are never manually edited; all ordinary tests are credential-free; public facade types never expose Reqwest or generated types.

**Scale/Scope**: 100% operation/model coverage of pinned `twilio_api_v2010.json`; handwritten P1 facade for Calls and Messages; reusable pagination/error/authentication across all generated operations; initial TwiML contract covers the voice and messaging verbs enumerated in [public-api.md](contracts/public-api.md); three published crates and seven compiling examples.

## Constitution Check

*GATE: Passed before Phase 0 and re-checked after Phase 1 design.*

| Principle or Gate | Pre-Research Evaluation | Post-Design Evidence |
|-------------------|-------------------------|----------------------|
| I. Twilio specification is source of truth | PASS: scope is one deliberately adopted api_v2010 revision. | `research.md` and `generation-provenance.md` pin commit `5aa7f31977ce5812f7b7bc1f46a38555ebaa2888`, source path, and file checksum; generated deviations live only in reviewed templates. |
| II. Reproducible generation | PASS: plan requires immutable generator, spec, and template inputs plus clean regeneration. | `generation-provenance.md` defines exact generator/JAR/spec pins, template tree hash, clean temporary generation, formatting, and full-tree comparison. |
| III. Rust-native developer experience | PASS: handwritten stable facade owns task-focused builders, errors, examples, and stream pagination. | `public-api.md` defines private-field builders, stable string-backed values, typed errors, `Pager<T>`, webhook and TwiML APIs without generated or transport types. |
| IV. Contract confidence | PASS: deterministic contracts are required for high-value endpoints and corrected defects. | `quickstart.md` and `protocol-behavior.md` define mock-server, webhook vector, TwiML fixture, compile-boundary, regression, and optional test-credential checks. |
| V. Compatibility and maintenance | PASS: generated schema churn is isolated and stable APIs follow Semantic Versioning. | Crate graph prevents generated re-exports; all crates publish in lockstep, facade changes receive semver checks, and provenance/update reviews are mandatory. |
| Toolchain and CI gates | PASS: MSRV will be explicit and examples/test matrix automated. | CI design covers Rust 1.85 and stable, format, lint with warnings denied, workspace/all-feature tests, docs, clean regeneration, TLS feature sets, and semver checks. |
| Secrets, dependencies, attribution | PASS: dedicated secret type/redaction and minimal justified dependencies planned. | Data model makes secrets non-serializable/redacted; protocol contract allowlists trace fields; provenance contract preserves MIT notices and justifies each dependency category. |

No constitution violations or exceptions are required.

## Project Structure

### Documentation (this feature)

```text
specs/001-community-twilio-sdk/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   ├── generation-provenance.md
│   ├── protocol-behavior.md
│   └── public-api.md
└── tasks.md                    # Created by $speckit-tasks, not this phase
```

### Source Code (repository root)

```text
Cargo.toml                      # Virtual workspace, resolver 3, shared metadata/dependencies
Cargo.lock
crates/
├── client-core/               # Published as dialkit-core; no generated dependency
│   ├── Cargo.toml
│   ├── src/
│       ├── auth.rs
│       ├── error.rs
│       ├── lib.rs
│       ├── pagination.rs
│       ├── request.rs
│       ├── retry.rs
│       └── trace.rs
│   └── tests/
│       ├── transport_contract.rs
│       └── fixtures/
├── api-generated/             # Published companion; generated files never edited manually
│   ├── Cargo.toml
│   ├── README.md
│   ├── src/
│   └── tests/
│       ├── generated_contract.rs
│       └── fixtures/
└── client/                    # Published as dialkit; stable handwritten facade
    ├── Cargo.toml
    ├── README.md
    ├── examples/
    │   ├── handle_error.rs
    │   ├── make_call.rs
    │   ├── paginate_calls.rs
    │   ├── send_message.rs
    │   ├── setup_client.rs
    │   ├── twiml_response.rs
    │   └── validate_webhook.rs
    ├── src/
        ├── calls.rs
        ├── client.rs
        ├── error.rs
        ├── lib.rs
        ├── messages.rs
        ├── twiml/
        └── webhook.rs
    └── tests/
        ├── public_contract.rs
        ├── compile/
        ├── fixtures/
        └── webhook_vectors/
codegen/
├── generator-config.yaml
├── generation-manifest.toml
├── regenerate.sh
├── spec/
│   └── twilio_api_v2010.json
└── templates/
    └── rust/                   # Minimal overrides extracted from generator 7.25.0
.github/workflows/
├── ci.yml
└── upstream-spec-check.yml     # Reports drift; never publishes automatically
```

**Structure Decision**: Use a virtual workspace with one-way dependencies: `dialkit -> {dialkit-core, dialkit-api-generated}` and `dialkit-api-generated -> dialkit-core`. All three packages carry path plus version dependencies and publish in dependency order at one lockstep version because crates.io cannot publish a package with unpublished path-only dependencies. `dialkit-api-generated` is documented as an unstable companion for complete schema access but is never re-exported from `dialkit`; explicit conversions at the facade boundary prevent generated types, Reqwest types, and generator naming from entering the stable public API.

## Phase 0: Research Outcome

All technical unknowns were resolved in [research.md](research.md). The selected approach uses the stable OpenAPI Generator Rust client with custom pinned templates, one shared transport core, a stable handwritten facade, conservative operation-aware retries, exact externally visible webhook inputs, typed XML construction, and deterministic credential-free contracts. No `NEEDS CLARIFICATION` markers remain.

## Phase 1: Design Outcome

- [data-model.md](data-model.md) defines configuration, credentials, requests/results, errors, pages, webhook inputs/results, TwiML nodes, provenance, validation rules, relationships, and state transitions.
- [public-api.md](contracts/public-api.md) fixes the stable facade shape and the separately consumable generated-companion boundary.
- [protocol-behavior.md](contracts/protocol-behavior.md) fixes authentication, encoding, error normalization, pagination, retry, tracing, webhook, and TwiML behavior.
- [generation-provenance.md](contracts/generation-provenance.md) fixes immutable inputs, generation steps, drift checks, update review, and release evidence.
- [quickstart.md](quickstart.md) provides runnable implementation-validation scenarios and expected results without embedding implementation bodies.

The post-design constitution re-check passes with no exceptions.
