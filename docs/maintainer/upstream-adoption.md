# Adopting a Twilio specification revision

1. Record the candidate full commit SHA and review both API-2010 and Messaging-v1 diffs before
   generating code. The scheduled workflow is report-only and never changes a pin.
2. Classify added and removed operations, requiredness changes, types/enums, serialization,
   documented behavior, and licensing changes.
3. Update the reviewed vendored documents and every related hash in
   `codegen/generation-manifest.toml`; never combine a source-pin change with an unreviewed
   generator/template change.
4. Run `codegen/regenerate.sh --update` and review the complete generated diff, including deleted
   paths. Generator corrections belong in templates or `normalize-generated.sh`, never generated files.
5. Add focused fixtures for high-value or behaviorally changed endpoints and every corrected defect.
6. Reconcile every source-qualified operation with the 139/58 coverage matrices, then reconcile
   TwiML relationships and webhook families against their documentation manifests. Removed,
   renamed, or later-source entries require explicit review rather than silent substitution.
7. Record stable-facade compatibility impact. Prefer deprecation when migration is practical; use a
   major version for incompatible removal and provide migration guidance.
8. Run `codegen/regenerate.sh --check`, `codegen/audit-coverage.sh`, the quickstart, package,
   documentation, MSRV/stable, feature-matrix, security, and semver gates before merging. Generate
   the release-evidence file only after the matrix is green.

To roll back, restore the prior manifest, both specifications, configurations, templates,
normalization script, coverage matrices, and generated trees as one reviewed change. Regenerate
using those immutable prior values; never fetch a moving branch during rollback. Rerun the same
compatibility assessment because rollback safety includes the stable facade, not only generated
files.
