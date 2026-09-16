# Contract: Generation Provenance and Release Reproduction

## Initial immutable inputs

| Input | Required value |
|-------|----------------|
| Generator | OpenAPI Generator `rust` |
| Library | `reqwest` |
| Generator version | `7.25.0` |
| Generator tag commit | `ef964b04480889ef86b56cfae84ade8ad4c91c41` |
| Generator CLI JAR SHA-256 | `41ce4f6b07f196676439d710759fa1ced7a08066d06ff1bf314681470289efae` |
| Twilio repository | `twilio/twilio-oai` |
| Twilio commit | `5aa7f31977ce5812f7b7bc1f46a38555ebaa2888` |
| Twilio source path | `spec/json/twilio_api_v2010.json` |
| Twilio document SHA-256 | `170b3ccd0f891416840083d72f1795b1499b14a18d4873fd2b39f47ef84642d6` |
| Template revision | `dialkit-rust-v1` |
| Generation timestamps | Hidden |
| Async | Enabled |
| Single request parameter | Enabled |

`codegen/generation-manifest.toml` records these values plus deterministic hashes for generator configuration and the sorted custom-template tree. Release notes reproduce or link to the manifest.

## Input acquisition

- The generator artifact is downloaded from the version-addressed official Maven release and verified before execution, or executed from an OCI image pinned by immutable digest with equivalent recorded provenance.
- The Twilio document is committed under `codegen/spec/` or downloaded only from the full commit-addressed raw URL and verified before use.
- Custom templates originate from the same generator release. Only required overrides are committed.
- Applicable Twilio, OpenAPI Generator, and template license/attribution notices remain in the repository and release packages.
- A checksum mismatch stops generation before any committed output is modified.

## Deterministic regeneration

`codegen/regenerate.sh` is non-interactive and performs these stages:

1. Validate manifest shape and every immutable input checksum.
2. Validate the OpenAPI document.
3. Create a fresh temporary output directory.
4. Invoke the generator once with the committed configuration/templates and explicit deterministic environment (`UTC`, stable locale, hidden timestamps).
5. Apply only the committed formatting/normalization step using the declared Rust toolchain.
6. Compile and run generated-layer fixture tests.
7. Compare the complete temporary tree with `crates/api-generated`, including added, changed, and deleted paths.
8. In check mode, exit nonzero on any difference. In update mode, replace only the generated output after all prior stages pass and print the review summary.

Generated files carry a clear do-not-edit notice where the template supports it. Corrections belong in the pinned input, generator config, template, or a documented handwritten conversion layer.

## Continuous integration

Every pull request that can affect generation runs:

- input checksum and OpenAPI validation;
- clean regeneration in check mode;
- generated crate compilation on Rust 1.85 and stable;
- unknown-value and representative serialization/deserialization fixtures;
- high-value request/response contract tests;
- formatting, lint, documentation, workspace, and feature-matrix gates.

The drift workflow may report a newer upstream commit and produce an informational diff artifact. It must never commit, merge, publish, or adopt the revision automatically.

## Upstream adoption review

Before changing the pinned Twilio commit:

1. Review the upstream specification diff before accepting generated output.
2. Classify operation additions/removals, required/optional field changes, type/enum changes, serialization changes, and documented behavior changes.
3. Regenerate into a temporary tree and review the generated diff for public API, model, error, docs, and coverage effects.
4. Update or add contract fixtures for every high-value or behaviorally significant change.
5. Record facade compatibility impact and planned Semantic Versioning treatment.
6. Update migration guidance for any developer-facing break.
7. Update all provenance values and hashes together.

A generator or template change follows the same process and cannot be bundled invisibly into a spec-only update.

## Release contract

Release readiness requires:

- identical output from unchanged pinned inputs;
- 100% pinned-operation coverage audit;
- all contract, documentation, MSRV/stable, feature, format, and lint gates passing;
- `cargo-semver-checks` result against the latest published `dialkit` facade;
- dependency-order dry run/package verification for core, generated, then facade;
- release notes naming the specification SHA, generator version, template revision, notable upstream changes, compatibility classification, and migration guidance;
- no secrets or sensitive Twilio data in packages, fixtures, generated docs, or CI output.

An unexplained generated difference, missing provenance value, failed compatibility review, or untested corrected defect blocks release.

## Rollback

Because every release records full inputs, maintainers can restore the prior manifest, spec file, templates, config, and generated tree as one versioned change. Rollback never fetches `main` or `latest`; it uses the prior immutable values and reruns the same validation gates before publication.
