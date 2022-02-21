fn main() {
    use std::fs;
    use std::time::Instant;

    let crabs = load(&fs::read_to_string("./input.txt").unwrap());

    let t1 = Instant::now();
    let fuel = part_one(&crabs);
    let t2 = Instant::now();
    println!("Part 1: {fuel} {:?}", t2 - t1);

    let t1 = Instant::now();
    let fuel = part_two(&crabs);
    let t2 = Instant::now();
    println!("Part 2: {fuel} {:?}", t2 - t1);
}

fn load(input: &str) -> Vec<i32> {
    let mut v = input.split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    v.sort_unstable();

    v
}

fn part_one(crabs: &[i32]) -> i32 {
    let first = crabs.first().unwrap();
    let last  = crabs.last().unwrap();
    (*first..=*last).map(|pos| 
        crabs.iter().map(|v| (v - pos).abs()).sum()
    ).min().unwrap()
}

fn part_two(crabs: &[i32]) -> i32 {
    let first = crabs.first().unwrap();
    let last  = crabs.last().unwrap();
    (*first..=*last).map(|pos| 
        crabs.iter().map(|v| fuel(*v, pos)).sum()
    ).min().unwrap()
}

fn fuel(from: i32, to: i32) -> i32 {
    // Each change of 1 step cost 1 more than the previous; the
    // first step costs 1, the second step costs 2, the third
    // steps costs 3, etc.
    (1..=(from - to).abs()).sum()
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let crabs = load(&fs::read_to_string("./input.txt").unwrap());

        let fuel = part_one(&crabs);
        assert_eq!(fuel, 325528);

        let fuel = part_two(&crabs);
        assert_eq!(fuel, 85015836);
    }
}