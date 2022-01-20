
fn main() {
    use std::{fs, time::Instant};

    let input = fs::read_to_string("./input.txt").unwrap();
    let disks = load(&input);

    let t1 = Instant::now();
    let pairs = part_one(&disks);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", pairs, t2 - t1);

    let t1 = Instant::now();
    let steps = part_two(&disks);
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", steps, t2 - t1);
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Disk {
    x: i32,
    y: i32,
    size: i32,
    used: i32,
}

impl Disk {
    fn can_hold(&self, d: &Disk) -> bool {
        (self.size - self.used) >= d.used
    }
}

type Disks = Vec<Disk>;
type State = ((i32, i32), Disks);

fn load(input: &str) -> Disks {
    input.lines().skip(2).map(|s| {
        let v = s.split_ascii_whitespace().collect::<Vec<_>>();
        let size = v[1][0..v[1].len()-1].parse::<i32>().unwrap();
        let used = v[2][0..v[2].len()-1].parse::<i32>().unwrap();
        let v = v[0].split('-').collect::<Vec<_>>();
        let x = v[1][1..].parse::<i32>().unwrap();
        let y = v[2][1..].parse::<i32>().unwrap();
        Disk { x, y, size, used }
    })
    .collect()
}

fn part_one(disks: &Disks) -> usize {
    use itertools::Itertools;

    disks.iter().permutations(2)
        .filter(|v| v[0].used != 0 && v[1].can_hold(v[0]))
        .count()
}

fn part_two(disks: &Disks) -> usize {
    use pathfinding::prelude::bfs;

    let goal = (0, 0);
    let result = bfs(&((34, 0), disks.clone()), |st| do_xfers(st), |st| st.0 == goal);

    result.unwrap().len() - 1
}

fn do_xfers((node, disks): &State) -> Vec<((i32, i32), Disks)> {
    use itertools::Itertools;

    let xfers = disks.iter()
        .permutations(2)
        .filter(|v| can_xfer(&node, v[0], v[1]))
        .map(|v| {
            let mut d1 = *v[0];
            let mut d2 = *v[1];
            d2.used += d1.used;
            d1.used = 0;
            let n = if *node == (d1.x, d1.y) {
                (d2.x, d2.y)
            } else {
                *node
            };
                
            (n, d1, d2)
        })
        .collect::<Vec<_>>();

    xfers.iter().map(|(n, d1, d2)| {
        let dsks = disks.iter().map(|d|
            if d.x == d1.x && d.y == d1.y {
                *d1
            } else if d.x == d2.x && d.y == d2.y {
                *d2
            } else {
                *d
            }
        )
        .collect::<Vec<_>>();

        (*n, dsks)
    })
    .collect()
}

fn can_xfer(p: &(i32, i32), d1: &Disk, d2: &Disk) -> bool {
    let md = (d1.x - d2.x).abs() + (d1.y - d2.y).abs();
    if md == 1 {
        if *p == (d1.x, d1.y) {
            // Can only move target data onto an empty disk
            d2.used == 0 && d2.can_hold(d1)
        } else if *p == (d2.x, d2.y) {
            // Can't mix target data with other data
            false
        } else {
            d1.used != 0 && d2.can_hold(d1)
        }
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let disks = load(&input);
        
        let pairs = part_one(&disks);
        assert_eq!(pairs, 1003);
    }
}