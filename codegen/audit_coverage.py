#!/usr/bin/env python3
"""Join pinned coverage entries to tests that passed in this audit run."""

from __future__ import annotations

import argparse
import json
import pathlib
import re
import subprocess
import sys
import time
import tomllib

ROOT = pathlib.Path(__file__).resolve().parents[1]
SUITES = {
    "api": ["-p", "dialkit-api-generated", "--test", "runtime_contracts"],
    "msg": ["-p", "dialkit-messaging-generated", "--test", "runtime_contracts"],
    "api_pages": ["-p", "dialkit-api-generated", "--test", "pagination_contracts"],
    "msg_pages": ["-p", "dialkit-messaging-generated", "--test", "pagination_contracts"],
    "twiml": ["-p", "dialkit", "--test", "contracts", "twiml"],
    "webhook": ["-p", "dialkit", "--test", "webhook_runtime_matrix"],
}


def load(path):
    with path.open("rb") as handle:
        return tomllib.load(handle)


def ident(value):
    value = re.sub(r"(.)([A-Z][a-z]+)", r"\1_\2", value)
    value = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", value)
    return re.sub(r"[^a-zA-Z0-9]+", "_", value).strip("_").lower()


def webhook_ident(value):
    return re.sub(r"[^a-zA-Z0-9]+", "_", value).strip("_").lower()


def run_suites():
    results, errors = {}, []
    for suite, args in SUITES.items():
        result = subprocess.run(
            ["cargo", "test", *args, "--", "--test-threads=8"],
            cwd=ROOT, capture_output=True, text=True,
        )
        results[suite] = set(re.findall(r"^test ([\w:]+) \.\.\. ok$", result.stdout, re.M))
        if result.returncode:
            errors.append(f"{suite}: executable suite failed")
            errors += result.stdout.splitlines()[-10:] + result.stderr.splitlines()[-10:]
    return results, errors


def resolve(spec, value):
    reference = value.get("$ref") if isinstance(value, dict) else None
    if not reference:
        return value
    found = spec
    for part in reference.removeprefix("#/").split("/"):
        found = found[part]
    return found


CONSTRAINT_KEYS = {"enum", "pattern", "minimum", "maximum", "minLength", "maxLength", "minItems", "maxItems", "format"}
REST_DIMENSIONS = ("path_parameters", "query_parameters", "header_parameters", "form_parameters", "json_body", "constraints", "request_media_type", "response_fields")


def applicable_dimensions(spec, operation, route):
    params = [resolve(spec, p) for p in operation.get("parameters", [])]
    content = operation.get("requestBody", {}).get("content", {})
    form = "application/x-www-form-urlencoded" in content
    json_body = "application/json" in content
    constraints = any(CONSTRAINT_KEYS & resolve(spec, p.get("schema", {})).keys() for p in params)
    for media in content.values():
        schema = resolve(spec, media.get("schema", {}))
        constraints |= any(CONSTRAINT_KEYS & resolve(spec, prop).keys() for prop in schema.get("properties", {}).values())
    response_fields = False
    for response in operation.get("responses", {}).values():
        examples = response.get("content", {}).get("application/json", {}).get("examples", {})
        for example in examples.values():
            value = example.get("value", {})
            if isinstance(value, dict):
                response_fields |= any(
                    (key.endswith("sid") and isinstance(item, str) and re.fullmatch(r"[A-Z]{2}[0-9a-fA-F]{32}", item))
                    or isinstance(item, (list, int, float, bool)) for key, item in value.items()
                )
    return {
        "path_parameters": "{" in route,
        "query_parameters": any(p.get("in") == "query" for p in params),
        "header_parameters": any(p.get("in") == "header" for p in params),
        "form_parameters": form,
        "json_body": json_body,
        "constraints": constraints,
        "request_media_type": form or json_body,
        "response_fields": response_fields,
    }


def pinned_operations(path):
    spec = json.loads(path.read_text())
    found = {}
    for route, methods in spec["paths"].items():
        for method, op in methods.items():
            if isinstance(op, dict) and "operationId" in op:
                found[op["operationId"]] = (method.upper(), route, applicable_dimensions(spec, op, route))
    return found


