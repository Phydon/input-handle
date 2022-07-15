#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[test]
fn test_test() {
    let indeed: bool = true;
    assert!(indeed);
}

#[test]
#[should_panic(expected = "panic msg")]
fn panic_test() {
    panic!("panic msg");
}
