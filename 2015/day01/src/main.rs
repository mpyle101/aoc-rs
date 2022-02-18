fn main() {
    let input = include_str!("./input.txt");

    let floor = part_one(input);
    println!("Part 1: {floor}");

    let pos = part_two(input);
    println!("Part 2: {pos}");
}

fn part_one(input: &str) -> i32 {
    input.chars().fold(0, |acc, c| acc + if c == '(' { 1 } else { -1 })
}

fn part_two(input: &str) -> usize {
    let mut floor = 0;
    input.chars()
        .enumerate()
        .map(|(i, c)| { floor += if c == '(' { 1 } else { -1 }; (i, floor) })
        .find_map(|(i, f)| (f < 0).then(||i+1))
        .unwrap()
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = include_str!("./input.txt");

    let floor = part_one(input);
    assert_eq!(floor, 280);

    let pos = part_two(input);
    assert_eq!(pos, 1797);
  }
}