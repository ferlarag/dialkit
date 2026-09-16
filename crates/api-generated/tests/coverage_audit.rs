use serde_json::Value;
use std::{collections::BTreeSet, fs, path::Path};

fn snake(value: &str) -> String {
    let mut output = String::new();
    for (index, ch) in value.chars().enumerate() {
        if ch.is_ascii_uppercase() {
            if index > 0 {
                output.push('_');
            }
            output.push(ch.to_ascii_lowercase());
        } else if ch.is_ascii_alphanumeric() || ch == '_' {
            output.push(ch.to_ascii_lowercase());
        }
    }
    output
}

#[test]
fn every_pinned_operation_is_generated_and_documented() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec: Value =
        serde_json::from_str(include_str!("../../../codegen/spec/twilio_api_v2010.json")).unwrap();
    let mut operations = Vec::new();
    for item in spec["paths"].as_object().unwrap().values() {
        for operation in item.as_object().unwrap().values() {
            if let Some(id) = operation.get("operationId").and_then(Value::as_str) {
                let tag = operation["tags"][0].as_str().expect("operation tag");
                operations.push((id.to_owned(), tag.to_owned()));
            }
        }
    }
    assert!(!operations.is_empty());
    for (id, tag) in operations {
        let function = snake(&id);
        let source_path = root
            .join("src/apis")
            .join(format!("{}_api.rs", snake(&tag)));
        let source = fs::read_to_string(&source_path)
            .unwrap_or_else(|error| panic!("{}: {error}", source_path.display()));
        assert!(
            source.contains(&format!("pub async fn {function}(")),
            "{id} has no compiled generated function in {}",
            source_path.display()
        );
        assert!(
            source.contains(&format!("pub struct {id}Params")),
            "{id} has no generated request input"
        );
        let signature = source
            .split(&format!("pub async fn {function}("))
            .nth(1)
            .and_then(|value| value.split(" {\n").next())
            .expect("generated function signature");
        assert!(
            signature.contains("-> Result<"),
            "{id} has no generated response result"
        );

        let doc_path = root.join("docs").join(format!("{tag}Api.md"));
        let docs = fs::read_to_string(&doc_path)
            .unwrap_or_else(|error| panic!("{}: {error}", doc_path.display()));
        for marker in [
            format!("[**{function}**]"),
            format!("## {function}"),
            "### Parameters".to_owned(),
            "### Return type".to_owned(),
        ] {
            assert!(
                docs.contains(&marker),
                "{id} documentation is missing {marker:?} in {}",
                doc_path.display()
            );
        }
    }
}

#[test]
fn every_generated_string_enum_preserves_unknown_values() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let listed: Value =
        serde_json::from_str(include_str!("fixtures/unknown_enum_values.json")).unwrap();
    let listed: BTreeSet<String> = listed["enum_types"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect();
    let mut found = BTreeSet::new();
    for entry in fs::read_dir(root.join("src/models"))
        .unwrap()
        .filter_map(Result::ok)
    {
        if entry.path().extension().is_none_or(|ext| ext != "rs") {
            continue;
        }
        let source = fs::read_to_string(entry.path()).unwrap();
        for suffix in source.split("pub enum ").skip(1) {
            let name = suffix.split_whitespace().next().unwrap();
            let block = suffix.split("\n}").next().unwrap_or(suffix);
            if block.contains("Unknown(String)") {
                found.insert(name.to_owned());
            }
        }
    }
    assert_eq!(listed, found);
    let runtime_contract = include_str!("generated_contract.rs");
    for enum_name in &listed {
        assert!(
            runtime_contract.contains(enum_name),
            "{enum_name} is missing from the runtime unknown-value contract"
        );
    }
}
