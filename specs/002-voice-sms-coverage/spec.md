# Feature Specification: Complete Voice and Messaging Coverage

**Feature Branch**: `sms-voice-coverage`

**Created**: 2026-09-17

**Status**: Draft

**Input**: User description: "Extend Dialkit to provide complete, idiomatic Rust coverage of Twilio Programmable Voice and SMS/MMS (including REST operations, TwiML, webhooks, callbacks, calls, conferences, recordings, queues, SIP, messages, media, and messaging services) without breaking existing APIs. Define an authoritative pinned scope, explicit exclusions, and a testable coverage matrix for every supported Twilio operation."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Use every pinned Voice and Messaging REST operation (Priority: P1)

As a Dialkit user, I can discover and invoke every REST operation in the pinned Programmable Voice and SMS/MMS scope with domain-appropriate inputs, outputs, pagination, errors, and authentication, without dropping to a different Twilio library or constructing raw requests.

**Why this priority**: Complete REST coverage is the foundation for all other workflows and removes the largest source of product gaps.

**Independent Test**: For every operation identifier in the REST coverage matrices, run its contract fixture and verify the HTTP method, path, authentication, parameter placement, encoding, response type, error type, and pagination behavior where applicable.

**Acceptance Scenarios**:

1. **Given** any operation listed as supported, **When** a developer supplies a valid request, **Then** Dialkit produces the request defined by the pinned Twilio contract and returns a typed success value.
2. **Given** a supported list operation with multiple pages, **When** the developer follows or iterates its page tokens, **Then** all items are available without manually assembling page URLs.
3. **Given** Twilio returns a documented or forward-compatible error payload, **When** Dialkit handles the response, **Then** the developer receives a structured error without losing Twilio's status, code, message, or detail URI.

---

### User Story 2 - Build complete Voice and Messaging TwiML (Priority: P1)

As a developer handling an inbound call or message, I can construct every TwiML verb and noun in the pinned matrix with valid attributes and nesting, safe escaping, deterministic output, and useful validation errors.

**Why this priority**: TwiML is required to control live Voice and Messaging interactions and is as central as outbound REST requests.

**Independent Test**: Build one valid document for every supported node and every allowed parent-child relationship, compare it with canonical XML, and prove that each prohibited nesting or invalid constrained value is rejected.

**Acceptance Scenarios**:

1. **Given** any supported TwiML node and valid attributes, **When** a developer builds a response, **Then** the result is well-formed TwiML with exact case, order, escaping, and nesting.
2. **Given** a node in an invalid context or a value outside a documented constraint, **When** the response is built, **Then** Dialkit reports an actionable validation error before the response is sent.
3. **Given** an existing Dialkit TwiML program, **When** the expanded feature is adopted, **Then** it continues to compile and produces equivalent XML unless the developer opts into a newly supported capability.

---

### User Story 3 - Receive and verify webhooks and callbacks (Priority: P1)

As a webhook application author, I can verify Twilio signatures and parse Voice and Messaging requests, action callbacks, and status callbacks into typed known fields while retaining unknown fields added by Twilio.

**Why this priority**: Inbound webhooks and asynchronous callbacks complete the operational loop and must be safe against spoofing and schema evolution.

**Independent Test**: Replay canonical GET, form-encoded POST, and JSON webhook fixtures for every row in the webhook matrix, including valid and invalid signatures and extra unknown parameters.

**Acceptance Scenarios**:

1. **Given** the exact externally visible URL, request parameters or raw JSON body, and a valid `X-Twilio-Signature`, **When** the request is verified, **Then** Dialkit accepts it and exposes typed event data.
2. **Given** any change to the signed URL/body or an invalid signature, **When** verification runs, **Then** Dialkit rejects the request without exposing credentials.
3. **Given** Twilio adds an unrecognized webhook parameter or enum value, **When** Dialkit parses the request, **Then** known data remains usable and the unknown value remains inspectable.

---

### User Story 4 - Upgrade without an API break (Priority: P1)

As an existing Dialkit user, I can upgrade within the current major version without changing working source code or observing changed behavior in existing calls, messages, TwiML, errors, or pagination.

