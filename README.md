# Dialkit

[![Crates.io](https://img.shields.io/crates/v/dialkit.svg)](https://crates.io/crates/dialkit)
[![Documentation](https://docs.rs/dialkit/badge.svg)](https://docs.rs/dialkit)
[![CI](https://github.com/ferlarag/dialkit/actions/workflows/ci.yml/badge.svg)](https://github.com/ferlarag/dialkit/actions/workflows/ci.yml)

An async Rust SDK for Twilio. Dialkit provides a small, typed API for common workflows such as
sending messages, making calls, pagination, webhook validation, and TwiML.

> Community maintained. Not an official Twilio SDK.

[Crate](https://crates.io/crates/dialkit) · [API docs](https://docs.rs/dialkit) · [Releases](https://github.com/ferlarag/dialkit/releases)

## Install

```toml
[dependencies]
dialkit = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Dialkit requires Rust 1.85 or newer.

## Send a message

Set your credentials outside your source code:

```bash
export TWILIO_ACCOUNT_SID="AC..."
export TWILIO_AUTH_TOKEN="..."
```

```rust
use dialkit::{AccountSid, Client, CreateMessage, MessageSender, PhoneEndpoint};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(
        AccountSid::new(std::env::var("TWILIO_ACCOUNT_SID")?)?,
        std::env::var("TWILIO_AUTH_TOKEN")?,
    )?;

    let request = CreateMessage::new(
        MessageSender::phone(PhoneEndpoint::new("+15005550006")?),
        PhoneEndpoint::new("+15005550009")?,
    )
    .body("Hello from Dialkit")
    .build()?;

    let message = client.messages().create(request).await?;
    println!("{}: {}", message.sid().as_str(), message.status().as_str());

    Ok(())
}
```

## Make a call

Add `url = "2"` to your dependencies, then:

```rust
use dialkit::{AccountSid, CallInstructions, Client, CreateCall, PhoneEndpoint};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(
        AccountSid::new(std::env::var("TWILIO_ACCOUNT_SID")?)?,
        std::env::var("TWILIO_AUTH_TOKEN")?,
    )?;

    let request = CreateCall::new(
        PhoneEndpoint::new("+15005550009")?,
        PhoneEndpoint::new("+15005550006")?,
        CallInstructions::Url(Url::parse("https://example.com/voice.xml")?),
    )?;

    let call = client.calls().create(request).await?;
    println!("{}: {}", call.sid().as_str(), call.status().as_str());

    Ok(())
}
```

More examples are available in [`crates/client/examples`](crates/client/examples).

## Features

| Feature | Default | Purpose |
|---|---:|---|
| `rustls-tls` | Yes | TLS using Rustls |
| `native-tls` | No | TLS using the platform backend |
| `webhooks` | Yes | Validate Twilio webhook signatures |
| `twiml` | Yes | Build voice and messaging TwiML |

To use native TLS:

```toml
dialkit = { version = "0.1", default-features = false, features = ["native-tls", "webhooks", "twiml"] }
```

## What is in this repository?

- `dialkit` — stable, handwritten API intended for application code.
- `dialkit-api-generated` — complete API generated from the pinned Twilio OpenAPI specification.
- `dialkit-core` — shared authentication, HTTP, retry, error, and tracing behavior.

Credentials and sensitive request values are redacted from library-generated errors, debug output,
and traces. Read operations use conservative retries; ambiguous mutations are not retried unless
explicitly enabled.

## Development

```bash
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
bash codegen/regenerate.sh --check
```

Release and generation notes are in [`docs/maintainer`](docs/maintainer).

## License

[MIT](LICENSE)
