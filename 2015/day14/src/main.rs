fn main() {
    use std::time::Instant;

    let reindeer = load(include_str!("./input.txt"));

    let t1 = Instant::now();
    let dist = part_one(&reindeer, 2503);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", dist, t2 - t1);
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

fn part_one(v: &[Reindeer], secs: i32) -> i32 {
    use std::cmp::min;

    v.iter().map(|r| {
        let intervals = secs / (r.time + r.rest);
        let traveled  = intervals * r.rate * r.time;
        let time_left = secs - (r.time + r.rest) * intervals;
        traveled + min(r.time, time_left) * r.rate
    }).max().unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let reindeer = load(include_str!("./input.txt"));

    let dist = part_one(&reindeer, 2503);
    assert_eq!(dist, 2655);
  }
}