def audit_matrix_rows(specification, manifest_paths, errors):
    text = specification.read_text()
    rows = []
    for source_id, start, end, manifest_path in (
        ("API-2010", "### REST Coverage Matrix A", "### REST Coverage Matrix B", manifest_paths[0]),
        ("MSG-V1", "### REST Coverage Matrix B", "### TwiML Coverage Matrix", manifest_paths[1]),
    ):
        section = text.split(start, 1)[1].split(end, 1)[0]
        actual_by_domain = {}
        for operation in load(manifest_path).get("operation", []):
            actual_by_domain.setdefault(operation.get("domain", ""), []).append(operation.get("operation_id", ""))
        expected_domains = set()
        for line in section.splitlines():
            if not line.startswith("|"):
                continue
            cells = [cell.strip() for cell in line.split("|")[1:-1]]
            if len(cells) != 3 or not cells[1].isdigit():
                continue
            domain, expected_count = cells[0], int(cells[1])
            expected_ids = re.findall(r"`([A-Z][A-Za-z0-9]+)`", cells[2])
            expected_domains.add(domain)
            actual_ids = actual_by_domain.get(domain, [])
            valid = len(expected_ids) == expected_count and len(actual_ids) == expected_count and sorted(expected_ids) == sorted(actual_ids)
            rows.append({
                "source_id": source_id, "domain": domain,
                "expected": expected_count, "actual": len(actual_ids),
                "operation_ids": expected_ids, "status": "pass" if valid else "fail",
            })
            if not valid:
                errors.append(f"{source_id}:{domain}: matrix-row operation total or identity mismatch")
        for domain in actual_by_domain.keys() - expected_domains:
            errors.append(f"{source_id}:{domain}: domain absent from pinned matrix")
    if len(rows) != 66:
        errors.append(f"matrix row count: expected 66, got {len(rows)}")
    if sum(row["expected"] for row in rows if row["source_id"] == "API-2010") != 139:
        errors.append("API-2010 matrix rows do not total 139")
    if sum(row["expected"] for row in rows if row["source_id"] == "MSG-V1") != 58:
        errors.append("MSG-V1 matrix rows do not total 58")
    return rows


def record(ledger, errors, key, test, valid):
    ledger.append({"selector": key, "test": test, "status": "pass" if valid else "fail"})
    if not valid:
        errors.append(f"{key}: missing or failing per-entry executable evidence")


def audit_rest(path, spec, source_id, count, crate, passed, pages, ledger, errors):
    manifest = load(path)
    rows = manifest.get("operation", [])
    if manifest.get("source_id") != source_id:
        errors.append(f"{source_id}: wrong source_id")
    if len(rows) != count:
        errors.append(f"{source_id}: expected {count}, got {len(rows)}")
    pinned = pinned_operations(spec)
    seen, test_names = set(), set()
    for row in rows:
        op = row.get("operation_id", "")
        prefix = f"{source_id}:{op}"
        if op in seen:
            errors.append(f"{prefix}: duplicate key")
        seen.add(op)
        pinned_row = pinned.get(op)
        if pinned_row is None or pinned_row[:2] != (row.get("method"), row.get("path")):
            errors.append(f"{prefix}: unresolved or substituted method/path")
        symbol = row.get("generated_symbol", "")
        if "::" not in symbol:
            errors.append(f"{prefix}: missing generated symbol")
            continue
        module, function = symbol.split("::", 1)
        api_source = ROOT / f"crates/{crate}/src/apis/{module}.rs"
        if not api_source.is_file() or f"pub async fn {function}(" not in api_source.read_text():
            errors.append(f"{prefix}: unresolved generated symbol")
        runtime = f"contract_{function}"
        if runtime in test_names:
            errors.append(f"{prefix}: shared test-name placeholder")
        test_names.add(runtime)
        runtime_ref = f"crates/{crate}/tests/runtime_contracts.rs#{runtime}"
        runtime_source = (ROOT / f"crates/{crate}/tests/runtime_contracts.rs").read_text()
        for dimension in ("method_path", "authentication", "parameters", "success", "error"):
            valid = (
                row.get("evidence", {}).get(dimension) == runtime_ref
                and f"async fn {runtime}()" in runtime_source
                and runtime in passed
            )
            record(ledger, errors, f"{prefix}:{dimension}", runtime_ref, valid)
        for dimension in REST_DIMENSIONS:
            value = row.get("evidence", {}).get(dimension)
            if pinned_row and pinned_row[2][dimension]:
                valid = value == runtime_ref and f"async fn {runtime}()" in runtime_source and runtime in passed
            else:
                valid = isinstance(value, dict) and value.get("n_a") == f"pinned operation declares no {dimension}"
            record(ledger, errors, f"{prefix}:{dimension}", value, valid)
        value = row.get("evidence", {}).get("pagination")
        if row.get("pagination"):
            name = f"adapter_{function}"
            ref = f"crates/{crate}/tests/pagination_contracts.rs#{name}"
            source = (ROOT / f"crates/{crate}/tests/pagination_contracts.rs").read_text()
            valid = value == ref and f"fn {name}()" in source and name in pages
        else:
            ref = value
            valid = isinstance(value, dict) and bool(value.get("n_a")) and not op.startswith("List")
        record(ledger, errors, f"{prefix}:pagination", ref, valid)
    return len(rows)


