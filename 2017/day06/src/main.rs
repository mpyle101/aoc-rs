
fn main() {
    use std::fs;
    use std::time::Instant;
    
    let input  = fs::read_to_string("./input.txt").unwrap();
    let blocks = load(&input);

    let t1 = Instant::now();
    let cycles = part_one(&blocks);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", cycles, t2 - t1);

    let t1 = Instant::now();
    let cycles = part_two(&blocks);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", cycles, t2 - t1);
}

fn load(input: &str) -> Vec<i32> {
    input.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect()
}

fn part_one(blocks: &[i32]) -> i32 {
    use std::collections::HashSet;

    let mut banks  = blocks.to_vec();
    let mut states = HashSet::new();
    while !states.contains(&banks) {
        states.insert(banks.clone());
        banks = cycle(&banks);
    }

    states.len() as i32
}

fn part_two(blocks: &[i32]) -> i32 {
    use std::collections::HashSet;

    let mut banks  = blocks.to_vec();
    let mut states = HashSet::new();
    while !states.contains(&banks) {
        states.insert(banks.clone());
        banks = cycle(&banks);
    }

    let state = banks.clone();
    let mut cycles = 1;
    banks = cycle(&banks);
    while banks != state { 
        banks = cycle(&banks);
        cycles += 1;
    }

    cycles
}


fn cycle(banks: &[i32]) -> Vec<i32> {
    let mut n = *banks.iter().max().unwrap();
    let mut i = banks.iter().position(|v| *v == n).unwrap();

    let mut blocks = banks.to_vec();
    blocks[i] = 0;
    while n > 0 {
        i = (i + 1) % banks.len();
        blocks[i] += 1;
        n -= 1;
    }
    
    blocks
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input  = fs::read_to_string("./input.txt").unwrap();
        let blocks = load(&input);

        let cycles = part_one(&blocks);
        assert_eq!(cycles, 11137);

        let cycles = part_two(&blocks);
        assert_eq!(cycles, 1037);
    }
}