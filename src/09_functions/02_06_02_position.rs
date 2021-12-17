pub trait Iterator {
  // The type being iterated over.
  type Item;

  // `find` takes `&mut self` meaning the caller may be borrowed
  // and modified, but not consumed.
  fn find<P>(&mut self, _predicate: P) -> Option<Self::Item>
  where
    // `FnMut` meaning any captured variable may at most be
    // modified, not consumed. `&Self::Item` states it takes
    // arguments to the closure by reference.
    P: FnMut(&Self::Item) -> bool,
  {
    None
    //^ to satisfy compiler
  }
}

fn main() {
  let vec = vec![1, 9, 3, 3, 13, 2];

  let index_of_first_even_number = vec.iter().position(|x| x % 2 == 0);
  assert_eq!(index_of_first_even_number, Some(5));

  let index_of_first_negative_number = vec.iter().position(|x| x < &0);
  assert_eq!(index_of_first_negative_number, None);
}
