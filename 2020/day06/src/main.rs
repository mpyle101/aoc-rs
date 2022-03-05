use std::collections::HashSet;

fn main() {
    let answers = load(include_str!("./answers.txt"));

    timeit("Part 1", || part_one(&answers));
    timeit("Part 2", || part_two(&answers));
}

fn timeit<T>(s: &str, func: impl Fn() -> T)
    where T: std::fmt::Debug
{
    let t = std::time::Instant::now();
    let result = func();
    println!("{s}: {:?} ({:?})", result, t.elapsed());
}

fn load(input: &str) -> Vec<Vec<&str>> {
    input.split("\n\n").map(|s| s.split('\n').collect()).collect()
}

fn part_one(answers: &[Vec<&str>]) -> usize {
    // Union the vectors for each group and sum the lengths.
    answers.iter()
        .map(|v| {
            let f: HashSet<&u8> = HashSet::new();
            v.iter().fold(f, |set, s| &set | &to_set(s)).len()
        })
        .sum()
}

fn part_two(answers: &[Vec<&str>]) -> usize {
    // Intersect the vectors for each group and sum the lengths.
    answers.iter()
        .map(|v| {
            let f: HashSet<&u8> = v[0].as_bytes().iter().collect();
            v.iter().fold(f, |set, s| &set & &to_set(s)).len()
        })
        .sum()
}

fn to_set(s: &str) -> HashSet<&u8> {
    s.as_bytes().iter().collect()
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