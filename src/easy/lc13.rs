//! #13: Roman to Integer

pub fn roman_to_int(s: impl AsRef<str>) -> u16 {
  let mut s = s.as_ref();
  let mut n = 0;

  while let Some((atom, rest)) = atom(s) {
    s = rest;
    n += atom;
  }

  n
}

fn tag<'a>(s: &'a str, tag: &str) -> Option<(&'a str, &'a str)> {
  if s.starts_with(tag) {
    Some(s.split_at(tag.len()))
  } else {
    None
  }
}

fn atom(s: &str) -> Option<(u16, &str)> {
  #[rustfmt::skip]
  const ATOMS: [(&str, u16); 21] = [
    ("MMM", 3000), ("CCC", 300), ("XXX",  30), ("III",  3),

    ( "MM", 2000), ( "CC", 200), ( "XX",  20), ( "II",  2),
    ( "CM",  900), ( "CD", 400), ( "XC",  90), ( "XL", 40),
    ( "IX",    9), ( "IV",   4),

    (  "M", 1000), (  "D", 500), (  "C", 100), (  "L", 50),
    (  "X",   10), (  "V",   5), (  "I",   1),
  ];

  for (key, val) in ATOMS {
    if let Some((_, rest)) = tag(s, key) {
      return Some((val, rest));
    }
  }

  None
}

crate::solution! {
  impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
      roman_to_int(s).into()
    }
  }
}

crate::tests! {
  roman_to_int,
  case1:     "III" => 3,
  case2:   "LVIII" => 58,
  case3: "MCMXCIV" => 1994,
  tail1:  "MMblah" => 2000,
  tail2:   "Iroha" => 1,
}
