use dialkit::{WebhookFamily, WebhookValidator};

fn main() -> Result<(), dialkit::Error> {
    let parameters = vec![
        (
            "MessageSid".to_owned(),
            "SM00000000000000000000000000000000".to_owned(),
        ),
        ("MessageStatus".to_owned(), "delivered".to_owned()),
    ];
    let verified = WebhookValidator::new("read-auth-token-from-secret-store").verify_form(
        WebhookFamily::MessageStatus,
        "https://public.example.test/message-status",
        &parameters,
        "X-Twilio-Signature header value",
    )?;
    let event = verified
        .parse_messaging()
        .map_err(|error| dialkit::Error::WebhookValidation(error.to_string()))?;
    println!(
        "status={}",
        event.status().map_or("unknown", |value| value.as_str())
    );
    Ok(())
}
