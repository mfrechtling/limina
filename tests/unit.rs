use limina::version;

#[test]
fn version_returns_current_version() {
    assert_eq!(version(), "0.1.0");
}
