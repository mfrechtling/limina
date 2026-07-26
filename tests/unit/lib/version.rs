use limina::version;

#[test]
fn version_returns_current_version() {
    assert_eq!(version(), env!("CARGO_PKG_VERSION"));
}
