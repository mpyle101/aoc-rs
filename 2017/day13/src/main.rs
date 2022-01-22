
fn main() {
    use std::{fs, time::Instant};
    
    let input = fs::read_to_string("./input.txt").unwrap();
    let scanners = load(&input);

    let t1 = Instant::now();
    let severity = part_one(&scanners);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", severity, t2 - t1);

    let t1 = Instant::now();
    let delay = part_two(&scanners);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", delay, t2 - t1);
}

fn load(input: &str) -> Vec<(i32, i32)> {
    input.lines().map(|l| {
        let mut it = l.split(": ");
        let depth = it.next().unwrap().parse::<i32>().unwrap();
        let range = it.next().unwrap().parse::<i32>().unwrap();

        (depth, range)
    })
    .collect()
}

fn part_one(scanners: &[(i32, i32)]) -> i32 {
    scanners.iter().map(|(d, r)|
        if d % (r * 2 - 2) == 0 { d * r } else { 0 }
    )
    .sum()
}

fn part_two(scanners: &[(i32, i32)]) -> i32 {
    let mut delay = 0;

    loop {
        let mut safe = true;
        for (d, r) in scanners {
            if (d + delay) % (r * 2 - 2) == 0 {
                safe = false;
                break;
            }
        }

        if safe { break delay }
        delay += 1
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let scanners = load(&input);
    
        let severity = part_one(&scanners);
        assert_eq!(severity, 1840);
    
        let delay = part_one(&scanners);
        assert_eq!(delay, 3850260);
    }

    #[test]
    fn example() {
        let scanners = load("0: 3\n1: 2\n4: 4\n6: 4");
    
        let delay = part_two(&scanners);
        assert_eq!(delay, 10);
    }
}
