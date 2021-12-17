struct ToDrop;

impl Drop for ToDrop {
  fn drop(&mut self) {
    println!("ToDrop is being dropped");
  }
}

pub fn example() {
  let _x = ToDrop;
  println!("Made a ToDrop!");
}