def twiml_name(row):
    family, parent, name = row["family"], row["parent"], row["name"]
    if parent == "Response|Gather":
        return f"{family}_response_gather_{ident(name)}"
    if parent == "Pay|Stream|VirtualAgent|Client":
        return "voice_parameter_parents"
    if parent == "Start|Stop":
        return f"voice_start_stop_{ident(name)}"
    return f"{family}_{ident(parent)}_{ident(name)}"


def audit_twiml(path, passed, ledger, errors):
    rows = load(path).get("nodes", [])
    source = (ROOT / "crates/client/tests/contracts/twiml_rendered.rs").read_text()
    seen, selectors = set(), set()
    for row in rows:
        key = (row.get("family"), row.get("parent"), row.get("name"))
        if key in seen:
            errors.append(f"TwiML:{key}: duplicate coverage key")
        seen.add(key)
        if not all(field in row for field in ("family", "name", "parent", "kind", "attributes", "children")):
            errors.append(f"TwiML:{key}: missing row fields")
            continue
        name = twiml_name(row)
        if name in selectors:
            errors.append(f"TwiML:{key}: shared test-name placeholder")
        selectors.add(name)
        declaration = rf'contract!\s*\(\s*{re.escape(name)}\s*,\s*"{re.escape(row["family"])}"\s*,\s*"{re.escape(row["name"])}"\s*,\s*"{re.escape(row["parent"])}"\s*\)'
        record(ledger, errors, "TwiML:" + ":".join(key), name, re.search(declaration, source) is not None and f"twiml_rendered::{name}" in passed)
        invalid = f"invalid_{name}"
        invalid_declaration = rf'invalid_contract!\s*\(\s*{re.escape(invalid)}\s*,\s*"{re.escape(row["family"])}"\s*,\s*"{re.escape(row["name"])}"\s*,\s*"{re.escape(row["parent"])}"\s*\)'
        record(ledger, errors, "TwiML:" + ":".join(key) + ":invalid", invalid,
               re.search(invalid_declaration, source) is not None and f"twiml_rendered::{invalid}" in passed)
    for name in (
        "twiml_manifest::documented_constraints_and_content_exclusion_are_executable",
        "twiml_manifest::terminal_control_flow_is_explicit_without_breaking_legacy_rendering",
    ):
        if name not in passed:
            errors.append(f"TwiML:{name}: missing cross-cutting validation result")
    return len(rows)


