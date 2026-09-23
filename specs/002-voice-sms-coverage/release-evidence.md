# Release evidence

Status: **PASS**. Generated from executed local gates and pinned repository inputs; no live Twilio traffic was used.

## Immutable inputs

Twilio OpenAPI commit: `5aa7f31977ce5812f7b7bc1f46a38555ebaa2888`  
OpenAPI Generator: `7.25.0` (`41ce4f6b07f196676439d710759fa1ced7a08066d06ff1bf314681470289efae`)  
Template revision/hash: `dialkit-rust-v1` / `4328530d019d0424febe49a23ca9123c037c8d7db12cf922b652248f3fb99ed0`

| Source | Vendored file | Spec SHA-256 | Config SHA-256 | Output inventory SHA-256 | Selected |
|---|---|---|---|---|---:|
| API-2010 | `codegen/spec/twilio_api_v2010.json` | `170b3ccd0f891416840083d72f1795b1499b14a18d4873fd2b39f47ef84642d6` | `3ce7c1eb52b9953812bd9e53a125fcb3d5068e0298014012f335851a60bb4b40` | `97dae50ad9424ef4f5ed1c773da8898f634691982f59cbcc5109d7def6a9188b` | 139 |
| MSG-V1 | `codegen/spec/twilio_messaging_v1.json` | `611c6fde586347615039a7d8c87a4b10d39c6d8566b7d1a691353ef995aaa473` | `692712009be60d6605c97084b861d3341f281ffb0d28e7636bb842bbae55db3e` | `97dae50ad9424ef4f5ed1c773da8898f634691982f59cbcc5109d7def6a9188b` | 58 |

## Coverage result

| Evidence | Result |
|---|---:|
| API-2010 | 139 |
| Messaging v1 | 58 |
| REST union | 197 |
| TwiML relationships | 37 |
| Webhook families | 16 |
| Missing/invalid evidence | 0 |

### Pinned REST matrix rows

| Source | Domain/resource | Pinned | Supported | Audit |
|---|---|---:|---:|---|
| API-2010 | Voice applications | 5 | 5 | PASS |
| API-2010 | Calls | 5 | 5 | PASS |
| API-2010 | Call events | 1 | 1 | PASS |
| API-2010 | Call notifications | 2 | 2 | PASS |
| API-2010 | Call recordings | 5 | 5 | PASS |
| API-2010 | Real-time call transcriptions | 2 | 2 | PASS |
| API-2010 | Conferences | 3 | 3 | PASS |
| API-2010 | Conference recordings | 4 | 4 | PASS |
| API-2010 | Conference participants | 5 | 5 | PASS |
| API-2010 | Voice payments | 2 | 2 | PASS |
| API-2010 | Queues | 5 | 5 | PASS |
| API-2010 | Queue members | 3 | 3 | PASS |
| API-2010 | Recordings | 3 | 3 | PASS |
| API-2010 | Recording add-on results | 3 | 3 | PASS |
| API-2010 | Recording add-on payloads | 3 | 3 | PASS |
| API-2010 | Recording add-on payload data | 1 | 1 | PASS |
| API-2010 | Recording transcriptions | 3 | 3 | PASS |
| API-2010 | Account transcriptions | 3 | 3 | PASS |
| API-2010 | SIP call credential-list auth mappings | 4 | 4 | PASS |
| API-2010 | SIP call IP-ACL auth mappings | 4 | 4 | PASS |
| API-2010 | SIP registration credential-list auth mappings | 4 | 4 | PASS |
| API-2010 | SIP credentials | 5 | 5 | PASS |
| API-2010 | SIP credential lists | 5 | 5 | PASS |
| API-2010 | SIP domain credential-list mappings | 4 | 4 | PASS |
| API-2010 | SIP domains | 5 | 5 | PASS |
| API-2010 | SIP IP access-control lists | 5 | 5 | PASS |
| API-2010 | SIP domain IP-ACL mappings | 4 | 4 | PASS |
| API-2010 | SIP IP addresses | 5 | 5 | PASS |
| API-2010 | SIPREC sessions | 2 | 2 | PASS |
| API-2010 | Call media streams | 2 | 2 | PASS |
| API-2010 | Voice client tokens | 1 | 1 | PASS |
| API-2010 | Call user-defined messages | 1 | 1 | PASS |
| API-2010 | Call user-defined message subscriptions | 2 | 2 | PASS |
| API-2010 | Outgoing caller IDs | 4 | 4 | PASS |
| API-2010 | Caller-ID validation | 1 | 1 | PASS |
| API-2010 | Incoming phone numbers | 5 | 5 | PASS |
| API-2010 | Local incoming phone numbers | 2 | 2 | PASS |
| API-2010 | Mobile incoming phone numbers | 2 | 2 | PASS |
| API-2010 | Toll-free incoming phone numbers | 2 | 2 | PASS |
| API-2010 | Messages | 5 | 5 | PASS |
| API-2010 | Message feedback | 1 | 1 | PASS |
| API-2010 | Message media collection | 1 | 1 | PASS |
| API-2010 | Message media instances | 2 | 2 | PASS |
| API-2010 | Account SMS short codes | 3 | 3 | PASS |
| MSG-V1 | Alpha senders | 4 | 4 | PASS |
| MSG-V1 | A2P brand registrations | 4 | 4 | PASS |
| MSG-V1 | A2P brand registration OTP | 1 | 1 | PASS |
| MSG-V1 | A2P brand vettings | 3 | 3 | PASS |
| MSG-V1 | Channel senders | 4 | 4 | PASS |
| MSG-V1 | Deactivations | 1 | 1 | PASS |
| MSG-V1 | Destination alpha senders | 4 | 4 | PASS |
| MSG-V1 | Link-shortening domain certificates | 3 | 3 | PASS |
| MSG-V1 | Link-shortening domain configuration | 2 | 2 | PASS |
| MSG-V1 | Service domain configuration | 1 | 1 | PASS |
| MSG-V1 | Link-shortening DNS validation | 1 | 1 | PASS |
| MSG-V1 | Externally registered US A2P campaigns | 1 | 1 | PASS |
| MSG-V1 | Service/domain link-shortening association | 2 | 2 | PASS |
| MSG-V1 | Service link-shortening domain lookup | 1 | 1 | PASS |
| MSG-V1 | Service phone-number senders | 4 | 4 | PASS |
| MSG-V1 | Managed certificate requests | 1 | 1 | PASS |
| MSG-V1 | Messaging Services | 5 | 5 | PASS |
| MSG-V1 | Service short-code senders | 4 | 4 | PASS |
| MSG-V1 | Toll-free verifications | 5 | 5 | PASS |
| MSG-V1 | US A2P campaigns | 5 | 5 | PASS |
| MSG-V1 | US A2P campaign use case | 1 | 1 | PASS |
| MSG-V1 | Messaging Service use cases | 1 | 1 | PASS |

