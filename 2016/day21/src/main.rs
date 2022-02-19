use std::{fmt, str::FromStr};

fn main() {
    use std::{fs, time::Instant};

    let input = fs::read_to_string("./input.txt").unwrap();
    let actions = load(&input);

    let t1 = Instant::now();
    let password = part_one(&actions, "abcdefgh");
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", password, t2 - t1);

    let t1 = Instant::now();
    let password = part_two(&actions, "fbgdceah");
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", password, t2 - t1);
}

#[derive(Debug)]
enum Action {
    MovePositions(usize, usize),
    ReverseRange(usize, usize),
    RotateLeft(usize),
    RotateRight(usize),
    RotateLetter(char),
    SwapLetters(char, char),
    SwapPositions(usize, usize),
    UnrotateLetter(char),
}

impl Action {
    fn encode(&self, arr: &mut [char]) {
        use std::cmp::Ordering;
        use Action::*;

        match self {
            RotateLeft(n)  => arr.rotate_left(*n),
            RotateRight(n) => arr.rotate_right(*n),
            RotateLetter(c) => {
                let n = arr.iter().position(|v| v == c).unwrap();
                arr.rotate_right(1);
                arr.rotate_right(n);
                if n > 3 { arr.rotate_right(1) }
            },
            ReverseRange(a, b)  => arr[*a..=*b].reverse(),
            SwapLetters(c1, c2) => {
                let a = arr.iter().position(|c| c == c1).unwrap();
                let b = arr.iter().position(|c| c == c2).unwrap();
                arr.swap(a, b)
            },
            SwapPositions(a, b) => arr.swap(*a, *b),
            MovePositions(a, b) => {
                let c = arr[*a];
                let mut i = *a;
                match a.cmp(b) {
                    Ordering::Greater => while i > *b { arr[i] = arr[i-1]; i -= 1; },
                    Ordering::Less    => while i < *b { arr[i] = arr[i+1]; i += 1; },
                    Ordering::Equal   => {}
                }
                arr[*b] = c
            },
            UnrotateLetter(c) => {
                // Right rotations to get back to unrotated state.
                let m = [7, 7, 2, 6, 1, 5, 0, 4];
                let n = arr.iter().position(|v| v == c).unwrap();
                arr.rotate_right(m[n]);
            }
        }
    }

    // Rotations and final position
    // 0 => 1 (1) 
    // 1 => 2 (3)
    // 2 => 3 (5)
    // 3 => 4 (7)
    // 4 => 6 (2)
    // 5 => 7 (4)
    // 6 => 8 (6)
    // 7 => 9 (0)

    fn decode(&self, arr: &mut [char]) {
        use Action::*;

        let action = match self {
            RotateLeft(n)   => RotateRight(*n),
            RotateRight(n)  => RotateLeft(*n),
            RotateLetter(c) => UnrotateLetter(*c),
            ReverseRange(a, b)  => ReverseRange(*a, *b),
            SwapLetters(c1, c2) => SwapLetters(*c1, *c2),
            SwapPositions(a, b) => SwapPositions(*a, *b),
            MovePositions(a, b) => MovePositions(*b, *a),
            UnrotateLetter(c)   => RotateLetter(*c),
        };

        action.encode(arr)
    }
}

fn load(input: &str) -> Vec<Action> {
    use Action::*;

    input.lines().map(|s| {
        let v = s.split(' ').collect::<Vec<_>>();
        match v[1] {
            "left"      => RotateLeft(number(v[2])),
            "right"     => RotateRight(number(v[2])),
            "based"     => RotateLetter(letter(v[6])),
            "letter"    => SwapLetters(letter(v[2]), letter(v[5])),
            "positions" => ReverseRange(number(v[2]), number(v[4])),
            "position" if v[0] == "move" => MovePositions(number(v[2]), number(v[5])),
            "position" if v[0] == "swap" => SwapPositions(number(v[2]), number(v[5])),
            _ => panic!("Unknown action: {} {}", v[0], v[1])
        }
    })
    .collect()
}

fn part_one(actions: &[Action], password: &str) -> String {
    let mut arr = password.chars().collect::<Vec<_>>();
    actions.iter().for_each(|action| action.encode(&mut arr));

    arr.iter().collect::<String>()
}

fn part_two(actions: &[Action], password: &str) -> String {
    let mut arr = password.chars().collect::<Vec<_>>();
    actions.iter().rev().for_each(|action| action.decode(&mut arr));

    arr.iter().collect::<String>()
}

fn letter(s: &str) -> char {
    s.chars().next().unwrap()
}

fn number<T>(s: &str) -> T 
    where T: FromStr, <T as FromStr>::Err: fmt::Debug
{
    s.parse::<T>().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let actions = load(&input);
        
        let password = part_one(&actions, "abcdefgh");
        assert_eq!(password, "baecdfgh");
        
        let password = part_two(&actions, "fbgdceah");
        assert_eq!(password, "cegdahbf");
        
        let password = part_two(&actions, "baecdfgh");
        assert_eq!(password, "abcdefgh");
    }
}