# Contract: Protocol Behavior and Evidence

## Request routing and authentication

API-2010 operations resolve only against their API endpoint profile; Messaging-v1 operations resolve only against their Messaging profile. Basic credentials are attached after URL resolution and origin validation. Production defaults require HTTPS. Local HTTP requires explicit test configuration.

Every selected operation preserves the pinned method, path, parameter name/location/cardinality/requiredness, media type, documented success shape, and Twilio error normalization. Request traces use route templates, never concrete paths, URLs, query strings, headers, or bodies.

## Success, error, and security

- Documented 2xx responses, including empty 204 bodies, are typed successes.
- Non-2xx responses attempt Twilio error parsing independent of the generated success type.
- Stable errors retain status, optional code/message/more-info, request ID, attempts, retry-after, and safe timing/concurrency metadata.
- Malformed error bodies are bounded and redacted; raw generated/Reqwest errors and payloads never escape.
- Mutation retry stays conservative; 429 behavior and explicit mutation opt-in retain existing policy.
- Redaction canaries cover credentials, concrete SIDs/numbers, query/form/JSON values, message/TwiML, media URLs, signatures, SIP credentials, payment fields, redirects, generated error content, and unknown bags.

## Pagination

Both `next_page_uri` and `meta.next_page_url` adapt to the same pager contract. Initial filters are sent once. Continuations are opaque, order is preserved, absent/null ends traversal, repeated values fail once, and an origin mismatch is rejected before credentials attach. Absolute and relative continuations and reserved cursor characters have fixtures.

## Forward compatibility

Generated string enums serialize/deserialise unknown values verbatim. Existing generated structs tolerate additive unknown JSON fields by ignoring them. Stable open values preserve the raw string. Webhook events additionally preserve all unconsumed values. Forward compatibility does not add an operation absent from the pinned manifest.

## Media behavior

JSON metadata and binary download use distinct APIs. Byte responses are streamed, can be cancelled, and never pass through JSON decoding. Redirects follow a documented credential-safe policy. Fixtures cover binary content, empty content, content metadata, redirect acceptance/rejection, timeout, and structured/non-structured errors.

## TwiML behavior

- One case-sensitive `Response` root and UTF-8 XML declaration.
- Builder insertion order is output order.
- Text/attributes are XML-writer escaped; no raw interpolation.
- Every matrix node, attribute, legal relationship, enum/range, repeated child, and content rule has canonical/negative evidence.
- Forbidden relationships fail before output; parent-specific APIs make them unrepresentable where practical.
- Control-transfer/terminal rules are validated or explicitly documented when static rejection would break valid dynamic flows.
- Existing goldens remain byte-equivalent.

## Webhook behavior

Verification consumes the exact externally visible URL plus original form pairs or raw JSON bytes. Form canonicalization retains repeated and empty pairs; JSON verifies `bodySHA256` before the signature. Comparison is constant-time. Strict URL behavior remains default and port compatibility remains explicit.

Parsing occurs only for an explicitly selected family. Known fields are typed; unknown fields and open values survive. Validity does not imply replay protection, idempotency, ordering, or persistence. Payment parsing exposes only documented safe result/token metadata and never stores sensitive payment values.

## Evidence ledger

For each REST entry:

| Dimension | Required proof |
|-----------|----------------|
| Method/path | Local server receives exact method and encoded path. |
| Authentication | Synthetic Basic credentials attached only to trusted origin. |
| Parameters | Every applicable location, name, omission, repetition, and encoding is checked. |
| Success | Every documented success category used by the operation decodes, including empty bodies. |
| Error | Structured and malformed Twilio failures normalize safely. |
| Pagination | Multi/zero-page, missing/absolute/reserved/loop/cross-origin cases, or explicit N/A. |

For each TwiML entry: canonical output, every attribute constraint, every legal parent relationship, escaping/order, and at least one relevant invalid case.

For each webhook entry: valid and invalid signature, known-field parse, unknown field/value retention, every applicable encoding, and one relevant malformed/forward-compatibility case.

All fixtures are synthetic. A fixture containing a real credential, signature tied to a real token, user message, SIP password, or payment datum is a release failure.

## Compatibility evidence

Before implementation, freeze current public compile fixtures and existing Calls/Messages wire, TwiML XML, webhook vector, pagination, and error results. Every phase reruns them plus cargo-semver-checks. Any intended additive change requires a focused regression fixture; an unavoidable break is deferred to a major release.

## Completion gates

No entry is complete with missing or failing evidence. The release gate requires 197/197 REST entries, every TwiML entry/relationship, every webhook family/variant, zero compatibility regressions, all examples/docs, clean regeneration, security scans, and a release report generated in under the SC-007 budget.
