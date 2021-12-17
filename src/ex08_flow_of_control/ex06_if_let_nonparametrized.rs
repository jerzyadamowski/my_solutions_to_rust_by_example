// This enum purposely neither implements nor derives PartialEq.
// That is why comparing Foo::Bar == a fails below.
enum Foo {
  Bar,
  _Zone,
}

pub fn example() {
  let a = Foo::Bar;

  // Variable a matches Foo::Bar
  if let Foo::Bar = a {
    // ^-- this causes a compile-time error. Use `if let` instead.
    println!("a is foobar");
  } else {
    println!("a not foobar");
  }
}
