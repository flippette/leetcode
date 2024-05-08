use crate::tests;

pub fn longest_valid_parentheses(s: impl AsRef<str>) -> i32 {
    todo!()
}

tests! {
    longest_valid_parentheses,
    "(()" => 2,
    ")()())" => 4,
    "" => 0,
}
