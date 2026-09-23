use dialkit::{
    AccountSid, ApplicationSid, Client, ConferenceSid, CredentialListSid, IncomingPhoneNumberSid,
    IpAccessControlListSid, MessageMediaSid, MessagingServiceSid, ParticipantCallSid, QueueSid,
    RecordingSid, SipDomainSid, TranscriptionSid,
};

#[test]
fn common_domain_entry_points_and_typed_identifiers_compile() {
    let client = Client::new(
        AccountSid::new("AC00000000000000000000000000000000").unwrap(),
        "synthetic-domain-token",
    )
    .unwrap();
    let _ = client.applications();
    let _ = client.conferences();
    let _ = client.recordings();
    let _ = client.queues();
    let _ = client.sip();
    let _ = client.phone_numbers();
    let _ = client.media();
    let _ = client.messaging_services();

    assert!(ApplicationSid::new("AP00000000000000000000000000000000").is_ok());
    assert!(ConferenceSid::new("CF00000000000000000000000000000000").is_ok());
    assert!(ParticipantCallSid::new("CA00000000000000000000000000000000").is_ok());
    assert!(RecordingSid::new("RE00000000000000000000000000000000").is_ok());
    assert!(TranscriptionSid::new("TR00000000000000000000000000000000").is_ok());
    assert!(QueueSid::new("QU00000000000000000000000000000000").is_ok());
    assert!(SipDomainSid::new("SD00000000000000000000000000000000").is_ok());
    assert!(CredentialListSid::new("CL00000000000000000000000000000000").is_ok());
    assert!(IpAccessControlListSid::new("AL00000000000000000000000000000000").is_ok());
    assert!(IncomingPhoneNumberSid::new("PN00000000000000000000000000000000").is_ok());
    assert!(MessageMediaSid::new("ME00000000000000000000000000000000").is_ok());
    assert!(MessagingServiceSid::new("MG00000000000000000000000000000000").is_ok());
}
