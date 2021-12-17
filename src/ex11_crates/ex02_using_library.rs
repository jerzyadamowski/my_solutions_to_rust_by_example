// extern crate rary; // May be required for Rust 2015 edition or earlier

pub fn example() {
  // create library and uncomment line below:
  // rary::public_function();

  // Error! `private_function` is private
  //rary::private_function();

  // create library and uncomment line below:
  // rary::indirect_access();
}

// # Where library.rlib is the path to the compiled library, assumed that it's
// # in the same directory here:
// $ rustc 02_using_library.rs --extern rary=library.rlib --edition=2021 && ./02_using_library
// called rary's `public_function()`
// called rary's `indirect_access()`, that
// > called rary's `private_function()`