**Why this priority**: Compatibility is an explicit product promise and a constitutional release gate.

**Independent Test**: Compile the repository's frozen public-API compatibility suite and run existing behavioral fixtures against both the baseline and candidate release.

**Acceptance Scenarios**:

1. **Given** code using an existing public item, **When** it is compiled against the expanded release, **Then** it compiles without source changes.
2. **Given** an existing serialized request, parsed response, rendered TwiML document, or error, **When** the same fixture is run against the expanded release, **Then** its externally observable behavior is unchanged.
3. **Given** a pinned upstream shape that conflicts with an existing public abstraction, **When** support is added, **Then** the existing abstraction remains available and any new representation is additive.

---

### User Story 5 - Audit coverage and future scope changes (Priority: P2)

As a maintainer, I can prove which upstream revision defines support, see a pass/fail result for every supported operation and event, and detect additions, removals, or changes before adopting another Twilio revision.

**Why this priority**: “Complete” is only sustainable when scope is immutable for a release and machine-auditable.

**Independent Test**: Regenerate an inventory from the pinned sources and compare it with the matrices; the check fails for a missing, duplicate, renamed, or unexpectedly added operation.

**Acceptance Scenarios**:

1. **Given** the pinned source files, **When** the coverage audit runs, **Then** every in-scope operation appears exactly once and every matrix entry resolves to one source operation.
2. **Given** a candidate upstream revision with a changed operation set, **When** maintainers evaluate it, **Then** the diff is reported without silently changing the supported scope.
3. **Given** a release candidate, **When** release evidence is inspected, **Then** it identifies the source commit, file hashes, coverage totals, exceptions, and compatibility result.

### Edge Cases

- Twilio adds optional response fields, webhook parameters, or enum values after the pin; parsing remains forward-compatible while the declared supported operation inventory stays fixed.
- A callback is delivered more than once or out of order; parsing does not imply deduplication or ordering guarantees and exposes stable identifiers/statuses needed by the application.
- A list response has zero items, a missing next-page link, an absolute next-page URL, or a page token containing reserved characters.
- A request combines mutually exclusive sources such as `From` and `MessagingServiceSid`, or multiple call instruction sources; invalid combinations are rejected before transmission where the pinned contract is unambiguous.
- Media URLs and recording media responses are binary or redirect outside the normal JSON resource shape; metadata operations remain typed and media download behavior is explicit.
- TwiML includes XML-sensitive characters, Unicode, repeated nodes, empty optional text, unreachable nodes after a control-transfer verb, or deeply nested valid content.
- A webhook arrives through a reverse proxy; signature verification uses the exact public URL supplied by the application and never guesses from untrusted forwarding headers.
- Form parameters repeat, contain empty values, or arrive in a different order; verification follows Twilio's canonicalization rules without losing values needed for parsing.
- A SIP URI contains user info, headers, transport parameters, or reserved characters; serialization preserves valid URI meaning and rejects structurally invalid values.
- A call, recording, conference, queued member, scheduled message, or messaging-service resource changes state between requests; the SDK exposes Twilio's returned state and does not manufacture consistency.

## Requirements *(mandatory)*

### Authoritative Pinned Scope

The feature's release scope is frozen to `twilio/twilio-oai` commit `5aa7f31977ce5812f7b7bc1f46a38555ebaa2888`. A supported REST operation is one named in the matrices below from exactly these source files:

| Source ID | Pinned file | SHA-256 | In-scope operations |
|-----------|-------------|---------|---------------------|
| `API-2010` | `spec/json/twilio_api_v2010.json` | `170b3ccd0f891416840083d72f1795b1499b14a18d4873fd2b39f47ef84642d6` | 139 |
| `MSG-V1` | `spec/json/twilio_messaging_v1.json` | `611c6fde586347615039a7d8c87a4b10d39c6d8566b7d1a691353ef995aaa473` | 58 |
| **Total** | | | **197** |

The TwiML and webhook matrices are normative snapshots of Twilio's official Voice, Messaging, and webhook documentation as reviewed on 2026-09-17. Their enumerated rows—not later documentation edits—define this feature's release scope. A future upstream adoption requires a separately reviewed scope change.

