use anyhow::Result;

fn main() {
  let total_fuel: i32 = include_str!("./modules.txt")
    .lines()
    .map(|l| l.parse::<i32>())
    .map(Result::unwrap)
    .map(fuel_needed)
    .sum();

  println!("Total fuel required: {}", total_fuel);
}

fn fuel_needed(mass: i32) -> i32 {
  let fuel = mass / 3 - 2;
  if fuel <= 0 {
    0
  } else {
    fuel + fuel_needed(fuel)
  }
}


/** Unit Tests */
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(fuel_needed(12), 2);
    assert_eq!(fuel_needed(14), 2);
    assert_eq!(fuel_needed(1969), 966);
    assert_eq!(fuel_needed(100756), 50346);
  }
}
