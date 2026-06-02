use agy7rust::codec::encode::build_plain_v1_package;
use agy7rust::config::Config;
use agy7rust::hash::sha256_hex;

#[test]
fn test_build_plain_v1_package_payload_exact() {
    let input_text = "Hallo Alex\n";
    let package = build_plain_v1_package(input_text).unwrap();
    assert_eq!(package.payload, input_text);
}

#[test]
fn test_build_plain_v1_package_version_1() {
    let package = build_plain_v1_package("test").unwrap();
    assert_eq!(package.version, 1);
}

#[test]
fn test_build_plain_v1_package_has_one_ledger_entry() {
    let package = build_plain_v1_package("test").unwrap();
    assert_eq!(package.entries.len(), 1);
}

#[test]
fn test_build_plain_v1_package_ledger_root_matches_entry_hash() {
    let package = build_plain_v1_package("test").unwrap();
    let entry = &package.entries[0];
    assert_eq!(package.ledger_root, entry.entry_hash);
}

#[test]
fn test_build_plain_v1_package_config_hash_is_deterministic() {
    let package1 = build_plain_v1_package("test1").unwrap();
    let package2 = build_plain_v1_package("test2").unwrap();
    assert_eq!(package1.config_hash, package2.config_hash);

    let config = Config::default();
    let config_json = agy7rust::config::config_to_stable_json(&config).unwrap();
    let expected_config_hash = sha256_hex(config_json.as_bytes());
    assert_eq!(package1.config_hash, expected_config_hash);
}
