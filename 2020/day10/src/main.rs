use std::cmp::min;

fn main() {
    let adapters = load(include_str!("./adapters.txt"));

    let jolts = part_one(&adapters);
    println!("Part1: {}", jolts);

    let jolts = part_two(&adapters);
    println!("Part2: {}", jolts);
}

fn load(adapters: &str) -> Vec<u32> {
    let mut v: Vec<_> = adapters.lines().map(|v| v.parse::<u32>().unwrap())
        .collect();
    v.sort();

    let last = *v.last().unwrap();
    v.insert(0, 0);
    v.push(last + 3);
    v
}

fn part_one(adapters: &[u32]) -> u32 {
    let mut prev = adapters[0];
    let (ones, threes) = adapters.iter().fold((0, 0), |mut acc, &v| {
        if v - prev == 1 {
            acc = (acc.0 + 1, acc.1)
        } else if v - prev == 3 {
            acc = (acc.0, acc.1 + 1)
        }
        prev = v;
        acc
    });

    ones * threes
}

fn part_two(adapters: &[u32]) -> i64 {
    let arr = adapters;
    let mut dp = vec![0i64; adapters.len()];

    dp[0] = 1;
    (0..dp.len()).for_each(|i| {
        let l = min(dp.len() - i, 4);
        (1..l).for_each(|j| if arr[i+j] - arr[i] <= 3 { dp[i+j] += dp[i] })
    });

    dp[dp.len()-1]
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let adapters = load(include_str!("./adapters.txt"));
    
    let jolts = part_one(&adapters);
    assert_eq!(jolts, 2574);
    
    let jolts = part_two(&adapters);
    assert_eq!(jolts, 2644613988352);
  }

  #[test]
  fn small_works() {
    let adapters = load(include_str!("./test_s.txt"));
    
    let jolts = part_one(&adapters);
    assert_eq!(jolts, 35);

    let count = part_two(&adapters);
    assert_eq!(count, 8);
  }

  #[test]
  fn medium_works() {
    let adapters = load(include_str!("./test_m.txt"));
    
    let jolts = part_one(&adapters);
    assert_eq!(jolts, 220);

    let count = part_two(&adapters);
    assert_eq!(count, 19208);
  }
}