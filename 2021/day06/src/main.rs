
fn main() {
    use std::fs;
    use std::time::Instant;

    let fish = load(&fs::read_to_string("./input.txt").unwrap());

    let t1 = Instant::now();
    let population = doit(&fish, 80);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", population, t2 - t1);

    let t1 = Instant::now();
    let population = doit(&fish, 256);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", population, t2 - t1);
}

fn load(input: &str) -> Vec<i32> {
    input.split(',').map(|s| s.parse::<i32>().unwrap()).collect()
}

fn doit(fish: &[i32], days: i32) -> i64 {
    use std::collections::HashMap;
    use num::range_step_inclusive as range;

    let mut total: HashMap<_, _> = (0..9).map(|n| (days - n, 1)).collect();
    (2..=days-9).rev().for_each(|n| {
        let children = range(n + 9, days, 7).fold(1,
            |acc, d| acc + total.get(&d).unwrap()
        );
        total.insert(n, children);
    });

    let fish_growth = (1..=5).fold(HashMap::new(), |mut map, n| {
        let children = range(n + 1, days, 7).fold(1,
            |acc, d| acc + total.get(&d).unwrap()
        );
        map.insert(n, children);
        map
    });

    fish.iter().map(|n| fish_growth.get(n).unwrap()).sum()
}


#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn it_works() {
    let fish = load(&fs::read_to_string("./input.txt").unwrap());

    let population = doit(&fish, 80);
    assert_eq!(population, 350917);

    let population = doit(&fish, 80);
    assert_eq!(population, 1592918715629);
  }
}