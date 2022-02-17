fn main() {
    let directions = include_str!("./input.txt");

    let houses = part_one(directions);
    println!("Part 1: {houses}");

    let houses = part_two(directions);
    println!("Part 2: {houses}");
}

fn part_one(directions: &str) -> usize {
    use std::collections::HashSet;

    let houses: HashSet<_> = directions.chars()
        .scan((0, 0), |h, c| {
            *h = step(c, h);
            Some(*h)
        })
        .collect();

    houses.len() + 1
}

fn part_two(directions: &str) -> usize {
    use std::collections::HashSet;

    let mut pos = [(0, 0), (0, 0)];
    let mut houses: HashSet<_> = directions.chars()
        .enumerate()
        .map(|(i, c)| {
            let house = &mut pos[i%2];
            *house = step(c, house);
            *house
        })
        .collect();
    houses.insert((0, 0));

    houses.len()
}

fn step(c: char, house: &(i32, i32)) -> (i32, i32) {
    match c {
        '>' => (house.0 + 1, house.1),
        '<' => (house.0 - 1, house.1),
        '^' => (house.0, house.1 - 1),
        'v' => (house.0, house.1 + 1),
        _ => panic!("Invalid direction")
    }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let directions = include_str!("./input.txt");

    let houses = part_one(directions);
    assert_eq!(houses, 2081);

    let houses = part_two(directions);
    assert_eq!(houses, 2341);
  }

  #[test]
  fn example_1() {
    let directions = "^v";

    let houses = part_two(directions);
    assert_eq!(houses, 3);
  }

  #[test]
  fn example_2() {
    let directions = "^>v<";

    let houses = part_two(directions);
    assert_eq!(houses, 3);
  }

  #[test]
  fn example_3() {
    let directions = "^v^v^v^v^v";

    let houses = part_two(directions);
    assert_eq!(houses, 11);
  }
}