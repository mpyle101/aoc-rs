fn main() {
    use std::{fs, time::Instant};

    let input = fs::read_to_string("./input.txt").unwrap();
    let digits = load(&input);

    let t1 = Instant::now();
    let captcha = part_one(&digits);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", captcha, t2 - t1);

    let t1 = Instant::now();
    let captcha = part_two(&digits);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", captcha, t2 - t1);
}

fn load(input: &str) -> Vec<u8> {
    input.chars().map(|c| c as u8 - '0' as u8).collect()
}

fn part_one(digits: &[u8]) -> i32 {
    let captcha = (1..digits.len()).fold(0, |acc, i|
        if digits[i-1] == digits[i] { acc + digits[i] as i32 } else { acc }
    );
    if digits.last() == digits.first() {
        captcha + *digits.last().unwrap() as i32
    } else {
        captcha
    }
}

fn part_two(digits: &[u8]) -> i32 {
    let n = digits.len() / 2;
    (0..digits.len()).fold(0, |acc, i| {
        let ix = (i + n) % digits.len();
        if digits[i] == digits[ix] { acc + digits[i] as i32 } else { acc }
    })
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let digits = load(&input);

        let captcha = part_one(&digits);
        assert_eq!(captcha, 1119);

        let captcha = part_two(&digits);
        assert_eq!(captcha, 1420);
    }
}