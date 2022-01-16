fn main() {
    use std::{fs, time::Instant};

    let input = fs::read_to_string("./input.txt").unwrap();
    let discs = load(&input);

    let t1 = Instant::now();
    let when = part_one(&discs);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", when, t2 - t1);

    let t1 = Instant::now();
    let when = part_two(&discs);
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", when, t2 - t1);
}

fn load(input: &str) -> Vec<(i64, i64)> {
    input.lines().map(|s| {
        let v = s.split(' ').collect::<Vec<_>>();
        let positions = v[3].parse::<i64>().unwrap();
        let start = v[11].trim_end_matches('.').parse::<i64>().unwrap();
        (positions, start)
    })
    .collect()
}

fn part_one(discs: &[(i64, i64)]) -> i64 {
    let mut time = discs[0].0 - discs[0].1 - 1;

    loop {
        let capsule = discs.iter().enumerate()
            .all(|(i, (n, p))| (time + p + 1 + i as i64) % n == 0);
        if capsule {
            break time;
        }

        time += discs[0].0;
    }
}

fn part_two(discs: &[(i64, i64)]) -> i64 {
    let mut discs2 = discs.iter().cloned().collect::<Vec<_>>();
    discs2.push((11, 0));

    let mut time = discs2[0].0 - discs2[0].1 - 1;

    loop {
        let capsule = discs2.iter().enumerate()
            .all(|(i, (n, p))| (time + p + 1 + i as i64) % n == 0);
        if capsule {
            break time;
        }

        time += discs2[0].0;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let discs = load(&input);
        
        let when = part_one(&discs);
        assert_eq!(when, 376777);
        
        let when = part_two(&discs);
        assert_eq!(when, 3903937);
    }
}