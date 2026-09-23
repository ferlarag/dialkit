use regex::Regex;
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::{Path, PathBuf},
};

#[derive(Clone)]
struct Operation {
    id: String,
    domain: String,
    method: String,
    path: String,
    module: String,
    pagination: bool,
    evidence_applicability: BTreeMap<&'static str, bool>,
}

fn resolve<'a>(spec: &'a Value, value: &'a Value) -> &'a Value {
    let Some(reference) = value.get("$ref").and_then(Value::as_str) else {
        return value;
    };
    spec.pointer(reference.trim_start_matches('#'))
        .unwrap_or_else(|| panic!("unresolved OpenAPI reference {reference}"))
}

fn evidence_applicability(
    spec: &Value,
    operation: &Value,
    route: &str,
) -> BTreeMap<&'static str, bool> {
    let parameters = operation["parameters"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|parameter| resolve(spec, parameter))
        .collect::<Vec<_>>();
    let form = operation["requestBody"]["content"]["application/x-www-form-urlencoded"].is_object();
    let json_body = operation["requestBody"]["content"]["application/json"].is_object();
    let constrained = parameters.iter().any(|parameter| {
        let schema = resolve(spec, &parameter["schema"]);
        [
            "enum",
            "pattern",
            "minimum",
            "maximum",
            "minLength",
            "maxLength",
            "minItems",
            "maxItems",
            "format",
        ]
        .iter()
        .any(|key| !schema[key].is_null())
    }) || ["application/x-www-form-urlencoded", "application/json"]
        .iter()
        .any(|media_type| {
            let schema = resolve(
                spec,
                &operation["requestBody"]["content"][media_type]["schema"],
            );
            schema["properties"].as_object().is_some_and(|properties| {
                properties.values().any(|property| {
                    let property = resolve(spec, property);
                    [
                        "enum",
                        "pattern",
                        "minimum",
                        "maximum",
                        "minLength",
                        "maxLength",
                        "minItems",
                        "maxItems",
                        "format",
                    ]
                    .iter()
                    .any(|key| !property[key].is_null())
                })
            })
        });
    let response_fields = operation["responses"].as_object().is_some_and(|responses| {
        responses.values().any(|response| {
            response["content"]["application/json"]["examples"]
                .as_object()
                .is_some_and(|examples| {
                    examples.values().any(|example| {
                        example["value"].as_object().is_some_and(|value| {
                            value.iter().any(|(key, item)| {
                                (key.ends_with("sid")
                                    && item.as_str().is_some_and(|text| {
                                        text.len() == 34
                                            && text
                                                .chars()
                                                .take(2)
                                                .all(|ch| ch.is_ascii_uppercase())
                                    }))
                                    || item.is_array()
                                    || item.is_number()
                                    || item.is_boolean()
                            })
                        })
                    })
                })
        })
    });
    BTreeMap::from([
        ("path_parameters", route.contains('{')),
        (
            "query_parameters",
            parameters
                .iter()
                .any(|parameter| parameter["in"] == "query"),
        ),
        (
            "header_parameters",
            parameters
                .iter()
                .any(|parameter| parameter["in"] == "header"),
        ),
        ("form_parameters", form),
        ("json_body", json_body),
        ("constraints", constrained),
        ("request_media_type", form || json_body),
        ("response_fields", response_fields),
    ])
}

fn main() {
    let root = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("../.."));
    let spec_text = fs::read_to_string(root.join("specs/002-voice-sms-coverage/spec.md"))
        .expect("read feature specification");
    generate_source(
        &root,
        &spec_text,
        "API-2010",
        "### REST Coverage Matrix A",
        "### REST Coverage Matrix B",
        "codegen/spec/twilio_api_v2010.json",
        "crates/api-generated/src/apis",
        "dialkit-api-generated",
        "codegen/coverage/rest-api-2010.toml",
        139,
    );
    generate_source(
        &root,
        &spec_text,
        "MSG-V1",
        "### REST Coverage Matrix B",
        "### TwiML Coverage Matrix",
        "codegen/spec/twilio_messaging_v1.json",
        "crates/messaging-generated/src/apis",
        "dialkit-messaging-generated",
        "codegen/coverage/rest-messaging-v1.toml",
        58,
    );
}

