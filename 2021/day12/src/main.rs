use std::collections::{HashMap, HashSet};

fn main() {
    use std::fs;
    use std::time::Instant;
    
    let input = fs::read_to_string("./input.txt").unwrap();
    let caves = load(&input);

    let t1 = Instant::now();
    let paths = part_one(&caves);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", paths, t2 - t1);

    let t1 = Instant::now();
    let paths = part_two(&caves);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", paths, t2 - t1);
}

fn load(input: &str) -> HashMap<&str, Vec<&str>> {
    // Create an adjacency list of caves
    input.lines()
        .map(|s| s.split('-'))
        .fold(HashMap::new(), |mut m, mut caves| {
            let a = caves.next().unwrap();
            let b = caves.next().unwrap();
            m.entry(a).or_insert(Vec::new()).push(b);
            m.entry(b).or_insert(Vec::new()).push(a);
            m
        })
}

#[derive(Clone, Debug)]
struct Path<'a> {
    caves: Vec<&'a str>,
    visited: HashSet<&'a str>,
}

impl<'a> Path<'a> {
    fn new() -> Path<'a> {
        Path {
            caves: vec!["start"],
            visited: HashSet::from_iter(vec!["start"])
        }
    }
}

fn part_one(caves: &HashMap<&str, Vec<&str>>) -> usize {
    use std::collections::VecDeque;

    let mut paths = Vec::new();
    let mut q = VecDeque::from_iter(vec![Path::new()]);
    while let Some(path) = q.pop_front() {
        let cave = path.caves.last().unwrap();
        let adjacent = caves.get(cave).unwrap();
        adjacent.iter().for_each(|s| {
            // See if we'll either be at the end (Yay!) or we're allowed
            // to enter the next cave (we can only visit lowercase caves
            // once).
            if *s == "end" || !path.visited.contains(s) {
                let mut p = path.clone();
                p.caves.push(s);
                if *s == "end" { 
                    paths.push(p)
                } else { 
                    let c = s.chars().next().unwrap();
                    if c.is_ascii_lowercase() {
                        p.visited.insert(s);
                    }
                    q.push_back(p)
                }
            }
        })
    }

    paths.len()
}

fn part_two(caves: &HashMap<&str, Vec<&str>>) -> usize {
    use std::collections::VecDeque;

    let mut paths = Vec::new();
    let mut q = VecDeque::from_iter(vec![Path::new()]);
    while let Some(path) = q.pop_front() {
        let cave = path.caves.last().unwrap();
        let adjacent = caves.get(cave).unwrap();
        adjacent.iter().for_each(|s| {
            if *s == "end" {
                let mut p = path.clone();
                p.caves.push(s);
                paths.push(p);
            } else {
                
            }
            if *s == "end" || !path.visited.contains(s) {
                let mut p = path.clone();
                p.caves.push(s);
                if *s == "end" { 
                    paths.push(p)
                } else { 
                    let c = s.chars().next().unwrap();
                    if c.is_ascii_lowercase() {
                        p.visited.insert(s);
                    }
                    q.push_back(p)
                }
            }
        })
    }

    paths.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let caves = load(&input);

        let paths = part_one(&caves);
        assert_eq!(paths, 4970);

        let paths = part_two(&caves);
        assert_eq!(paths, 4970);
    }
}