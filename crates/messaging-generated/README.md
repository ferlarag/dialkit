# dialkit-messaging-generated

This crate exposes every operation and model from the Twilio Messaging v1 specification pinned by
dialkit. It is generated code: names and model shapes can change when a reviewed upstream revision
is adopted. Application code should prefer the stable `dialkit` facade for common workflows.

Use exactly the same version as `dialkit` and `dialkit-core`:

```toml
[dependencies]
dialkit = "0.2.0"
dialkit-messaging-generated = "0.2.0"
```

The generated `apis::configuration::Configuration` accepts a
`dialkit_core::request::HttpClient`, so operations use the shared credential, HTTPS, timeout,
retry, redaction, and tracing policies. The default Messaging endpoint is
`https://messaging.twilio.com`; local tests may supply an explicit endpoint profile.

Generated string enums retain unknown values. Generation provenance is included in
`PROVENANCE.md`. Do not edit `src/` or `docs/` directly; update a pinned input or template and run
`codegen/regenerate.sh`.

All 58 pinned operations are generated. This includes Messaging Services and alpha, channel,
destination-alpha, phone-number, and short-code senders; A2P brand/campaign resources; toll-free
verification and deactivation resources; and link-shortening domains, certificates, DNS validation,
managed certificates, and service/domain associations. The source-qualified evidence inventory is
`codegen/coverage/rest-messaging-v1.toml`.
