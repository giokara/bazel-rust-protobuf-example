use rust_example::lib;

#[test]
fn test_basic() {
    let out = lib::set_protos();
    assert_eq!(out.name(), Some(""));
}
