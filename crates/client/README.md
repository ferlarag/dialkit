# dialkit

`dialkit` is an asynchronous, community-maintained Rust SDK for Twilio. It offers a stable,
task-oriented facade for Calls, Messages, Voice controls, media, and Messaging Services. The
separately versioned `dialkit-api-generated` and `dialkit-messaging-generated` companions expose
the complete pinned API-2010 and Messaging-v1 surfaces without leaking generated types into the
stable facade.

```rust,no_run
use dialkit::{AccountSid, Client};

# fn example() -> Result<(), dialkit::Error> {
 # let account_sid = AccountSid::new("AC00000000000000000000000000000000")?;
  let client = Client::new(account_sid, "read-this-from-a-secret-store")?;
  let calls = client.calls();
  let conferences = client.conferences();
  let recordings = client.recordings();
  let queues = client.queues();
  let sip = client.sip();
  let messaging_services = client.messaging_services();
# let _ = calls;
# let _ = (conferences, recordings, queues, sip, messaging_services);
# Ok(())
# }
```

Credentials are redacted from `Debug`, errors, and library-generated traces. Rustls is the
default TLS backend. Native TLS is available with `--no-default-features --features native-tls`;
if dependency feature unification enables both backends, Rustls is selected deterministically.
The task index below links each supported workflow to a compiling example.

## Task-oriented workflow index

Each example uses synthetic account and resource identifiers. Set credentials from your own
secret store before using an example against Twilio. The examples use typed Dialkit methods,
verified webhook inputs, and TwiML builders; no hand-authored XML or direct HTTP is needed.

| Task | Start with | Compiling example | Synthetic input to replace |
|---|---|---|---|
| Create an outbound call | `Client::calls().create` with `CreateCall` | [make_call.rs](examples/make_call.rs) | `AC000…`, `+1555…`, callback URL |
| Respond to an inbound call with TwiML | `TwimlResponse::voice()` and `Gather` | [inbound_call_twiml.rs](examples/inbound_call_twiml.rs) | Prompt text and callback URL |
| Verify a call-status callback | `WebhookValidator::verify_form` and `WebhookFamily::CallProgress` | [verify_call_status.rs](examples/verify_call_status.rs) | Public callback URL, form pairs, signature |
| Add or update a conference participant | `Client::conferences().participants` | [conference_participant.rs](examples/conference_participant.rs) | `CF000…`, `CA000…` |
| Start and download a recording | `Client::recordings().start` and `.download` | [recording_download.rs](examples/recording_download.rs) | `CA000…`, recording format |
| Control a queue member | `Client::queues().members` | [queue_member.rs](examples/queue_member.rs) | `QU000…`, `CA000…` |
| Configure a SIP domain and credential | `Client::sip()` | [sip_setup.rs](examples/sip_setup.rs) | Synthetic domain and credential identifiers |
| Send an SMS | `Client::messages().create` with `CreateMessage` | [send_message.rs](examples/send_message.rs) | `+1555…` sender/recipient and sample body |
| Send MMS with media | `CreateMessage::media_url` | [send_mms.rs](examples/send_mms.rs) | Sample media URL and `+1555…` numbers |
| Parse an inbound message | `WebhookValidator::verify_form` and `parse_messaging` | [parse_inbound_message.rs](examples/parse_inbound_message.rs) | Public URL, received form pairs, signature |
| Verify a delivery-status callback | `WebhookFamily::MessageStatus` and `parse_messaging` | [verify_delivery_status.rs](examples/verify_delivery_status.rs) | Public URL, received form pairs, signature |
| Manage a Messaging Service sender | `Client::messaging_services().senders` | [manage_service_sender.rs](examples/manage_service_sender.rs) | `MG000…`, `PN000…` |
| Configure an application and phone number; inspect MMS media metadata | `Client::applications().configure`, `Client::phone_numbers().configure`, `Client::media().metadata` | [configure_voice_application.rs](examples/configure_voice_application.rs) | `AP000…`, `PN000…`, `SM000…`, `ME000…` |

For a list request, call the generated one-page `list_*` method and pass its response to
`pagination::follow(configuration, first_page)` for automatic traversal. The pager follows
Twilio's opaque continuation links, including reserved page tokens, and rejects cross-origin
links. The stable `Client::calls().list` example remains available in
[paginate_calls.rs](examples/paginate_calls.rs).

