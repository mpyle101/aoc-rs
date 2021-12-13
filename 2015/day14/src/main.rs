fn main() {
    use std::time::Instant;

    let reindeer = load(include_str!("./input.txt"));

    let t1 = Instant::now();
    let dist = part_one(&reindeer);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", dist, t2 - t1);

    let t1 = Instant::now();
    let winner = part_two(&reindeer);
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", winner, t2 - t1);
}

struct Reindeer {
    rate: i32,
    time: i32,
    rest: i32,
}

fn load(input: &str) -> Vec<Reindeer> {
    input.lines().map(|l| l.split(' ').collect::<Vec<&str>>())
        .map(|v| Reindeer {
            rate: v[3].parse::<i32>().unwrap(),
            time: v[6].parse::<i32>().unwrap(),
            rest: v[13].parse::<i32>().unwrap(),
        })
        .collect()
}

fn part_one(v: &[Reindeer]) -> i32 {
    v.iter().map(|r| traveled(r, 2503)).max().unwrap()
}

fn part_two(v: &[Reindeer]) -> i32 {
    let scores = (1..=2503).fold(vec![0i32;v.len()], |mut scores, t| {
        let dist = v.iter().map(|r| traveled(r, t)).collect::<Vec<_>>();
        let maxd = dist.iter().max().unwrap();
        dist.iter().enumerate().for_each(|(i, n)| scores[i] += (n == maxd) as i32);
        scores
    });

    *scores.iter().max().unwrap()
}

fn traveled(r: &Reindeer, secs: i32) -> i32 {
    let intervals = secs / (r.time + r.rest);
    let traveled  = intervals * r.rate * r.time;
    let time_left = secs - (r.time + r.rest) * intervals;

    traveled + std::cmp::min(r.time, time_left) * r.rate
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let reindeer = load(include_str!("./input.txt"));

    let dist = part_one(&reindeer);
    assert_eq!(dist, 2655);

    let winner = part_two(&reindeer);
    assert_eq!(winner, 1059);
  }
}