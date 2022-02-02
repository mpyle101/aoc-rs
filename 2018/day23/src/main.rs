fn main() {
    use std::time::Instant;

    let bots = load(include_str!("./input.txt"));

    let t1 = Instant::now();
    let in_range = part_one(&bots);
    let t2 = Instant::now();
    println!("Part 1: {}  ({:?})", in_range, t2 - t1);
}

#[derive(Clone, Copy, Debug)]
struct Bot {
    x: i64,
    y: i64,
    z: i64,
    r: i64,
}

impl Bot {
    fn md(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() +
        (self.y - other.y).abs() +
        (self.z - other.z).abs()
    }
}

fn load(input: &str) -> Vec<Bot> {
    input.lines().map(|l| {
        let mut it = l.split(">, r=");
        let s = &it.next().unwrap()[5..];
        let r = read(&mut it);
        
        let mut it = s.split(",");
        let x = read(&mut it);
        let y = read(&mut it);
        let z = read(&mut it);

        Bot { x, y, z, r }
    })
    .collect()
}

fn read(it: &mut std::str::Split<&str>) -> i64 {
    it.next().map_or(0, |v| v.parse::<i64>().map_or(0, |n| n))
}

fn part_one(bots: &[Bot]) -> usize {
    let mut bots = bots.to_vec();
    bots.sort_by(|a, b| b.r.cmp(&a.r));
    let bot = bots[0];

    bots.iter().filter(|b| bot.md(b) <= bot.r).count()
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let bots = load(include_str!("./input.txt"));

    let in_range = part_one(&bots);
    assert_eq!(in_range, 481);
  }
}
