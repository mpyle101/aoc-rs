use std::collections::{HashMap, HashSet};

fn main() {
    use std::time::Instant;

    let clay = load(include_str!("./input.txt"));

    let t1 = Instant::now();
    let water = part_one(&clay);
    let t2 = Instant::now();
    println!("Part 1: {}  ({:?})", water, t2 - t1);

    let clay = load(include_str!("./input.txt"));
    let t1 = Instant::now();
    let water = part_two(&clay);
    let t2 = Instant::now();
    println!("Part 2: {}  ({:?})", water, t2 - t1);
}

type Edges = HashSet<(char, (i32, i32))>;
type Clay  = HashSet<(i32, i32)>;
type Water = HashMap<(i32, i32), char>;
type Stack = Vec<(i32, i32)>;

fn load(input: &str) -> Clay {
    let clay = input.lines().map(|l| {
        let v = l.split(", ")
            .map(|s| s.split("=").collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let r  = v[1][1].split("..").collect::<Vec<_>>();
        let r1 = r[0].parse::<i32>().unwrap();
        let r2 = r[1].parse::<i32>().unwrap();

        let axis = v[0][0].chars().nth(0).unwrap();
        let av   = v[0][1].parse::<i32>().unwrap();

        if axis == 'x' {
            (r1..=r2).map(|y| (av, y)).collect::<Vec<_>>()
        } else {
            (r1..=r2).map(|x| (x, av)).collect::<Vec<_>>()
        }
    })
    .flatten()
    .collect();

    clay
}

#[allow(dead_code)]
fn part_one(clay: &Clay) -> i32 {
    let min_y = clay.iter().min_by_key(|a| a.1).unwrap().1;
    let max_y = clay.iter().max_by_key(|a| a.1).unwrap().1;

    // Spring is (500, 0)
    let head = (500, 1);
    let mut water = HashMap::from([(head, '|')]);
    let mut stack = vec![head];
    let mut edges = HashSet::new();

    while let Some((x, y)) = stack.pop() {
        let mut p = (x, y + 1);
        while !clay.contains(&p) && !water.contains_key(&p) && p.1 <= max_y {
            water.insert(p, '|');
            p = (p.0, p.1 + 1);
        }

        if p.1 > max_y {
            continue
        } else {
            if water.contains_key(&p) {
                p = (p.0, p.1 + 1);
            }
            fill(&p, clay, &mut water, &mut edges, &mut stack);
        }
    }

    water.len() as i32 - min_y + 1
}

fn part_two(clay: &Clay) -> usize {
    let max_y = clay.iter().max_by_key(|a| a.1).unwrap().1;

    // Spring is (500, 0)
    let head = (500, 1);
    let mut water = HashMap::from([(head, '|')]);
    let mut stack = vec![head];
    let mut edges = HashSet::new();

    while let Some((x, y)) = stack.pop() {
        let mut p = (x, y + 1);
        while !clay.contains(&p) && !water.contains_key(&p) && p.1 <= max_y {
            water.insert(p, '|');
            p = (p.0, p.1 + 1);
        }

        if p.1 > max_y {
            continue
        } else {
            if water.contains_key(&p) {
                p = (p.0, p.1 + 1);
            }
            let n = fill(&p, clay, &mut water, &mut edges, &mut stack);
            let mut m = p.0;
            while let Some(c) = water.get_mut(&(m, n)) {
                if *c == '|' { break } else { *c = '|' }
                m += 1;
            }
            let mut m = p.0 - 1;
            while let Some(c) = water.get_mut(&(m, n)) {
                if *c == '|' { break } else { *c = '|' }
                m -= 1;
            }
        }
    }

    // print(clay, &water);

    water.values().filter(|&c| *c == '~').count()
}

fn fill(
    (x, y): &(i32, i32),
    clay: &Clay,
    water: &mut Water,
    edges: &mut Edges,
    stack: &mut Stack
) -> i32 {
    // Back up while we have walls on both sides.
    let mut offset  = -1;

    loop {
        // Fill left
        let p = (*x - 1, *y + offset);
        let left = fill_dir(&p, -1, 'L', clay, water, edges, stack);

        // Fill right
        let p = (*x + 1, *y + offset);
        let right = fill_dir(&p, 1, 'R', clay, water, edges, stack);

        // Fill center
        let p = (*x, *y + offset);
        water.insert(p, '~');

        if left && right {
            offset -= 1;
        } else {
            break *y + offset
        }
    }
}

fn fill_dir(
    pt: &(i32, i32),
    n: i32,
    dir: char,
    clay: &Clay,
    water: &mut Water,
    edges: &mut Edges,
    stack: &mut Stack
) -> bool {
    let mut p = *pt;
    while !clay.contains(&p) {
        water.insert(p, '~');

        let below = (p.0, p.1 + 1);
        if edges.contains(&(dir, p)) {
            if !water.contains_key(&(p.0 + n, p.1 + 1)) {
                return false
            }
        } else if !water.contains_key(&below) && !clay.contains(&below) {
            water.insert(p, '|');
            edges.insert((dir, p));
            stack.push(p);
            return false
        }

        p = (p.0 + n, p.1);
    }

    true
}

#[allow(dead_code)]
fn print(clay: &Clay, water: &Water) {
    let min_x = clay.iter().min().unwrap().0;
    let max_x = clay.iter().max().unwrap().0;
    let max_y = clay.iter().max_by_key(|a| a.1).unwrap().1;

    for y in 0..max_y + 1 {
        for x in min_x-2..max_x+1 {
            if x == 500 && y == 0 {
                print!("+")
            } else if clay.contains(&(x, y)) {
                print!("#")
            } else if water.contains_key(&(x, y)) {
                print!("{}", water.get(&(x, y)).unwrap())
            } else {
                print!(".")
            }
        }
        println!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let clay = load(include_str!("./input.txt"));

        let water = part_one(&clay);
        assert_eq!(water, 39877);

        let water = part_two(&clay);
        assert_eq!(water, 33291);
    }
}
