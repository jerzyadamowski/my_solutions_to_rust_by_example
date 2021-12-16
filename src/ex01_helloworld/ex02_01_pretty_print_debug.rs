#[derive(Debug)]
struct Person<'a> {
    _name: &'a str,
    _age: u8,
}

pub fn example() {
    let name = "Peter";
    let age = 27;
    let peter = Person {
        _name: name,
        _age: age,
    };

    // Pretty print
    println!("{:#?}", peter);
}
