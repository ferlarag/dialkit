use serde_json::Value;
use std::{collections::BTreeSet, fs, path::Path};

fn verify_manifest(path: &Path, spec: &Value, expected_source: &str, expected_count: usize) {
    let manifest: toml::Value = toml::from_str(&fs::read_to_string(path).unwrap()).unwrap();
    assert_eq!(manifest["source_id"].as_str(), Some(expected_source));
    assert_eq!(
        manifest["selected_operations"].as_integer(),
        Some(expected_count as i64)
    );
    let operations = manifest["operation"].as_array().unwrap();
    assert_eq!(operations.len(), expected_count);
    let mut seen = BTreeSet::new();
    for operation in operations {
        let id = operation["operation_id"].as_str().unwrap();
        assert!(seen.insert(id), "duplicate {expected_source}:{id}");
        let expected_method = operation["method"].as_str().unwrap();
        let expected_path = operation["path"].as_str().unwrap();
        let pinned = &spec["paths"][expected_path][&expected_method.to_ascii_lowercase()];
        assert_eq!(
            pinned["operationId"].as_str(),
            Some(id),
            "wrong source or method/path for {expected_source}:{id}"
        );
        let symbol = operation["generated_symbol"].as_str().unwrap();
        let (module, function) = symbol.split_once("::").unwrap();
        let crate_dir = if expected_source == "API-2010" {
            Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
        } else {
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../messaging-generated")
        };
        let generated =
            fs::read_to_string(crate_dir.join("src/apis").join(format!("{module}.rs"))).unwrap();
        assert!(
            generated.contains(&format!("pub async fn {function}(")),
            "unresolved generated function {expected_source}:{id}"
        );
        for dimension in [
            "method_path",
            "authentication",
            "parameters",
            "success",
            "error",
            "pagination",
        ] {
            assert!(
                operation["evidence"].get(dimension).is_some(),
                "missing {dimension} evidence for {expected_source}:{id}"
            );
        }
    }
}

#[test]
fn selected_union_is_exactly_139_plus_58_source_qualified_operations() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let api: Value =
        serde_json::from_str(include_str!("../../../codegen/spec/twilio_api_v2010.json")).unwrap();
    let messaging: Value = serde_json::from_str(include_str!(
        "../../../codegen/spec/twilio_messaging_v1.json"
    ))
    .unwrap();
    verify_manifest(
        &root.join("codegen/coverage/rest-api-2010.toml"),
        &api,
        "API-2010",
        139,
    );
    verify_manifest(
        &root.join("codegen/coverage/rest-messaging-v1.toml"),
        &messaging,
        "MSG-V1",
        58,
    );
}
