pub trait Iterator {
  // The type being iterated over.
  type Item;

  // `any` takes `&mut self` meaning the caller may be borrowed
  // and modified, but not consumed.
  fn any<F>(&mut self, _f: F) -> bool
  where
    // `FnMut` meaning any captured variable may at most be
    // modified, not consumed. `Self::Item` states it takes
    // arguments to the closure by value.
    F: FnMut(Self::Item) -> bool,
  {
    true
    //^ to satisfy compiler
  }
}

pub fn example() {
  let vec1 = vec![1, 2, 3];
  let vec2 = vec![4, 5, 6];

  // `iter()` for vecs yields `&i32`. Destructure to `i32`.
  println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
  // `into_iter()` for vecs yields `i32`. No destructuring required.
  println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

  let array1 = [1, 2, 3];
  let array2 = [4, 5, 6];

  // `iter()` for arrays yields `&i32`.
  println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
  // `into_iter()` for arrays unusually yields `&i32`.
  println!("2 in array2: {}", array2.into_iter().any(|x| x == 2));
}