Normative source locations are the commit-addressed files in `https://github.com/twilio/twilio-oai/tree/5aa7f31977ce5812f7b7bc1f46a38555ebaa2888/spec/json`, Twilio's Voice TwiML reference at `https://www.twilio.com/docs/voice/twiml`, Messaging TwiML reference at `https://www.twilio.com/docs/messaging/twiml`, Voice webhook reference at `https://www.twilio.com/docs/usage/webhooks/voice-webhooks`, Messaging webhook reference at `https://www.twilio.com/docs/usage/webhooks/messaging-webhooks`, and webhook-security reference at `https://www.twilio.com/docs/usage/webhooks/webhooks-security`.

### Functional Requirements

- **FR-001**: Dialkit MUST expose all 197 REST operation identifiers in the pinned REST matrices and MUST expose no operation as part of this feature merely because it appears in a later upstream revision.
- **FR-002**: Each supported REST operation MUST preserve the pinned method, path, authentication, path/query/header/form/body parameters, requiredness, multiplicity, documented constraints, response shapes, and error semantics.
- **FR-003**: Each supported list operation MUST provide a consistent way to request one page and to traverse all available pages while preserving filters and Twilio page tokens.
- **FR-004**: Calls, messages, recordings, conferences, queues, participants/members, SIP resources, media metadata, applications, phone-number configuration, and Messaging Services MUST have discoverable domain-oriented entry points and typed resource identifiers, statuses, dates, URLs, and constrained values where doing so does not break an existing API.
- **FR-005**: Existing public Dialkit items and their documented behavior MUST remain source- and behavior-compatible throughout the current major version; additions MUST be additive, and unavoidable breaking changes MUST be deferred to a major release.
- **FR-006**: Dialkit MUST build every TwiML node in the TwiML matrix, including every documented attribute and valid nesting relationship present in the pinned snapshot.
- **FR-007**: TwiML construction MUST produce one correctly cased `Response` root, deterministic child order, valid XML escaping, and no raw unescaped interpolation.
- **FR-008**: TwiML construction MUST reject invalid required values, documented mutually exclusive attributes, invalid enum/range values, and prohibited parent-child relationships with actionable errors.
- **FR-009**: Dialkit MUST represent every inbound request, action callback, and status callback family in the webhook matrix, including its documented event/status variants.
- **FR-010**: Webhook parsing MUST expose typed known fields and preserve unknown parameters or values so Twilio's additive webhook changes do not make otherwise valid requests unusable.
- **FR-011**: Webhook verification MUST cover GET/query, form-encoded POST, and JSON body verification using the exact public URL, all received parameters or the raw body as applicable, and constant-time signature comparison.
- **FR-012**: Credentials, auth tokens, signatures, message bodies, SIP credentials, and payment details MUST NOT appear in debug output, errors, fixtures, examples, or logs unless explicitly supplied as synthetic non-secret test data.
- **FR-013**: Binary recording/media retrieval and JSON metadata retrieval MUST be distinguishable so callers do not attempt to decode media bytes as resource JSON.
- **FR-014**: Every REST operation MUST have a deterministic contract test that proves method/path and applicable parameter encoding, authentication, success decoding, Twilio error decoding, and pagination; requirements that do not apply MUST be recorded as not applicable rather than silently omitted.
- **FR-015**: Every TwiML node and webhook family MUST have at least one positive fixture and one relevant negative or forward-compatibility fixture; every corrected defect MUST gain a focused regression fixture.
- **FR-016**: A coverage audit MUST fail if any pinned in-scope operation is missing, duplicated, mapped to the wrong source, or lacks its required evidence.
- **FR-017**: Release evidence MUST record both pinned file hashes, the upstream commit, supported operation totals by matrix row, TwiML/webhook coverage totals, compatibility results, and every approved exception.
- **FR-018**: Public examples and task-oriented documentation MUST demonstrate at minimum: outbound call creation, inbound-call TwiML, call status verification, conference participation, recording control/download, queue/member control, SIP domain and credential setup, SMS send, MMS media use, inbound-message parsing, delivery-status verification, and Messaging Service sender management.
- **FR-019**: Live-account tests MAY supplement deterministic tests but MUST NOT be required for ordinary development or pull-request validation and MUST NOT incur Twilio charges without explicit operator action.
- **FR-020**: A later Twilio specification or documentation revision MUST NOT silently expand or contract support; adoption MUST update the pin, matrices, compatibility assessment, and test evidence together.

