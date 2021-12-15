fn main() {
    use std::time::Instant;

    let containers = load(include_str!("./input.txt"));

    let t1 = Instant::now();
    let count = part_one(&containers);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", count, t2 - t1);

    let t1 = Instant::now();
    let count = part_two(&containers);
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", count, t2 - t1);
}

fn load(input: &str) -> Vec<i32> {
    input.lines().map(|s| s.parse::<i32>().unwrap()).collect()
}

fn part_one(containers: &[i32]) -> i32 {
    use itertools::Itertools;

    (4..containers.len()).flat_map(|n|
        containers.iter().combinations(n)
            .filter(|v| v.iter().cloned().sum::<i32>() == 150)
    ).count() as i32
}

fn part_two(containers: &[i32]) -> i32 {
    use itertools::Itertools;

    let mut seqs = (4..containers.len()).flat_map(|n|
        containers.iter().combinations(n)
            .filter(|v| v.iter().cloned().sum::<i32>() == 150)
            .map(|v| v.len())
            .collect::<Vec<_>>()
    ).collect::<Vec<_>>();
    seqs.sort();

    seqs.iter().filter(|&n| *n == seqs[0]).count() as i32
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let containers = load(include_str!("./input.txt"));

    let count = part_one(&containers);
    assert_eq!(count, 4372);

    let count = part_two(&containers);
    assert_eq!(count, 4);
  }
}