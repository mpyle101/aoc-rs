fn main() {
    use std::{fs, time::Instant};
    
    let input = fs::read_to_string("./input.txt").unwrap();

    let t1 = Instant::now();
    let value = part_one(&input);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", value, t2 - t1);

    let t1 = Instant::now();
    let value = part_two(&input);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", value, t2 - t1);
}

fn part_one(input: &str) -> i32 {
    use std::collections::HashMap;

    let registers = input.lines()
        .fold(HashMap::new(), |mut m, s| {
            let v = s.split(' ').collect::<Vec<_>>();
            let r2 = *m.entry(v[4]).or_insert(0);
            let r1 = m.entry(v[0]).or_insert(0);
            let p1 = v[2].parse::<i32>().unwrap();
            let p2 = v[6].parse::<i32>().unwrap();
            let op = v[5];
            if check(r2, p2, op) {
                if v[1] == "inc" {
                    *r1 += p1
                } else {
                    *r1 -= p1
                }
            }

            m
        });

    *registers.values().max().unwrap()
}

fn part_two(input: &str) -> i32 {
    use std::collections::HashMap;

    let mut highest = 0;
    input.lines()
        .fold(HashMap::new(), |mut m, s| {
            let v = s.split(' ').collect::<Vec<_>>();
            let r2 = *m.entry(v[4]).or_insert(0);
            let r1 = m.entry(v[0]).or_insert(0);
            let p1 = v[2].parse::<i32>().unwrap();
            let p2 = v[6].parse::<i32>().unwrap();
            let op = v[5];
            if check(r2, p2, op) {
                if v[1] == "inc" {
                    *r1 += p1
                } else {
                    *r1 -= p1
                }
            }
            highest = highest.max(*r1);

            m
        });

    highest
}

fn check(r: i32, p: i32, op: &str) -> bool {
    match op {
        "<"  => r < p,
        ">"  => r > p,
        ">=" => r >= p,
        "<=" => r <= p,
        "==" => r == p,
        "!=" => r != p,
        _ => panic!("Unknown operator: {}", op)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input  = fs::read_to_string("./input.txt").unwrap();

        let value = part_one(&input);
        assert_eq!(value, 4888);

        let value = part_two(&input);
        assert_eq!(value, 7774);
    }
}