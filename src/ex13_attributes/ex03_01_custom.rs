#[cfg(some_condition)]
fn conditional_function() {
  println!("condition met!");
}

pub fn example() {
  // $ rustc --cfg some_condition 03_01_custom.rs && ./03_01_custom
  // condition met! below uncomment with
  // conditional_function();
}
