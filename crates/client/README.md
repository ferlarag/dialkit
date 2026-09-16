# dialkit

`dialkit` is an asynchronous, community-maintained Rust SDK for Twilio. It offers a stable,
task-oriented facade for Calls and Messages while the separately versioned
`dialkit-api-generated` crate exposes the complete pinned api_v2010 surface.

```rust,no_run
use dialkit::{AccountSid, Client};

# fn example() -> Result<(), dialkit::Error> {
let account_sid = AccountSid::new("AC00000000000000000000000000000000")?;
let client = Client::new(account_sid, "read-this-from-a-secret-store")?;
let calls = client.calls();
# let _ = calls;
# Ok(())
# }
```

Credentials are redacted from `Debug`, errors, and library-generated traces. Rustls is the
default TLS backend. Native TLS is available with `--no-default-features --features native-tls`;
if dependency feature unification enables both backends, Rustls is selected deterministically.
See the compiling examples for call creation, message sending, pagination,
error handling, webhook validation, and TwiML.

This project is not an official Twilio SDK and does not claim Twilio endorsement.
