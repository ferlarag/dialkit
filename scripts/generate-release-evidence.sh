#!/usr/bin/env bash
set -uo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
evidence_dir="$repo_root/target/release-evidence"
logs_dir="$evidence_dir/logs"
results_tsv="$evidence_dir/gates.tsv"
mkdir -p "$logs_dir"
: >"$results_tsv"
started=$SECONDS
failed=0

run_gate() {
  local name=$1
  shift
  local gate_started=$SECONDS
  local log="$logs_dir/$name.log"
  local status
  if (cd "$repo_root" && "$@") >"$log" 2>&1; then
    status=pass
  else
    status=fail
    failed=1
  fi
  printf '%s\t%s\t%s\t%s\n' "$name" "$status" "$((SECONDS - gate_started))" "target/release-evidence/logs/$name.log" >>"$results_tsv"
  printf '%-24s %s\n' "$name" "${status^^}"
}

check_task_completion() {
  if rg -n '^- \[ \]' specs/002-voice-sms-coverage/tasks.md; then
    echo "release candidate still has incomplete spec-kit tasks" >&2
    return 1
  fi
}

run_gate task_completion check_task_completion
run_gate clean_regeneration ./codegen/regenerate.sh --check
run_gate coverage_audit_bats bats codegen/tests/coverage_audit.bats
run_gate upstream_diff_bats bats codegen/tests/upstream_diff.bats
run_gate coverage_report ./codegen/report-coverage.sh
run_gate formatting cargo fmt --all -- --check
run_gate clippy cargo clippy --workspace --all-targets --all-features -- -D warnings
run_gate workspace_tests cargo test --workspace --all-features
run_gate doctests cargo test --workspace --doc --all-features
run_gate documentation cargo doc --workspace --all-features --no-deps
run_gate compatibility cargo test -p dialkit --test compatibility --all-features
run_gate workflow_examples cargo check -p dialkit --examples --all-features
run_gate workflow_contracts cargo test -p dialkit --test contracts workflow_facades --all-features
run_gate default_features cargo check -p dialkit
run_gate no_default_features cargo check -p dialkit --no-default-features
run_gate rustls_features cargo check -p dialkit --no-default-features --features rustls-tls,webhooks,twiml
run_gate native_tls_features cargo check -p dialkit --no-default-features --features native-tls,webhooks,twiml
run_gate performance cargo bench -p dialkit --bench performance
run_gate sensitive_data ./scripts/audit-sensitive-data.sh

baseline=$(git -C "$repo_root" describe --tags --abbrev=0 2>/dev/null || true)
if [[ -n "$baseline" ]]; then
  if command -v cargo-semver-checks >/dev/null 2>&1; then
    run_gate semver cargo semver-checks check-release -p dialkit --baseline-rev "$baseline"
  else
    printf '%s\t%s\t%s\t%s\n' semver fail 0 "target/release-evidence/logs/semver.log" >>"$results_tsv"
    printf '%s\n' "cargo-semver-checks is required for baseline $baseline" >"$logs_dir/semver.log"
    printf '%-24s %s\n' semver FAIL
    failed=1
  fi
else
  baseline="none (first release)"
  printf '%s\t%s\t%s\t%s\n' semver not-applicable 0 "target/release-evidence/logs/semver.log" >>"$results_tsv"
  printf '%s\n' "No tagged baseline exists; semver comparison is not applicable." >"$logs_dir/semver.log"
  printf '%-24s %s\n' semver NOT-APPLICABLE
fi

python3 - "$repo_root" "$((SECONDS - started))" "$baseline" "$failed" <<'PY'
import datetime
import json
import pathlib
import platform
import subprocess
import sys
import tomllib

root = pathlib.Path(sys.argv[1])
elapsed = int(sys.argv[2])
baseline = sys.argv[3]
shell_failed = bool(int(sys.argv[4]))
evidence_dir = root / "target/release-evidence"

required = {
    "task_completion", "clean_regeneration", "coverage_report", "coverage_audit_bats", "upstream_diff_bats", "formatting", "clippy",
    "workspace_tests", "doctests", "documentation", "compatibility",
    "workflow_examples", "workflow_contracts", "default_features",
    "no_default_features", "rustls_features", "native_tls_features",
    "performance", "sensitive_data", "semver",
}
gates = []
for line in (evidence_dir / "gates.tsv").read_text().splitlines():
    name, status, duration, log = line.split("\t")
    gates.append({"name": name, "status": status, "duration_seconds": int(duration), "log": log})

names = {gate["name"] for gate in gates}
missing_gates = sorted(required - names)
duplicate_gates = sorted(name for name in names if sum(g["name"] == name for g in gates) != 1)
bad_gates = [gate["name"] for gate in gates if gate["status"] not in {"pass", "not-applicable"}]

