use crate::tests;

pub fn merge_alternately(
    word1: impl AsRef<str>,
    word2: impl AsRef<str>,
) -> String {
    let word1 = word1.as_ref();
    let word2 = word2.as_ref();
    let mut acc = String::with_capacity(word1.len() + word2.len());
    let mut chars1 = word1.chars().enumerate();
    let mut chars2 = word2.chars().enumerate();

    loop {
        match (chars1.next(), chars2.next()) {
            (Some((_, ch1)), Some((_, ch2))) => {
                acc.push(ch1);
                acc.push(ch2);
            }
            (Some((i1, _)), None) => {
                acc.push_str(&word1[i1..]);
                break;
            }
            (None, Some((i2, _))) => {
                acc.push_str(&word2[i2..]);
                break;
            }
            _ => break,
        }
    }

    acc
}

tests! {
    |(a, b)| merge_alternately(a, b),
    ("abc", "pqr") => "apbqcr",
    ("ab", "pqrs") => "apbqrs",
    ("abcd", "pq") => "apbqcd",
}
