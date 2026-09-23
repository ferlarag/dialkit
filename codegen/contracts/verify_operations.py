#!/usr/bin/env python3
"""Verify every selected generated operation against its pinned OpenAPI contract.

The generated functions share one tested HTTP executor, so this verifier checks
the operation-specific surface exhaustively and leaves transport behavior to the
wire-level core contracts. It emits a selector-addressable evidence ledger used
by the coverage audit.
"""

import argparse
import json
import pathlib
import re
import sys
import tomllib


SOURCES = {
    "API-2010": (
        "codegen/spec/twilio_api_v2010.json",
        "codegen/coverage/rest-api-2010.toml",
        "crates/api-generated",
    ),
    "MSG-V1": (
        "codegen/spec/twilio_messaging_v1.json",
        "codegen/coverage/rest-messaging-v1.toml",
        "crates/messaging-generated",
    ),
}
DIMENSIONS = ("method_path", "authentication", "parameters", "success", "error", "pagination")


def snake(value: str) -> str:
    value = re.sub(r"(.)([A-Z][a-z]+)", r"\1_\2", value)
    value = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", value)
    value = re.sub(r"[^A-Za-z0-9]+", "_", value)
    return value.strip("_").lower()


def block_after(source: str, marker: str, next_marker: str) -> str:
    start = source.find(marker)
    if start < 0:
        return ""
    end = source.find(next_marker, start + len(marker))
    return source[start:] if end < 0 else source[start:end]


def resolve(spec: dict, value: dict) -> dict:
    reference = value.get("$ref")
    if not reference:
        return value
    resolved = spec
    for part in reference.removeprefix("#/").split("/"):
        resolved = resolved[part]
    return resolved


def operation_parameters(spec: dict, operation: dict) -> list[tuple[str, str, bool]]:
    found = []
    for unresolved in operation.get("parameters", []):
        parameter = resolve(spec, unresolved)
        found.append((parameter["name"], parameter.get("in", "path"), bool(parameter.get("required"))))
    body = operation.get("requestBody", {}).get("content", {})
    for media in body.values():
        schema = resolve(spec, media.get("schema", {}))
        required = set(schema.get("required", []))
        for name in schema.get("properties", {}):
            found.append((name, "body", name in required))
    return found


