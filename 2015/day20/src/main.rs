fn main() {
    use std::time::Instant;
/*
    let t1 = Instant::now();
    let house = part_one(34000000);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", house, t2 - t1);
*/
    let t1 = Instant::now();
    let house = part_two(34000000);
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", house, t2 - t1);
}

#[allow(dead_code)]
fn part_one(presents: i32) -> i32 {
    let mut elves = vec![];

    let mut house = 0;
    let mut count = 0;

    while count < presents {
        house += 1;
        elves.push((house * 2, house));
        count = house * 10;
        for i in 0..house/2 {
            let mut elf = elves.get_mut(i as usize).unwrap();
            if elf.0 == house {
                elf.0 += elf.1;
                count += elf.1 * 10;
            }
        }
    }

    house
}

#[allow(dead_code)]
fn part_two(presents: i32) -> i32 {
    use std::collections::HashMap;

    let mut elves = HashMap::new();

    let mut house = 0;
    let mut count = 0;

    while count < presents {
        house += 1;
        elves.insert(house, (house, 0));
        count = factors(house).iter().fold(0, |acc, n| {
            if let Some(elf) = elves.get_mut(n) {
                let v = n * 11;
                elf.0 += n;
                elf.1 += 1;
                if elf.1 == 50 { elves.remove(n); }
                acc + v
            } else {
                acc
            }
        });

        if house % 10000 == 0 { println!("{} {}", house, elves.len()) }
    }

    house
}

fn factors(v: i32) -> Vec<i32> {
    (1..=v/2).filter_map(|n| if v % n == 0 { Some(n) } else { None }).collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let house = part_one(34000000);
    assert_eq!(house, 786240);
  }
}