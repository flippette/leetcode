#![allow(unused_macros)]

#[macro_export]
macro_rules! tests {
    ($fn:expr, $($in:expr => $out:expr),+ $(,)?) => {
        #[allow(dead_code)]
        #[cfg(test)]
        #[test]
        fn tests() {
            $({
                let res = $fn($in);
                if res != $out {
                    panic!(
                        "verify failed
    input: {:?}
   output: {:?}
 expected: {:?}",
                        $in, res, $out
                    );
                }
            })+
        }
    };
}
