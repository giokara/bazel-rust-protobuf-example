#[test]
fn test_basic() {
    let out = rust_example::set_protos();
    assert_eq!(out.get_name(), "");
}