List operations use the same bounded `Pager<T>` abstraction. Continuation URLs from either
`next_page_uri` or Messaging v1's `meta.next_page_url` are treated as opaque and must remain on the
initiating endpoint origin. `ClientBuilder::base_url` continues to configure API-2010 only;
`messaging_base_url` is an independent override intended primarily for local contract tests.

Recording and MMS metadata remain typed JSON resources. Binary downloads are explicit and return
`MediaBody`, a transport-neutral byte stream with safe content type and length accessors. Redirects
are not followed automatically, preventing authenticated requests from crossing origins.

Generated companions are the exhaustive escape hatch for less-common operations. Their errors and
model shapes are generator-versioned; stable facade errors normalize status, Twilio code, request
ID, retry metadata, and safe service context.

## TwiML grammar

`TwimlResponse::voice()` accepts the response verbs `Connect`, `Dial`, `Echo`, `Enqueue`, `Gather`,
`Hangup`, `Leave`, `Pause`, `Pay`, `Play`, `Record`, `Redirect`, `Refer`, `Reject`, `Say`, `Start`,
and `Stop`. Typed builders restrict `Connect` to ConversationRelay/Room/Stream/VirtualAgent; `Dial`
to Application/Client/Conference/Number/Queue/Sip; `Gather` to Say/Play/Pause; `Pay` to Prompt and
Parameter; `Start`/`Stop` to Recording/Siprec/Stream/Transcription; and `Refer` to Sip. Messaging
responses accept `Message` (ordered Body and repeated Media children) and `Redirect`. Builders
preserve insertion order, escape text and attributes, validate required values and constrained
attributes, and provide no raw-XML escape hatch. `TwimlNodeKind::is_terminal` identifies Hangup,
Reject, and Redirect control transfers; the legacy ordered builder still renders later siblings for
backward compatibility even though Twilio will not execute them. Content-bearing debug output is
redacted.

For a new unidirectional media stream, use `Start::new().stream_to(name, websocket_url)` with a
`wss` URL without query parameters. To begin SIPREC, use
`Start::new().siprec_with_connector(name, connector_name)`. The older name-only
`Start::stream` and `Start::siprec` methods remain available for source/XML compatibility, but
omit information needed to establish those sessions. Fresh `TwimlNode` constructions require the
URL or connector under `Start`; `Stop` addresses a running session by name.
For payment prompts, use `Pay::prompt_for(step, text)` to include the required payment step;
the older `Pay::prompt(text)` remains for XML compatibility. `Gather::timeout_auto()` expresses
Twilio's automatic speech timeout without constructing raw attributes.

See `examples/inbound_call_twiml.rs` for a compiling inbound-call response.

## Webhook verification and parsing

Always pass the exact externally visible URL plus the original ordered form pairs or raw JSON bytes
to `WebhookValidator`. Select a `WebhookFamily` explicitly, call `verify_get` for GET/query,
`verify_form` for form POST, or `verify_json` for raw JSON, and
only then invoke the matching Voice, Voice-control, realtime, or Messaging parser. This avoids
ambiguous field-based family inference. Form extras preserve duplicates and empty values; JSON
properties remain structured; service-controlled status/event strings use `OpenValue` and retain
unknown values. Parsed events make no replay, idempotency, ordering, routing, or persistence
guarantee. Applications behind proxies must reconstruct the public URL from trusted configuration,
not untrusted forwarding headers. Debug and parse errors omit signatures, bodies, identifiers,
media URLs, payment values, and unknown-field bags.
GET signatures cover the exact public URL bytes, including query ordering and encoding; query
pairs are decoded only after verification and retain repeats and empty values for parsing.

For JSON-capable Stream/SIPREC, Transcription, User-Defined Message, and Message Status callbacks,
call `VerifiedJsonWebhook::parse_family_specific()` after verification for named fields and
structured extra properties. Unsupported families return `None`; `parse_typed()` remains available
for a generic verified projection.
For Message Status JSON, `MessageStatusJsonWebhook::channel_data()` returns the structured
`ChannelData` object. Verification rejects a present non-object `ChannelData` value; unknown
status strings and other future properties remain accessible.

This project is not an official Twilio SDK and does not claim Twilio endorsement.
