#[derive(Debug)]
struct Person<'a> {
  // The 'a defines a lifetime
  _name: &'a str,
  _age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
  x: f32,
  y: f32,
}

impl Point {
  fn square(&self, a: f32) -> Rectangle {
    Rectangle {
      top_left: Point { ..*self },
      bottom_right: Point {
        x: self.x + a,
        y: self.y + a,
      },
    }
  }
}

// Structs can be reused as fields of another struct
// #[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
  // A rectangle can be specified by where the top left and bottom right
  // corners are in space.
  top_left: Point,
  bottom_right: Point,
}

impl Rectangle {
  fn rect_area(&self) -> f32 {
    let (x, y) = (
      (self.top_left.x - self.bottom_right.x).abs(),
      (self.top_left.y - self.bottom_right.y).abs(),
    );
    x * y
  }
}

pub fn example() {
  // Create struct with field init shorthand
  let name = "Peter";
  let age = 27;
  let peter = Person {
    _name: name,
    _age: age,
  };

  // Print debug struct
  println!("{:?}", peter);

  // Instantiate a `Point`
  let point: Point = Point { x: 10.3, y: 0.4 };

  println!("Square: {:?}", point.square(2.0));

  // Access the fields of the point
  println!("point coordinates: ({}, {})", point.x, point.y);

  // Make a new point by using struct update syntax to use the fields of our
  // other one
  let bottom_right = Point { x: 5.2, ..point };

  // `bottom_right.y` will be the same as `point.y` because we used that field
  // from `point`
  println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

  // Destructure the point using a `let` binding
  let Point {
    x: top_edge,
    y: left_edge,
  } = point;

  println!("{:?}", top_edge);
  println!("{:?}", left_edge);

  let rectangle = Rectangle {
    // struct instantiation is an expression too
    top_left: Point {
      x: left_edge,
      y: top_edge,
    },
    bottom_right: bottom_right,
  };

  println!("rectangle area: {}", rectangle.rect_area());

  // Instantiate a unit struct
  let _unit = Unit;

  // Instantiate a tuple struct
  let pair = Pair(1, 0.1);

  // Access the fields of a tuple struct
  println!("pair contains {:?} and {:?}", pair.0, pair.1);

  // Destructure a tuple struct
  let Pair(integer, decimal) = pair;

  println!("pair contains {:?} and {:?}", integer, decimal);
}
