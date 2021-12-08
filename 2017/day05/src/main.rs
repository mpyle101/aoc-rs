fn main() {
    use std::fs;
    use std::time::Instant;

    let input = load(&fs::read_to_string("./input.txt").unwrap());

    let t1 = Instant::now();
    let steps = part_one(&input);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", steps, t2 - t1);

    let t1 = Instant::now();
    let steps = part_two(&input);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", steps, t2 - t1);
}

fn load(input: &str) -> Vec<i32> {
    input.lines().map(|s| s.parse::<i32>().unwrap()).collect()
}

fn part_one(input: &[i32]) -> i32 {
    let mut jumps = input.to_vec();

    let mut curr  = 0i32;
    let mut steps = 0;
    while curr >= 0 && curr < jumps.len() as i32 {
        let tmp = jumps[curr as usize];
        jumps[curr as usize] += 1;
        curr += tmp;
        steps += 1;
    }

    steps
}

fn part_two(input: &[i32]) -> i32 {
    let mut jumps = input.to_vec();

    let mut curr  = 0i32;
    let mut steps = 0;
    while curr >= 0 && curr < jumps.len() as i32 {
        let tmp = jumps[curr as usize];
        jumps[curr as usize] += if tmp > 2 { -1 } else { 1 };
        curr += tmp;
        steps += 1;
    }

    steps
}


#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn it_works() {
    let input = load(&fs::read_to_string("./input.txt").unwrap());

    let steps = part_one(&input);
    assert_eq!(steps, 315613);

    let steps = part_two(&input);
    assert_eq!(steps, 22570529);
  }
}