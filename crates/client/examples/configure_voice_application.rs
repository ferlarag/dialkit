use dialkit::{
    AccountSid, ApplicationSid, Client, ConfigureApplication, ConfigurePhoneNumber,
    IncomingPhoneNumberSid, MessageMediaRef, MessageMediaSid, MessageSid,
};
use url::Url;

async fn workflow(client: &Client) -> Result<(), dialkit::Error> {
    let application = ApplicationSid::new("AP00000000000000000000000000000000")?;
    let number = IncomingPhoneNumberSid::new("PN00000000000000000000000000000000")?;
    client
        .applications()
        .configure(
            &application,
            ConfigureApplication::new()
                .voice_url(Url::parse("https://example.invalid/voice").unwrap()),
        )
        .await?;
    client
        .phone_numbers()
        .configure(
            &number,
            ConfigurePhoneNumber::new().voice_application(application),
        )
        .await?;
    let media = MessageMediaRef::new(
        MessageSid::new("SM00000000000000000000000000000000")?,
        MessageMediaSid::new("ME00000000000000000000000000000000")?,
    );
    let metadata = client.media().metadata(&media).await?;
    println!("media content type: {}", metadata.content_type());
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(
        AccountSid::new("AC00000000000000000000000000000000")?,
        "synthetic-token",
    )?;
    // This compiles without contacting Twilio; invoke workflow only with configured credentials.
    let _ = (client, workflow);
    Ok(())
}
