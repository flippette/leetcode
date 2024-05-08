use crate::tests;

pub fn single_number(nums: impl AsRef<[i32]>) -> i32 {
    nums.as_ref().iter().copied().fold(0, |acc, n| acc ^ n)
}

tests! {
    single_number,
    [2, 2, 1] => 1,
    [4, 1, 2, 1, 2] => 4,
    [1] => 1,
}
