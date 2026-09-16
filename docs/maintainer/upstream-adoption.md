# Adopting a Twilio specification revision

1. Record the candidate full commit SHA and review the upstream diff before generating code.
2. Classify added and removed operations, requiredness changes, types/enums, serialization,
   documented behavior, and licensing changes.
3. Update the vendored document and every related hash in `codegen/generation-manifest.toml`.
4. Run `codegen/regenerate.sh --update` and review the complete generated diff, including deleted
   paths. Generator corrections belong in templates or `normalize-generated.sh`, never generated files.
5. Add focused fixtures for high-value or behaviorally changed endpoints and every corrected defect.
6. Record stable-facade compatibility impact. Prefer deprecation when migration is practical; use a
   major version for incompatible removal and provide migration guidance.
7. Run the quickstart, clean regeneration, package, documentation, MSRV/stable, feature-matrix, and
   semver gates before merging.

To roll back, restore the prior manifest, specification, configuration, templates, normalization
script, and generated tree as one reviewed change. Regenerate using those immutable prior values;
never fetch a moving branch during rollback.

