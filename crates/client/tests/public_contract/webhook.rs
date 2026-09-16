use dialkit::{InvalidReason, PortCompatibility, ValidationResult, WebhookValidator};

#[test]
fn validates_form_signatures_and_distinguishes_malformed_input() {
    let vector: serde_json::Value =
        serde_json::from_str(include_str!("../webhook_vectors/form.json")).unwrap();
    let params = vector["parameters"]
        .as_array()
        .unwrap()
        .iter()
        .map(|pair| {
            (
                pair[0].as_str().unwrap().to_owned(),
                pair[1].as_str().unwrap().to_owned(),
            )
        })
        .collect::<Vec<_>>();
    let validator = WebhookValidator::new(vector["token"].as_str().unwrap());
    assert_eq!(
        validator
            .validate_form(
                vector["url"].as_str().unwrap(),
                &params,
                vector["signature"].as_str().unwrap()
            )
            .unwrap(),
        ValidationResult::Valid
    );
    assert_eq!(
        validator
            .validate_form(
                vector["url"].as_str().unwrap(),
                &params,
                "AAAAAAAAAAAAAAAAAAAAAAAAAAAA"
            )
            .unwrap(),
        ValidationResult::Invalid(InvalidReason::SignatureMismatch)
    );
    assert!(
        validator
            .validate_form(vector["url"].as_str().unwrap(), &params, "not base64")
            .is_err()
    );
}

#[test]
fn validates_raw_json_digest_and_signature() {
    let vector: serde_json::Value =
        serde_json::from_str(include_str!("../webhook_vectors/json.json")).unwrap();
    let validator = WebhookValidator::new(vector["token"].as_str().unwrap())
        .port_compatibility(PortCompatibility::Strict);
    let body = vector["body"].as_str().unwrap().as_bytes();
    assert_eq!(
        validator
            .validate_json(
                vector["url"].as_str().unwrap(),
                body,
                vector["signature"].as_str().unwrap()
            )
            .unwrap(),
        ValidationResult::Valid
    );
    assert_eq!(
        validator
            .validate_json(
                vector["url"].as_str().unwrap(),
                b"altered",
                vector["signature"].as_str().unwrap()
            )
            .unwrap(),
        ValidationResult::Invalid(InvalidReason::BodyDigestMismatch)
    );
}

#[test]
fn repeated_form_names_and_unicode_participate_in_canonicalization() {
    let validator = WebhookValidator::new("12345");
    let params = vec![
        ("Name".to_owned(), "B".to_owned()),
        ("Note".to_owned(), "✓".to_owned()),
        ("Name".to_owned(), "A".to_owned()),
    ];
    assert_eq!(
        validator
            .validate_form(
                "https://example.com/hook",
                &params,
                "v5oaanNcefafPqDHgC8/vNsLnV4="
            )
            .unwrap(),
        ValidationResult::Valid
    );
}

#[test]
fn standard_port_compatibility_is_explicit_opt_in() {
    let params = vec![("CallSid".to_owned(), "CA123".to_owned())];
    let strict = WebhookValidator::new("12345");
    assert_eq!(
        strict
            .validate_form(
                "https://example.com:443/hook",
                &params,
                "cwV1UJdJpOhRyrfhJy9lqYFqWU8="
            )
            .unwrap(),
        ValidationResult::Invalid(InvalidReason::SignatureMismatch)
    );
    let compatible = WebhookValidator::new("12345")
        .port_compatibility(PortCompatibility::AddOrRemoveStandardPort);
    assert_eq!(
        compatible
            .validate_form(
                "https://example.com:443/hook",
                &params,
                "cwV1UJdJpOhRyrfhJy9lqYFqWU8="
            )
            .unwrap(),
        ValidationResult::Valid
    );
}
