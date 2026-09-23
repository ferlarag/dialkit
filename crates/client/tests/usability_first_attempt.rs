//! Ten original, compile-only workflow constructions for the SC-010 review.
//! None of these functions is executed by this test target.

use dialkit::twiml::TwimlResponse;
use dialkit::{
    CallInstructions, CallSid, Client, ConferenceSid, CreateCall, CreateMessage,
    IncomingPhoneNumberSid, MessageSender, MessagingServiceSid, ParticipantCallSid,
    ParticipantUpdate, PhoneEndpoint, QueueSid, RecordingFormat, RecordingSid, WebhookFamily,
    WebhookValidator,
};
use url::Url;

type Outcome = Result<(), Box<dyn std::error::Error>>;

async fn task_01_outbound_call(client: &Client) -> Outcome {
    let request = CreateCall::new(
        PhoneEndpoint::new("+15005550006")?,
        PhoneEndpoint::new("+15005550009")?,
        CallInstructions::Url(Url::parse("https://example.invalid/answer")?),
    )?;
    client.calls().create(request).await?;
    Ok(())
}

fn task_02_inbound_twiml() -> Outcome {
    let response = TwimlResponse::voice()
        .say("Hello from the example")
        .build()?;
    let _xml = response.to_xml()?;
    Ok(())
}

fn task_03_call_status(url: &str, pairs: &[(String, String)], signature: &str) -> Outcome {
    let verified = WebhookValidator::new("synthetic-token").verify_form(
        WebhookFamily::CallProgress,
        url,
        pairs,
        signature,
    )?;
    let _status = verified.parse_family_specific();
    Ok(())
}

async fn task_04_conference_participant(client: &Client) -> Outcome {
    let conference = ConferenceSid::new("CF00000000000000000000000000000000")?;
    let call = ParticipantCallSid::new("CA00000000000000000000000000000000")?;
    client
        .conferences()
        .participants(conference)
        .update(&call, ParticipantUpdate::new().muted(true))
        .await?;
    Ok(())
}

async fn task_05_recording_download(client: &Client) -> Outcome {
    let recording = RecordingSid::new("RE00000000000000000000000000000000")?;
    let _body = client
        .recordings()
        .download(&recording, RecordingFormat::Mp3)
        .await?;
    Ok(())
}

async fn task_06_queue_member(client: &Client) -> Outcome {
    let queue = QueueSid::new("QU00000000000000000000000000000000")?;
    let call = CallSid::new("CA00000000000000000000000000000000")?;
    client
        .queues()
        .members(queue)
        .redirect(&call, &Url::parse("https://example.invalid/next")?)
        .await?;
    Ok(())
}

async fn task_07_sip_setup(client: &Client) -> Outcome {
    let sip = client.sip();
    let domain = sip.create_domain("example.sip.invalid").await?;
    let list = sip.create_credential_list("Synthetic agents").await?;
    let _credential = sip
        .create_credential(&list, "agent", "synthetic-password")
        .await?;
    let _ = domain;
    Ok(())
}

async fn task_08_sms(client: &Client) -> Outcome {
    let request = CreateMessage::new(
        MessageSender::phone(PhoneEndpoint::new("+15005550006")?),
        PhoneEndpoint::new("+15005550009")?,
    )
    .body("synthetic message")
    .build()?;
    client.messages().create(request).await?;
    Ok(())
}

async fn task_09_mms(client: &Client) -> Outcome {
    let request = CreateMessage::new(
        MessageSender::phone(PhoneEndpoint::new("+15005550006")?),
        PhoneEndpoint::new("+15005550009")?,
    )
    .media_url(Url::parse("https://example.invalid/image.png")?)
    .build()?;
    client.messages().create(request).await?;
    Ok(())
}

async fn task_10_service_sender(client: &Client) -> Outcome {
    let service = MessagingServiceSid::new("MG00000000000000000000000000000000")?;
    let number = IncomingPhoneNumberSid::new("PN00000000000000000000000000000000")?;
    client
        .messaging_services()
        .senders(service)
        .add_phone_number(&number)
        .await?;
    Ok(())
}

#[test]
fn all_ten_workflows_are_type_checked_without_execution() {
    let _ = (
        task_01_outbound_call,
        task_02_inbound_twiml,
        task_03_call_status,
        task_04_conference_participant,
        task_05_recording_download,
        task_06_queue_member,
        task_07_sip_setup,
        task_08_sms,
        task_09_mms,
        task_10_service_sender,
    );
}
