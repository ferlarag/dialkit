use dialkit::{AccountSid, Client, CreateMessage, MessageSender, PhoneEndpoint};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(
        AccountSid::new("AC00000000000000000000000000000000")?,
        "replace-with-a-secret",
    )?;
    let request = CreateMessage::new(
        MessageSender::phone(PhoneEndpoint::new("+15005550006")?),
        PhoneEndpoint::new("+15005550009")?,
    )
    .body("Hello from dialkit")
    .build()?;
    let message = client.messages().create(request).await?;
    println!("created message with status {}", message.status().as_str());
    Ok(())
}
