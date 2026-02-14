use magrext::metadata::Metadata;

#[test]
fn metadata_parses_json() {
    let json = br#"{"name":"demo","version":"1.0.0","author":"me"}"#;
    let metadata = Metadata::from_json_bytes(json).unwrap();
    assert_eq!(metadata.name, "demo");
    assert_eq!(metadata.version, "1.0.0");
    assert_eq!(metadata.author, "me");
}
