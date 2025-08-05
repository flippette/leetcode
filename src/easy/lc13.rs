//! #13: Roman to Integer

pub fn roman_to_int(s: impl AsRef<str>) -> u16 {
  let mut s = s.as_ref();
  let mut n = 0;

  while let Some((atom, rest)) = atom(s) {
    let frag = match atom {
      "MMM" => 3000,
      "CCC" => 300,
      "XXX" => 30,
      "III" => 3,

      "MM" => 2000,
      "CC" => 200,
      "XX" => 20,
      "II" => 2,

      "CM" => 900,
      "CD" => 400,
      "XC" => 90,
      "XL" => 40,
      "IX" => 9,
      "IV" => 4,

      "M" => 1000,
      "D" => 500,
      "C" => 100,
      "L" => 50,
      "X" => 10,
      "V" => 5,
      "I" => 1,

      other => unreachable!("unknown atom: {other}"),
    };

    s = rest;
    n += frag;
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

fn atom(s: &str) -> Option<(&str, &str)> {
  #[rustfmt::skip]
  const ATOMS: [&str; 21] = [
    "MMM", "CCC", "XXX", "III",
    "MM", "CC", "XX", "II",
    "CM", "CD", "XC", "XL", "IX", "IV",
    "M", "D", "C", "L", "X", "V", "I",
  ];

  for atom in ATOMS {
    if let some @ Some(_) = tag(s, atom) {
      return some;
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
}
