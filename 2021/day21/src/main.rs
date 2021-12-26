fn main() {
    use std::fs;
    use std::time::Instant;
    
    let input = fs::read_to_string("./test.txt").unwrap();
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

#[derive(Clone, Debug)]
struct Game {
    dice: i32,
    pos: [i32;2],
    scores: [i32;2],
    rolls: i32,
    wins: i32,
}

impl Game {
    fn new(start: &[i32], dice: i32, wins: i32) -> Game {
        Game {
            dice,
            wins,
            pos: [start[0], start[1]],
            scores: [0, 0],
            rolls: 0,
        }
    }

    fn eval(&self) -> i32 {
        self.scores.iter().min().unwrap() * self.rolls
    }

    fn done(&self) -> bool {
        self.scores[0] >= self.wins || self.scores[1] >= self.wins
    }
}

fn part_one(starting: &[i32]) -> i32 {
    let mut game = Game::new(starting, 0, 1000);
    while !game.done() {
        if !game.done() {
            do_turn(0, &mut game);
        }
        if !game.done() {
            do_turn(1, &mut game);
        }    
    }

    game.eval()
}

fn part_two(starting: &[i32]) -> u64 {
    println!("P1: {}", dirac(21 - starting[0] as usize));
    println!("P2: {}", dirac(21 - starting[1] as usize));

    0
}

fn dirac(goal: usize) -> u64 {
    let mut ways = vec![0;goal + 1];
    ways[0] = 1;
    for d in 1..=3 {
        for n in d..goal + 1 {
            ways[n] += ways[n - d]
        }
    }

    ways[goal]
}

fn do_turn(p: usize, game: &mut Game) {
    (0..3).for_each(|_| {
        game.dice = if game.dice == 100 { 1 } else { game.dice + 1 };
        game.pos[p] = (game.pos[p] + game.dice) % 10;
        game.rolls += 1;
    });

    game.scores[p] += score(game.pos[p]);
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