#!/usr/bin/env python3
"""Generate wire-level contracts for every selected generated operation.

The output is checked in so ordinary `cargo test` executes the same exhaustive
contract set without requiring a code generator or network access.
"""

from __future__ import annotations

import argparse
import base64
import datetime
import json
import pathlib
import re
import subprocess
import tomllib
import urllib.parse
import xml.etree.ElementTree


SOURCES = {
    "API-2010": {
        "spec": "codegen/spec/twilio_api_v2010.json",
        "manifest": "codegen/coverage/rest-api-2010.toml",
        "crate": "crates/api-generated",
        "rust_crate": "dialkit_api_generated",
    },
    "MSG-V1": {
        "spec": "codegen/spec/twilio_messaging_v1.json",
        "manifest": "codegen/coverage/rest-messaging-v1.toml",
        "crate": "crates/messaging-generated",
        "rust_crate": "dialkit_messaging_generated",
    },
}


def snake(value: str) -> str:
    value = re.sub(r"(.)([A-Z][a-z]+)", r"\1_\2", value)
    value = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", value)
    return re.sub(r"[^A-Za-z0-9]+", "_", value).strip("_").lower()


def resolve(spec: dict, value: dict) -> dict:
    reference = value.get("$ref")
    if not reference:
        return value
    found = spec
    for part in reference.removeprefix("#/").split("/"):
        found = found[part]
    return found


def operation_parameters(spec: dict, operation: dict) -> list[dict]:
    result = []
    for unresolved in operation.get("parameters", []):
        parameter = resolve(spec, unresolved).copy()
        parameter["schema"] = resolve(spec, parameter.get("schema", {}))
        result.append(parameter)
    for media_type, media in operation.get("requestBody", {}).get("content", {}).items():
        schema = resolve(spec, media.get("schema", {}))
        required = set(schema.get("required", []))
        for name, unresolved in schema.get("properties", {}).items():
            result.append(
                {
                    "name": name,
                    "in": "body",
                    "required": name in required,
                    "schema": resolve(spec, unresolved),
                    "media_type": media_type,
                }
            )
    return result


def params_block(source: str, operation_id: str) -> str | None:
    match = re.search(
        rf"pub struct {re.escape(operation_id)}Params \{{(.*?)\n\}}", source, re.S
    )
    return match.group(1) if match else None


def function_block(source: str, symbol: str) -> str:
    match = re.search(
        rf"pub async fn {re.escape(symbol)}\((.*?)(?=\n///|\Z)", source, re.S
    )
    if not match:
        raise ValueError(f"missing function {symbol}")
    return match.group(0)


def rust_fields(block: str) -> dict[str, str]:
    return {
        name: " ".join(type_name.split())
        for name, type_name in re.findall(r"^\s*pub (\w+):\s*(.*?),\s*$", block, re.M | re.S)
    }


def schema_value(field: str, rust_type: str, schema: dict):
    inner = rust_type.removeprefix("Option<").removesuffix(">")
    if inner.startswith("Vec<"):
        item_schema = schema.get("items", {})
        count = max(2, schema.get("minItems", 0))
        return [schema_value(field, "String", item_schema) for _ in range(count)]
    if "NaiveDate" in inner:
        return "2024-01-02"
    if "DateTime" in inner:
        return "2024-01-02T03:04:05+00:00"
    if inner == "bool":
        return True
    if inner in ("u64", "u32", "i32"):
        return min(max(int(schema.get("minimum", 2)), 2), int(schema.get("maximum", 2**31 - 1)))
    if inner == "f64":
        return min(max(float(schema.get("minimum", 2.5)), 2.5), float(schema.get("maximum", 1e12)))
    if inner == "serde_json::Value":
        return {"synthetic": "value"}
    if schema.get("enum"):
        return schema["enum"][0]
    if pattern := schema.get("pattern"):
        prefix = re.match(r"\^([A-Z]{2})\[0-9a-fA-F\]\{32\}\$", pattern)
        if prefix:
            return prefix.group(1) + "a" * 32
        if pattern == "^(SM|MM)[0-9a-fA-F]{32}$":
            return "SM" + "a" * 32
        raise ValueError(f"unhandled pinned pattern {pattern}")
    if schema.get("format") == "uri":
        return "https://example.invalid/resource"
    if schema.get("format") in ("endpoint", "phone-number"):
        return "+15005550006"
    if schema.get("format") == "twiml":
        return "<Response><Say>synthetic</Say></Response>"
    if schema.get("format") == "date":
        return "2024-01-02"
    if schema.get("format") == "date-time":
        return "2024-01-02T03:04:05+00:00"
    if field == "account_sid":
        return "ACaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    size = max(1, schema.get("minLength", 0))
    return "x" * min(size, schema.get("maxLength", size))


