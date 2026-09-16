<!--
Sync Impact Report
- Version change: unratified scaffold -> 1.0.0
- Modified principles:
  - Placeholder Principle 1 -> I. Twilio Specification Is the Source of Truth
  - Placeholder Principle 2 -> II. Reproducible Generation
  - Placeholder Principle 3 -> III. Rust-Native Developer Experience
  - Placeholder Principle 4 -> IV. Contract Confidence
  - Placeholder Principle 5 -> V. Compatibility and Sustainable Maintenance
- Added sections:
  - Technical and Release Constraints
  - Development Workflow and Quality Gates
- Removed sections: none
- Follow-up TODOs: none
-->
# dialkit Constitution

## Core Principles

### I. Twilio Specification Is the Source of Truth
All supported Twilio resources, operations, request fields, response models, and documented
constraints MUST derive from the latest deliberately adopted revision of the MIT-licensed
`twilio/twilio-oai` repository. Every adoption MUST record the exact specification commit SHA.
Hand-written deviations from that revision MUST be documented, tested, and narrowly scoped to
correct demonstrably unsuitable generated Rust behavior. Upstream changes MUST be reviewed before
adoption; "latest" means the newest revision that dialkit has intentionally validated and pinned,
not an unreviewed moving branch. This keeps the SDK current without making releases
non-reproducible or silently exposing users to upstream schema breakage.

### II. Reproducible Generation
Every generated artifact MUST be reproducible from version-controlled inputs. Each release MUST pin
the OpenAPI Generator version, the Twilio specification commit SHA, and the custom template version
or immutable template revision. Generation commands and configuration MUST be automated and
documented. CI MUST regenerate all generated output from those pins and fail when the result differs
from committed output. Generated files MUST NOT receive unexplained manual edits; fixes belong in
the specification overlay, generator configuration, or versioned templates so future regeneration
preserves them. Reproducibility makes releases auditable and maintenance dependable.

### III. Rust-Native Developer Experience
Public APIs MUST feel idiomatic, predictable, and safe to Rust developers rather than exposing raw
generator mechanics. Common tasks MUST require minimal boilerplate and provide useful types,
discoverable naming, actionable errors, and complete rustdoc examples. Public interfaces MUST avoid
unnecessary allocations, surprising panics, hidden global state, and needless exposure of transport
details. Examples MUST compile in CI. Any generator customization that improves ergonomics MUST
remain reproducible under Principle II. Developer experience is a release criterion because users
must be able to adopt dialkit as infrastructure they understand and trust.

### IV. Contract Confidence
Schema validation and successful code generation are necessary but insufficient. The project MUST
maintain automated contract tests for its most important Twilio endpoints, including representative
request serialization, authentication and headers, path and query encoding, response deserialization,
Twilio error handling, and pagination where applicable. Every corrected generator defect or SDK
regression MUST gain a focused regression test. Tests MUST be deterministic and MUST NOT require
developers or pull-request CI to spend money or depend on a live Twilio account unless a separately
declared integration job explicitly requires it. This verifies the behavior users actually rely on.

### V. Compatibility and Sustainable Maintenance
dialkit MUST use Semantic Versioning for its public crate API and MUST treat generator- or
specification-driven public API changes by their user-visible compatibility impact. Breaking schema
changes MUST NOT bypass a major-version decision merely because they originated upstream. Releases
MUST include generated-diff review, migration notes for breaking changes, and traceability to all
three generation pins. Deprecation MUST be preferred when a reasonable migration path exists.
Maintenance automation MUST surface upstream specification changes and dependency or toolchain
drift without automatically publishing unreviewed output. This protects long-lived downstream
projects while allowing the SDK to evolve with Twilio.

## Technical and Release Constraints

- The crate and its public examples MUST support the Rust toolchain policy documented by the
  project; changes to the minimum supported Rust version MUST be deliberate and release-noted.
- Public API design MUST separate user-facing SDK ergonomics from generated transport internals so
  generator changes do not cause avoidable downstream churn.
- Authentication credentials and sensitive Twilio data MUST never appear in source, fixtures,
  snapshots, logs, generated documentation, or CI output.
- Release artifacts MUST contain or link to the pinned generator version, Twilio specification SHA,
  and custom template version used to produce them.
- Dependencies and custom generation code MUST be kept minimal, maintained, and justified by a
  concrete reliability, compatibility, security, or developer-experience need.
- The project MUST preserve applicable attribution and license notices for Twilio specifications,
  generator templates, and other incorporated upstream material.

## Development Workflow and Quality Gates

1. A specification update MUST be reviewed first as an upstream diff, with breaking and behaviorally
   significant changes identified before generated output is accepted.
2. Generation MUST run only from pinned inputs. The resulting diff MUST be reviewed for public API,
   serialization, error-model, documentation, and endpoint-coverage effects.
3. Pull requests that alter generated behavior MUST include or update focused tests. Changes to a
   high-value endpoint MUST include applicable contract-test coverage.
4. CI MUST pass formatting, linting with warnings denied for maintained code, compilation for the
   supported feature and toolchain matrix, unit and contract tests, documentation tests, and the
   clean-regeneration check.
5. A release MUST record its generation provenance, summarize notable upstream specification
   changes, state compatibility impact, and provide migration guidance when required.
6. Reviewers MUST explicitly verify constitution compliance. Any exception MUST be documented in
   the change, limited in duration and scope, and assigned a follow-up owner or issue.

## Governance

This constitution is the highest-priority engineering policy for dialkit. Specifications, plans,
reviews, and release decisions MUST demonstrate compliance with it. An amendment requires a pull
request that states the motivation, identifies affected principles and workflows, describes any
migration or enforcement changes, and receives maintainer approval. Amendments take effect only
after this document and its Sync Impact Report are updated.

Constitution versions follow Semantic Versioning: MAJOR for removal or incompatible redefinition of
a principle or governance guarantee, MINOR for a new principle or materially expanded obligation,
and PATCH for non-semantic clarification. Every pull request MUST undergo a proportionate compliance
review; changes involving generation, public API, high-value endpoint behavior, or releases require
explicit evidence for the relevant quality gates. Maintainers MUST review the constitution at least
once per year and whenever the generation pipeline, upstream specification strategy, or release
policy materially changes. Exceptions never establish precedent and MUST include an expiry or a
tracked remediation decision.

**Version**: 1.0.0 | **Ratified**: 2026-09-15 | **Last Amended**: 2026-09-15
