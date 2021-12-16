enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
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
    let x = Operations::Add;
    let y = Operations::Subtract;

    let sum = x.run(1, 2);
    let diff = y.run(2, 1);
    println!("Add.run(1,2): {}", sum);
    println!("Subtract.run(2,1): {}", diff);
}
