fn drink(beverage: &str) {
  // You shouldn't drink too much sugary beverages.
  if beverage == "lemonade" {
    // uncomment to use painc!
    // panic!("AAAaaaaa!!!!");
  }

  println!("Some refreshing {} is all I need.", beverage);
}

pub fn example() {
  drink("water");
  drink("lemonade");
}
