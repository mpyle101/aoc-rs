use regex::Regex;

fn main() {
    use std::time::Instant;

    let input = include_str!("./input.txt");

    let t1 = Instant::now();
    let sum = part_one(&input);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", sum, t2 - t1);
}

fn part_one(s: &str) -> i32 {
    let re = Regex::new(r"(?-u:\-?\d+)").unwrap();
    re.find_iter(s)
        .map(|v| v.as_str().parse::<i32>().unwrap())
        .sum()
}



#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = include_str!("./input.txt");

    let sum = part_one(&input);
    assert_eq!(sum, 191164);
  }
}