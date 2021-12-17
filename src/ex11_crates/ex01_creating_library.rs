pub fn _public_function() {
  println!("called rary's `public_function()`");
}

fn _private_function() {
  println!("called rary's `private_function()`");
}

pub fn _indirect_access() {
  print!("called rary's `indirect_access()`, that\n> ");

  _private_function();
}

pub fn example() {
  // $ rustc --crate-type=lib 01_creating_library.rs --crate-name=rary
  // $ ls lib*
  // lib01_creating_library.rlib
}
