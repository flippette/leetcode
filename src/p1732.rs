use crate::tests;

pub fn largest_altitude(gain: impl AsRef<[i32]>) -> i32 {
    let mut cur = 0;
    gain.as_ref()
        .iter()
        .map(|change| {
            cur += change;
            cur
        })
        .max()
        .unwrap()
        .max(0)
}

tests! {
    largest_altitude,
    [-5, 1, 5, 0, -7] => 1,
    [-4, -3, -2, -1, 4, 3, 2] => 0,
}
