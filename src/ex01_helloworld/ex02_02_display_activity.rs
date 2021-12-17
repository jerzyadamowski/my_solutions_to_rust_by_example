use std::fmt; // Import `fmt`

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Complex {
  real: f64,
  imag: f64,
}

// Similarly, implement `Display` for `Point2D`
impl fmt::Display for Complex {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // Customize so only `x` and `y` are denoted.
    if self.imag.signum() == 1.0 {
      write!(f, "{} + {}i", self.real, self.imag)
    } else {
      write!(f, "{} - {}i", self.real, self.imag.abs())
    }
  }
}

pub fn example() {
  let point = Complex {
    real: 3.3,
    imag: -7.2,
  };

  println!("Compare points:");
  println!("Display: {}", point);
  println!("Debug: {:?}", point);
}
