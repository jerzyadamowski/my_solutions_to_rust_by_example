#[cfg(some_condition)]
fn conditional_function() {
  println!("condition met!");
}

fn main() {
  conditional_function();
}

// $ rustc --cfg some_condition 03_01_custom.rs && ./03_01_custom
// condition met!
