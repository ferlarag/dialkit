# TwiML constraint inventory

This certifies the finite-value, range, required-value, and cross-field rules
for the 2026-09-17 node/attribute snapshot in `twiml-nodes.toml`. The
manifest-wide attribute-scope contract and selector-addressable negative
fixtures are in `crates/client/tests/contracts/twiml_manifest.rs` and
`crates/client/tests/contracts/twiml_validation.rs`.

| Rule | Nodes/attributes | Enforced value or boundary |
|---|---|---|
| HTTP callback method | `method`, `*CallbackMethod`, `waitUrlMethod` | `GET` or `POST` |
| Required content | Text-bearing nouns, parent verbs, `Parameter` name/value | Non-empty required value |
| Required connection configuration | `ConversationRelay.url`, `Connect/Stream.url`, `VirtualAgent.connectorName`, `Start/Stream.url`, `Start/Siprec.connectorName`, `Stop` noun `name`, `Pay/Prompt.for` | Required on fresh typed nodes; legacy builders are noted below |
| Attribute scope and uniqueness | Every node in `twiml-nodes.toml` | Fresh typed nodes accept exactly their declared attributes, each at most once |
| Input choice | `Gather.input` | `dtmf`, `speech`, `dtmf speech` |
| Rejection reason | `Reject.reason` | `rejected`, `busy` |
| Positive integer | `timeout`, `timeLimit`, `maxSpeechTime`, `numDigits`, `length` | Greater than zero, except `Record.timeout`; `Pause.length` at most 60 |
| Gather timeout | `Gather.timeout` | Positive integer or `auto`; `Dial.timeout` remains numeric |
| Recording silence timeout | `Record.timeout` | Non-negative integer; `0` disables silence timeout |
| Speech timeout | `Gather.speechTimeout` | `auto` or positive integer |
| Repeat count | `Say.loop`, `Play.loop` | Non-negative integer; `0` is valid |
| Boolean | `hangupOnStar`, `answerOnBridge`, `profanityFilter`, `bargeIn`, `actionOnEmptyResult`, `playBeep`, `dtmfDetection`, `securityCode` | `true` or `false` |
| Conversation interruption | `ConversationRelay.interruptible` | `none`, `dtmf`, `speech`, `any`, plus legacy `true`/`false` |
| Conversation providers | `ConversationRelay.ttsProvider`, `transcriptionProvider` | `Google`/`Amazon`/`ElevenLabs` and `Google`/`Deepgram`, respectively; case-insensitive to match documented examples |
| Audio track | `Stream.track` | `inbound_track`, `outbound_track`, `both_tracks` |
| Bidirectional audio track | `Connect/Stream.track` | `inbound_track` only |
| Recording trim | `Dial.trim`, `Record.trim` | `trim-silence`, `do-not-trim` |
| Recording mode | `Dial.record` | Five named modes plus legacy `true`/`false` aliases |
| Conference recording mode | `Conference.record` | `do-not-record` or `record-from-start` |
| Conversation relay transport | `ConversationRelay.url` | Required on fresh typed nodes and must use `wss` on all builders |
| Record length | `Record.maxLength` | 2–86,400 seconds |
| Payment retry count | `Pay.maxAttempts` | 1–3 |
| Payment input | `Pay.input` | `dtmf` only |
| Payment amount | `Pay.chargeAmount` | Finite number from 0 to 1,000,000 |
| Payment method | `Pay.paymentMethod` | `credit-card`, `ach-debit`; legacy `card`/`ach` remain accepted for compatibility |
| Payment token | `Pay.tokenType` | `one-time`, `reusable`, `payment-method` |
| Payment callback | `Pay.action`, `Pay.method` | HTTPS action URL and `POST` method only |
| Payment prompt step | `Prompt.for` | Required for fresh typed nodes; one of the seven documented payment steps |
| Payment prompt attempt | `Prompt.attempt` | One or more space-delimited integers from 1 to 10 |
| DTMF terminator | `Gather.finishOnKey` | One digit, `*`, or `#`; empty collects until timeout |
| Recording terminators | `Record.finishOnKey` | Non-empty sequence of digits, `*`, and `#` |
| Mutually exclusive content | `Play.digits`/URL text, text/children | `Play` accepts either digits or an audio URL; `Gather` cannot nest `Play.digits`; typed nodes cannot mix text and children |
| Recording events | `recordingStatusCallbackEvent` | Space-delimited `in-progress`, `completed`, `absent` |
| Conference events | `Conference.statusCallbackEvent` | Space-delimited `start`, `end`, `join`, `leave`, `mute`, `hold`, `modify`, `speaker`, `announcement` |
| Call-leg events | `Number`, `Sip`, `Client` `statusCallbackEvent` | Space-delimited `initiated`, `ringing`, `answered`, `completed` |

Public Twilio references reviewed: [Dial](https://www.twilio.com/docs/voice/twiml/dial),
[Gather](https://www.twilio.com/docs/voice/twiml/gather),
[Record](https://www.twilio.com/docs/voice/twiml/record),
[Pay](https://www.twilio.com/docs/voice/twiml/pay), and
[Stream](https://www.twilio.com/docs/voice/twiml/stream),
[Conference](https://www.twilio.com/docs/voice/twiml/conference), and
[ConversationRelay](https://www.twilio.com/docs/voice/twiml/connect/conversationrelay),
[VirtualAgent](https://www.twilio.com/docs/voice/twiml/connect/virtualagent/virtualagent-dialogflow-cx),
and [Play](https://www.twilio.com/docs/voice/twiml/play). These moving pages must
not silently redefine the adopted snapshot; review any differences against
the pinned manifest and frozen compatibility fixtures before adding rejection.

Values such as voice, language, speech model, ring tone, and caller ID remain
open strings because their accepted sets depend on service configuration or
provider catalogs. The snapshot does not declare a one-child limit for
`Connect`, `Start`, or `Stop`, so builders do not invent one. Current online
documentation sometimes differs from the snapshot (for example, new
`Transcription` attributes); those changes require a separate adoption.
The Play documentation's loop table says `0`-`9` while its example uses `10`;
the validator therefore accepts any non-negative integer rather than
rejecting a documented example.

Compatibility decision (2026-09-23): retain the frozen name-only XML emitted
by `Start::stream` and `Start::siprec`, but document that it cannot establish a
new session. New `Start::stream_to` and `Start::siprec_with_connector` builders
provide the documented inputs. Fresh `TwimlNode` construction rejects missing
`url` or `connectorName` under `Start`; `Stop` requires a name. The adopted
manifest now includes the missing `Siprec.connectorName` attribute. This is a
narrow legacy exception; no existing golden XML is rewritten.

The same policy keeps `Pay::prompt(text)` byte-compatible while adding
`Pay::prompt_for(step, text)` for the documented required `Prompt.for` step.
Fresh `TwimlNode` Prompt construction requires `for` under Pay.
