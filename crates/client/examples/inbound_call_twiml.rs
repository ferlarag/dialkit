use dialkit::twiml::{Gather, TwimlResponse};

fn main() -> Result<(), dialkit::Error> {
    let response = TwimlResponse::voice()
        .say("Welcome")
        .gather(
            Gather::new()
                .input("dtmf speech")
                .timeout(5)
                .say("Press 1 or say sales"),
        )
        .build()?;
    println!("{}", response.to_xml()?);
    Ok(())
}
