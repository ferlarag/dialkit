#[test]
fn voice_control_operation_contracts_are_evidenced() {
    super::verify_domains(&[
        "Call recordings",
        "Conferences",
        "Conference recordings",
        "Conference participants",
        "Voice payments",
        "Queues",
        "Queue members",
        "Recordings",
        "Recording add-on results",
        "Recording add-on payloads",
        "Recording add-on payload data",
        "Recording transcriptions",
        "Account transcriptions",
    ]);
}
