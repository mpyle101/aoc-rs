
fn main() {
    use std::{fs, time::Instant};
    
    let input = fs::read_to_string("./input.txt").unwrap();
    let moves = load(&input);

    let t1 = Instant::now();
    let programs = part_one(&moves);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", programs, t2 - t1);

    let t1 = Instant::now();
    let programs = part_two(&moves);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", programs, t2 - t1);
}

enum Move {
    Spin(usize),
    Swap(usize, usize),
    Xfer(char, char),
}

impl Move {
    fn dance(&self, programs: &mut [char;16]) {
        use Move::*;

        match self {
            Spin(n)    => programs.rotate_right(*n),
            Swap(a, b) => programs.swap(*a, *b),
            Xfer(a, b) => {
                let i = programs.iter().position(|c| *c == *a).unwrap();
                let j = programs.iter().position(|c| *c == *b).unwrap();
                programs.swap(i, j);
            }
        }
    }
}

fn load(input: &str) -> Vec<Move> {
    use Move::*;

    input.split(',').map(|s|
        match s.chars().next().unwrap() {
            's' => { 
                let n = s[1..].parse::<usize>().unwrap();
                Spin(n % 16)
            },
            'x' => {
                let mut it = s[1..].split('/');
                let a = it.next().unwrap().parse::<usize>().unwrap();
                let b = it.next().unwrap().parse::<usize>().unwrap();
                Swap(a, b)
            },
            'p' => {
                let a = s.chars().nth(1).unwrap();
                let b = s.chars().nth(3).unwrap();
                Xfer(a, b)
            },
            _ => panic!("Unknown dance move: {}", s)
        }
    )
    .collect()
}

fn part_one(moves: &[Move]) -> String {
    // Setup programs a-p
    let mut programs = ['a';16];
    (0..16).for_each(|i| programs[i] = (programs[i] as u8 + i as u8) as char);

    moves.iter().for_each(|m| m.dance(&mut programs));

    programs.iter().collect()
}

fn part_two(moves: &[Move]) -> String {
    // Setup programs a-p
    let mut programs = ['a';16];
    (0..16).for_each(|i| programs[i] = (programs[i] as u8 + i as u8) as char);

    let mut cycle = 1;
    let mut dancers = programs;
    moves.iter().for_each(|m| m.dance(&mut dancers));

    // Find the number of iterations producing a cycle.
    while dancers != programs {
        moves.iter().for_each(|m| m.dance(&mut dancers));
        cycle += 1;
    };

    (0..1_000_000_000 % cycle).for_each(|_|
        moves.iter().for_each(|m| m.dance(&mut dancers))
    );
    
    dancers.iter().collect()
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let moves = load(&input);
    
        let programs = part_one(&moves);
        assert_eq!(programs, "bijankplfgmeodhc");
    
        let programs = part_two(&moves);
        assert_eq!(programs, "bpjahknliomefdgc");
    }
}
