use dialkit::{WebhookFamily, WebhookValidator};

fn main() -> Result<(), dialkit::Error> {
    let parameters = vec![
        (
            "CallSid".to_owned(),
            "CA00000000000000000000000000000000".to_owned(),
        ),
        ("CallStatus".to_owned(), "completed".to_owned()),
    ];
    let validator = WebhookValidator::new("read-auth-token-from-secret-store");
    let verified = validator.verify_form(
        WebhookFamily::CallProgress,
        "https://public.example.test/call-status",
        &parameters,
        "X-Twilio-Signature header value",
    )?;
    let event = verified
        .parse_voice()
        .map_err(|error| dialkit::Error::WebhookValidation(error.to_string()))?;
    println!(
        "status={}",
        event.status().map_or("unknown", |value| value.as_str())
    );
    Ok(())
}
