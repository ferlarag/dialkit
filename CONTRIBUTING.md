# Contributing

Use Rust 1.85 or stable and run the commands in `specs/001-community-twilio-sdk/quickstart.md`.
Generated source must never be edited directly: update the pinned specification, configuration,
templates, or normalization script and run `codegen/regenerate.sh --update`. Every behavior fix
requires a focused regression test. Use fake credentials and loopback servers in ordinary tests.

Pull requests must explain public compatibility impact and confirm the dialkit constitution. Upstream
adoptions additionally require the diff classification and provenance steps in
`docs/maintainer/upstream-adoption.md`.

