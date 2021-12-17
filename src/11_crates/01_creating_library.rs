pub fn public_function() {
  println!("called rary's `public_function()`");
}

fn private_function() {
  println!("called rary's `private_function()`");
}

pub fn indirect_access() {
  print!("called rary's `indirect_access()`, that\n> ");

  private_function();
}

// $ rustc --crate-type=lib 01_creating_library.rs --crate-name=rary
// $ ls lib*
// lib01_creating_library.rlib