### REST Coverage Matrix A — `API-2010` (139 operations)

Every comma-separated identifier in the `Operations` column is individually supported and individually subject to FR-002 and FR-014.

| Domain/resource | Count | Operations |
|-----------------|------:|------------|
| Voice applications | 5 | `CreateApplication`, `ListApplication`, `DeleteApplication`, `FetchApplication`, `UpdateApplication` |
| Calls | 5 | `CreateCall`, `ListCall`, `DeleteCall`, `FetchCall`, `UpdateCall` |
| Call events | 1 | `ListCallEvent` |
| Call notifications | 2 | `FetchCallNotification`, `ListCallNotification` |
| Call recordings | 5 | `CreateCallRecording`, `ListCallRecording`, `UpdateCallRecording`, `FetchCallRecording`, `DeleteCallRecording` |
| Real-time call transcriptions | 2 | `CreateRealtimeTranscription`, `UpdateRealtimeTranscription` |
| Conferences | 3 | `FetchConference`, `UpdateConference`, `ListConference` |
| Conference recordings | 4 | `ListConferenceRecording`, `UpdateConferenceRecording`, `FetchConferenceRecording`, `DeleteConferenceRecording` |
| Conference participants | 5 | `FetchParticipant`, `UpdateParticipant`, `DeleteParticipant`, `CreateParticipant`, `ListParticipant` |
| Voice payments | 2 | `CreatePayments`, `UpdatePayments` |
| Queues | 5 | `FetchQueue`, `UpdateQueue`, `DeleteQueue`, `ListQueue`, `CreateQueue` |
| Queue members | 3 | `FetchMember`, `UpdateMember`, `ListMember` |
| Recordings | 3 | `FetchRecording`, `DeleteRecording`, `ListRecording` |
| Recording add-on results | 3 | `FetchRecordingAddOnResult`, `DeleteRecordingAddOnResult`, `ListRecordingAddOnResult` |
| Recording add-on payloads | 3 | `FetchRecordingAddOnResultPayload`, `DeleteRecordingAddOnResultPayload`, `ListRecordingAddOnResultPayload` |
| Recording add-on payload data | 1 | `FetchRecordingAddOnResultPayloadData` |
| Recording transcriptions | 3 | `FetchRecordingTranscription`, `DeleteRecordingTranscription`, `ListRecordingTranscription` |
| Account transcriptions | 3 | `FetchTranscription`, `DeleteTranscription`, `ListTranscription` |
| SIP call credential-list auth mappings | 4 | `CreateSipAuthCallsCredentialListMapping`, `ListSipAuthCallsCredentialListMapping`, `FetchSipAuthCallsCredentialListMapping`, `DeleteSipAuthCallsCredentialListMapping` |
| SIP call IP-ACL auth mappings | 4 | `CreateSipAuthCallsIpAccessControlListMapping`, `ListSipAuthCallsIpAccessControlListMapping`, `FetchSipAuthCallsIpAccessControlListMapping`, `DeleteSipAuthCallsIpAccessControlListMapping` |
| SIP registration credential-list auth mappings | 4 | `CreateSipAuthRegistrationsCredentialListMapping`, `ListSipAuthRegistrationsCredentialListMapping`, `FetchSipAuthRegistrationsCredentialListMapping`, `DeleteSipAuthRegistrationsCredentialListMapping` |
| SIP credentials | 5 | `ListSipCredential`, `CreateSipCredential`, `FetchSipCredential`, `UpdateSipCredential`, `DeleteSipCredential` |
| SIP credential lists | 5 | `ListSipCredentialList`, `CreateSipCredentialList`, `FetchSipCredentialList`, `UpdateSipCredentialList`, `DeleteSipCredentialList` |
| SIP domain credential-list mappings | 4 | `CreateSipCredentialListMapping`, `ListSipCredentialListMapping`, `FetchSipCredentialListMapping`, `DeleteSipCredentialListMapping` |
| SIP domains | 5 | `ListSipDomain`, `CreateSipDomain`, `FetchSipDomain`, `UpdateSipDomain`, `DeleteSipDomain` |
| SIP IP access-control lists | 5 | `ListSipIpAccessControlList`, `CreateSipIpAccessControlList`, `FetchSipIpAccessControlList`, `UpdateSipIpAccessControlList`, `DeleteSipIpAccessControlList` |
| SIP domain IP-ACL mappings | 4 | `FetchSipIpAccessControlListMapping`, `DeleteSipIpAccessControlListMapping`, `CreateSipIpAccessControlListMapping`, `ListSipIpAccessControlListMapping` |
| SIP IP addresses | 5 | `ListSipIpAddress`, `CreateSipIpAddress`, `FetchSipIpAddress`, `UpdateSipIpAddress`, `DeleteSipIpAddress` |
| SIPREC sessions | 2 | `CreateSiprec`, `UpdateSiprec` |
| Call media streams | 2 | `CreateStream`, `UpdateStream` |
| Voice client tokens | 1 | `CreateToken` |
| Call user-defined messages | 1 | `CreateUserDefinedMessage` |
| Call user-defined message subscriptions | 2 | `CreateUserDefinedMessageSubscription`, `DeleteUserDefinedMessageSubscription` |
| Outgoing caller IDs | 4 | `FetchOutgoingCallerId`, `UpdateOutgoingCallerId`, `DeleteOutgoingCallerId`, `ListOutgoingCallerId` |
| Caller-ID validation | 1 | `CreateValidationRequest` |
| Incoming phone numbers | 5 | `UpdateIncomingPhoneNumber`, `FetchIncomingPhoneNumber`, `DeleteIncomingPhoneNumber`, `ListIncomingPhoneNumber`, `CreateIncomingPhoneNumber` |
| Local incoming phone numbers | 2 | `ListIncomingPhoneNumberLocal`, `CreateIncomingPhoneNumberLocal` |
| Mobile incoming phone numbers | 2 | `ListIncomingPhoneNumberMobile`, `CreateIncomingPhoneNumberMobile` |
| Toll-free incoming phone numbers | 2 | `ListIncomingPhoneNumberTollFree`, `CreateIncomingPhoneNumberTollFree` |
| Messages | 5 | `CreateMessage`, `ListMessage`, `DeleteMessage`, `FetchMessage`, `UpdateMessage` |
| Message feedback | 1 | `CreateMessageFeedback` |
| Message media collection | 1 | `ListMedia` |
| Message media instances | 2 | `DeleteMedia`, `FetchMedia` |
| Account SMS short codes | 3 | `FetchShortCode`, `UpdateShortCode`, `ListShortCode` |
| **Total** | **139** | |

