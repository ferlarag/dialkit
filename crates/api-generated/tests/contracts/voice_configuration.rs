#[test]
fn voice_configuration_operation_contracts_are_evidenced() {
    super::verify_domains(&[
        "SIP call credential-list auth mappings",
        "SIP call IP-ACL auth mappings",
        "SIP registration credential-list auth mappings",
        "SIP credentials",
        "SIP credential lists",
        "SIP domain credential-list mappings",
        "SIP domains",
        "SIP IP access-control lists",
        "SIP domain IP-ACL mappings",
        "SIP IP addresses",
        "SIPREC sessions",
        "Outgoing caller IDs",
        "Caller-ID validation",
        "Incoming phone numbers",
        "Local incoming phone numbers",
        "Mobile incoming phone numbers",
        "Toll-free incoming phone numbers",
    ]);
}
