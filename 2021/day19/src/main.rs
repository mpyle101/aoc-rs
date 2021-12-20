fn main() {
    use std::fs;
    use std::time::Instant;
    
    let input = fs::read_to_string("./input.txt").unwrap();
    let scanners = load(&input);

    let t1 = Instant::now();
    let beacons = part_one(&scanners);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", beacons, t2 - t1);
}

#[derive(Debug)]
struct Beacon {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug)]
struct Scanner {
    facing: i32,
    rotation: i32,
    beacons: Vec<Beacon>,
}

fn load(input: &str) -> Vec<Scanner> {
    input.split("\n\n").map(|s| {
        Scanner {
            facing: 0,
            rotation: 0,
            beacons: 
                s.lines().skip(1).map(|b| {
                    let mut v = b.split(',').map(|n| n.parse().unwrap());
                    Beacon {
                        x: v.next().unwrap(),
                        y: v.next().unwrap(),
                        z: v.next().unwrap(),
                    }
                }).collect(),
        }
    }).collect()
}

fn part_one(scanners: &[Scanner]) -> i64 {

    0
}