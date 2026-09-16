use dialkit::{AccountSid, CallInstructions, Client, CreateCall, PhoneEndpoint};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(
        AccountSid::new("AC00000000000000000000000000000000")?,
        "replace-with-a-secret",
    )?;
    let request = CreateCall::new(
        PhoneEndpoint::new("+15005550006")?,
        PhoneEndpoint::new("+15005550009")?,
        CallInstructions::Url(Url::parse("https://example.com/voice")?),
    )?;
    let call = client.calls().create(request).await?;
    println!("created call with status {}", call.status().as_str());
    Ok(())
}
