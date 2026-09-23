# Representative-task usability review

Reviewed 2026-09-22. This is an offline developer walkthrough, not a claim of live Twilio delivery or an independent human study. The reviewer started at the public [workflow index](../../crates/client/README.md#task-oriented-workflow-index), used its named facade entrypoint and linked example, and checked the unmodified examples on the first attempt with `cargo check -p dialkit --examples --all-features` (exit 0). Synthetic identifiers and URLs were retained; no secrets or network calls were used. The criterion was finding and compiling a complete SDK workflow without raw HTTP or hand-authored XML.

| Representative task | Documentation/example | First attempt |
|---|---|---|
| Create an outbound call | `Client::calls().create` / `make_call.rs` | Pass |
| Respond to an inbound call | `TwimlResponse::voice()` / `inbound_call_twiml.rs` | Pass |
| Verify call status | `WebhookValidator::verify_form` / `verify_call_status.rs` | Pass |
| Manage a conference participant | `Client::conferences().participants` / `conference_participant.rs` | Pass |
| Start and download a recording | `Client::recordings()` / `recording_download.rs` | Pass |
| Control a queue member | `Client::queues().members` / `queue_member.rs` | Pass |
| Configure SIP domain and credential | `Client::sip()` / `sip_setup.rs` | Pass |
| Send an SMS | `Client::messages().create` / `send_message.rs` | Pass |
| Send MMS with media | `CreateMessage::media_url` / `send_mms.rs` | Pass |
| Parse an inbound message | `WebhookValidator::verify_form` and `parse_messaging` / `parse_inbound_message.rs` | Pass |

Result: 10/10 prewritten examples compiled. The remaining two FR-018 examples (`verify_delivery_status.rs`, `manage_service_sender.rs`) were also included in the same all-examples check. This establishes example compilation for synthetic inputs, not first-attempt construction from documentation alone or application-level behavior with a live Twilio account.

## New construction exercise (not yet qualifying as docs-only)

The ten original, unexecuted constructions are in `crates/client/tests/usability_first_attempt.rs`.
On the first `cargo test -p dialkit --test usability_first_attempt --no-run`, compilation stopped at
`E0432`: `TwimlResponse` is not exported from the crate root. After changing only that import to
`dialkit::twiml::TwimlResponse`, the entire test target compiled. This means nine untouched
constructions type-checked on the first draft; inbound TwiML needed one correction.

| Task construction | First draft | Observable result |
|---|---|---|
| Outbound call with URL instructions | Pass | Compiled unchanged after import correction |
| Inbound voice TwiML | Fail | Incorrect crate-root import; corrected to `dialkit::twiml` |
| Call-status verification and family parse | Pass | Compiled unchanged after import correction |
| Conference participant mute | Pass | Compiled unchanged after import correction |
| Recording byte download | Pass | Compiled unchanged after import correction |
| Queue-member redirect | Pass | Compiled unchanged after import correction |
| SIP domain and credential setup | Pass | Compiled unchanged after import correction |
| SMS send | Pass | Compiled unchanged after import correction |
| MMS send | Pass | Compiled unchanged after import correction |
| Messaging Service sender addition | Pass | Compiled unchanged after import correction |

This is a Codex self-review, not an independent docs-only usability result: the reviewer had
previously inspected implementation signatures while working on this feature. Therefore this
9/10 construction exercise is useful diagnostic evidence, but **does not close SC-010/T123**.

## Independent docs-only first-attempt review

On 2026-09-22, a fresh reviewer agent received no implementation history and used only the
root README, generated public `dialkit` rustdoc, and the client package manifest. It did not
inspect implementation source, existing tests, prewritten examples, this review, or the task
list. The reviewer wrote ten original workflow functions in
`crates/client/tests/usability_fresh_review.rs` before running its first compile check. The
unchanged first draft has SHA-256
`19176886cc0bd816774670cad50825d477506f4109b5514727d89a7522ab05e9`.
The first command, `cargo test -p dialkit --test usability_fresh_review --no-run`, exited 0;
there were no compiler errors or post-draft corrections. The functions deliberately perform
no network calls in CI; this review measures construction and compilation, not Twilio delivery.
After recording the first-attempt result, `cargo fmt --all` made formatting-only changes to
the file; its formatted SHA-256 is
`b85b3d34cc435cf038b3db5ac874a9c2502828ba87164b0709fb50edd8b61463`.

| Original task construction | First compile | Usability outcome |
|---|---|---|
| Send an SMS | Pass | Pass |
| Send an MMS | Pass | Pass |
| Send using a Messaging Service | Pass | Pass |
| Fetch message status | Pass | Pass |
| List messages | Pass | Ambiguous: calls the result `first_page_sids`, but caps an item-wise pager at 20; it does not guarantee one provider page |
| Place a call using a URL | Pass | Pass |
| Place a call using inline TwiML | Pass | Pass |
| List completed calls | Pass | Pass |
| Start and download a recording | Pass | Pass |
| Verify and parse an inbound SMS webhook | Pass | Pass |

**Conservative SC-010 score: 9/10 workflows completed on the first attempt.** All ten
type-checked, but the misleading page boundary in the list-messages construction is counted
as a usability miss rather than silently treated as success. The review also suggests making
the distinction between `Pager::next` (one item) and `Pager::pages` (provider pages) clearer
in public documentation.
