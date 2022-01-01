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

fn part_one(starting: &[i32]) -> i32 {
    let mut pos = [starting[0] - 1, starting[1] - 1];
    let mut score = [0, 0];

    let mut p = 1;
    let mut r = 1;

    while score[p] < 1000 {
        p = 1 - p;
        pos[p] = (pos[p] + 3 * r + 3) % 10;
        score[p] += pos[p] + 1;
        r += 3;
    }

    score.iter().min().unwrap() * (r - 1)  
}

fn part_two(starting: &[i32]) -> u64 {
    // roll result to frequency
    let rf = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    let pos = [(starting[0] - 1) as u64, (starting[1] - 1) as u64];
    let won = wins(pos[0], 21, pos[1], 21, &rf);

    won.0.max(won.1)
}

fn wins(p1: u64, s1: i64, p2: u64, s2: i64, rf: &[(u64, u64); 7],
) -> (u64, u64) {
    if s2 <= 0 {
        (0, 1)
    } else {
        rf.iter().fold((0, 0), |w, (r, f)| {
            let c = wins(p2, s2, (p1+r) % 10, s1 - 1 - ((p1+r) % 10) as i64, rf);
            (w.0 + f * c.1, w.1 + f * c.0)
        })
    }
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

        let wins = part_two(&positions);
        assert_eq!(wins, 131180774190079);
    }

    #[test]
    fn small() {
        let input = fs::read_to_string("./test.txt").unwrap();
        let positions = load(&input);

        let score = part_one(&positions);
        assert_eq!(score, 739785);
    }
}