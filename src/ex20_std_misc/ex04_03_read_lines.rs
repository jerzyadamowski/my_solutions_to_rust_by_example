use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn example() {
  // File hosts must exist in current path before this produces output
  if let Ok(lines) = read_lines("./hosts") {
    // Consumes the iterator, returns an (Optional) String
    for line in lines {
      if let Ok(ip) = line {
        println!("{}", ip);
      }
    }
  }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

// $ echo -e "127.0.0.1\n192.168.0.1\n" > hosts
// $ rustc read_lines.rs && ./read_lines