def check_schema_fixture(schema: dict, value, label: str) -> None:
    if isinstance(value, list):
        assert len(value) >= schema.get("minItems", 0), label
        assert len(value) <= schema.get("maxItems", len(value)), label
        for item in value:
            check_schema_fixture(schema.get("items", {}), item, label)
        return
    if schema.get("enum"):
        assert value in schema["enum"], label
    if isinstance(value, str):
        assert len(value) >= schema.get("minLength", 0), label
        assert len(value) <= schema.get("maxLength", len(value)), label
        if schema.get("pattern"):
            assert re.fullmatch(schema["pattern"], value), label
        format_name = schema.get("format")
        if format_name == "uri":
            assert urllib.parse.urlparse(value).scheme in {"http", "https"}, label
        elif format_name in {"endpoint", "phone-number"}:
            assert value.startswith("+") and value[1:].isdigit(), label
        elif format_name == "date":
            datetime.date.fromisoformat(value)
        elif format_name == "date-time":
            datetime.datetime.fromisoformat(value)
        elif format_name == "twiml":
            assert xml.etree.ElementTree.fromstring(value).tag == "Response", label
    if isinstance(value, (int, float)) and not isinstance(value, bool):
        assert value >= schema.get("minimum", value), label
        assert value <= schema.get("maximum", value), label


def rust_json(value) -> str:
    return json.dumps(value, separators=(",", ":"))


def rust_value(field: str, rust_type: str, schema: dict) -> str:
    value = rust_json(schema_value(field, rust_type, schema))
    expression = f"fixture(serde_json::json!({value}))"
    return f"Some({expression})" if rust_type.startswith("Option<") else expression


def string_values(field: str, rust_type: str, schema: dict) -> list[str]:
    value = schema_value(field, rust_type, schema)
    if "DateTime" in rust_type:
        value = "2024-01-02 03:04:05 +00:00"
    values = value if isinstance(value, list) else [value]
    rendered = []
    for item in values:
        if isinstance(item, bool):
            rendered.append(str(item).lower())
        elif isinstance(item, dict):
            rendered.append(json.dumps(item, separators=(",", ":")))
        else:
            rendered.append(str(item))
    return rendered


def field_for(operation_id: str, parameter: dict, fields: dict[str, str]) -> str:
    candidate = snake(parameter["name"])
    if (
        operation_id == "UpdateIncomingPhoneNumber"
        and parameter["name"] == "AccountSid"
        and parameter.get("in") == "body"
        and "new_account_sid" in fields
    ):
        return "new_account_sid"
    if candidate in fields:
        return candidate
    raise ValueError(f"{operation_id}: unresolved parameter {parameter['name']} ({candidate})")


def rust_pairs(pairs: list[tuple[str, str]]) -> str:
    return ", ".join(
        f"({json.dumps(name)}.to_owned(), {json.dumps(value)}.to_owned())"
        for name, value in pairs
    )