### REST Coverage Matrix B — `MSG-V1` (58 operations)

| Domain/resource | Count | Operations |
|-----------------|------:|------------|
| Alpha senders | 4 | `CreateAlphaSender`, `ListAlphaSender`, `FetchAlphaSender`, `DeleteAlphaSender` |
| A2P brand registrations | 4 | `FetchBrandRegistrations`, `UpdateBrandRegistrations`, `ListBrandRegistrations`, `CreateBrandRegistrations` |
| A2P brand registration OTP | 1 | `CreateBrandRegistrationOtp` |
| A2P brand vettings | 3 | `CreateBrandVetting`, `ListBrandVetting`, `FetchBrandVetting` |
| Channel senders | 4 | `ListChannelSender`, `CreateChannelSender`, `FetchChannelSender`, `DeleteChannelSender` |
| Deactivations | 1 | `FetchDeactivation` |
| Destination alpha senders | 4 | `CreateDestinationAlphaSender`, `ListDestinationAlphaSender`, `FetchDestinationAlphaSender`, `DeleteDestinationAlphaSender` |
| Link-shortening domain certificates | 3 | `UpdateDomainCertV4`, `FetchDomainCertV4`, `DeleteDomainCertV4` |
| Link-shortening domain configuration | 2 | `UpdateDomainConfig`, `FetchDomainConfig` |
| Service domain configuration | 1 | `FetchDomainConfigMessagingService` |
| Link-shortening DNS validation | 1 | `FetchDomainDnsValidation` |
| Externally registered US A2P campaigns | 1 | `CreateExternalCampaign` |
| Service/domain link-shortening association | 2 | `CreateLinkshorteningMessagingService`, `DeleteLinkshorteningMessagingService` |
| Service link-shortening domain lookup | 1 | `FetchLinkshorteningMessagingServiceDomainAssociation` |
| Service phone-number senders | 4 | `CreatePhoneNumber`, `ListPhoneNumber`, `DeletePhoneNumber`, `FetchPhoneNumber` |
| Managed certificate requests | 1 | `UpdateRequestManagedCert` |
| Messaging Services | 5 | `CreateService`, `ListService`, `UpdateService`, `FetchService`, `DeleteService` |
| Service short-code senders | 4 | `CreateShortCode`, `ListShortCode`, `DeleteShortCode`, `FetchShortCode` |
| Toll-free verifications | 5 | `FetchTollfreeVerification`, `UpdateTollfreeVerification`, `DeleteTollfreeVerification`, `ListTollfreeVerification`, `CreateTollfreeVerification` |
| US A2P campaigns | 5 | `CreateUsAppToPerson`, `ListUsAppToPerson`, `DeleteUsAppToPerson`, `FetchUsAppToPerson`, `UpdateUsAppToPerson` |
| US A2P campaign use case | 1 | `FetchUsAppToPersonUsecase` |
| Messaging Service use cases | 1 | `FetchUsecase` |
| **Total** | **58** | |

