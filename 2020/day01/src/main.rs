use std::{error, fmt};
use itertools::Itertools;

fn main() -> Result<(), Box<dyn error::Error>> {
    let expenses = load(include_str!("./input.txt"))?;

    let result = part_one(&expenses)?;
    println!("Part 1: {result}");

    let result = part_two(&expenses)?;
    println!("Part 2: {result}");

    Ok(())
}

#[derive(Debug)]
struct NotFound;
impl fmt::Display for NotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No combination found")
    }
}
impl error::Error for NotFound {}

fn load(input: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
    input.lines()
        .map(|v| v.parse::<i32>())
        .into_iter()
        .collect()
}

fn part_one(expenses: &[i32]) -> Result<i32, NotFound> {
    expenses.iter().combinations(2)
        .filter_map(|v| (v[0] + v[1] == 2020).then(|| v[0] * v[1]))
        .next().ok_or(NotFound)
}

fn part_two(expenses: &[i32]) -> Result<i32, NotFound> {
    expenses.iter().combinations(3)
        .filter(|v| v[0] + v[1] + v[2] == 2020)
        .map(|v| v[0] * v[1] * v[2])
        .next().ok_or(NotFound)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), Box<dyn error::Error>> {
        let expenses = load(include_str!("./input.txt"))?;

        let result = part_one(&expenses)?;
        assert_eq!(result, 878724);

        let result = part_two(&expenses)?;
        assert_eq!(result, 201251610);

        Ok(())
    }
}