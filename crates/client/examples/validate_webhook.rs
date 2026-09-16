use dialkit::{ValidationResult, WebhookValidator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let validator = WebhookValidator::new("read-the-auth-token-from-a-secret-store");
    let result = validator.validate_form(
        "https://example.com/twilio/webhook",
        &[(
            "CallSid".into(),
            "CA00000000000000000000000000000000".into(),
        )],
        "replace-with-X-Twilio-Signature",
    );
    match result {
        Ok(ValidationResult::Valid) => println!("authentic webhook"),
        Ok(ValidationResult::Invalid(reason)) => println!("invalid webhook: {reason:?}"),
        Ok(_) => println!("webhook validator returned a newer result variant"),
        Err(error) => println!("malformed webhook input: {error}"),
    }
    Ok(())
}
