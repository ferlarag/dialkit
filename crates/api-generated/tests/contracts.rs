use std::{collections::BTreeSet, fs, path::Path};

#[test]
fn every_api_2010_operation_resolves_to_a_unique_runtime_contract() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let manifest: toml::Value = toml::from_str(
        &fs::read_to_string(root.join("codegen/coverage/rest-api-2010.toml")).unwrap(),
    )
    .unwrap();
    let runtime = fs::read_to_string("tests/runtime_contracts.rs").unwrap();
    let operations = manifest["operation"].as_array().unwrap();
    assert_eq!(operations.len(), 139);
    let mut selectors = BTreeSet::new();
    for operation in operations {
        let selector = operation["evidence"]["method_path"].as_str().unwrap();
        assert!(
            selectors.insert(selector),
            "shared runtime selector: {selector}"
        );
        let test_name = selector.split_once('#').unwrap().1;
        assert!(runtime.contains(&format!("async fn {test_name}()")));
        for dimension in ["authentication", "parameters", "success", "error"] {
            assert_eq!(operation["evidence"][dimension].as_str(), Some(selector));
        }
        if operation["pagination"].as_bool() == Some(true) {
            let pagination = operation["evidence"]["pagination"].as_str().unwrap();
            let test_name = pagination.split_once('#').unwrap().1;
            let source = fs::read_to_string("tests/pagination_contracts.rs").unwrap();
            assert!(source.contains(&format!("fn {test_name}()")));
        }
    }
}

fn verify_domains(expected: &[&str]) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let manifest: toml::Value = toml::from_str(
        &fs::read_to_string(root.join("codegen/coverage/rest-api-2010.toml")).unwrap(),
    )
    .unwrap();
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    let matched = manifest["operation"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|operation| expected.contains(operation["domain"].as_str().unwrap()))
        .collect::<Vec<_>>();
    assert!(!matched.is_empty());
    for operation in matched {
        assert!(
            operation["method"]
                .as_str()
                .is_some_and(|value| !value.is_empty())
        );
        assert!(
            operation["path"]
                .as_str()
                .is_some_and(|value| value.starts_with('/'))
        );
        assert!(operation["evidence"]["authentication"].as_str().is_some());
        assert!(operation["evidence"]["parameters"].as_str().is_some());
        assert!(operation["evidence"]["success"].as_str().is_some());
        assert!(operation["evidence"]["error"].as_str().is_some());
    }
}

#[path = "contracts/messaging_api2010.rs"]
mod messaging_api2010;
#[path = "contracts/voice_configuration.rs"]
mod voice_configuration;
#[path = "contracts/voice_control.rs"]
mod voice_control;
#[path = "contracts/voice_core.rs"]
mod voice_core;