### TwiML Coverage Matrix

Each node requires tests for all documented attributes, allowed content, valid parents/children, escaping, and invalid constraints in the pinned snapshot.

| Family | Supported nodes | Required coverage evidence |
|--------|-----------------|----------------------------|
| Voice response verbs | `Connect`, `Dial`, `Echo`, `Enqueue`, `Gather`, `Hangup`, `Leave`, `Pause`, `Pay`, `Play`, `Record`, `Redirect`, `Refer`, `Reject`, `Say`, `Start`, `Stop` | Canonical XML per verb; all attributes and enum/range constraints; execution-order/control-transfer rules |
| `Connect` nouns | `ConversationRelay`, `Room`, `Stream`, `VirtualAgent` | Parent restriction, noun attributes, custom parameters where documented, action/status callback shapes |
| `Dial` nouns | `Application`, `Client`, `Conference`, `Number`, `Queue`, `Sip` | Multiple targets where allowed; noun attributes; recording and status callbacks; SIP/number encoding |
| `Gather` children | `Say`, `Play`, `Pause` | Allowed nesting and input-specific attributes/callbacks |
| `Pay` children | `Parameter`, `Prompt` with allowed prompt content | Payment method/token type constraints; redaction-safe errors; action/status callbacks |
| `Start` and `Stop` nouns | `Recording`, `Siprec`, `Stream`, `Transcription` in each documented start/stop relationship | Name-based stop behavior, attributes, custom parameters, and recording/stream/transcription status callbacks |
| Streaming/custom data children | `Parameter` | Repeated name/value handling and valid parents |
| SIP refer children | `Sip` | Valid nesting and refer action result |
| Messaging response verbs | `Message`, `Redirect` | Canonical XML; action/status callback attributes; control-transfer behavior |
| `Message` nouns | `Body`, `Media` | Ordered/repeated media, text escaping, SMS-only and MMS shapes |

### Webhook and Callback Coverage Matrix

