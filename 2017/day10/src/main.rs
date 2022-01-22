fn main() {
    use std::{fs, time::Instant};
    
    let input = fs::read_to_string("./input.txt").unwrap();

    let t1 = Instant::now();
    let score = part_one(&input);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", score, t2 - t1);

    let t1 = Instant::now();
    let hash = part_two(&input);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", hash, t2 - t1);
}

fn part_one(input: &str) -> u32 {
    let mut list = [0u32;256];
    (0..256).for_each(|i| list[i] = i as u32);

    let mut pos  = 0;
    let mut skip = 0;
    input.split(',').for_each(|s| {
        let length = s.parse::<usize>().unwrap();
        if length > 0 {
            let mut i = 0;
            let mut j = length - 1;
            while i < j {
                list.swap((pos + i) % 256, (pos + j) % 256);
                i += 1;
                j -= 1;
            }
        }

        pos = (pos + length + skip) % 256;
        skip += 1;
    });
    
    list[0] * list[1]
}

fn part_two(input: &str) -> String {
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
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();

        let score = part_one(&input);
        assert_eq!(score, 37230);

        let hash = part_two(&input);
        assert_eq!(hash, "70b856a24d586194331398c7fcfa0aaf");
    }

    #[test]
    fn examples() {
        let hash = part_two("");
        assert_eq!(hash, "a2582a3a0e66e6e86e3812dcb672a272");

        let hash = part_two("AoC 2017");
        assert_eq!(hash, "33efeb34ea91902bb2f59c9920caa6cd");

        let hash = part_two("1,2,3");
        assert_eq!(hash, "3efbe78a8d82f29979031a4aa0b16a9d");

        let hash = part_two("1,2,4");
        assert_eq!(hash, "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}