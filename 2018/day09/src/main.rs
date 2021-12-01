fn main() {
    use std::time::Instant;

    let (players, last_marble) = load(include_str!("./input.txt"));

    let t1 = Instant::now();
    let score = part_one(players, last_marble);
    let t2 = Instant::now();
    println!("Part 1: {}  ({:?})", score, t2 - t1);

    let t1 = Instant::now();
    let score = part_two(players, last_marble);
    let t2 = Instant::now();
    println!("Part 2: {}  ({:?})", score, t2 - t1);
}

fn part_one(players: u32, last_marble: u32) -> u32 {
    use std::collections::{HashMap, VecDeque};

    let mut scores = HashMap::new();
    let mut q = VecDeque::new();

    // Double ended queue with the current marble always at the end.
    q.push_back(0);
    for m in 1..last_marble + 1 {
        if m % 23 == 0 {
            q.rotate_right(7);
            *scores.entry(m % players).or_insert(0) += m + q.pop_back().unwrap();
            q.rotate_left(1);
        } else {
            q.rotate_left(1);
            q.push_back(m);
        }
    }

    *scores.values().max().unwrap()
}

fn part_two(players: u32, last_marble: u32) -> u32 {
    part_one(players, last_marble * 100)
}

fn load(input: &str) -> (u32, u32) {
    let v: Vec<_> = input.split(' ').collect();

    (v[0].parse::<u32>().unwrap(), v[6].parse::<u32>().unwrap())
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let (players, last_marble) = load(include_str!("./input.txt"));

    let score = part_one(players, last_marble);
    assert_eq!(score, 375465);

    let score = part_one(players, last_marble);
    assert_eq!(score, 3037741441);
  }
}
