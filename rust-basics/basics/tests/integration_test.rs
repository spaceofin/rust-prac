use basics::{adder, multiplier};
mod common;

#[test]
fn test_add() {
    // using common code.
    common::setup();
    assert_eq!(adder::add(3, 2), 5);
}

#[test]
fn test_add_then_multiply() {
    assert_eq!(multiplier::multiply(adder::add(3, 4), 5), 35);
}