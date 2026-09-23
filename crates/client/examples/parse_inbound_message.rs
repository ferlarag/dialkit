use dialkit::{WebhookFamily, WebhookValidator};

fn main() -> Result<(), dialkit::Error> {
    let parameters = vec![
        (
            "MessageSid".to_owned(),
            "SM00000000000000000000000000000000".to_owned(),
        ),
        ("Body".to_owned(), "synthetic example".to_owned()),
        ("NumMedia".to_owned(), "0".to_owned()),
    ];
    let verified = WebhookValidator::new("read-auth-token-from-secret-store").verify_form(
        WebhookFamily::IncomingMessage,
        "https://public.example.test/inbound-message",
        &parameters,
        "X-Twilio-Signature header value",
    )?;
    let event = verified
        .parse_messaging()
        .map_err(|error| dialkit::Error::WebhookValidation(error.to_string()))?;
    println!(
        "message={} media={}",
        event.message_sid(),
        event.media().len()
    );
    Ok(())
}
