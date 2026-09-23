use dialkit::{AccountSid, Client, IncomingPhoneNumberSid, MessagingServiceSid};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(
        AccountSid::new("AC00000000000000000000000000000000")?,
        "replace-with-a-secret",
    )?;
    let service = MessagingServiceSid::new("MG00000000000000000000000000000000")?;
    let phone_number = IncomingPhoneNumberSid::new("PN00000000000000000000000000000000")?;
    let senders = client.messaging_services().senders(service);
    senders.add_phone_number(&phone_number).await?;
    println!("added sender {}", phone_number.as_str());
    Ok(())
}
