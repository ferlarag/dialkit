use dialkit::{AccountSid, CallSid, Client, QueueSid};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(
        AccountSid::new("AC00000000000000000000000000000000")?,
        "replace-with-a-secret",
    )?;
    let queue = QueueSid::new("QU00000000000000000000000000000000")?;
    let call = CallSid::new("CA00000000000000000000000000000000")?;
    let members = client.queues().members(queue);
    members
        .redirect(&call, &Url::parse("https://example.test/queue-exit")?)
        .await?;
    println!("redirected queue member {}", call.as_str());
    Ok(())
}
