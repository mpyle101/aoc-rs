use itertools::Itertools;

fn main() {
    let expenses: Vec<_> = include_str!("./expenses.txt")
        .lines().map(|v| v.parse::<i32>().unwrap())
        .collect();

    let result = part_one(&expenses);
    println!("Part1: {}", result);

    let result = part_two(&expenses);
    println!("Part2: {}", result);
}

fn part_one(expenses: &Vec<i32>) -> i32 {
    expenses.iter().combinations(2)
        .filter(|v| v[0] + v[1] == 2020)
        .map(|v| v[0] * v[1])
        .collect::<Vec<_>>()[0]
}

fn part_two(expenses: &Vec<i32>) -> i32 {
    expenses.iter().combinations(3)
        .filter(|v| v[0] + v[1] + v[2] == 2020)
        .map(|v| v[0] * v[1] * v[2])
        .collect::<Vec<_>>()[0]
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let expenses: Vec<_> = include_str!("./expenses.txt")
        .lines().map(|v| v.parse::<i32>().unwrap())
        .collect();

    let result = part_one(&expenses);
    assert_eq!(result, 878724);

    let result = part_two(&expenses);
    assert_eq!(result, 201251610);
  }
}