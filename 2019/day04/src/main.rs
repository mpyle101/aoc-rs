// Break the password value into an array of bytes containing
// the digits from left to right.
// Run through digits applying the rules: non increasing and at
// least one pair+ for Part 1.
// Non increasing and one pair for Part 2 (keep track of digit
// runs and mark the password valid if a run of length 2 is found).

use std::cell::RefCell;

fn main() {
  let buf: RefCell<[u8; 6]> = RefCell::new([0; 6]);
  let count = (109165..=576723).map(|v| make_password(v, &buf))
    .filter(|&p| check_password2(p))
    .count();

  println!("{} valid passwords", count);
}

fn make_password(v: u32, rc: &RefCell<[u8; 6]>) -> &RefCell<[u8; 6]> {
  let mut buf = rc.borrow_mut();
  let mut num = v;
  let mut idx: i8 = 5;
  while num > 0 {
    buf[idx as usize] = (num % 10) as u8;
    num /= 10;
    idx -= 1;
  }
  rc
}

#[allow(dead_code)]
fn check_password1(rc: &RefCell<[u8; 6]>) -> bool {
  let passwd = rc.borrow();
  let mut last = passwd[0];
  let mut valid = false;
  for digit in passwd.iter().skip(1) {
    match *digit {
      d if d < last  => return false,
      d if d == last => valid = true,
      _ => ()
    }
    last = *digit
  }

  valid
}

fn check_password2(rc: &RefCell<[u8; 6]>) -> bool {
  let passwd = rc.borrow();
  let mut run = 1;
  let mut last = passwd[0];
  let mut valid = false;
  for digit in passwd.iter().skip(1) {
    match *digit {
      d if d < last  => return false,
      d if d == last => run += 1,
      _ => {
        if run == 2 { valid = true }
        run = 1;
      }
    }
    last = *digit;
  }

  valid || run == 2
}


/** Unit Tests */
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_make_passwords() {
    let buf: RefCell<[u8; 6]> = RefCell::new([0; 6]);
    let digits = make_password(109165, &buf);
    let expected: [u8; 6] = [1,0,9,1,6,5];

    assert_eq!(*digits.borrow(), expected);
  }

  #[test]
  fn one_valid_password1() {
    let buf: RefCell<[u8; 6]> = RefCell::new([0; 6]);
    let digits = make_password(111111, &buf);

    assert!(check_password1(digits));
  }

  #[test]
  fn one_valid_password2() {
    let buf: RefCell<[u8; 6]> = RefCell::new([0; 6]);
    let digits = make_password(223459, &buf);

    assert!(check_password1(digits));
  }

  #[test]
  fn one_non_increasing_fails() {
    let buf: RefCell<[u8; 6]> = RefCell::new([0; 6]);
    let digits = make_password(223450, &buf);

    assert!(!check_password1(digits));
  }

  #[test]
  fn one_no_double_digit_fails() {
    let buf: RefCell<[u8; 6]> = RefCell::new([0; 6]);
    let digits = make_password(123789, &buf);

    assert!(!check_password1(digits));
  }

  #[test]
  fn two_valid_password1() {
    let buf: RefCell<[u8; 6]> = RefCell::new([0; 6]);
    let digits = make_password(112233, &buf);

    assert!(check_password2(digits));
  }

  #[test]
  fn two_valid_password2() {
    let buf: RefCell<[u8; 6]> = RefCell::new([0; 6]);
    let digits = make_password(567899, &buf);

    assert!(check_password2(digits));
  }

  #[test]
  fn two_valid_password3() {
    let buf: RefCell<[u8; 6]> = RefCell::new([0; 6]);
    let digits = make_password(111122, &buf);

    assert!(check_password2(digits));
  }

  #[test]
  fn two_triple_fails() {
    let buf: RefCell<[u8; 6]> = RefCell::new([0; 6]);
    let digits = make_password(123444, &buf);

    assert!(!check_password2(digits));
  }
}
