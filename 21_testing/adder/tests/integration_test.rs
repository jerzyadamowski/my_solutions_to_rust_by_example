// importing common module.
mod common;

#[test]
fn test_add() {
  // using common code.
  common::setup();
  assert_eq!(adder::add(3, 2), 5);
}

// cargo test in this directory
