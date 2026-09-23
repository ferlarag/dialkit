use dialkit::{AccountSid, CallSid, Client, RecordingFormat};
use futures_util::StreamExt as _;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(
        AccountSid::new("AC00000000000000000000000000000000")?,
        "replace-with-a-secret",
    )?;
    let call = CallSid::new("CA00000000000000000000000000000000")?;
    let sid = client.recordings().start(&call).await?;
    let mut recording = client
        .recordings()
        .download(&sid, RecordingFormat::Mp3)
        .await?;
    while let Some(chunk) = recording.next().await {
        println!("received {} bytes", chunk?.len());
    }
    Ok(())
}
