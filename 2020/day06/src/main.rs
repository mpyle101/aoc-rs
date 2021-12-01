fn main() {
    let answers = load(include_str!("./answers.txt"));

    let count = part_one(&answers);
    println!("Part 1: {}", count);

    let count = part_two(&answers);
    println!("Part 2: {}", count);
}

fn load(input: &str) -> Vec<Vec<&str>> {
    input.split("\n\n").map(|s| s.split('\n').collect()).collect()
}

fn part_one(answers: &[Vec<&str>]) -> usize {
    use std::collections::HashSet;

    // Join vectors into a single set and get then length.
    answers.iter().map(|v| 
        v.iter().flat_map(|&s| s.as_bytes().iter().collect::<Vec<_>>())
        .collect::<HashSet<_>>().len()
    ).sum()
}

fn part_two(answers: &[Vec<&str>]) -> usize {
    use std::collections::HashSet;

    // Intersect the vectors for each group and sum the lengths.
    answers.iter().map(|v| {
        let f = v[0].as_bytes().iter().collect::<HashSet<_>>();
        v.iter().fold(f, |acc, &s| {
            let h = s.as_bytes().iter().collect::<HashSet<_>>();
            acc.intersection(&h).cloned().collect::<HashSet<_>>()
        }).len()
    }).sum()
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let answers = load(include_str!("./answers.txt"));

    let count = part_one(&answers);
    assert_eq!(count, 6430);

    let count = part_two(&answers);
    assert_eq!(count, 3125);
  }
}