| Family | Supported request/event variants | Required coverage evidence |
|--------|----------------------------------|----------------------------|
| Voice instruction requests | Inbound call, outbound call answer URL, redirect/fallback request, SIP-originated call additions | GET and form POST; standard call fields; optional geo/SIP fields; unknown-field retention; TwiML response example |
| Call progress callbacks | `initiated`, `ringing`, `answered`, `completed`; terminal `busy`, `failed`, `no-answer`, `canceled` representations | Signature verification; status/event mapping; parent/child call identifiers; idempotent replay fixture |
| Answering-machine callbacks | asynchronous detection results supported by the pinned call request | Result enum including unknown future value; call correlation; signature verification |
| TwiML action callbacks | `Connect`, `Dial`, `Enqueue`, `Gather`, `Pay`, `Record`, `Redirect`, `Refer`, and Messaging `Message` action requests | One fixture per action family and documented outcome fields; response/no-response distinction |
| Recording callbacks | call, `Dial`, conference, and `Record` recording status events including `in-progress`, `completed`, and `absent` where offered | Recording/call/conference correlation, duration/channels/source fields, unknown event retention |
| Conference callbacks | `start`, `end`, `join`, `leave`, `mute`, `hold`, `speaker` events and participant updates | Conference/call identifiers, event-specific fields, timestamp/order independence |
| Queue callbacks | enqueue action, wait URL request, leave/dequeue outcomes | Queue/call identifiers, queue position/time, documented result enums |
| Gather callbacks | final action request and partial speech-result callback | DTMF/speech variants, confidence where present, empty/no-input outcome |
| Stream/SIPREC callbacks | stream status, stream messages represented by the documented HTTP callback contract, and SIPREC status | Start/stop/error variants, call/stream identifiers, unknown-field retention |
| Real-time transcription callbacks | transcription status and result callbacks represented by the pinned Voice scope | Call/transcription identifiers, result/status variants, unknown-field retention |
| Voice payment callbacks | `Pay` action and status callbacks | Result/status/token metadata only; fixture proves sensitive payment values are not exposed or logged |
| User-defined call messages | subscription-delivered message request | Call/subscription identifiers, payload preservation, signature verification |
| Incoming Messaging webhook | SMS and MMS, including zero/one/multiple media items and Messaging Service context | Standard message fields, indexed media fields, unknown-field retention, TwiML response example |
| Message status callback | `accepted`, `scheduled`, `canceled`, `queued`, `sending`, `sent`, `receiving`, `received`, `delivered`, `undelivered`, `failed`, and `read` values when emitted for the direction/channel, plus unknown future values | Message/service identifiers, error fields, channel data, signature verification, no-TwiML response example |
| Messaging Service request routing | inbound request URL, fallback URL, and status callback URL configured by a service | Primary/fallback distinction, service identifier, same signature and forward-compatibility guarantees |
| JSON webhook verification | Any supported Twilio callback delivered as JSON with `bodySHA256` | Raw-body hash verification, exact URL handling, changed-body rejection |

### Explicit Exclusions

- Twilio products outside Programmable Voice and SMS/MMS, including Verify, Conversations, Chat, Video, Flex, TaskRouter, Studio, Segment, SendGrid, Fax, Wireless, Sync, Notify, Lookup, Marketplace management, and IoT APIs, except where a Voice TwiML noun merely connects to another product and is explicitly listed above.
- WhatsApp, RCS, Facebook Messenger, and other non-SMS/MMS channel-specific payload helpers. Generic channel addresses or fields already present in a pinned model remain usable, but no channel-specific completeness claim is made.
- Account/subaccount lifecycle, balances, usage records/triggers, API/signing keys, Connect Apps, addresses, available-number search, dependent numbers, and incoming-number add-on assignment operations not listed in Matrix A.
- Voice Insights, Trust Hub, Emergency Calling, BYOC trunks, Elastic SIP Trunking's separate API, Media Streams WebSocket frame handling, ConversationRelay WebSocket protocol handling, and live audio/media processing. TwiML nodes and listed REST controls remain in scope.
- Content API/template authoring, message redaction, message scheduling features not represented by pinned `CreateMessage` fields, carrier registration APIs outside Matrix B, and automatic regulatory eligibility decisions.
- Application web-server adapters, webhook routing, persistence, retries, deduplication, job scheduling, observability backends, and business workflow orchestration.
- Automatic download/transcoding/storage of recordings or MMS media; Dialkit covers listed metadata operations and an explicit byte-stream retrieval boundary only.
- Live Twilio account provisioning or billable end-to-end traffic as a mandatory test dependency.
- Operations added after the pinned commit and documentation behavior added after the 2026-09-17 TwiML/webhook snapshot until a reviewed scope update adopts them.

