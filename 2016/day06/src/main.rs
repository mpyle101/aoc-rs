fn main() {
    use std::fs;
    use std::time::Instant;

    let input = fs::read_to_string("./input.txt").unwrap();

    let t1 = Instant::now();
    let message = part_one(&input);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", message, t2 - t1);

    let t1 = Instant::now();
    let message = part_two(&input);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", message, t2 - t1);
}

fn part_one(input: &str) -> String {
    let mut signal = [[0u8;26]; 8];

    input.lines().for_each(|s|
        s.chars().enumerate().for_each(|(i, c)| {
            let ix = c as usize - 'a' as usize;
            signal[i][ix] += 1;
        })
    );
    signal.iter().map(|v|
        v.iter().enumerate()
            .max_by_key(|(_, n)| *n).map(|(i, _)| i).unwrap()
    )
    .map(|n| (n as u8 + b'a') as char)
    .collect()
}

fn part_two(input: &str) -> String {
    let mut signal = [[0u8;26]; 8];

    input.lines().for_each(|s|
        s.chars().enumerate().for_each(|(i, c)| {
            let ix = c as usize - 'a' as usize;
            signal[i][ix] += 1;
        })
    );
    signal.iter().map(|v|
        v.iter().enumerate()
            .min_by_key(|(_, n)| *n).map(|(i, _)| i).unwrap()
    )
    .map(|n| (n as u8 + b'a') as char)
    .collect()
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();

        let message = part_one(&input);
        assert_eq!(message, "gebzfnbt");

        let message = part_two(&input);
        assert_eq!(message, "fykjtwyn");
    }
}