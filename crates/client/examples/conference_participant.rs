use dialkit::{AccountSid, Client, ConferenceSid, ParticipantCallSid, ParticipantUpdate};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(
        AccountSid::new("AC00000000000000000000000000000000")?,
        "replace-with-a-secret",
    )?;
    let conference = ConferenceSid::new("CF00000000000000000000000000000000")?;
    let call = ParticipantCallSid::new("CA00000000000000000000000000000000")?;
    let participants = client.conferences().participants(conference);
    participants
        .update(&call, ParticipantUpdate::new().muted(false).hold(false))
        .await?;
    println!("updated conference participant {}", call.as_str());
    Ok(())
}
