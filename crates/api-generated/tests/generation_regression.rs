use dialkit_api_generated::models::CallEnumStatus;

#[test]
fn unknown_string_enum_fix_survives_generation() {
    let unknown: CallEnumStatus = serde_json::from_str("\"introduced-after-pin\"").unwrap();
    assert_eq!(unknown.as_str(), "introduced-after-pin");
    assert_eq!(
        serde_json::to_string(&unknown).unwrap(),
        "\"introduced-after-pin\""
    );
    assert!(include_str!("../src/models/call_enum_status.rs").contains("DO NOT EDIT"));
}

#[test]
fn duplicated_model_namespaces_are_normalized() {
    let generated = include_str!("../src/apis/api20100401_call_api.rs");
    assert!(!generated.contains("models::models::"));
}

#[test]
fn serde_json_values_are_not_qualified_as_models() {
    let generated = include_str!("../src/models/api_v2010_account.rs");
    assert!(!generated.contains("models::serde_json::Value"));
}

#[test]
fn required_incoming_phone_account_sid_is_not_treated_as_optional() {
    let generated = include_str!("../src/apis/api20100401_incoming_phone_number_api.rs");
    assert!(generated.contains("urlencode(&params.account_sid)"));
    assert!(generated.contains("let param_value = params.account_sid;"));
    assert!(!generated.contains("if let Some(param_value) = params.account_sid"));
}
