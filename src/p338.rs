use crate::tests;

pub fn count_bits(n: i32) -> Vec<i32> {
    let mut res = Vec::with_capacity(n as usize);
    res.push(0);
    (1..=n as usize).for_each(|i| res.push(res[i >> 1] + (i as i32 & 1)));
    res
}

tests! {
    count_bits,
    2 => [0, 1, 1],
    5 => [0, 1, 1, 2, 1, 2],
}
