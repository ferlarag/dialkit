#[test]
fn api_2010_messaging_operation_contracts_are_evidenced() {
    super::verify_domains(&[
        "Messages",
        "Message feedback",
        "Message media collection",
        "Message media instances",
        "Account SMS short codes",
    ]);
}
