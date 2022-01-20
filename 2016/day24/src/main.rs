use std::collections::{HashMap, HashSet};

type Point  = (i32, i32);
type Points = HashSet<Point>;
type Wire   = (char, Point);
type Wires  = Vec<Wire>;
type State0 = (Wires, Points);

fn main() {
    use std::{fs, time::Instant};

    let input = fs::read_to_string("./input.txt").unwrap();
    let state = load(&input);

    let t1 = Instant::now();
    let steps = part_one(&state);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", steps, t2 - t1);

    let t1 = Instant::now();
    let steps = part_two(&state);
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", steps, t2 - t1);
}

fn load(input: &str) -> State0 {
    let mut wires = vec![];
    let open = input.lines().enumerate()
        .map(|(y, l)| {
            l.chars().enumerate().filter_map(|(x, c)| {
                if c == '#' {
                    None
                } else {
                    let pt = (x as i32, y as i32);
                    if c != '.' { wires.push((c, pt)) }
                    Some(pt)
                }
            })
            .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<HashSet<_>>();

    wires.sort();

    (wires, open)
}

fn part_one((wires, open): &State0) -> usize {
    use itertools::Itertools;

    // Get the lengths of the paths between all pairs of wires.
    let paths = all_paths(wires, open);

    // Brute force our way through finding the sequence
    // with the smallest number of steps.
    let doors = wires.iter().skip(1).map(|w| w.0).collect::<Vec<_>>();
    doors.iter().permutations(doors.len())
        .map(|v| {
            let steps = paths.get(&('0', *v[0])).unwrap();
            (1..doors.len()).fold(*steps, |acc, i|
                if let Some(n) = paths.get(&(*v[i-1], *v[i])) {
                    acc + n
                } else {
                    acc + paths.get(&(*v[i], *v[i-1])).unwrap()
                }
            )
        })
        .min()
        .unwrap()
}

fn part_two((wires, open): &State0) -> usize {
    use itertools::Itertools;

    // Get the lengths of the paths between all pairs of wires.
    let paths = all_paths(wires, open);

    // Brute force our way through finding the sequence
    // with the smallest number of steps.
    let doors = wires.iter().skip(1).map(|w| w.0).collect::<Vec<_>>();
    doors.iter().permutations(doors.len())
        .map(|v| {
            let steps = 
                paths.get(&('0', **v.first().unwrap())).unwrap() +
                paths.get(&('0', **v.last().unwrap())).unwrap();
            (1..doors.len()).fold(steps, |acc, i|
                if let Some(n) = paths.get(&(*v[i-1], *v[i])) {
                    acc + n
                } else {
                    acc + paths.get(&(*v[i], *v[i-1])).unwrap()
                }
            )
        })
        .min()
        .unwrap()
}

fn all_paths(wires: &Wires, open: &Points) -> HashMap<(char, char), usize> {
    use itertools::Itertools;
    use pathfinding::prelude::bfs;

    wires.iter()
        .combinations(2)
        .map(|v| {
            let path = bfs(&v[0].1, |p| neighbors(p, open), |p| *p == v[1].1);
            if let Some(p) = path {
                ((v[0].0, v[1].0), p.len() - 1)
            } else {
                ((v[0].0, v[1].0), usize::MAX)
            }
        })
        .collect()
}

fn neighbors(p: &Point, open: &Points) -> Vec<Point> {
    let delta = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    let pts = delta.iter()
        .filter_map(|d| {
            let pt = (p.0 + d.0, p.1 + d.1);
            open.get(&pt).map(|v| *v)
        })
        .collect();

    pts
}

#[allow(dead_code)]
fn print(wires: &Wires, open: &Points) {
    (0..39).for_each(|y| {
        (0..186).for_each(|x| {
            let pt = (x as i32, y as i32);
            let c = if let Some(i) = wires.iter().position(|v| v.1 == pt) {
                wires[i].0
            } else if open.get(&pt).is_some() {
                '.'
            } else {
                '#'
            };
            print!("{}", c);
        });
        println!();
    })
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let state = load(&input);
    
        let steps = part_one(&state);
        assert_eq!(steps, 448);
    
        let steps = part_two(&state);
        assert_eq!(steps, 672);
    }
}