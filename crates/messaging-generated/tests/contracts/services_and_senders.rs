#[test]
fn services_and_sender_operation_contracts_are_evidenced() {
    super::verify_domains(&[
        "Alpha senders",
        "Channel senders",
        "Destination alpha senders",
        "Service phone-number senders",
        "Messaging Services",
        "Service short-code senders",
    ]);
}
