fn main() {
    use std::time::Instant;

    let t1 = Instant::now();
    let used = part_one("ffayrhll");
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", used, t2 - t1);

    let t1 = Instant::now();
    let regions = part_two("ffayrhll");
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", regions, t2 - t1);
}

fn part_one(key: &str) -> i32 {
    (0..128).fold(0, |used, row| {
        let hash = knot(&format!("{}-{}", key, row));
        let bits = hash.bytes().map(|c| {
            let mut n = if c <= '9' as u8 { c - '0' as u8 } else { c - 'a' as u8 + 10 };
            let mut bits = 0;
            while n > 0 { bits += n & 1; n /= 2; }
            bits as i32
        })
        .sum::<i32>();

        used + bits
    })
}

fn part_two(key: &str) -> i32 {
    use std::collections::{HashSet, VecDeque};

    let mut used = HashSet::new();
    (0..128).for_each(|y| {
        let hash = knot(&format!("{}-{}", key, y));
        let bits = hash.bytes().map(|c| {
                let n = if c <= '9' as u8 { c - '0' as u8 } else { c - 'a' as u8 + 10 };
                format!("{:04b}", n)
            })
            .collect::<Vec<_>>()
            .join("");
        bits.chars().enumerate()
            .for_each(|(x, c)| if c == '1' { used.insert((x as i32, y)); } )
    });

    let delta = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    let mut q = VecDeque::new();
    let mut regions = 0;
    while !used.is_empty() {
        regions += 1;

        let p = *used.iter().next().unwrap();
        used.remove(&p);

        q.push_back(p);
        while let Some(p) = q.pop_front() {
            delta.iter().for_each(|d| {
                let pt = (p.0 + d.0, p.1 + d.1);
                if used.contains(&pt) {
                    used.remove(&pt);
                    q.push_back(pt);
                }
            })
        }
    }

    regions
}

fn knot(input: &str) -> String {
    let mut sparse = [0u8;256];
    (0..256).for_each(|i| sparse[i] = i as u8);

    let mut pos  = 0;
    let mut skip = 0;

    let suffix = [17u8, 31, 73, 47, 23];
    let stream = input.as_bytes();

    (0..64).for_each(|_|
        stream.iter().chain(suffix.iter()).for_each(|b| {
            let length = *b as usize;
            if length > 0 {
                let mut i = 0;
                let mut j = length - 1;
                while i < j {
                    sparse.swap((pos + i) % 256, (pos + j) % 256);
                    i += 1;
                    j -= 1;
                }
            }
    
            pos = (pos + length + skip) % 256;
            skip += 1;
        })
    );

    let dense = (0..256)
        .step_by(16)
        .map(|i| (i..i+16).skip(1).fold(sparse[i], |v, n| v ^ sparse[n]))
        .map(|v| format!("{:02x}", v))
        .collect::<Vec<_>>();
    
    dense.join("")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let used = part_one("ffayrhll");
        assert_eq!(used, 8190);

        let regions = part_two("ffayrhll");
        assert_eq!(regions, 1134);
    }

    #[test]
    fn example() {
        let used = part_one("flqrgnkx");
        assert_eq!(used, 8108);

        let regions = part_two("flqrgnkx");
        assert_eq!(regions, 1242);
    }
}