def verify_source(root: pathlib.Path, source_id: str) -> tuple[list[dict], list[str]]:
    spec_name, manifest_name, crate_name = SOURCES[source_id]
    spec = json.loads((root / spec_name).read_text())
    with (root / manifest_name).open("rb") as handle:
        manifest = tomllib.load(handle)
    ledger, errors = [], []
    for row in manifest.get("operation", []):
        operation_id = row["operation_id"]
        method = row["method"]
        path = row["path"]
        operation = spec.get("paths", {}).get(path, {}).get(method.lower())
        prefix = f"{source_id}:{operation_id}"
        results = {dimension: [] for dimension in DIMENSIONS}
        if not operation or operation.get("operationId") != operation_id:
            errors.append(f"{prefix}: operation does not resolve to pinned method/path")
            operation = {}

        module, function = row["generated_symbol"].split("::", 1)
        source_path = root / crate_name / "src/apis" / f"{module}.rs"
        source = source_path.read_text() if source_path.is_file() else ""
        function_block = block_after(source, f"pub async fn {function}(", "\n///")
        params_block = block_after(source, f"pub struct {operation_id}Params", "\n}}")

        results["method_path"].extend([
            bool(function_block),
            f"reqwest::Method::{method}" in function_block,
            path in function_block,
            re.search(r"crate::apis::urlencode\(&?params\.", function_block) is not None or "{" not in path,
        ])
        security = operation.get("security", spec.get("security", []))
        results["authentication"].extend([
            bool(security),
            "configuration.client" in function_block,
            ".request(" in function_block,
            "configuration.client.execute(req).await" in function_block,
        ])

        parameters = operation_parameters(spec, operation)
        parameter_checks = [bool(params_block) if parameters else True]
        for name, location, required in parameters:
            field = (
                "new_account_sid"
                if operation_id == "UpdateIncomingPhoneNumber"
                and name == "AccountSid"
                and location == "body"
                else snake(name)
            )
            field_line = re.search(rf"pub {re.escape(field)}:\s*([^,]+),", params_block)
            parameter_checks.append(field_line is not None)
            if field_line:
                parameter_checks.append(("Option<" not in field_line.group(1)) == required)
            parameter_checks.append(
                re.search(rf"params\s*\.\s*{re.escape(field)}", function_block) is not None
            )
            if location == "path":
                parameter_checks.append(
                    re.search(rf"urlencode\(&?params\.{re.escape(field)}\)", function_block) is not None
                )
            elif location == "body":
                parameter_checks.append(
                    re.search(rf'insert\(\s*"{re.escape(name)}"', function_block) is not None
                )
                body_schema = operation.get("requestBody", {}).get("content", {})
                property_schema = None
                for media in body_schema.values():
                    schema = resolve(spec, media.get("schema", {}))
                    if name in schema.get("properties", {}):
                        property_schema = resolve(spec, schema["properties"][name])
                        break
                if property_schema and property_schema.get("type") == "array":
                    parameter_checks.append(
                        re.search(
                            rf"for param_value in (?:param_values|params\s*\.\s*{re.escape(field)})",
                            function_block,
                        )
                        is not None
                    )
            elif location == "query":
                parameter_checks.append(
                    re.search(rf'\(\s*"{re.escape(name)}"', function_block) is not None
                )
        results["parameters"] = parameter_checks

        responses = operation.get("responses", {})
        success_codes = [code for code in responses if not str(code).startswith(("4", "5"))]
        results["success"].extend([
            bool(success_codes),
            "-> Result<" in function_block,
            "!status.is_client_error() && !status.is_server_error()" in function_block,
            "serde_json::from_str" in function_block or "Ok(())" in function_block,
        ])
        results["error"].extend([
            "Error::ResponseError(ResponseContent" in function_block,
            "status," in function_block,
            "entity," in function_block,
        ])

        if row.get("pagination"):
            results["pagination"].extend([
                operation_id.startswith("List"),
                all(f"pub {name}:" in params_block for name in ("page_size", "page", "page_token")),
                all(f"params.{name}" in function_block for name in ("page_size", "page", "page_token")),
                "PageSize" in function_block and "PageToken" in function_block,
            ])
        else:
            results["pagination"] = [True]

        for dimension, checks in results.items():
            passed = bool(checks) and all(checks)
            selector = f"{prefix}:{dimension}"
            ledger.append({"selector": selector, "status": "pass" if passed else "fail"})
            if not passed:
                failed = [str(index) for index, check in enumerate(checks) if not check]
                errors.append(f"{selector}: generated contract checks failed at {','.join(failed)}")
    return ledger, errors


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=pathlib.Path, default=pathlib.Path(__file__).resolve().parents[2])
    parser.add_argument("--source", choices=[*SOURCES, "all"], default="all")
    parser.add_argument("--output", type=pathlib.Path)
    args = parser.parse_args()
    selected = SOURCES if args.source == "all" else (args.source,)
    ledger, errors = [], []
    for source_id in selected:
        source_ledger, source_errors = verify_source(args.root.resolve(), source_id)
        ledger.extend(source_ledger)
        errors.extend(source_errors)
    report = {"status": "pass" if not errors else "fail", "evidence": ledger, "errors": errors}
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(json.dumps(report, indent=2) + "\n")
    for error in errors:
        print(error, file=sys.stderr)
    if not errors:
        print(f"verified {len(ledger) // len(DIMENSIONS)} operation contracts")
    return 1 if errors else 0


if __name__ == "__main__":
    raise SystemExit(main())