def audit_webhooks(path, passed, ledger, errors):
    rows = load(path).get("families", [])
    source = (ROOT / "crates/client/tests/webhook_runtime_matrix.rs").read_text()
    seen = set()
    for row in rows:
        family = row.get("family", "")
        if family in seen:
            errors.append(f"webhooks:{family}: duplicate family")
        seen.add(family)
        if not all(field in row for field in ("variants", "encodings", "parser", "known_fields", "open_values")):
            errors.append(f"webhooks:{family}: missing matrix fields")
            continue
        for variant in row["variants"]:
            for encoding in row["encodings"]:
                name = f"contract_{webhook_ident(family)}_{webhook_ident(variant)}_{webhook_ident(encoding)}"
                record(ledger, errors, f"webhooks:{family}:{variant}:{encoding}", name, f"fn {name}()" in source and name in passed)
    return len(rows)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--api-manifest", type=pathlib.Path, default=ROOT / "codegen/coverage/rest-api-2010.toml")
    parser.add_argument("--messaging-manifest", type=pathlib.Path, default=ROOT / "codegen/coverage/rest-messaging-v1.toml")
    parser.add_argument("--twiml-manifest", type=pathlib.Path, default=ROOT / "codegen/coverage/twiml-nodes.toml")
    parser.add_argument("--webhook-manifest", type=pathlib.Path, default=ROOT / "codegen/coverage/webhook-families.toml")
    parser.add_argument("--output", type=pathlib.Path, default=ROOT / "target/coverage/report.json")
    args = parser.parse_args()
    started = time.monotonic()
    errors = []
    for command in (
        [sys.executable, str(ROOT / "codegen/contracts/generate_runtime_contracts.py"), "--root", str(ROOT), "--check"],
        [sys.executable, str(ROOT / "codegen/contracts/generate_pagination.py"), "--root", str(ROOT), "--crate", str(ROOT / "crates/api-generated"), "--source", "API-2010", "--test-output", str(ROOT / "crates/api-generated/tests/pagination_contracts.rs"), "--check"],
        [sys.executable, str(ROOT / "codegen/contracts/generate_pagination.py"), "--root", str(ROOT), "--crate", str(ROOT / "crates/messaging-generated"), "--source", "MSG-V1", "--test-output", str(ROOT / "crates/messaging-generated/tests/pagination_contracts.rs"), "--check"],
        [sys.executable, str(ROOT / "codegen/contracts/generate_webhook_matrix.py"), "--root", str(ROOT), "--check"],
    ):
        result = subprocess.run(command, cwd=ROOT, capture_output=True, text=True)
        if result.returncode:
            errors.append(f"stale executable evidence: {' '.join(command[1:3])}")
            errors.extend(result.stdout.splitlines() + result.stderr.splitlines())
    passed, suite_errors = run_suites()
    errors += suite_errors
    ledger = []
    api = audit_rest(args.api_manifest, ROOT / "codegen/spec/twilio_api_v2010.json", "API-2010", 139, "api-generated", passed["api"], passed["api_pages"], ledger, errors)
    msg = audit_rest(args.messaging_manifest, ROOT / "codegen/spec/twilio_messaging_v1.json", "MSG-V1", 58, "messaging-generated", passed["msg"], passed["msg_pages"], ledger, errors)
    matrix_rows = audit_matrix_rows(ROOT / "specs/002-voice-sms-coverage/spec.md", (args.api_manifest, args.messaging_manifest), errors)
    twiml = audit_twiml(args.twiml_manifest, passed["twiml"], ledger, errors)
    webhooks = audit_webhooks(args.webhook_manifest, passed["webhook"], ledger, errors)
    report = {
        "status": "pass" if not errors else "fail", "api_2010": api, "messaging_v1": msg,
        "rest_total": api + msg, "twiml_entries": twiml, "webhook_families": webhooks,
        "matrix_rows": matrix_rows, "executable_results": ledger, "missing": len(errors), "errors": errors,
        "duration_seconds": round(time.monotonic() - started, 3),
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(f"API-2010 selected: {api}\nMSG-V1 selected: {msg}\nREST total: {api+msg}\nTwiML entries: {twiml}\nWebhook families: {webhooks}\nmissing: {len(errors)}")
    for error in errors:
        print(error, file=sys.stderr)
    return int(bool(errors))


if __name__ == "__main__":
    raise SystemExit(main())
