use crate::tests;

pub fn is_subsequence(s: impl AsRef<str>, t: impl AsRef<str>) -> bool {
    let mut s = s.as_ref().chars();
    let mut t = t.as_ref().chars();
    let mut sc;

    macro_rules! next_or_ret {
        ($iter:expr, $ret:expr) => {
            if let Some(item) = $iter.next() {
                item
            } else {
                return $ret;
            }
        };
    }

    loop {
        sc = next_or_ret!(s, true);
        while next_or_ret!(t, false) != sc {}
    }
}

tests! {
    |(s, t)| is_subsequence(s, t),
    ("abc", "agbgdc") => true,
    ("axc", "ahbgdc") => false,
}
