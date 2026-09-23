use dialkit_messaging_generated::models::{
    BrandRegistrationsEnumBrandFeedback, BrandRegistrationsEnumIdentityStatus,
    BrandRegistrationsEnumStatus, BrandVettingEnumVettingProvider, ServiceEnumScanMessageContent,
    TollfreeVerificationEnumBusinessRegistrationAuthority, TollfreeVerificationEnumBusinessType,
    TollfreeVerificationEnumOptInType, TollfreeVerificationEnumStatus,
    TollfreeVerificationEnumVettingProvider,
    messaging_v1_service::{FallbackMethod, InboundMethod},
    messaging_v1_tollfree_verification::UseCaseCategories,
};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::{collections::BTreeSet, fs, path::Path};

fn round_trip<T: DeserializeOwned + Serialize>() {
    let value = "\"introduced-after-pin\"";
    let decoded: T = serde_json::from_str(value).unwrap();
    assert_eq!(serde_json::to_string(&decoded).unwrap(), value);
}

#[test]
fn every_generated_string_enum_round_trips_unknown_values() {
    round_trip::<BrandRegistrationsEnumBrandFeedback>();
    round_trip::<BrandRegistrationsEnumIdentityStatus>();
    round_trip::<BrandRegistrationsEnumStatus>();
    round_trip::<BrandVettingEnumVettingProvider>();
    round_trip::<FallbackMethod>();
    round_trip::<InboundMethod>();
    round_trip::<ServiceEnumScanMessageContent>();
    round_trip::<TollfreeVerificationEnumBusinessRegistrationAuthority>();
    round_trip::<TollfreeVerificationEnumBusinessType>();
    round_trip::<TollfreeVerificationEnumOptInType>();
    round_trip::<TollfreeVerificationEnumStatus>();
    round_trip::<TollfreeVerificationEnumVettingProvider>();
    round_trip::<UseCaseCategories>();
}

#[test]
fn fixture_inventory_matches_every_unknown_enum_shape() {
    let fixture: Value =
        serde_json::from_str(include_str!("fixtures/unknown_enum_values.json")).unwrap();
    let expected = fixture["enum_types"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap().to_owned())
        .collect::<BTreeSet<_>>();
    let mut found = BTreeSet::new();
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/models");
    for entry in fs::read_dir(root).unwrap().filter_map(Result::ok) {
        let source = fs::read_to_string(entry.path()).unwrap();
        for suffix in source.split("pub enum ").skip(1) {
            let name = suffix.split_whitespace().next().unwrap();
            let block = suffix.split("\n}").next().unwrap_or(suffix);
            if block.contains("Unknown(String)") {
                found.insert(name.to_owned());
            }
        }
    }
    assert_eq!(found, expected);
}

#[test]
fn normalization_and_generated_markers_survive_regeneration() {
    let api = include_str!("../src/apis/messaging_v1_service_api.rs");
    assert!(api.contains("DO NOT EDIT"));
    assert!(!api.contains("models::models::"));
    assert!(!api.contains("basic_auth("));
}
