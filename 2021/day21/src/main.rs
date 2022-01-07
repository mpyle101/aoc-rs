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
        p = 1 - p;  // flips the player back and forth
        pos[p] = (pos[p] + 3 * r + 3) % 10;
        score[p] += pos[p] + 1;
        r += 3;
    }

    score.iter().min().unwrap() * (r - 1)  
}

fn part_two(starting: &[i32]) -> u64 {
    // The number of won states at each roll.
    let wins1 = wins(starting[0]);
    let wins2 = wins(starting[1]);

    // Number of unfinished states at each roll. The total number
    // of possible states minus the number of won states. The total
    // number of possible states is the last number of states times
    // 27 (possible 2 die rolls).
    let states1 = wins1.iter()
        .skip(1).fold(vec![27], |mut v, w| {
            if let Some(&n) = v.last() { v.push(n * 27 - w) }
            v
        });
    let states2 = wins2.iter()
        .skip(1).fold(vec![27], |mut v, w| {
            if let Some(&n) = v.last() { v.push(n * 27 - w) }
            v
        });

    // Since player 1 goes first, they match against the unfinished
    // states in player 2's previous roll.
    let tot1 = wins1.iter().skip(1).zip(states2.iter())
        .map(|(w, s)| w * s)
        .collect::<Vec<_>>();

    // Since player 2 goes second, they match against the same roll in
    // player 1's unfinished states.
    let tot2 = wins2.iter().zip(states1.iter())
        .map(|(w, s)| w * s)
        .collect::<Vec<_>>();

    let sum1 = tot1.iter().sum::<u64>();
    let sum2 = tot2.iter().sum::<u64>();

    sum1.max(sum2)
}

fn wins(pos: i32) -> Vec<u64> {
    use std::collections::HashMap;

    // dice result to frequency of rolls producing that value
    let df = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    // Number of unfinished states for a given position/score
    // combination.
    let mut states = HashMap::new();
    states.insert((pos, 0), 1);

    // Number of wins on a given roll
    let mut wins = Vec::new();
    while !states.is_empty() {
        // Count the wins for the next set of rolls.
        wins.push(0);
        states = states.iter().fold(HashMap::new(), |mut m, ((p, s), c)| {
            df.iter().for_each(|(d, f)| {
                let pos   = (p + d - 1) % 10 + 1;
                let score = s + pos;
                if score < 21 {
                    // Gonna need to roll again, add the new number of possibilities,
                    // which is all the old ones time the frequency of this dice roll.
                    *m.entry((pos, score)).or_insert(0) += c * f;
                } else if let Some(last) = wins.last_mut() {
                    // Number of wins goes up by the number of possibliities
                    // for the state times the frequency of this roll result.
                    *last += c * f;
                }
            });

            m
        })
    }

    wins
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