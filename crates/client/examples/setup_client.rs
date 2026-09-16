use dialkit::{AccountSid, Client};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let account_sid = AccountSid::new("AC00000000000000000000000000000000")?;
    let _client = Client::new(account_sid, "replace-with-a-secret")?;
    Ok(())
}
