#[test]
fn voice_core_operation_contracts_are_evidenced() {
    super::verify_domains(&[
        "Voice applications",
        "Calls",
        "Call events",
        "Call notifications",
        "Real-time call transcriptions",
        "Call media streams",
        "Voice client tokens",
        "Call user-defined messages",
        "Call user-defined message subscriptions",
    ]);
}
