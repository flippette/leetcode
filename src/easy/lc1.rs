//! #1: Two Sum

use std::collections::HashMap;

pub fn two_sum(nums: impl AsRef<[i32]>, target: i32) -> Option<(usize, usize)> {
  let nums = nums.as_ref();
  let mut diffs = HashMap::with_capacity(nums.len());
  for (i, num) in nums.iter().enumerate() {
    if let Some(e) = diffs.get(num) {
      return Some((*e, i));
    } else {
      diffs.insert(target - num, i);
    }
  }
  None
}

crate::solution! {
  impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
      two_sum(nums, target)
        .map(|(a, b)| vec![a as _, b as _])
        .unwrap()
    }
  }
}

crate::tests! {
  two_sum,
  case1: [2, 7, 11, 15],  9 => Some((0, 1)),
  case2:      [3, 2, 4],  6 => Some((1, 2)),
  case3:         [3, 3],  6 => Some((0, 1)),
  none1:      [1, 2, 3], 10 => None,
  none2:     [2, 4, -5], -2 => None,
}
