use dialkit::{AccountSid, Client, ListCalls};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(
        AccountSid::new("AC00000000000000000000000000000000")?,
        "replace-with-a-secret",
    )?;
    let mut calls = client.calls().list(ListCalls::new().page_size(50));
    while let Some(call) = calls.next().await {
        println!("{}", call?.sid().as_str());
    }
    Ok(())
}
