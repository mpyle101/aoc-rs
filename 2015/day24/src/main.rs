
fn main() {
    use std::time::Instant;

    let weights = load(include_str!("./input.txt"));

    let t1 = Instant::now();
    let qe = part_one(&weights);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", qe, t2 - t1);

    let t1 = Instant::now();
    let qe = part_two(&weights);
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", qe, t2 - t1);
}

fn load(input: &str) -> Vec<i64> {
    input.lines().map(|l| l.parse::<i64>().unwrap()).collect()
}

fn part_one(weights: &[i64]) -> i64 {
    use itertools::Itertools;

    let target = weights.iter().sum::<i64>() / 3;

    // Manually worked our way to 6 because 2-5 returned no results.
    // Could put an outer loop to work our way up to handle any data
    // set.
    weights.iter().combinations(6).filter_map(|v| 
        if v.iter().cloned().sum::<i64>() == target { Some(v) } else { None }
    )
    .map(|v| v.iter().cloned().product::<i64>())
    .min()
    .unwrap()
}


fn part_two(weights: &[i64]) -> i64 {
    use itertools::Itertools;

    let target = weights.iter().sum::<i64>() / 4;

    // Manually worked our way to 6 because 2 & 3 returned no results.
    // Could put an outer loop to work our way up to handle any data
    // set.
    weights.iter().combinations(4).filter_map(|v| 
        if v.iter().cloned().sum::<i64>() == target { Some(v) } else { None }
    )
    .map(|v| v.iter().cloned().product::<i64>())
    .min()
    .unwrap()
}



#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let weights = load(include_str!("./input.txt"));

    let qe = part_one(&weights);
    assert_eq!(qe, 11846773891);

    let qe = part_two(&weights);
    assert_eq!(qe, 80393059);
  }
}