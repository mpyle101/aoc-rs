
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
    // We only need the number of fish at a given age.
    input.split(',').fold(vec![0i32;6], |mut v, s| {
        v[s.parse::<usize>().unwrap()] += 1;
        v
    })
}

fn doit(fish: &[i32], days: usize) -> i64 {
    use num::range_step_inclusive as range;

    let population = (2..=days-9).rev()
        .fold(vec![1i64; days+1], |mut v, n| {
            v[n] = range(n + 9, days, 7).fold(1,
                |acc, d| acc + v[d]
            );
            v
        });

    let fish_growth = (1..=5)
        .fold(vec![0i64;6], |mut v, n| {
            v[n] = range(n + 1, days, 7).fold(1,
                |acc, d| acc + population[d]
            );
            v
        });

    fish.iter().enumerate()
        .map(|(i, &n)| n as i64 * fish_growth[i])
        .sum()
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

    let population = doit(&fish, 256);
    assert_eq!(population, 1592918715629);
  }
}