
fn main() {
    use std::time::Instant;

    let input = load(include_str!("./input.txt"))
        .expect("Loading failed: ");

    let t1 = Instant::now();
    let freq = part_one(&input);
    let t2 = Instant::now();
    println!("Part 1: {freq} ({:?})", t2 - t1);

    let t1 = Instant::now();
    let freq = part_two(&input);
    let t2 = Instant::now();
    println!("Part 2: {freq} ({:?})", t2 - t1);
}

fn load(input: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
    input.lines().map(|v| v.parse::<i32>()).into_iter().collect()
}

fn part_one(deltas: &[i32]) -> i32 {
    deltas.iter().sum()
}

fn part_two(deltas: &[i32]) -> i32 {
    use std::collections::HashSet;

    let mut value = 0;
    let mut seen = HashSet::new();
    let mut iter = deltas.iter().cycle();
    while seen.insert(value) {
        value += iter.next().unwrap();
    }

    value
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = load(include_str!("./input.txt"))
        .expect("Loading failed: ");

    let freq = part_one(&input);
    assert_eq!(freq, 445);

    let freq = part_two(&input);
    assert_eq!(freq, 219);
  }
}
