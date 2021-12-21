fn main() {
    use std::fs;
    use std::time::Instant;
    
    let input = fs::read_to_string("./input.txt").unwrap();
    let positions = load(&input);

    let t1 = Instant::now();
    let score = part_one(&positions);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", score, t2 - t1);

    let t1 = Instant::now();
    let universes = part_two(&positions);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", universes, t2 - t1);
}

fn load(input: &str) -> Vec<i32> {
    input.lines().map(|s| {
        let v = s.split(' ').collect::<Vec<_>>();
        v[4].parse().unwrap()
    }).collect()
}

fn part_one(positions: &[i32]) -> i32 {
    let mut p1 = positions[0];
    let mut p2 = positions[1];

    let mut score1 = 0;
    let mut score2 = 0;

    let mut dice  = 0;
    let mut rolls = 0;
    loop {
        p1 = (p1 + roll(&mut dice)) % 10;
        rolls += 3;
        score1 += score(p1);
        if score1 >= 1000 {
            break
        }

        p2 = (p2 + roll(&mut dice)) % 10;
        rolls += 3;
        score2 += score(p2);
        if score2 >= 1000 {
            break
        }
    }

    score1.min(score2) * rolls
}

fn part_two(positions: &[i32]) -> i32 {
    let mut p1 = positions[0];
    let mut p2 = positions[1];

    let mut score1 = 0;
    let mut score2 = 0;

    let mut dice  = 0;
    let mut rolls = 0;
    loop {
        p1 = (p1 + roll(&mut dice)) % 10;
        rolls  += 3;
        score1 += score(p1);
        if score1 >= 1000 {
            break
        }

        p2 = (p2 + roll(&mut dice)) % 10;
        rolls  += 3;
        score2 += score(p2);
        if score2 >= 1000 {
            break
        }
    }

    score1.min(score2) * rolls
}

fn roll(dice: &mut i32) -> i32 {
    (0..3).map(|_| {
        *dice = if *dice == 100 { 1 } else { *dice + 1 };
        *dice
    }).sum()
}

fn score(pos: i32) -> i32 {
    if pos == 0 { 10 } else { pos }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let positions = load(&input);

        let score = part_one(&positions);
        assert_eq!(score, 678468);
    }

    #[test]
    fn small() {
        let input = fs::read_to_string("./test.txt").unwrap();
        let positions = load(&input);

        let score = part_one(&positions);
        assert_eq!(score, 739785);
    }
}