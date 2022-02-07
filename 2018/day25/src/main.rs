fn main() {
    use std::time::Instant;

    let pts = load(include_str!("./input.txt"));

    let t1 = Instant::now();
    let constellations = part_one(&pts);
    let t2 = Instant::now();
    println!("Part 1: {}  ({:?})", constellations, t2 - t1);
}

type Point = Vec<i32>;

fn load(input: &str) -> Vec<Point> {
    input.lines()
        .map(|l| 
            l.split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        )
        .collect()
}

fn part_one(pts: &[Point]) -> i32 {
    use std::collections::VecDeque;

    let mut pts = pts.to_vec();

    let mut count = 0;
    while pts.len() > 0 {
        let mut q = VecDeque::from([pts.remove(0)]);
        while let Some(a) = q.pop_front() {
            let v = pts.iter()
                .enumerate()
                .filter_map(|(i, b)| if md(&a, b) <= 3 { Some(i) } else { None })
                .collect::<Vec<_>>();
            v.iter().rev().for_each(|i| q.push_back(pts.remove(*i)));
        }

        count += 1;
    }

    count
}

fn md(a: &[i32], b: &[i32]) -> i32 {
    a.iter().zip(b.iter()).map(|(v1, v2)| (v1 - v2).abs()).sum()
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let pts = load(include_str!("./input.txt"));

    let constellations = part_one(&pts);
    assert_eq!(constellations, 377);
  }
}
