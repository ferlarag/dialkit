# dialkit-api-generated

This crate exposes every operation and model from the Twilio api_v2010 specification pinned by
dialkit. It is generated code: names and model shapes can change when a reviewed upstream revision
is adopted. Application code should prefer the stable `dialkit` facade for Calls and Messages.

Use exactly the same version as `dialkit` and `dialkit-core`:

```toml
[dependencies]
dialkit = "0.1.0"
dialkit-api-generated = "0.1.0"
```

The generated `apis::configuration::Configuration` provides access to operations that are not yet
wrapped by the facade. It accepts a `dialkit_core::request::HttpClient`, so generated operations use
the same protected credentials, HTTPS enforcement, connection pool, timeouts, retry policy, error
normalization, and tracing as the facade:

```rust,no_run
use dialkit_api_generated::apis::configuration::Configuration;
use dialkit_core::{
    auth::{AccountSid, Credentials},
    request::{ClientConfiguration, HttpClient},
    retry::RetryPolicy,
};
use secrecy::SecretString;
use std::time::Duration;
use url::Url;

# fn example() -> Result<(), dialkit_core::Error> {
let transport = HttpClient::new(ClientConfiguration {
    credentials: Credentials::account_token(
        AccountSid::new("AC00000000000000000000000000000000").expect("valid account SID"),
        SecretString::from("read-from-a-secret-store".to_owned()),
    ),
    base_url: Url::parse("https://api.twilio.com/").expect("constant URL"),
    connect_timeout: Duration::from_secs(10),
    request_timeout: Duration::from_secs(30),
    retry_policy: RetryPolicy::conservative(),
    allow_http_for_tests: false,
})?;
let generated = Configuration::new(transport);
# let _ = generated;
# Ok(())
# }
```

Generated configuration debug output is redacted, and generated string enums retain unknown values.

Generation provenance is included in `PROVENANCE.md`. Do not edit `src/` directly; update a pinned
input or template and run `codegen/regenerate.sh`.
