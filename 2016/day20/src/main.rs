fn main() {
    use std::{fs, time::Instant};

    let input = fs::read_to_string("./input.txt").unwrap();
    let excluded = load(&input);

    let t1 = Instant::now();
    let ip = part_one(&excluded);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", ip, t2 - t1);

    let t1 = Instant::now();
    let ips = part_two(&excluded);
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", ips, t2 - t1);
}

fn load(input: &str) -> Vec<(u32, u32)> {
    input.lines().map(|s| {
        let mut it = s.split('-');
        let start = it.next().unwrap().parse::<u32>().unwrap();
        let end   = it.next().unwrap().parse::<u32>().unwrap();
        (start, end)
    })
    .collect()
}

fn part_one(excluded: &[(u32, u32)]) -> u32 {
    let mut v = excluded.to_vec();
    v.sort();

    let mut n = v[0].1 + 1;
    for (start, end) in v.iter().skip(1) {
        if *start > n {
            return n
        } if *end >= n {
            n = end + 1
        }
    }

    0
}

fn part_two(excluded: &[(u32, u32)]) -> u32 {
    let mut v = excluded.to_vec();
    v.sort();

    let mut n = v[0].1 + 1;
    let mut count = 0;
    for (start, end) in v.iter().skip(1) {
        if *start > n {
            count += *start - n;
            n = if *end < u32::MAX { end + 1 } else { u32::MAX }
        } if *end >= n {
            n = if *end < u32::MAX { end + 1 } else { u32::MAX }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let excluded = load(&input);
        
        let ip = part_one(&excluded);
        assert_eq!(ip, 4793564);
        
        let ips = part_two(&excluded);
        assert_eq!(ips, 146);
    }
}