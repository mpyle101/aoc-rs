use std::collections::HashMap;

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

            // We can't go back to the start or end
            if a != "end" && b != "start" {
                m.entry(a).or_insert_with(Vec::new).push(b);
            }
            if a != "start" && b != "end" {
                m.entry(b).or_insert_with(Vec::new).push(a);
            }
            m
        })
}

#[derive(Clone, Debug)]
struct Path<'a> {
    cave: &'a str,
    visited: HashMap<&'a str, u8>,
}

impl<'a> Path<'a> {
    fn new() -> Path<'a> {
        Path {
            cave: "start",
            visited: HashMap::from_iter(vec![("start", 0)])
        }
    }
}

fn part_one(caves: &HashMap<&str, Vec<&str>>) -> usize {
    use std::collections::VecDeque;

    let mut paths = 0;
    let mut q = VecDeque::from_iter(vec![Path::new()]);
    while let Some(path) = q.pop_front() {
        let adjacent = caves.get(path.cave).unwrap();
        adjacent.iter().for_each(|s| {
            // See if we'll either be at the end (Yay!) or we're allowed
            // to enter the next cave (we can only visit small caves once).
            if *s == "end" || !path.visited.contains_key(s) {
                if *s == "end" { 
                    paths += 1
                } else { 
                    let mut p = path.clone();
                    let c = s.chars().next().unwrap();
                    if c.is_ascii_lowercase() {
                        p.visited.insert(s, 1);
                    }
                    p.cave = s;
                    q.push_back(p)
                }
            }
        })
    }

    paths
}

fn part_two(caves: &HashMap<&str, Vec<&str>>) -> usize {
    use std::collections::VecDeque;

    let mut paths = 0;
    let mut q = VecDeque::from_iter(vec![Path::new()]);
    while let Some(path) = q.pop_front() {
        let adjacent = caves.get(path.cave).unwrap();
        adjacent.iter().for_each(|s| {
            let c = s.chars().next().unwrap();

            if c.is_ascii_uppercase() {
                let mut p = path.clone();
                p.cave = s;
                q.push_back(p)
            } else if *s == "end" {
                paths += 1
            } else {
                // We can visit one small cave twice.
                let twice = path.visited.values().any(|v| *v == 2);
                if twice {
                    if !path.visited.contains_key(s) {
                        let mut p = path.clone();
                        p.cave = s;
                        p.visited.insert(s, 1);
                        q.push_back(p)
                    }
                } else {
                    let mut p = path.clone();
                    p.cave = s;
                    *p.visited.entry(s).or_insert(0) += 1;
                    q.push_back(p)
                }
            }
        })
    }

    paths
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
        assert_eq!(paths, 137948);
    }
}