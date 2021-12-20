// Re-implementation of integer division (/)
fn division(dividend: i32, divisor: i32) -> i32 {
  if divisor == 0 {
    // Division by zero triggers a panic
    // uncomment line below to get panic!
    // panic!("division by zero");
    eprintln!("division by zero");
    0
  } else {
    dividend / divisor
  }
}

// The `main` task
pub fn example() {
  // Heap allocated integer
  let _x = Box::new(0i32);

  // This operation will trigger a task failure
  division(3, 0);

  println!("This point won't be reached!");

  // `_x` should get destroyed at this point
}
