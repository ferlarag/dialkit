#[test]
fn compliance_and_link_operation_contracts_are_evidenced() {
    super::verify_domains(&[
        "A2P brand registrations",
        "A2P brand registration OTP",
        "A2P brand vettings",
        "Deactivations",
        "Link-shortening domain certificates",
        "Link-shortening domain configuration",
        "Service domain configuration",
        "Link-shortening DNS validation",
        "Externally registered US A2P campaigns",
        "Service/domain link-shortening association",
        "Service link-shortening domain lookup",
        "Managed certificate requests",
        "Toll-free verifications",
        "US A2P campaigns",
        "US A2P campaign use case",
        "Messaging Service use cases",
    ]);
}
