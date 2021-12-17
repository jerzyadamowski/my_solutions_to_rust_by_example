fn used_function() {}

// `#[allow(dead_code)]` is an attribute that disables the `dead_code` lint
#[allow(dead_code)]
fn unused_function() {}

fn _noisy_unused_function() {}
// FIXME ^ Add an attribute to suppress the warning

pub fn example() {
  used_function();
}
