use dialkit::Error;

fn report(error: &Error) {
    match error {
        Error::Api(api) => eprintln!(
            "Twilio rejected the request: code={:?}, request_id={:?}, message={}",
            api.code(),
            api.request_id(),
            api.message()
        ),
        Error::RateLimited { retry_after, .. } => {
            eprintln!("rate limited; retry after {retry_after:?}")
        }
        other => eprintln!("request failed: {other}"),
    }
}

fn main() {
    report(&Error::Validation("example invalid input".into()));
}
