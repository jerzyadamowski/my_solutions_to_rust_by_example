use std::convert::From;

#[derive(Debug)]
struct Number {
  _value: i32,
}

impl From<i32> for Number {
  fn from(item: i32) -> Self {
    Number { _value: item }
  }
}

pub fn exmaple() {
  let int = 5;
  // Try removing the type declaration
  let num: Number = int.into();
  println!("My number is {:?}", num);
}
