# Contract: Generation and Coverage

## Immutable inputs

| Source | Upstream path | SHA-256 | Selected operations | Generated package |
|--------|---------------|---------|--------------------:|-------------------|
| `API-2010` | `spec/json/twilio_api_v2010.json` | `170b3ccd0f891416840083d72f1795b1499b14a18d4873fd2b39f47ef84642d6` | 139 | `dialkit-api-generated` |
| `MSG-V1` | `spec/json/twilio_messaging_v1.json` | `611c6fde586347615039a7d8c87a4b10d39c6d8566b7d1a691353ef995aaa473` | 58 | `dialkit-messaging-generated` |

Both are from `twilio/twilio-oai` commit `5aa7f31977ce5812f7b7bc1f46a38555ebaa2888`. Generator 7.25.0, its artifact hash, template revision/tree hash, source-specific configuration hashes, and output-inventory hashes are mandatory manifest fields. TwiML and webhook manifests identify the official documentation URLs and snapshot date `2026-09-17`.

## Package and namespace ownership

- Existing `dialkit-api-generated` names and module paths do not move or disappear.
- Messaging v1 is generated into a sibling crate because names collide and the service origin differs.
- Both generated crates depend on `dialkit-core`, share template behavior, publish before `dialkit`, and use the workspace lockstep version.
- `dialkit` depends on but does not re-export either companion.
- Every generator-owned path is listed; files outside those inventories are never overwritten.

## Regeneration transaction

1. Validate both vendored specs and all recorded hashes before invoking the generator.
2. Validate the pinned generator artifact, configuration files, templates, and output inventories.
3. Generate each source into a distinct clean temporary directory with fixed `TZ=UTC`, `LC_ALL=C`, and `LANG=C`.
4. Normalize source-specific known generator defects through version-controlled scripts/templates, then run Rustfmt deterministically.
5. Generate typed contract drivers and exhaustive unknown-enum fixtures from the same source.
6. Run both generated crates' compile, regression, and contract tests against staged output.
7. In check mode, compare every owned path and fail on additions, changes, or stale files.
8. In update mode, stage both trees, update both only after all validation passes, and restore both on failure.

Generation never fetches moving specifications. Downloading the pinned generator artifact is permitted only with exact checksum verification. A later source/template/generator update is a separately reviewed adoption.

## REST manifest schema

Each `rest-operations.toml` record contains:

```text
source_id, operation_id, domain, method, path,
generated_crate, generated_symbol, phase, pagination,
evidence.method_path, evidence.authentication,
evidence.parameters, evidence.success, evidence.error,
evidence.pagination
```

Evidence fields are either a fixture/test reference or `{ n_a = "reason" }`. Blank or implicit omission fails the audit.

## Union audit

The audit must prove all of the following:

- exactly 139 unique `API-2010` keys match the Matrix A identifiers;
- exactly 58 unique `MSG-V1` keys match all operation IDs in that pinned file;
- the union is exactly 197 source-qualified keys;
- every key resolves to one method/path in the correct pinned source and one compiled generated function;
- excluded API-2010 operations may remain generated for compatibility but do not count toward this feature;
- every evidence reference exists and every required test reports pass;
- every pagination flag agrees with the schema/approved override;
- every generated string-enum shape has unknown-value round-trip evidence;
- no later-source operation is admitted because it appears in generated output or online docs.

The fast inventory/evidence audit and heavier runtime contracts may be separate CI jobs, but both are mandatory release gates.

## TwiML and webhook audits

The TwiML audit maps every spec-matrix node and allowed parent relationship to its builder symbol, attribute inventory, canonical golden, and invalid/escaping evidence. The webhook audit maps every family/variant/encoding to its parser, signature vector, known-field fixture, unknown-retention fixture, and negative case.

The Markdown matrices are reviewed human-facing views. Machine manifests are source-controlled normative mirrors; CI fails if their normalized inventories disagree.

## Drift and upstream adoption

An upstream monitor may report differences but never regenerate, commit, or publish. Adoption requires:

1. review upstream diffs for operation/schema/documentation changes;
2. update commit, both applicable hashes, manifests, and exclusions together;
3. assess generated and stable API compatibility;
4. regenerate both companions as one transaction;
5. update every affected contract/golden and release evidence;
6. make a SemVer decision before release.

## Release evidence

The release report records immutable pins and hashes, 139/58/197 totals, TwiML/webhook totals, evidence pass/fail counts, clean regeneration, MSRV/stable/feature results, public compatibility and semver results, example compilation, security/redaction scans, audit duration, and every approved exception. A mismatch or unresolved entry blocks the completeness claim.
