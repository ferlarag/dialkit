use dialkit::twiml::{Gather, TwimlResponse};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = TwimlResponse::voice()
        .say("Welcome to dialkit")
        .gather(Gather::new().say("Press 1 for support"))
        .build()?;
    println!("{}", response.to_xml()?);
    Ok(())
}