def success_fixture(operation: dict, unit: bool) -> tuple[int, str, str | None]:
    responses = [
        (int(code), response)
        for code, response in operation.get("responses", {}).items()
        if code.isdigit() and 200 <= int(code) < 300
    ]
    if unit:
        return (responses[0][0] if responses else 204), "", None
    for status, response in responses:
        examples = response.get("content", {}).get("application/json", {}).get("examples", {})
        for example in examples.values():
            value = example.get("value")
            if not isinstance(value, dict):
                continue
            for key, item in value.items():
                if key.endswith("sid") and isinstance(item, str):
                    prefix = re.match(r"([A-Z]{2})[0-9a-fA-F]{32}$", item)
                    if prefix:
                        return status, rust_json({key: prefix.group(1) + "a" * 32}), key
            for key, item in value.items():
                if isinstance(item, list):
                    return status, rust_json({key: []}), key
            for key, item in value.items():
                if isinstance(item, (int, float, bool)):
                    return status, rust_json({key: item}), key
    return (responses[0][0] if responses else 200), "{}", None


def generate(root: pathlib.Path, source_id: str) -> str:
    config = SOURCES[source_id]
    spec = json.loads((root / config["spec"]).read_text())
    with (root / config["manifest"]).open("rb") as handle:
        manifest = tomllib.load(handle)
    crate = root / config["crate"]
    tests = []
    for row in manifest["operation"]:
        operation_id = row["operation_id"]
        module, symbol = row["generated_symbol"].split("::", 1)
        source = (crate / "src/apis" / f"{module}.rs").read_text()
        block = params_block(source, operation_id)
        function = function_block(source, symbol)
        fields = rust_fields(block) if block is not None else {}
        operation = spec["paths"][row["path"]][row["method"].lower()]
        parameters = operation_parameters(spec, operation)
        field_schemas = {
            field_for(operation_id, parameter, fields): parameter["schema"]
            for parameter in parameters
        }
        locations: dict[str, list[tuple[str, str]]] = {"query": [], "body": [], "header": []}
        minimum: dict[str, list[tuple[str, str]]] = {"query": [], "body": [], "header": []}
        required_fields = set()
        path = row["path"]
        has_form = False
        for parameter in parameters:
            field = field_for(operation_id, parameter, fields)
            check_schema_fixture(
                parameter["schema"],
                schema_value(field, fields[field], parameter["schema"]),
                f"{operation_id}:{parameter['name']}",
            )
            if parameter.get("required"):
                required_fields.add(field)
                if fields[field].startswith("Option<"):
                    raise ValueError(f"{operation_id}: required {parameter['name']} is optional in Rust")
            elif not fields[field].startswith("Option<"):
                raise ValueError(f"{operation_id}: optional {parameter['name']} is required in Rust")
            values = string_values(field, fields[field], parameter["schema"])
            location = parameter.get("in", "path")
            if location == "path":
                encoded = urllib.parse.quote_plus(values[0], safe="")
                path = path.replace("{" + parameter["name"] + "}", encoded)
            elif location in locations:
                locations[location].extend((parameter["name"], value) for value in values)
                if parameter.get("required"):
                    minimum[location].extend((parameter["name"], value) for value in values)
                has_form |= location == "body"
        if "{" in path:
            raise ValueError(f"{operation_id}: unresolved path {path}")
        params = ",\n            ".join(
            f"{field}: {rust_value(field, type_name, field_schemas.get(field, {}))}"
            for field, type_name in fields.items()
        )
        minimum_params = ",\n            ".join(
            f"{field}: {rust_value(field, type_name, field_schemas.get(field, {})) if field in required_fields else 'None'}"
            for field, type_name in fields.items()
        )
        unit = re.search(r"-> Result<\s*\(\)", function) is not None
        success_status, success_body, success_key = success_fixture(operation, unit)
        success_assertion = (
            f'assert_eq!(serde_json::to_value(&decoded).unwrap().get({json.dumps(success_key)}), serde_json::from_str::<serde_json::Value>({json.dumps(success_body)}).unwrap().get({json.dumps(success_key)}));'
            if success_key
            else ""
        )
        auth_value = "Basic " + base64.b64encode(
            b"ACaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa:synthetic-token"
        ).decode()
        assertions = f'.and(header("authorization", {json.dumps(auth_value)}))\n        '
        for name, value in locations["header"]:
            assertions += f'.and(header({json.dumps(name)}, {json.dumps(value)}))\n        '
        if has_form:
            assertions += '.and(header("content-type", "application/x-www-form-urlencoded"))\n        '
        minimum_assertions = f'.and(header("authorization", {json.dumps(auth_value)}))\n        '
        for name, value in minimum["header"]:
            minimum_assertions += f'.and(header({json.dumps(name)}, {json.dumps(value)}))\n        '
        for name, _ in locations["header"]:
            if name not in {required_name for required_name, _ in minimum["header"]}:
                minimum_assertions += f'.and(AbsentHeader({json.dumps(name)}))\n        '
        if minimum["body"]:
            minimum_assertions += '.and(header("content-type", "application/x-www-form-urlencoded"))\n        '
        setup_params = (
            f'''let params = {module}::{operation_id}Params {{
            {params}
    }};'''
            if fields
            else ""
        )
        minimum_setup = (
            f'''let minimum_params = {module}::{operation_id}Params {{
            {minimum_params}
    }};'''
            if fields
            else ""
        )
        call_params = ", params.clone()" if fields else ""
        final_call_params = ", params" if fields else ""
        success_call = f"{module}::{symbol}(&configuration{call_params}).await.unwrap();"
        if success_key:
            success_call = "let decoded = " + success_call
        tests.append(
            f'''#[tokio::test]
async fn contract_{snake(operation_id)}() {{
    let query = vec![{rust_pairs(locations["query"])}];
    let form = vec![{rust_pairs(locations["body"])}];
    let server = MockServer::start().await;
    Mock::given(method({json.dumps(row["method"])}))
        .and(path({json.dumps(path)}))
        {assertions}.respond_with(SequenceResponse::new({success_status}, {json.dumps(success_body)}))
        .expect(3)
        .mount(&server)
        .await;
    let configuration = make_configuration(&server);
    {setup_params}
    {success_call}
    {success_assertion}
    let structured = {module}::{symbol}(&configuration{call_params}).await;
    assert!(matches!(structured, Err(apis::Error::Core(dialkit_core::error::Error::Api(ref error))) if error.status() == 400 && error.code() == Some(20001) && error.message() == "synthetic"));
    let malformed = {module}::{symbol}(&configuration{final_call_params}).await;
    assert!(matches!(malformed, Err(apis::Error::Core(dialkit_core::error::Error::Api(ref error))) if error.status() == 400 && error.code().is_none()));
    let minimum_query = vec![{rust_pairs(minimum["query"])}];
    let minimum_form = vec![{rust_pairs(minimum["body"])}];
    let minimum_server = MockServer::start().await;
    Mock::given(method({json.dumps(row["method"])}))
        .and(path({json.dumps(path)}))
        {minimum_assertions}.and(operation_mock_guard(minimum_query, minimum_form))
        .respond_with(ResponseTemplate::new({success_status}).set_body_raw({json.dumps(success_body)}, "application/json"))
        .expect(1)
        .mount(&minimum_server)
        .await;
    let minimum_configuration = make_configuration(&minimum_server);
    {minimum_setup}
    {module}::{symbol}(&minimum_configuration{', minimum_params' if fields else ''}).await.unwrap();
}}
'''
        )
    imports = sorted({row["generated_symbol"].split("::", 1)[0] for row in manifest["operation"]})
    absent_header_impl = (
        """#[derive(Debug)]
struct AbsentHeader(&'static str);
impl Match for AbsentHeader {
    fn matches(&self, request: &Request) -> bool { !request.headers.contains_key(self.0) }
}
"""
        if source_id == "MSG-V1"
        else ""
    )
    output = f'''// @generated by codegen/contracts/generate_runtime_contracts.py.
// DO NOT EDIT: update pinned contracts and rerun the generator.

use {config["rust_crate"]}::apis::{{self, configuration::Configuration, {", ".join(imports)}}};
use dialkit_core::{{
    auth::{{AccountSid, Credentials}},
    request::{{ClientConfiguration, HttpClient}},
    retry::RetryPolicy,
}};
use secrecy::SecretString;
use serde::de::DeserializeOwned;
use std::{{collections::BTreeMap, sync::atomic::{{AtomicUsize, Ordering}}, time::Duration}};
use url::Url;
use wiremock::{{
    Match, Mock, MockServer, Request, Respond, ResponseTemplate,
    matchers::{{header, method, path}},
}};

fn fixture<T: DeserializeOwned>(value: serde_json::Value) -> T {{
    serde_json::from_value(value).unwrap()
}}

fn make_configuration(server: &MockServer) -> Configuration {{
    let client = HttpClient::new(ClientConfiguration {{
        credentials: Credentials::account_token(
            AccountSid::new("ACaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap(),
            SecretString::from("synthetic-token".to_owned()),
        ),
        base_url: Url::parse(&server.uri()).unwrap(),
        connect_timeout: Duration::from_secs(1),
        request_timeout: Duration::from_secs(2),
        retry_policy: RetryPolicy::conservative(),
        allow_http_for_tests: true,
    }}).unwrap();
    Configuration::new(client)
}}

#[derive(Debug)]
struct ParameterMatcher {{ query: Vec<(String, String)>, form: Vec<(String, String)> }}

impl Match for ParameterMatcher {{
    fn matches(&self, request: &Request) -> bool {{
        let actual_query = request.url.query_pairs().map(|(k, v)| (k.into_owned(), v.into_owned())).collect::<Vec<_>>();
        let actual_form = url::form_urlencoded::parse(&request.body).map(|(k, v)| (k.into_owned(), v.into_owned())).collect::<Vec<_>>();
        let matches = multimap(&actual_query) == multimap(&self.query) && multimap(&actual_form) == multimap(&self.form);
        if !matches {{
            eprintln!("parameter mismatch: expected query={{:?}} form={{:?}}; actual query={{:?}} form={{:?}}", self.query, self.form, actual_query, actual_form);
        }}
        matches
    }}
}}

fn multimap(values: &[(String, String)]) -> BTreeMap<&str, Vec<&str>> {{
    let mut result = BTreeMap::new();
    for (key, value) in values {{ result.entry(key.as_str()).or_insert_with(Vec::new).push(value.as_str()); }}
    result
}}

{absent_header_impl}

#[derive(Debug)]
struct SequenceResponse {{ calls: AtomicUsize, success_status: u16, success_body: &'static str }}

impl SequenceResponse {{
    const fn new(success_status: u16, success_body: &'static str) -> Self {{
        Self {{ calls: AtomicUsize::new(0), success_status, success_body }}
    }}
}}

impl Respond for SequenceResponse {{
    fn respond(&self, _: &Request) -> ResponseTemplate {{
        match self.calls.fetch_add(1, Ordering::SeqCst) {{
            0 => ResponseTemplate::new(self.success_status).set_body_raw(self.success_body, "application/json"),
            1 => ResponseTemplate::new(400).set_body_raw(r#"{{"code": 20001, "message": "synthetic"}}"#, "application/json"),
            _ => ResponseTemplate::new(400).set_body_raw("not-json", "application/json"),
        }}
    }}
}}

// This matcher performs exact, multiplicity-preserving query and form checks.
fn operation_mock_guard(query: Vec<(String, String)>, form: Vec<(String, String)>) -> ParameterMatcher {{
    ParameterMatcher {{ query, form }}
}}

{''.join(test.replace('.respond_with(SequenceResponse', '.and(operation_mock_guard(query.clone(), form.clone()))\n        .respond_with(SequenceResponse') for test in tests)}'''
    return subprocess.run(
        ["rustfmt", "--edition", "2024", "--emit", "stdout"],
        input=output,
        text=True,
        check=True,
        capture_output=True,
    ).stdout


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=pathlib.Path, default=pathlib.Path(__file__).resolve().parents[2])
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    root = args.root.resolve()
    changed = False
    for source_id, config in SOURCES.items():
        output = root / config["crate"] / "tests/runtime_contracts.rs"
        generated = generate(root, source_id)
        if args.check:
            if not output.is_file() or output.read_text() != generated:
                print(f"stale generated runtime contracts: {output.relative_to(root)}")
                changed = True
        else:
            output.write_text(generated)
    return int(changed)


if __name__ == "__main__":
    raise SystemExit(main())
