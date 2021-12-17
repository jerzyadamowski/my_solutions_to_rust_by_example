// A struct with annotation of lifetimes.
#[derive(Debug)]
struct Borrowed<'a> {
  _x: &'a i32,
}

// Annotate lifetimes to impl.
impl<'a> Default for Borrowed<'a> {
  fn default() -> Self {
    Self { _x: &10 }
  }
}

pub fn example() {
  let b: Borrowed = Default::default();
  println!("b is {:?}", b);
}
