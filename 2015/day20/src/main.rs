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

#[allow(dead_code)]
fn part_one(presents: i32) -> i32 {
    let mut house = 0;

    loop {
        house += 1;
        if elves(house).iter().map(|n| n * 10).sum::<i32>() >= presents {
            break house
        }
    }
}

#[allow(dead_code)]
fn part_two(presents: i32) -> i32 {
    use std::collections::HashMap;

    let mut active = HashMap::new();

    let mut house = 0;
    loop {
        house += 1;
        active.insert(house, (house, 0));
        let count = elves(house).iter().fold(0, |acc, n| {
            if let Some(elf) = active.get_mut(n) {
                elf.0 += n;
                elf.1 += 1;
                if elf.1 == 50 { active.remove(n); }
                acc + n * 11
            } else {
                acc
            }
        });
        if count >= presents {
            break house
        }
    }
}

fn elves(val: i32) -> Vec<i32> {
    let end = ((val as f32).sqrt() + 1.0) as i32;
    (1..=end).fold(vec![], |mut v, n| {
        if val % n == 0 {
            v.push(n);
            v.push(val / n);
        }

        v
    })
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