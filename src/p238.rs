use crate::tests;

pub fn product_except_self(nums: impl AsRef<[i32]>) -> Vec<i32> {
    let nums = nums.as_ref();
    let mut res = Vec::with_capacity(nums.len());

    // prefix pass
    let mut pre = 1; // leftmost item doesn't have a prefix
    (0..nums.len()).for_each(|i| {
        res.push(pre);
        pre *= nums[i];
    });

    // postfix pass
    let mut post = 1; // rightmost item doesn't have a postfix
    (0..nums.len()).rev().for_each(|i| {
        res[i] *= post;
        post *= nums[i];
    });

    res
}

tests! {
    product_except_self,
    [1, 2, 3, 4] => [24, 12, 8, 6],
    [-1, 1, 0, -3, 3] => [0, 0, 9, 0, 0],
}
