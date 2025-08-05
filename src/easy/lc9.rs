//! #9: Palindrome Number

pub fn is_palindrome(n: i32) -> bool {
  let n = if n.is_negative() {
    return false;
  } else {
    n.cast_unsigned()
  };

  n == {
    let mut n = n;
    let mut r = 0;

    while n != 0 {
      r = r * 10 + n % 10;
      n /= 10;
    }

    r
  }
}

crate::solution! {
  impl Solution {
    pub fn is_palindrome(n: i32) -> bool {
      is_palindrome(n)
    }
  }
}

crate::tests! {
  is_palindrome,
     case1:     121 => true,
     case2:    -121 => false,
     case3:      10 => false,
  case7757: 1000021 => false,
 case11511:       0 => true,
     hole1:   10201 => true,
     hole2:  100201 => false,
}
