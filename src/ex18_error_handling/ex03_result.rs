fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
  // Let's try using `unwrap()` to get the number out. Will it bite us?
  // change to unwrap to get panic!
  let first_number = first_number_str.parse::<i32>().unwrap_or_default();
  let second_number = second_number_str.parse::<i32>().unwrap_or_default();
  first_number * second_number
}

pub fn example() {
  let twenty = multiply("10", "2");
  println!("double is {}", twenty);

  let tt = multiply("t", "2");
  println!("double is {}", tt);
}
