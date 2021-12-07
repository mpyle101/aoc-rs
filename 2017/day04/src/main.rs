fn main() {
    use std::fs;
    use std::time::Instant;

    let input = fs::read_to_string("./input.txt").unwrap();
    let phrases = load(&input);

    let t1 = Instant::now();
    let valid = part_one(&phrases);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", valid, t2 - t1);

    let t1 = Instant::now();
    let valid = part_two(&phrases);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", valid, t2 - t1);
}

fn load(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn part_one(phrases: &[&str]) -> i32 {
    use std::collections::HashSet;

    phrases.iter().map(|s| {
        let words: Vec<_> = s.split(' ').collect();
        let unique = HashSet::<&&str>::from_iter(words.iter());
        unique.len() == words.len()
    }).filter(|valid| *valid).count() as i32
}

fn part_two(phrases: &[&str]) -> i32 {
    use itertools::Itertools;

    phrases.iter().map(|s| {
        s.split(' ').combinations(2).any(|v| {
            v[0].len() == v[1].len() &&  // a little optimization
            (*v[0]).chars().sorted().eq((*v[1]).chars().sorted())
        })
    }).filter(|invalid| !*invalid).count() as i32
}


#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn it_works() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let phrases = load(&input);

    let valid = part_one(&phrases);
    assert_eq!(valid, 325);

    let valid = part_two(&phrases);
    assert_eq!(valid, 119);
  }
}