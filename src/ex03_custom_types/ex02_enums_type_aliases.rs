enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    _Subtract,
}

// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

pub fn example() {
    // We can refer to each variant via its alias, not its long and inconvenient
    // name.
    let _x = Operations::Add;

    let sum = _x.run(1, 2);
    println!("Add.run(1,2): {}", sum);
}
