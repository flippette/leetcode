use crate::tests;

pub fn my_sqrt(x: i32) -> i32 {
    (0..=(x / 2 + 2).min(46340))
        .find(|n| n * n > x)
        .map(|n| n - 1)
        .unwrap_or(46340)
}

tests! {
    my_sqrt,
    4 => 2,
    8 => 2,
    1 => 1,
    0 => 0,
    2147395600 => 46340,
}
