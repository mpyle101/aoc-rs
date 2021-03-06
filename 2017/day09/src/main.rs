fn main() {
    use std::{fs, time::Instant};
    
    let input = fs::read_to_string("./input.txt").unwrap();

    let t1 = Instant::now();
    let score = part_one(&input);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", score, t2 - t1);

    let t1 = Instant::now();
    let garbage = part_two(&input);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", garbage, t2 - t1);
}

fn part_one(input: &str) -> i64 {
    let mut score = 0;

    let mut depth   = 0;
    let mut cancel  = false;
    let mut garbage = false;

    input.chars().for_each(|c|
        match c {
            _ if cancel => cancel = false,
            '>' => garbage = false,
            '!' => cancel = true,
            '<' => garbage = true,
            '{' if !garbage => depth += 1,
            '}' if !garbage => { score += depth; depth -= 1 },
             _  => {}
       }
    );

    score
}

fn part_two(input: &str) -> i64 {
    let mut cancel  = false;
    let mut garbage = false;

    input.chars().fold(0, |acc, c|
        match c {
            _ if cancel => { cancel = false; acc},
            '>' => { garbage = false; acc },
            '!' => { cancel = true; acc },
            _ if garbage => acc + 1,
            '<' => { garbage = true; acc },
            _ => acc
       }
    )
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input  = fs::read_to_string("./input.txt").unwrap();

        let score = part_one(&input);
        assert_eq!(score, 10050);

        let garbage = part_two(&input);
        assert_eq!(garbage, 4482);
    }

    #[test]
    fn samples() {
        let score = part_one("{}");
        assert_eq!(score, 1);

        let score = part_one("{{{}}}");
        assert_eq!(score, 6);

        let score = part_one("{{},{}}");
        assert_eq!(score, 5);

        let score = part_one("{{{},{},{{}}}}");
        assert_eq!(score, 16);

        let score = part_one("{<a>,<a>,<a>,<a>}");
        assert_eq!(score, 1);

        let score = part_one("{{<ab>},{<ab>},{<ab>},{<ab>}}");
        assert_eq!(score, 9);

        let score = part_one("{{<!!>},{<!!>},{<!!>},{<!!>}}");
        assert_eq!(score, 9);

        let score = part_one("{{<a!>},{<a!>},{<a!>},{<ab>}}");
        assert_eq!(score, 3);
    }
}