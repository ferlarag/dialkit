use std::{fs, path::Path};

#[test]
fn established_api_2010_modules_and_function_names_remain_present() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for (module, functions) in [
        (
            "api20100401_call_api",
            &["create_call", "fetch_call", "list_call"][..],
        ),
        (
            "api20100401_message_api",
            &["create_message", "fetch_message", "list_message"],
        ),
        (
            "api20100401_conference_api",
            &["fetch_conference", "list_conference"],
        ),
        (
            "api20100401_recording_api",
            &["fetch_recording", "list_recording"],
        ),
    ] {
        let source =
            fs::read_to_string(root.join("src/apis").join(format!("{module}.rs"))).unwrap();
        for function in functions {
            assert!(source.contains(&format!("pub async fn {function}(")));
        }
    }
}
