// This crate is a library
// uncomment line below to get create
// #![crate_type = "lib"]
// The library is named "rary"
// uncomment line below to get create
// #![crate_name = "rary"]

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
  // $ rustc 02_crates.rs
  // $ ls lib*
  // library.rlib
}
