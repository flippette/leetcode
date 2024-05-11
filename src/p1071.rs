use crate::tests;

pub fn gcd_of_strings(s1: impl AsRef<str>, s2: impl AsRef<str>) -> String {
    todo!()
}

tests! {
    gcd_of_strings,
    "ABCABC", "ABC" => "ABC",
    "ABABAB", "AB" => "AB",
    "LEET", "CODE" => "",
}
