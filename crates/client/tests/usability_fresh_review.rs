//! First-attempt API usability review. These ten workflows were drafted from
//! the README and generated public rustdoc, before the first compile check.
//! They deliberately make no network calls when the test target is run.

#[allow(dead_code)]
mod workflow_01_send_sms {
    use dialkit::{Client, CreateMessage, MessageSender, PhoneEndpoint};

    pub async fn send(
        client: &Client,
        from: &str,
        to: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let request = CreateMessage::new(
            MessageSender::phone(PhoneEndpoint::new(from)?),
            PhoneEndpoint::new(to)?,
        )
        .body("Your appointment is confirmed")
        .build()?;
        let message = client.messages().create(request).await?;
        Ok(message.sid().as_str().to_owned())
    }
}

#[allow(dead_code)]
mod workflow_02_send_mms {
    use dialkit::{Client, CreateMessage, MessageSender, PhoneEndpoint};
    use url::Url;

    pub async fn send(
        client: &Client,
        from: &str,
        to: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let request = CreateMessage::new(
            MessageSender::phone(PhoneEndpoint::new(from)?),
            PhoneEndpoint::new(to)?,
        )
        .body("Here is your receipt")
        .media_url(Url::parse("https://example.com/receipt.png")?)
        .build()?;
        let message = client.messages().create(request).await?;
        Ok(message.sid().as_str().to_owned())
    }
}

#[allow(dead_code)]
mod workflow_03_send_from_messaging_service {
    use dialkit::{Client, CreateMessage, MessageSender, PhoneEndpoint};

    pub async fn send(
        client: &Client,
        service_sid: &str,
        to: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sender = MessageSender::messaging_service(service_sid)?;
        let request = CreateMessage::new(sender, PhoneEndpoint::new(to)?)
            .body("Your order has shipped")
            .build()?;
        let message = client.messages().create(request).await?;
        Ok(message.sid().as_str().to_owned())
    }
}

#[allow(dead_code)]
mod workflow_04_check_message_status {
    use dialkit::{Client, MessageSid};

    pub async fn check(client: &Client, sid: &str) -> Result<String, Box<dyn std::error::Error>> {
        let sid = MessageSid::new(sid)?;
        let message = client.messages().get(&sid).await?;
        Ok(message.status().as_str().to_owned())
    }
}

#[allow(dead_code)]
mod workflow_05_list_messages {
    use dialkit::{Client, ListMessages};

    pub async fn first_page_sids(
        client: &Client,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut pager = client.messages().list(ListMessages::new().page_size(20));
        let mut sids = Vec::new();
        while let Some(message) = pager.next().await {
            sids.push(message?.sid().as_str().to_owned());
            if sids.len() == 20 {
                break;
            }
        }
        Ok(sids)
    }
}

#[allow(dead_code)]
mod workflow_06_place_call_with_url {
    use dialkit::{CallInstructions, Client, CreateCall, PhoneEndpoint};
    use url::Url;

    pub async fn place(
        client: &Client,
        from: &str,
        to: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let request = CreateCall::new(
            PhoneEndpoint::new(from)?,
            PhoneEndpoint::new(to)?,
            CallInstructions::Url(Url::parse("https://example.com/voice.xml")?),
        )?;
        let call = client.calls().create(request).await?;
        Ok(call.sid().as_str().to_owned())
    }
}

#[allow(dead_code)]
mod workflow_07_place_call_with_inline_twiml {
    use dialkit::twiml::TwimlResponse;
    use dialkit::{CallInstructions, Client, CreateCall, PhoneEndpoint};

    pub async fn place(
        client: &Client,
        from: &str,
        to: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let instructions = TwimlResponse::voice()
            .say("Your delivery is arriving today")
            .pause(2)
            .hangup()
            .build()?;
        let request = CreateCall::new(
            PhoneEndpoint::new(from)?,
            PhoneEndpoint::new(to)?,
            CallInstructions::Twiml(instructions),
        )?;
        let call = client.calls().create(request).await?;
        Ok(call.sid().as_str().to_owned())
    }
}

#[allow(dead_code)]
mod workflow_08_list_completed_calls {
    use dialkit::{Client, ListCalls};

    pub async fn recent_sids(client: &Client) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut pager = client
            .calls()
            .list(ListCalls::new().status("completed").page_size(25));
        let mut sids = Vec::new();
        while let Some(call) = pager.next().await {
            sids.push(call?.sid().as_str().to_owned());
            if sids.len() == 25 {
                break;
            }
        }
        Ok(sids)
    }
}

#[allow(dead_code)]
mod workflow_09_record_and_download_call {
    use dialkit::voice::RecordingFormat;
    use dialkit::{CallSid, Client};

    pub async fn start_and_download(
        client: &Client,
        call_sid: &str,
    ) -> Result<Option<u64>, Box<dyn std::error::Error>> {
        let call_sid = CallSid::new(call_sid)?;
        let recording_sid = client.recordings().start(&call_sid).await?;
        let recording = client
            .recordings()
            .download(&recording_sid, RecordingFormat::Mp3)
            .await?;
        Ok(recording.content_length())
    }
}

#[allow(dead_code)]
mod workflow_10_verify_incoming_sms {
    use dialkit::{WebhookFamily, WebhookValidator};

    pub fn incoming_body(
        auth_token: &str,
        public_url: &str,
        parameters: &[(String, String)],
        signature: &str,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let verified = WebhookValidator::new(auth_token).verify_form(
            WebhookFamily::IncomingMessage,
            public_url,
            parameters,
            signature,
        )?;
        let event = verified.parse_messaging()?;
        Ok(event.body().map(str::to_owned))
    }
}
