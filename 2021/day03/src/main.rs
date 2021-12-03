
fn main() {
    use std::fs;
    use std::time::Instant;

    let lines = load(&fs::read_to_string("./input.txt").unwrap());

    let t1 = Instant::now();
    let power = part_one(&lines);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", power, t2 - t1);

    let t1 = Instant::now();
    let life_support = part_two(&lines);
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", life_support, t2 - t1);
}

fn load(input: &str) -> Vec<u32> {
    input.lines()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect()
}

fn part_one(nums: &[u32]) -> u32 {
    let counts = get_counts(nums);
    let gamma = counts.iter()
        .fold(0, |result, &v| (result << 1) ^ ((v > 0) as u32));

    // Flip the bits and only use the lower 12
    let epsilon = !gamma & 0x00000FFF;

    gamma * epsilon
}

fn part_two(nums: &[u32]) -> u32 {
    let mut i = 11;
    let mut oxygen = nums.to_vec();
    while oxygen.len() > 1 {
        let b_cnt = get_counts(&oxygen);
        let mcb = (b_cnt[11 - i] >= 0) as u32;
        oxygen = oxygen.iter().cloned().filter(|&n| (n >> i) & 1 == mcb).collect();
        i -= 1;
    }

    i = 11;
    let mut co2 = nums.to_vec();
    while co2.len() > 1 {
        let b_cnt = get_counts(&co2);
        let lcb = (b_cnt[11 - i] < 0) as u32;
        co2 = co2.iter().cloned().filter(|&n| (n >> i) & 1 == lcb).collect();
        i -= 1;
    }

    oxygen[0] * co2[0]
}

fn get_counts(nums: &[u32]) -> [i32;12] {
    let mut counts: [i32;12] = [0;12];
    nums.iter().for_each(|n|
        (0..12).for_each(|i|
            counts[11 - i] += if n & (1 << i) == 0 { -1 } else { 1 }
        )
    );

    counts
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let lines = load(include_str!("../input.txt"));

    let power = part_one(&lines);
    assert_eq!(power, 2583164);

    let life_support = part_two(&lines);
    assert_eq!(life_support, 2784375);
  }
}