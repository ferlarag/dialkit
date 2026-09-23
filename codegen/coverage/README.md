# Coverage manifests

Coverage is keyed by immutable source identity rather than generated names.

## REST schema (`dialkit.rest-coverage.v1`)

Each record has a unique `(source_id, operation_id)` key, domain, exact method/path, generated
crate and symbol, delivery phase, pagination flag, and evidence for method/path, authentication,
parameters, success, error, and pagination. Evidence is a repository-relative fixture/test
reference or `{ n_a = "reason" }`; blank evidence is invalid.

## TwiML schema (`dialkit.twiml-coverage.v1`)

Each record is keyed by `(family, node, parent_context)` and declares kind, attributes, content,
control-flow behavior, canonical output, legal nesting, escaping, and invalid-case evidence.

## Webhook schema (`dialkit.webhook-coverage.v1`)

Each record is keyed by `(family, variant, encoding)` and declares known fields, open values,
unknown-retention policy, response behavior, valid/invalid signature vectors, typed parsing, and
forward-compatibility evidence.

All manifests are normative, version controlled, and synthetic-data only. Reviewers must update
the matching pinned source or dated documentation snapshot, tests, and release evidence together.
