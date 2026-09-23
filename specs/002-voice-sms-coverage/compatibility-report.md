# Compatibility assessment

The feature is additive for the stable `dialkit` facade.

- Existing Calls, Messages, `Pager`, `Page`, `Error`, `ApiError`, TwiML entry points, webhook
  validator methods, TLS defaults, and `ClientBuilder::base_url` remain available with frozen
  compile and byte-level behavior fixtures.
- `messaging_base_url` is independent and does not alter API-2010 routing.
- Generated API-2010 modules and common function names remain in place. Messaging v1 is isolated in
  the sibling `dialkit-messaging-generated` crate and is not re-exported by `dialkit`.
- New domain handles, typed identifiers, media streams, TwiML nodes, and verified webhook parsers
  are additive and expose neither Reqwest nor generated model types.
- Generated response/debug formatting now redacts response bodies and typed entities. This is an
  intentional security correction; success/error categories and stable facade accessors are
  unchanged.
- The older `Start::stream(name)` and `Start::siprec(name)` calls retain their XML bytes even though
  those name-only shapes cannot establish a new session under the documented Stream/SIPREC rules.
  New `Start::stream_to(name, wss_url)` and `Start::siprec_with_connector(name, connector_name)`
  provide complete forms. Fresh generic `TwimlNode` construction rejects missing required
  attributes under `Start`. The legacy builders are a deliberate compatibility exception, not a
  claim that their name-only XML is sufficient for Twilio execution.
- `Pay::prompt(text)` likewise retains its prior XML; new `Pay::prompt_for(step, text)` emits the
  documented required payment step, and fresh generic `TwimlNode` prompts require that step.

The compatibility exceptions above are owned by the dialkit maintainers and
expire at the next major-version review. Until then, documentation directs
new users to the complete builders; the frozen methods remain covered by XML
compatibility fixtures.

The compatibility, generated-path, TLS/feature, and semver jobs are release gates. A future
upstream pin change requires a new assessment.
