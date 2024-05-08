use crate::tests;

pub fn move_zeroes(mut nums: impl AsMut<[i32]>) {
    let nums = nums.as_mut();
    let mut ins = 0;

    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums.swap(i, ins);
            ins += 1;
        }
    }
}

tests! {
    |input| {
        let mut tmp = input;
        move_zeroes(&mut tmp);
        tmp
    },
    [0, 1, 0, 3, 12] => [1, 3, 12, 0, 0],
    [0] => [0],
}
