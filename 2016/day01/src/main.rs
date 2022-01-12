fn main() {
    use std::fs;
    use std::time::Instant;

    let input = fs::read_to_string("./input.txt").unwrap();
    let actions = load(&input);

    let t1 = Instant::now();
    let blocks = part_one(&actions);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", blocks, t2 - t1);

    let t1 = Instant::now();
    let blocks = part_two(&actions);
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", blocks, t2 - t1);
}

enum Action {
    Left(i32),
    Right(i32),
}

fn load(input: &str) -> Vec<Action> {
    input.split(", ").map(|v| {
        let blocks = v[1..].parse::<i32>().unwrap();
        match v.chars().nth(0) {
            Some('L') => Action::Left(blocks),
            Some('R') => Action::Right(blocks),
            _ => panic!("Unknown action: {:?}", v)
        }
    })
    .collect()
}

fn part_one(actions: &[Action]) -> i32 {
    // facing => 0:N, 1:E, 2:S, 3:W
    let (x, y, _) = actions.iter()
        .fold((0, 0, 0), |(x, y, f), action| {
            let (facing, blocks) = match action {
                Action::Left(n)  => (if f == 0 { 3 } else { f - 1 }, n),
                Action::Right(n) => (if f == 3 { 0 } else { f + 1 }, n),
            };
            let (dx, dy) = if facing % 2 == 0 {
                (0, if facing == 0 { *blocks } else { -blocks })
            } else {
                (if facing == 1 { *blocks } else { -blocks }, 0)
            };
            (x + dx, y + dy, facing)
        });
    
    x.abs() + y.abs()
}

fn part_two(actions: &[Action]) -> i32 {
    use std::collections::HashSet;

    let mut facing = 0; // 0:N, 1:E, 2:S, 3:W
    let mut p: (i32, i32) = (0, 0); // x, y
    let mut visited = HashSet::from([p]);

    for action in actions {
        let (dir, blocks) = match action {
            Action::Left(n)  => (if facing == 0 { 3 } else { facing - 1 }, n),
            Action::Right(n) => (if facing == 3 { 0 } else { facing + 1 }, n),
        };
        let (dx, dy) = if dir % 2 == 0 {
            (0, if dir == 0 { *blocks } else { -blocks })
        } else {
            (if dir == 1 { *blocks } else { -blocks }, 0)
        };
        facing = dir;

        let (stepx, stepy) = (dx.signum(), dy.signum());
        let target = (p.0 + dx, p.1 + dy);
        while p != target {
            p = (p.0 + stepx, p.1 + stepy);
            if !visited.insert(p) {
                return p.0.abs() + p.1.abs()
            }
        }
    }
    
    -1
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let actions = load(&input);

        let blocks = part_one(&actions);
        assert_eq!(blocks, 231);

        let blocks = part_two(&actions);
        assert_eq!(blocks, 147);
    }
}