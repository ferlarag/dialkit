# Contract: Twilio Protocol Behavior

## Outbound request lifecycle

For every api_v2010 operation, the SDK performs these observable stages:

1. Validate client and operation input without exposing secrets.
2. Resolve the endpoint only against the configured base origin.
3. Serialize path, query, form, multipart, or body values exactly as required by the pinned specification.
4. Add Basic authentication from the selected credential form over HTTPS.
5. Apply connect/total timeouts and operation-aware retry policy.
6. Emit allowlisted tracing fields.
7. Interpret success into the operation result or normalize failure into the stable error model.

The SDK must not panic for service input, response, timeout, cancellation, unknown enum, or malformed error cases.

## Authentication

- Account-token mode uses account SID as Basic username and auth token as password.
- API-key mode uses key SID as Basic username and key secret as password; account SID still scopes resource paths.
- `Authorization`, tokens, and secrets never appear in errors, `Debug`, traces, fixtures, or docs.
- Production/default endpoints require HTTPS. Plain HTTP is permitted only for an explicitly configured local test endpoint.

Contract fixtures assert the decoded Basic username only inside the test server; fixtures use fake values and never persist the encoded authorization header.

## Encoding

- Path segments are encoded as segments, not interpolated raw.
- Query values preserve repeated parameters and omit absent options.
- Create/update requests use the media type declared by the pinned operation, commonly form encoding and multipart where applicable.
- The request route template may be traced; complete URLs, query strings, and payloads may not.
- Representative Calls, Messages, Recordings, and Conferences fixtures verify field naming and encoding.

## Success and failure interpretation

- Every 2xx response documented by the pinned operation is treated as success, including an empty body when documented.
- Non-2xx responses attempt Twilio error decoding regardless of declared generated model.
- JSON/XML canonical Twilio errors preserve status, message, optional integer code, and optional `more_info`.
- `Twilio-Request-Id`, `Twilio-Concurrent-Requests`, and `Twilio-Request-Duration` are captured when present.
- A malformed or undocumented error becomes a decode/API error with status, safe headers, request ID, and a bounded redacted excerpt.
- Error excerpts have a fixed maximum size and apply the same credential/PII redaction before storage or formatting.

## Retry and timeout matrix

| Situation | Safe read/list default | Mutation default | Explicit mutation opt-in |
|-----------|------------------------|------------------|--------------------------|
| HTTP 429 | Retry; honor `Retry-After` | Retry because Twilio documents the request as unprocessed | Retry |
| Connect failure before request transmission is known | Retry | Do not retry unless transport proves no bytes were sent | Retry within configured bounds |
| HTTP 500/502/503/504 | Retry | Do not retry | Retry with duplicate-side-effect warning |
| Ambiguous timeout after transmission | Retry only for idempotent read | Do not retry | Retry with duplicate-side-effect warning |
| Validation/authentication/other 4xx | Do not retry | Do not retry | Do not retry |
| Decode or cross-origin continuation error | Do not retry | Do not retry | Do not retry |

All retries use bounded exponential backoff with full jitter and a maximum attempt count. `Retry-After` takes precedence within the configured delay cap. Cancellation stops pending delay/request work. Final errors report attempt count without payload or credentials.

## Pagination

- Start with the developer's filter and optional page-size request.
- Yield items in response order.
- Treat `next_page_uri` as opaque; do not reconstruct `Page`, `PageToken`, or `AfterSid`.
- Resolve relative continuation URIs against the configured base URL.
- Before authenticating, require the continuation to match configured scheme, host, and effective port.
- Stop when continuation is absent/null.
- Detect a repeated continuation URI and fail once to prevent cycles.
- A page-fetch failure is emitted exactly once, after any previously yielded items, then the stream terminates.

## Unknown enum values

- Generated string enum deserialization maps known strings to known variants and every other string to a raw-value-preserving variant.
- Facade conversion maps both known and unknown values into string-backed public values.
- Serializing an unknown value emits the original string unchanged.
- Contract fixtures include at least one unknown value for every generated enum shape present in api_v2010.

## Tracing

Allowed library-generated span/event fields:

- operation name;
- HTTP method;
- route template without concrete identifiers/query;
- attempt number;
- response status;
- Twilio request ID;
- elapsed duration;
- high-level outcome/error category.

Forbidden fields include Basic authorization, account/token/key secrets, concrete phone numbers/SIDs in paths, full URL/query, headers not explicitly allowlisted, form/body content, message text, media URLs, and TwiML. Capture-layer tests seed unique canary values into every sensitive position and require zero matches in all events and formatted errors.

## Webhook form signature

Inputs are the exact externally visible URL, all received form name/value pairs including duplicates, `X-Twilio-Signature`, and the auth token.

Behavior:

1. Reject missing/malformed input as `WebhookValidation` error.
2. Preserve URL spelling and query encoding; do not normalize.
3. Apply Twilio's documented name/value ordering and concatenation, including repeated values.
4. Compute HMAC-SHA1 with the auth token and Base64-encode the digest.
5. Compare the decoded signature using the HMAC library's constant-time verification.
6. Return `Valid` or `Invalid(SignatureMismatch)`.

Strict mode tries only the supplied URL. Opt-in port compatibility may try the documented official-helper with/without-port alternatives and reports no secret intermediate values.

## Webhook JSON signature

Inputs are the exact externally visible URL containing `bodySHA256`, unmodified raw body bytes, signature header, and auth token.

Behavior:

1. Parse the `bodySHA256` query value without changing the URL.
2. Compare it to the SHA-256 of the raw bytes; mismatch returns `Invalid(BodyDigestMismatch)`.
3. Apply the Twilio HMAC signature algorithm to the exact URL and verify in constant time.
4. Never parse and reserialize the JSON for validation.

Shared-key webhook signing and framework-specific URL reconstruction are outside this contract.

## TwiML

- Output is UTF-8 XML with one `<Response>` root.
- Element and attribute names match Twilio's case-sensitive spelling.
- Text and attribute values are escaped by the XML writer.
- Node order matches builder insertion order.
- Invalid root multiplicity, missing required values, or forbidden nesting fails before output.
- Voice and Messaging builders cannot be mixed into an invalid response mode.
- Snapshot and property tests cover all initial nodes, nested Gather/Dial/Message structures, escaping, empty text, Unicode, and invalid nesting.

## Test endpoint safety

All default CI contracts run against a loopback mock server using fake `AC`, `SK`, phone-number, token, message, and request-ID values. Live tests are opt-in, never run on pull requests by default, and use Twilio test credentials/magic values only for documented supported scenarios. Live tests must not expect callbacks or real calls/messages.
