
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
    // A vector of number of fish at a given age where the index
    // is the age and value is the count.
    input.split(',').fold(vec![0i32;6], |mut v, s| {
        v[s.parse::<usize>().unwrap()] += 1;
        v
    })
}

fn doit(fish: &[i32], days: usize) -> i64 {
    use num::range_step_inclusive as range;

    // Build a vector of total population value given a fish being
    // "born" on a given day. We do this by walking backwards and
    // building up previous totals from existing starting with any
    // fish born in the last nine days. For those we know the total
    // population is 1 since they don't have time to reproduce.
    let population = (2..=days-9).rev()
        .fold(vec![1i64; days+1], |mut v, n| {
            v[n] = range(n + 9, days, 7).fold(1,
                |acc, d| acc + v[d]
            );
            v
        });

    // Build a vector of total population for a fish having a given
    // age in the initial population. Build using the values from the
    // "born on" vector for each day an initial fish will reproduce.
    let fish_growth = (1..=5)
        .fold(vec![0i64;6], |mut v, n| {
            v[n] = range(n + 1, days, 7).fold(1,
                |acc, d| acc + population[d]
            );
            v
        });

    // The final population is the sum of all the fish created from
    // each initial fish. We only need to know how many of each age
    // were in the initial population. The index is the age of the
    // fish and the value is the number of fish that age.
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