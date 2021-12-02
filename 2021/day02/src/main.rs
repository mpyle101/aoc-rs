
fn main() {
    use std::fs;
    use std::time::Instant;

    let input = load(&fs::read_to_string("./input.txt").unwrap());

    let t1 = Instant::now();
    let pos = part_one(&input);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", pos, t2 - t1);

    let t1 = Instant::now();
    let pos = part_two(&input);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", pos, t2 - t1);
}

enum Action {
    Up(i32),
    Down(i32),
    Forward(i32),
}

fn load(input: &str) -> Vec<Action> {
    input.lines().map(|l| {
        let v = l.split(' ').collect::<Vec<_>>();
        let steps = v[1].parse::<i32>().unwrap();
        match v[0] {
            "up"   => Action::Up(steps),
            "down" => Action::Down(steps),
            _      => Action::Forward(steps)
        }
    }).collect()
}

fn part_one(actions: &[Action]) -> i32 {
    let (pos, depth) = actions.iter().fold((0, 0), |(pos, depth), action|
        match action {
            Action::Up(steps)      => (pos, depth - steps),
            Action::Down(steps)    => (pos, depth + steps),
            Action::Forward(steps) => (pos + steps, depth),
        }
    );
    
    pos * depth
}

fn part_two(actions: &[Action]) -> i32 {
    let (pos, depth, _) = actions.iter().fold((0, 0, 0),
     |(pos, depth, aim), action|
        match action {
            Action::Up(steps)      => (pos, depth, aim - steps),
            Action::Down(steps)    => (pos, depth, aim + steps),
            Action::Forward(steps) => (pos + steps, depth + (aim * steps), aim),
        }
    );
    
    pos * depth
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = load(include_str!("../input.txt"));

    let pos = part_one(&input);
    assert_eq!(pos, 1924923);

    let pos = part_two(&input);
    assert_eq!(pos, 1982495697);
  }
}