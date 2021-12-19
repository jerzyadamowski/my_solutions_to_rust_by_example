use std::ops::{Add, Mul, Sub};

macro_rules! assert_equal_len {
  // The `tt` (token tree) designator is used for
  // operators and tokens.
  ($a:expr, $b:expr, $func:ident, $op:tt) => {
    assert!(
      $a.len() == $b.len(),
      "{:?}: dimension mismatch: {:?} {:?} {:?}",
      stringify!($func),
      ($a.len(),),
      stringify!($op),
      ($b.len(),)
    );
  };
}

macro_rules! op {
  ($func:ident, $bound:ident, $op:tt, $method:ident) => {
    fn $func<T: $bound<T, Output = T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
      assert_equal_len!(xs, ys, $func, $op);

      for (x, y) in xs.iter_mut().zip(ys.iter()) {
        *x = $bound::$method(*x, *y);
        // *x = x.$method(*y);
      }
    }
  };
}

// Implement `add_assign`, `mul_assign`, and `sub_assign` functions.
op!(_add_assign, Add, +=, add);
op!(_mul_assign, Mul, *=, mul);
op!(_sub_assign, Sub, -=, sub);

mod test {
  macro_rules! test {
    ($func:ident, $x:expr, $y:expr, $z:expr) => {
      #[test]
      fn $func() {
        for size in 0usize..10 {
          let mut x: Vec<_> = std::iter::repeat($x).take(size).collect();
          let y: Vec<_> = std::iter::repeat($y).take(size).collect();
          let z: Vec<_> = std::iter::repeat($z).take(size).collect();

          super::$func(&mut x, &y);

          assert_eq!(x, z);
        }
      }
    };
  }

  // Test `add_assign`, `mul_assign`, and `sub_assign`.
  test!(_add_assign, 1u32, 2u32, 3u32);
  test!(_mul_assign, 2u32, 3u32, 6u32);
  test!(_sub_assign, 3u32, 2u32, 1u32);
}

pub fn _example() {}
//"rust": "cd $dir && rustc --test -g $fileName --out-dir $workspaceRoot/bin/$fileNameWithoutExt && $workspaceRoot/bin/$fileNameWithoutExt/$fileNameWithoutExt"
