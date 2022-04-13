
fn main() {
    use std::fs;
    use std::time::Instant;
    
    let input = fs::read_to_string("./input.txt").unwrap();
    let squid = load(&input);

    let t1 = Instant::now();
    let flashes = part_one(&squid);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", flashes, t2 - t1);

    let t1 = Instant::now();
    let step = part_two(&squid);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", step, t2 - t1);
}

fn load(input: &str) -> Vec<u32> {
    input.lines().flat_map(|l| l.chars().map(
        |c| c.to_digit(10).unwrap()).collect::<Vec<_>>()
    ).collect()
}

fn part_one(squid: &[u32]) -> u32 {
    let mut m = squid.to_vec();
    (0..100).fold(0, |acc, _| acc + step(&mut m))
}

fn part_two(squid: &[u32]) -> u32 {
    let mut m = squid.to_vec();

    let mut steps   = 0;
    let mut flashes = 0;
    while flashes != 100 {
        flashes = step(&mut m);
        steps += 1;
    }

    steps
}

fn step(m: &mut [u32]) -> u32 {
    use std::collections::VecDeque;

    m.iter_mut().for_each(|n| *n = (*n + 1) % 10);
    let mut q = m.iter().enumerate()
        .fold(VecDeque::new(), |mut q, (i, &n)| {
            if n == 0 { q.push_back(i) };
            q
        });

    let mut flashes = 0;
    while let Some(pos) = q.pop_front() {
        flashes += 1;
        neighbors(m, pos).iter()
            .filter(|(_, n)| *n > 0 && *n < 10)
            .for_each(|(i, n)| {
                m[*i] = (*n + 1) % 10;
                if m[*i] == 0 { q.push_back(*i) }
            }
        );
    }

    flashes
}

#[allow(dead_code)]
fn print(m: &[u32]) {
    m.iter().enumerate().for_each(|(i, n)|
        if (i + 1) % 10 == 0 { println!("{}", *n); } else { print!("{}", *n); }
    );
    println!();
}

fn neighbors(m: &[u32], p: usize) -> [(usize, u32);8] {
    use core::num::Wrapping;

    const ONE: Wrapping<usize>  = Wrapping(1usize);
    const ZERO: Wrapping<usize> = Wrapping(0usize);
    const XDIM: Wrapping<usize> = Wrapping(10usize);

    let pos = Wrapping(p);
    let top = pos - XDIM;
    let bot = pos + XDIM;
    let pre = Wrapping(if pos % XDIM == ZERO { 1000 } else { 1 });
    let pst = Wrapping(if (pos + ONE) % XDIM == ZERO { 1000 } else { 1 });
    let mut arr = [
        ((top - pre).0, u32::MAX), (top.0, u32::MAX), ((top + pst).0, u32::MAX),
        ((pos - pre).0, u32::MAX),                    ((pos + pst).0, u32::MAX),
        ((bot - pre).0, u32::MAX), (bot.0, u32::MAX), ((bot + pst).0, u32::MAX)
    ];

    arr.iter_mut()
        .filter(|(i, _)| *i < 100)
        .for_each(|(i, n)| *n = m[*i]);
    arr
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let squid = load(&input);

        let flashes = part_one(&squid);
        assert_eq!(flashes, 1546);

        let steps = part_two(&squid);
        assert_eq!(steps, 471);
    }
}