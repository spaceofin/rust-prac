use automated_tests::do_add;

mod common;

#[test]
fn it_adds_two() {
    common::setup();

    let result = do_add(2);
    assert_eq!(result, 4);
}

#[test]
fn it_adds_two_fail() {
    common::setup();
    let result = do_add(2);
    assert_eq!(result, 5); 
}