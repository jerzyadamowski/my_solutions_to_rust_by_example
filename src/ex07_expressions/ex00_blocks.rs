pub fn example() {
  let x = 5u32;

  let y = {
    let x_squared = x * x;
    let x_cube = x_squared * x;

    // This expression will be assigned to `y`
    x_cube + x_squared + x
  };

  let z = {
    // The semicolon suppresses this expression and `()` is assigned to `z`
    let _ = 2 * x;
    // Without semicolon
    2 * x
  };

  println!("x is {:?}", x);
  println!("y is {:?}", y);
  println!("z is {:?}", z);
}
