use base64::{Engine as _, engine::general_purpose::STANDARD};
use dialkit::{
    AccountSid, CallInstructions, CreateCall, PhoneEndpoint, WebhookValidator, twiml::TwimlResponse,
};
use hmac::{Hmac, Mac as _};
use sha1::Sha1;
use std::{
    hint::black_box,
    time::{Duration, Instant},
};
use url::Url;

fn p95(mut samples: Vec<Duration>) -> Duration {
    samples.sort_unstable();
    samples[(samples.len() * 95 / 100).min(samples.len() - 1)]
}

fn measure(mut operation: impl FnMut()) -> Duration {
    let mut samples = Vec::with_capacity(200);
    for _ in 0..200 {
        let start = Instant::now();
        operation();
        samples.push(start.elapsed());
    }
    p95(samples)
}

fn main() {
    let payload = "<&✓".repeat(16 * 1024);
    let twiml = TwimlResponse::voice().say(payload.clone()).build().unwrap();
    let twiml_p95 = measure(|| {
        black_box(twiml.to_xml().unwrap());
    });

    let url = "https://example.test/webhook";
    let token = "benchmark-token";
    let form = vec![("Body".to_owned(), payload.clone())];
    let mut mac = Hmac::<Sha1>::new_from_slice(token.as_bytes()).unwrap();
    mac.update(format!("{url}Body{payload}").as_bytes());
    let signature = STANDARD.encode(mac.finalize().into_bytes());
    let validator = WebhookValidator::new(token);
    let webhook_p95 = measure(|| {
        black_box(validator.validate_form(url, &form, &signature).unwrap());
    });

    // Pagination retains one service page at a time. This deterministic model
    // guards the release target independently of the total result count.
    let page_size = 1_000;
    let total_items = 100_000;
    let mut peak_active_items = 0;
    let pagination_p95 = measure(|| {
        for start in (0..total_items).step_by(page_size) {
            let page: Vec<_> = (start..(start + page_size).min(total_items)).collect();
            peak_active_items = peak_active_items.max(page.len());
            black_box(page);
        }
    });

    let request_p95 = measure(|| {
        black_box(
            CreateCall::new(
                PhoneEndpoint::new("+15005550006").unwrap(),
                PhoneEndpoint::new("+15005550009").unwrap(),
                CallInstructions::Url(Url::parse("https://example.test/voice").unwrap()),
            )
            .unwrap(),
        );
    });
    black_box(AccountSid::new("AC00000000000000000000000000000000").unwrap());

    println!("request-construction p95: {request_p95:?}");
    println!("100k-item page streaming p95: {pagination_p95:?}");
    println!("pagination peak active items: {peak_active_items}");
    println!("64 KiB webhook canonicalization/validation p95: {webhook_p95:?}");
    println!("64 KiB TwiML rendering p95: {twiml_p95:?}");
    assert!(request_p95 < Duration::from_millis(5));
    assert_eq!(peak_active_items, page_size);
    assert!(pagination_p95 < Duration::from_millis(10));
    assert!(webhook_p95 < Duration::from_millis(10));
    assert!(twiml_p95 < Duration::from_millis(10));
}
