fn main() {
    use std::time::Instant;

    let t1 = Instant::now();
    let count = part_one(591, 393);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", count, t2 - t1);

    let t1 = Instant::now();
    let count = part_two(591, 393);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", count, t2 - t1);
}

fn part_one(a: u64, b: u64) -> u64 {
    let (count, _, _) = (0..40_000_000).fold((0, a, b), |n, _| {
        let a1 = n.1 * 16807 % 2147483647;
        let b1 = n.2 * 48271 % 2147483647;
        let n1 = (a1 & 0xffff == b1 & 0xffff) as u64;

        (n.0 + n1, a1, b1)
    });

    count
}

fn part_two(a: u64, b: u64) -> usize {
    let mut gen_a = Vec::with_capacity(5_000_000);
    let mut a1 = a * 16807 % 2147483647;
    while gen_a.len() < 5_000_000 {
        if a1 % 4 == 0 { gen_a.push(a1) }
        a1 = a1 * 16807 % 2147483647;
    }

    let mut gen_b = Vec::with_capacity(5_000_000);
    let mut b1 = b * 48271 % 2147483647;
    while gen_b.len() < 5_000_000 {
        if b1 % 8 == 0 { gen_b.push(b1) }
        b1 = b1 * 48271 % 2147483647;
    }

    gen_a.iter().zip(gen_b.iter())
        .filter(|(a, b)| *a & 0xffff == *b & 0xffff)
        .count()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let count = part_one(591, 393);
        assert_eq!(count, 619);

        let count = part_two(591, 393);
        assert_eq!(count, 290);
    }

    #[test]
    fn example() {
        let count = part_one(65, 8921);
        assert_eq!(count, 588);

        let count = part_two(65, 8921);
        assert_eq!(count, 309);
    }
}
