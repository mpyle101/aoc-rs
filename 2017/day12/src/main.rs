use std::collections::{HashMap, HashSet};

fn main() {
    use std::{fs, time::Instant};
    
    let input = fs::read_to_string("./input.txt").unwrap();
    let pipes = load(&input);

    let t1 = Instant::now();
    let programs = part_one(&pipes);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", programs, t2 - t1);

    let t1 = Instant::now();
    let groups = part_two(&pipes);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", groups, t2 - t1);
}

fn load(input: &str) -> HashMap<i32, Vec<i32>> {
    input.lines().map(|l| {
        let v = l.split(' ').collect::<Vec<_>>();
        let n = v[0].parse::<i32>().unwrap();
        let m = v[2..].iter()
            .map(|s| s.trim_end_matches(',').parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        (n, m)
    })
    .collect()
}

fn part_one(pipes: &HashMap<i32, Vec<i32>>) -> usize {
    get_group(0, pipes).len()
}

fn part_two(pipes: &HashMap<i32, Vec<i32>>) -> usize {
    let mut keys = pipes.keys().collect::<Vec<_>>();
    keys.sort();

    let mut groups = 0;

    while !keys.is_empty() {
        let n = *keys[0];
        get_group(n, pipes).iter().for_each(|v| {
            let i = keys.iter().position(|x| *x == v).unwrap();
            keys.remove(i);
        });
        groups += 1;
    }

    groups
}

fn get_group(n: i32, pipes: &HashMap<i32, Vec<i32>>) -> HashSet<i32> {
    use std::collections::VecDeque;

    let mut programs = HashSet::new();

    let mut q = VecDeque::from([n]);
    while let Some(n) = q.pop_front() {
        programs.insert(n);
        if let Some(v) = pipes.get(&n) {
            v.iter().for_each(|p| 
                if programs.insert(*p) {
                    q.push_back(*p)
                }
            )
        }
    }
    
    programs
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let pipes = load(&input);
    
        let programs = part_one(&pipes);
        assert_eq!(programs, 175);
    
        let groups = part_two(&pipes);
        assert_eq!(groups, 213);
    }
}