use dialkit::{AccountSid, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(
        AccountSid::new("AC00000000000000000000000000000000")?,
        "replace-with-a-secret",
    )?;
    let sip = client.sip();
    let domain = sip.create_domain("example.sip.twilio.com").await?;
    let credentials = sip.create_credential_list("production agents").await?;
    let credential = sip
        .create_credential(&credentials, "agent", "replace-with-a-secret")
        .await?;
    println!(
        "created SIP domain {}, credential list {}, and credential {}",
        domain.as_str(),
        credentials.as_str(),
        credential.as_str(),
    );
    Ok(())
}