### Key Entities

- **Pinned Scope**: The upstream commit, source files, hashes, documentation snapshot date, operation inventory, and explicit exclusions that define the release claim.
- **Coverage Entry**: One REST operation, TwiML node, or webhook family linked to its source and required verification evidence.
- **REST Operation**: A stable operation identifier with method, path, parameters, success/error responses, authentication, and optional pagination contract.
- **Voice Resource**: A call, conference, participant, recording, transcription, queue/member, application, stream, payment session, or SIP-related resource and its lifecycle state.
- **Messaging Resource**: A message, media item, delivery feedback item, Messaging Service, sender, compliance registration, or link-shortening resource and its lifecycle state.
- **TwiML Node**: A case-sensitive verb or noun with attributes, text/content rules, allowed parent-child relationships, and deterministic XML representation.
- **Webhook Envelope**: The exact public URL, method, headers, query/form parameters or raw body, verified signature result, typed known fields, and preserved unknown fields.
- **Compatibility Baseline**: The frozen public symbols, accepted inputs, serialized outputs, and observed behaviors from the release immediately preceding this feature.
- **Coverage Evidence**: Deterministic fixtures and audit results proving that one entry satisfies its applicable contract dimensions.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: The coverage audit reports exactly 197 supported REST operations—139 from `API-2010` and 58 from `MSG-V1`—with zero missing, duplicate, or unresolved identifiers.
- **SC-002**: 100% of supported REST operations pass their applicable method/path, parameter, authentication, success, error, and pagination contract checks, with every non-applicable dimension explicitly recorded.
- **SC-003**: 100% of TwiML nodes in the matrix have canonical-output coverage, attribute coverage, nesting coverage, and at least one applicable invalid-input check.
- **SC-004**: 100% of webhook families pass valid-signature, invalid-signature, known-field parsing, and unknown-field preservation checks for every applicable transport encoding.
- **SC-005**: The frozen existing public-API suite has zero source-compatibility regressions and the existing behavior fixture suite has zero unapproved output changes.
- **SC-006**: A developer can complete each of the 12 representative tasks in FR-018 from published documentation, with each example compiling and requiring no raw HTTP or hand-authored XML.
- **SC-007**: A clean release audit can trace every supported entry to its pinned source and its passing evidence, and can produce the complete report in under 10 minutes on the project's standard CI runner.
- **SC-008**: Deterministic pull-request validation completes without a Twilio account, live network traffic to Twilio, billable activity, or real credentials.
- **SC-009**: Secret-scanning and fixture review find zero real credentials, auth tokens, webhook signatures tied to real tokens, message bodies from real users, SIP passwords, or payment data.
- **SC-010**: In a usability review, at least 9 of 10 representative Voice/Messaging tasks are completed on the first attempt using public API names and documentation alone.

## Assumptions

- The existing adopted commit is retained so this feature expands coverage without mixing a scope expansion with an upstream-version migration.
- The two recorded SHA-256 values are the authoritative byte-level identities for the OpenAPI inputs; a hash mismatch is a scope failure even if the path and commit label appear unchanged.
- Twilio's OpenAPI operation identifiers are stable keys for REST coverage, while the enumerated TwiML and webhook matrices provide the equivalent stable keys where no complete pinned machine-readable schema exists.
- “Complete Programmable Voice and SMS/MMS” means complete for the authoritative matrices and exclusions in this document, not every product reachable from Twilio's account or every later feature in online documentation.
- Existing low-level generated access may remain available; idiomatic task-oriented access is additive and must not conceal operations that lack a convenience facade.
- Forward compatibility means retaining unknown fields/values where practical; it does not claim behavioral support for a new operation absent from the pinned matrices.
- Twilio remains responsible for carrier behavior, delivery, telephony state transitions, regional availability, regulatory approval, and billing; Dialkit represents requests and responses but does not guarantee those external outcomes.
- The project constitution's pinned-source, reproducibility, contract-confidence, and semantic-versioning rules govern any conflict not resolved explicitly by this specification.
