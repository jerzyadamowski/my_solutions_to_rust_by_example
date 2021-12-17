// Bind the `deeply::nested::function` path to `other_function`.
use crate::ex10_modules::ex03_as_use::deeply::nested::function as crate_function;
use deeply::nested::function as other_function;

fn function() {
  println!("called `function()`");
}

mod deeply {
  pub mod nested {
    pub fn function() {
      println!("called `deeply::nested::function()`");
    }
  }
}

pub fn example() {
  // Easier access to `deeply::nested::function`
  other_function();

  println!("Entering block");
  {
    // This is equivalent to `use deeply::nested::function as function`.
    // This `function()` will shadow the outer one.
    use crate::ex10_modules::ex03_as_use::deeply::nested::function;

    // `use` bindings have a local scope. In this case, the
    // shadowing of `function()` is only in this block.
    function();

    println!("Leaving block");
  }

  function();

  crate_function();
}
