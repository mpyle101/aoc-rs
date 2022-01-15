use std::collections::HashMap;

fn main() {
    use std::time::Instant;

    let t1 = Instant::now();
    let steps = part_one((31, 39), 1350);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", steps, t2 - t1);

    let t1 = Instant::now();
    let nodes = part_two(1350);
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", nodes, t2 - t1);
}

fn part_one(goal: (i32, i32), input: i32) -> usize {
    use pathfinding::prelude::bfs;

    let steps = bfs(&(1, 1), |p| neighbors(p, input), |&p| p == goal);

    // The vector contains the initial state.
    steps.unwrap().len() - 1
}

fn part_two(input: i32) -> usize {
    use pathfinding::prelude::bfs_reach;

    let mut steps = HashMap::from([((1, 1), 0)]);
    let nodes = bfs_reach((1, 1), |p| reachable(p, &mut steps, input))
        .collect::<Vec<_>>();

    // The vector contains the initial state.
    nodes.len()
}

fn neighbors((x, y): &(i32, i32), input: i32) -> Vec<(i32, i32)> {
    [(0, -1), (0, 1), (-1, 0), (1, 0)].iter()
        .filter_map(|(dx, dy)| {
            let (px, py) = (x + dx, y + dy);
            if px >= 0 && py >= 0 && is_open(px, py, input) {
                Some((px, py))
            } else {
                None
            }
        })
        .collect()
}

fn reachable(
    (x, y): &(i32, i32),
    steps: &mut HashMap<(i32, i32), i32>,
    input: i32,
) -> Vec<(i32, i32)> {
    let n = *steps.get(&(*x, *y)).unwrap();

    [(0, -1), (0, 1), (-1, 0), (1, 0)].iter()
        .filter_map(|(dx, dy)| {
            let (px, py) = (x + dx, y + dy);
            if px >= 0 && py >= 0 && n < 50 && is_open(px, py, input) {
                steps.insert((px, py), n + 1);
                Some((px, py))
            } else {
                None
            }
        })
        .collect()
}

fn is_open(x: i32, y: i32, input: i32) -> bool {
    let mut ones = 0;
    let mut n = (x * x) + (3 * x) + (2 * x * y) + y + (y * y) + input;
    while n > 0 {
        if n % 2 != 0 { ones += 1 }
        n /= 2
    }

    ones % 2 == 0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let steps = part_one((31, 39), 1350);
        assert_eq!(steps, 92);

        let steps = part_two(1350);
        assert_eq!(steps, 124);
    }

    #[test]
    fn samples() {
        let steps = part_one((7, 4), 10);
        assert_eq!(steps, 11);
    }
}