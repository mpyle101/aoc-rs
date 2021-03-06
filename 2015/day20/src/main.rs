fn main() {
    use std::time::Instant;

    let t1 = Instant::now();
    let house = part_one(34000000);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", house, t2 - t1);

    let t1 = Instant::now();
    let house = part_two(34000000);
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", house, t2 - t1);
}

fn part_one(presents: i32) -> i32 {
    let mut house = 0;

    loop {
        house += 1;
        let end = (f64::from(house).sqrt() as i32) + 1;
        let count = (2..end).fold(house * 10 + 10, |acc, n|
            if house % n == 0 {
                acc + (n * 10) + (house / n * 10)
            } else {
                acc
            }
        );
        if count >= presents {
            break house
        }
    }
}

fn part_two(presents: i32) -> i32 {
    use std::collections::HashMap;

    let mut active = HashMap::new();
    let mut house = 0;
    loop {
        house += 1;
        active.insert(house, 1);
        let end = (f64::from(house).sqrt() as i32) + 1;
        let count = (2..end).fold(house * 11 + 11, |mut acc, n| {
            if house % n == 0 {
                if let Some(visits) = active.get_mut(&n) {
                    acc += n * 11;
                    *visits += 1;
                    if *visits == 50 { active.remove(&n); }
                }
                let v = house / n;
                if let Some(visits) = active.get_mut(&v) {
                    acc += v * 11;
                    *visits += 1;
                    if *visits == 50 { active.remove(&v); }
                }
            };

            acc
        });
        if count >= presents {
            break house
        }
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let house = part_one(34000000);
    assert_eq!(house, 786240);

    let house = part_two(34000000);
    assert_eq!(house, 831600);
  }
}