## Executed gates

| Gate | Result | Duration | Log |
|---|---|---:|---|
| `task_completion` | **PASS** | 0 s | `target/release-evidence/logs/task_completion.log` |
| `clean_regeneration` | **PASS** | 18 s | `target/release-evidence/logs/clean_regeneration.log` |
| `coverage_audit_bats` | **PASS** | 66 s | `target/release-evidence/logs/coverage_audit_bats.log` |
| `upstream_diff_bats` | **PASS** | 0 s | `target/release-evidence/logs/upstream_diff_bats.log` |
| `coverage_report` | **PASS** | 2 s | `target/release-evidence/logs/coverage_report.log` |
| `formatting` | **PASS** | 1 s | `target/release-evidence/logs/formatting.log` |
| `clippy` | **PASS** | 37 s | `target/release-evidence/logs/clippy.log` |
| `workspace_tests` | **PASS** | 144 s | `target/release-evidence/logs/workspace_tests.log` |
| `doctests` | **PASS** | 5 s | `target/release-evidence/logs/doctests.log` |
| `documentation` | **PASS** | 24 s | `target/release-evidence/logs/documentation.log` |
| `compatibility` | **PASS** | 38 s | `target/release-evidence/logs/compatibility.log` |
| `workflow_examples` | **PASS** | 21 s | `target/release-evidence/logs/workflow_examples.log` |
| `workflow_contracts` | **PASS** | 5 s | `target/release-evidence/logs/workflow_contracts.log` |
| `default_features` | **PASS** | 21 s | `target/release-evidence/logs/default_features.log` |
| `no_default_features` | **PASS** | 20 s | `target/release-evidence/logs/no_default_features.log` |
| `rustls_features` | **PASS** | 1 s | `target/release-evidence/logs/rustls_features.log` |
| `native_tls_features` | **PASS** | 20 s | `target/release-evidence/logs/native_tls_features.log` |
| `performance` | **PASS** | 48 s | `target/release-evidence/logs/performance.log` |
| `sensitive_data` | **PASS** | 1 s | `target/release-evidence/logs/sensitive_data.log` |
| `semver` | **PASS** | 48 s | `target/release-evidence/logs/semver.log` |

Semver baseline: `v0.1.0`. Total release-evidence duration: 520 seconds.  
Machine-readable ledger: `target/release-evidence/gate-results.json`.

## Exceptions

None.

CI-only environment matrices such as the Rust 1.85 runner are not claimed by this local report; publication still requires their independent CI results.
