//! utilities.

/// generate a `Solution` stub.
///
/// the function signatures used by the LeetCode `Solution` struct are most of
/// the time not idiomatic Rust. this macro allows us to implement our solution
/// idiomatically, and provide a stub function which can just be copied into the
/// submission editor along with our implementation.
#[doc(hidden)]
#[macro_export]
macro_rules! solution {
  (
    impl Solution {
      $stub:item
    }
  ) => {
    #[allow(dead_code)]
    #[cfg(not(tarpaulin))]
    struct Solution;

    #[allow(dead_code)]
    #[cfg(not(tarpaulin))]
    impl Solution {
      $stub
    }
  };
}

/// generate tests with given test cases.
///
/// this macro uses the [`assert_matches`] crate under the hood, so the output
/// pattern can be specified with that crate's syntax.
#[doc(hidden)]
#[macro_export]
macro_rules! tests {
  (
    $impl:ident,
    $($name:ident: $($input:expr),* $(,)? => $output:pat),+ $(,)?
  ) => {
    #[cfg(test)]
    mod tests {
      $(
        #[test]
        fn $name() {
          ::assert_matches::assert_matches!(
            super::$impl($($input),*),
            $output
          );
        }
      )+
    }
  }
}
