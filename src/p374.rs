extern "C" {
    fn guess(num: i32) -> i32;
}

// we're probably interfacing over FFI, hence the `unsafe`
#[allow(non_snake_case, clippy::missing_safety_doc)]
pub unsafe fn guessNumber(n: i32) -> i32 {
    if guess(1) == 0 {
        return 1;
    }
    if guess(n) == 0 {
        return n;
    }

    let mut lower = 1;
    let mut higher = n;
    let mut cur = lower + ((higher - lower) >> 1);

    loop {
        match guess(cur) {
            1 => lower = cur,
            -1 => higher = cur,
            0 => return cur,
            _ => unreachable!(),
        }
        cur = lower + ((higher - lower) >> 1);
    }
}

// no tests, i don't wanna deal with the build system