coverage_path = root / "target/coverage/report.json"
coverage = json.loads(coverage_path.read_text()) if coverage_path.exists() else None
coverage_valid = bool(
    coverage
    and coverage.get("status") == "pass"
    and coverage.get("api_2010") == 139
    and coverage.get("messaging_v1") == 58
    and coverage.get("rest_total") == 197
    and coverage.get("missing") == 0
    and len(coverage.get("matrix_rows", [])) == 66
    and all(row.get("status") == "pass" and row.get("actual") == row.get("expected") for row in coverage.get("matrix_rows", []))
    and sum(row["actual"] for row in coverage.get("matrix_rows", []) if row["source_id"] == "API-2010") == 139
    and sum(row["actual"] for row in coverage.get("matrix_rows", []) if row["source_id"] == "MSG-V1") == 58
    and len(coverage.get("executable_results", [])) == 2925
    and all(entry.get("status") == "pass" for entry in coverage.get("executable_results", []))
    and len({entry.get("selector") for entry in coverage.get("executable_results", [])}) == 2925
)

with (root / "codegen/generation-manifest.toml").open("rb") as handle:
    manifest = tomllib.load(handle)

overall_pass = not (shell_failed or missing_gates or duplicate_gates or bad_gates) and coverage_valid
result = {
    "schema_version": 1,
    "generated_at_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
    "status": "pass" if overall_pass else "fail",
    "duration_seconds": elapsed,
    "baseline": baseline,
    "toolchain": {
        "rustc": subprocess.run(["rustc", "--version"], check=True, capture_output=True, text=True).stdout.strip(),
        "cargo": subprocess.run(["cargo", "--version"], check=True, capture_output=True, text=True).stdout.strip(),
        "platform": platform.platform(),
    },
    "gates": gates,
    "missing_gates": missing_gates,
    "duplicate_gates": duplicate_gates,
    "failed_gates": bad_gates,
    "coverage": coverage,
}
(evidence_dir / "gate-results.json").write_text(json.dumps(result, indent=2) + "\n")

sources = "\n".join(
    f'| {source["id"]} | `{source["spec"]}` | `{source["sha256"]}` | '
    f'`{source["config_sha256"]}` | `{source["inventory_sha256"]}` | {source["selected_operations"]} |'
    for source in manifest["sources"]
)
gate_rows = "\n".join(
    f'| `{gate["name"]}` | **{gate["status"].upper()}** | {gate["duration_seconds"]} s | `{gate["log"]}` |'
    for gate in gates
)
coverage_rows = (
    f'| API-2010 | {coverage["api_2010"]} |\n'
    f'| Messaging v1 | {coverage["messaging_v1"]} |\n'
    f'| REST union | {coverage["rest_total"]} |\n'
    f'| TwiML relationships | {coverage["twiml_entries"]} |\n'
    f'| Webhook families | {coverage["webhook_families"]} |\n'
    f'| Missing/invalid evidence | {coverage["missing"]} |'
    if coverage else "| Coverage report | MISSING |"
)
matrix_rows = "\n".join(
    f'| {row["source_id"]} | {row["domain"]} | {row["expected"]} | {row["actual"]} | {row["status"].upper()} |'
    for row in coverage.get("matrix_rows", [])
) if coverage else "| MISSING | MISSING | — | — | FAIL |"
problems = missing_gates + duplicate_gates + bad_gates
exceptions = "None." if overall_pass else "Blocking results: " + ", ".join(problems or ["invalid or missing coverage report"])
status = "PASS" if overall_pass else "FAIL"
content = f'''# Release evidence

Status: **{status}**. Generated from executed local gates and pinned repository inputs; no live Twilio traffic was used.

## Immutable inputs

Twilio OpenAPI commit: `{manifest["repository"]["commit"]}`  
OpenAPI Generator: `{manifest["generator"]["version"]}` (`{manifest["generator"]["artifact_sha256"]}`)  
Template revision/hash: `{manifest["templates"]["revision"]}` / `{manifest["templates"]["tree_sha256"]}`

| Source | Vendored file | Spec SHA-256 | Config SHA-256 | Output inventory SHA-256 | Selected |
|---|---|---|---|---|---:|
{sources}

## Coverage result

| Evidence | Result |
|---|---:|
{coverage_rows}

### Pinned REST matrix rows

| Source | Domain/resource | Pinned | Supported | Audit |
|---|---|---:|---:|---|
{matrix_rows}

## Executed gates

| Gate | Result | Duration | Log |
|---|---|---:|---|
{gate_rows}

Semver baseline: `{baseline}`. Total release-evidence duration: {elapsed} seconds.  
Machine-readable ledger: `target/release-evidence/gate-results.json`.

## Exceptions

{exceptions}

CI-only environment matrices such as the Rust 1.85 runner are not claimed by this local report; publication still requires their independent CI results.
'''
(root / "specs/002-voice-sms-coverage/release-evidence.md").write_text(content)

if not overall_pass:
    raise SystemExit("release evidence failed: " + exceptions)
PY
python_status=$?

if (( failed != 0 || python_status != 0 )); then
  echo "release evidence contains blocking failures" >&2
  exit 1
fi

echo "wrote specs/002-voice-sms-coverage/release-evidence.md and target/release-evidence/gate-results.json"
