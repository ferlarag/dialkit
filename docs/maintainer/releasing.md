# Release checklist

Publish all crates at one version in this order: `dialkit-core`, `dialkit-api-generated`,
`dialkit-messaging-generated`, then `dialkit`. Each path dependency must also carry the release
version.

- Run clean regeneration and all CI commands on Rust 1.85 and stable.
- Run `cargo semver-checks -p dialkit` against the latest published facade, or record that the first
  unpublished release has no baseline.
- Inspect packaged file lists for licenses, READMEs, provenance, source inputs, and secrets.
- Record the specification commit/SHA-256, generator version/artifact SHA-256, template revision/hash,
  notable upstream changes, compatibility class, and migration guidance in the release notes.
- Dry-run packages in dependency order and verify no generated or Reqwest types enter facade APIs.
- Attach the operation coverage result, contract results, and constitution compliance review.
- Generate `specs/002-voice-sms-coverage/release-evidence.md` with
  `scripts/generate-release-evidence.sh`; zero unresolved entries or undocumented exceptions are
  permitted.

Missing provenance, an unexplained generated difference, a failed compatibility check, or an
untested corrected defect blocks publication.
