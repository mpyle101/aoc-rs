
fn main() {
    use std::time::Instant;

    let input = load(include_str!("./input.txt"));

    let t1 = Instant::now();
    let count = part_one(&input);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", count, t2 - t1);

    let t1 = Instant::now();
    let count = part_two(&input);
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", count, t2 - t1);
}

fn load(input: &str) -> Vec<i32> {
    input.lines().map(|v| v.parse::<i32>().unwrap()).collect()
}

fn part_one(depths: &[i32]) -> i32 {
    let mut iter = depths.iter();
    let first = iter.next().unwrap();
    let (_, count) = iter.fold((first, 0), |(last, count), v|
        (v, count + ((v > last) as i32))
    );
    
    count
}

fn part_two(depths: &[i32]) -> i32 {
    let mut iter = depths.windows(3);
    let w = iter.next().unwrap();
    let first = w.iter().sum::<i32>();
    let (_, count) = iter.fold((first, 0), |(last, count), v| {
        let val = v.iter().sum();
        (val, count + ((val > last) as i32))
    });
    
    count
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = load(include_str!("./input.txt"));

    let count = part_one(&input);
    assert_eq!(count, 1676);

    let count = part_two(&input);
    assert_eq!(count, 1706);
  }
}