use dialkit::Client;

#[test]
fn public_client_is_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Client>();
}

#[test]
fn generated_namespace_is_not_reexported() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/compile/generated_leak.rs");
}

#[test]
fn tls_backend_selection_is_unambiguous() {
    #[cfg(feature = "rustls-tls")]
    assert_eq!(dialkit::active_tls_backend(), dialkit::TlsBackend::Rustls);
    #[cfg(all(not(feature = "rustls-tls"), feature = "native-tls"))]
    assert_eq!(dialkit::active_tls_backend(), dialkit::TlsBackend::Native);
}