#[allow(clippy::too_many_arguments)]
fn generate_source(
    root: &Path,
    specification: &str,
    source_id: &str,
    start: &str,
    end: &str,
    openapi_path: &str,
    generated_api_dir: &str,
    generated_crate: &str,
    output_path: &str,
    expected: usize,
) {
    let matrix = specification
        .split_once(start)
        .and_then(|(_, rest)| rest.split_once(end).map(|(value, _)| value))
        .expect("coverage matrix headings");
    let operation_pattern = Regex::new(r"`([A-Z][A-Za-z0-9]+)`").unwrap();
    let mut domains = BTreeMap::new();
    for line in matrix.lines().filter(|line| line.starts_with('|')) {
        let cells = line.split('|').map(str::trim).collect::<Vec<_>>();
        if cells.len() < 5 || cells[1].is_empty() || cells[1].starts_with('-') {
            continue;
        }
        for capture in operation_pattern.captures_iter(cells[3]) {
            domains.insert(capture[1].to_owned(), cells[1].to_owned());
        }
    }
    assert_eq!(domains.len(), expected, "{source_id} matrix count");

    let openapi: Value = serde_json::from_str(
        &fs::read_to_string(root.join(openapi_path)).expect("read pinned OpenAPI"),
    )
    .expect("parse pinned OpenAPI");
    let mut contracts = BTreeMap::new();
    for (path, methods) in openapi["paths"].as_object().expect("paths object") {
        for (method, operation) in methods.as_object().expect("path item") {
            if !matches!(method.as_str(), "get" | "post" | "put" | "patch" | "delete") {
                continue;
            }
            if let Some(id) = operation["operationId"].as_str() {
                let pagination = operation["parameters"]
                    .as_array()
                    .is_some_and(|parameters| {
                        parameters.iter().any(|parameter| {
                            matches!(
                                parameter["name"].as_str(),
                                Some("PageSize" | "Page" | "PageToken")
                            )
                        })
                    });
                contracts.insert(
                    id.to_owned(),
                    (
                        method.to_uppercase(),
                        path.to_owned(),
                        pagination,
                        evidence_applicability(&openapi, operation, path),
                    ),
                );
            }
        }
    }

    let mut generated = BTreeMap::new();
    for entry in fs::read_dir(root.join(generated_api_dir)).expect("generated API directory") {
        let path = entry.expect("directory entry").path();
        if path.extension().and_then(|value| value.to_str()) != Some("rs") {
            continue;
        }
        let text = fs::read_to_string(&path).expect("generated API source");
        for capture in Regex::new(r"pub async fn ([a-zA-Z0-9_]+)")
            .unwrap()
            .captures_iter(&text)
        {
            generated.insert(
                capture[1].to_owned(),
                path.file_stem().unwrap().to_string_lossy().into_owned(),
            );
        }
    }

    let mut operations = Vec::new();
    for (id, domain) in domains {
        let (method, path, pagination, applicability) = contracts
            .get(&id)
            .unwrap_or_else(|| panic!("missing {source_id}:{id}"));
        let symbol = snake_case(&id);
        let module = generated
            .get(&symbol)
            .unwrap_or_else(|| panic!("unresolved generated symbol {source_id}:{id} -> {symbol}"));
        let pagination = id.starts_with("List") || *pagination;
        operations.push(Operation {
            id,
            domain,
            method: method.clone(),
            path: path.clone(),
            module: module.clone(),
            pagination,
            evidence_applicability: applicability.clone(),
        });
    }

    let mut output = format!(
        "schema = \"dialkit.rest-coverage.v1\"\nsource_id = \"{source_id}\"\ngenerated_crate = \"{generated_crate}\"\nselected_operations = {expected}\n\n"
    );
    let mut keys = BTreeSet::new();
    for operation in operations {
        assert!(keys.insert(operation.id.clone()));
        let symbol = snake_case(&operation.id);
        let pagination = operation.pagination;
        let runtime_test = format!(
            "crates/{}/tests/runtime_contracts.rs#contract_{}",
            if source_id == "API-2010" {
                "api-generated"
            } else {
                "messaging-generated"
            },
            symbol
        );
        output.push_str(&format!(
            "[[operation]]\noperation_id = \"{}\"\ndomain = {:?}\nmethod = \"{}\"\npath = {:?}\ngenerated_symbol = \"{}::{}\"\nphase = \"US1\"\npagination = {}\nevidence.method_path = {:?}\nevidence.authentication = {:?}\nevidence.parameters = {:?}\nevidence.success = {:?}\nevidence.error = {:?}\n{}\n\n",
            operation.id,
            operation.domain,
            operation.method,
            operation.path,
            operation.module,
            symbol,
            pagination,
            runtime_test,
            runtime_test,
            runtime_test,
            runtime_test,
            runtime_test,
            if pagination {
                format!(
                    "evidence.pagination = {:?}",
                    format!(
                        "crates/{}/tests/pagination_contracts.rs#adapter_{}",
                        if source_id == "API-2010" {
                            "api-generated"
                        } else {
                            "messaging-generated"
                        },
                        symbol
                    )
                )
            } else {
                "evidence.pagination = { n_a = \"operation is not a list operation\" }".to_owned()
            },
        ));
        // Applicability is source-derived, and every non-applicable dimension has a
        // reason instead of a missing or reusable placeholder reference.
        for (dimension, applicable) in &operation.evidence_applicability {
            if *applicable {
                output.push_str(&format!("evidence.{dimension} = {runtime_test:?}\n"));
            } else {
                output.push_str(&format!(
                    "evidence.{dimension} = {{ n_a = {:?} }}\n",
                    format!("pinned operation declares no {dimension}")
                ));
            }
        }
        output.push('\n');
    }
    fs::write(root.join(output_path), output).expect("write coverage manifest");
}

fn snake_case(value: &str) -> String {
    let mut output = String::new();
    for (index, character) in value.chars().enumerate() {
        if character.is_ascii_uppercase() && index != 0 {
            output.push('_');
        }
        output.push(character.to_ascii_lowercase());
    }
    output
}
