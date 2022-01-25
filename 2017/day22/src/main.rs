use std::collections::HashSet;

fn main() {
    use std::{fs, time::Instant};
    
    let input = fs::read_to_string("./input.txt").unwrap();
    let nodes = load(&input);

    let t1 = Instant::now();
    let infections = part_one(&nodes);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", infections, t2 - t1);

    let t1 = Instant::now();
    let infections = part_two(&nodes);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", infections, t2 - t1);
}

type Nodes = HashSet<(i32, i32)>;

#[derive(Clone, Copy)]
enum State {
    Flagged,
    Infected,
    Weakened,
}

fn load(input: &str) -> Nodes {
    input.lines().enumerate().map(|(y, s)| {
        s.chars()
            .enumerate()
            .filter_map(move |(x, c)| 
                if c == '#' { 
                    Some((x as i32, y as i32))
                } else { 
                    None 
                })
    })
    .flatten()
    .collect()
}

fn part_one(nodes: &Nodes) -> usize {
    // North, East, South, West
    // Left  = if facing == 0 { 3 } else { facing - 1 }
    // Right = (facing + 1) % 4
    let dir = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut facing   = 0;            // north
    let mut carrier  = (12, 12);     // middle
    let mut infected = nodes.clone();

    let mut infections = 0;
    (0..10_000).for_each(|_| {
        if infected.contains(&carrier) {
            // Clean and turn right
            infected.remove(&carrier);
            facing = (facing + 1) % 4;
        } else {
            // Infect and turn left
            infected.insert(carrier);
            facing = if facing == 0 { 3 } else { facing - 1 };
            infections += 1
        }
        carrier = (carrier.0 + dir[facing].0, carrier.1 + dir[facing].1)
    });

    infections
}

fn part_two(nodes: &Nodes) -> usize {
    use std::collections::HashMap;
    use State::*;

    // North, East, South, West
    // Left  = if facing == 0 { 3 } else { facing - 1 }
    // Right = (facing + 1) % 4
    let dir = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut facing   = 0;            // north
    let mut carrier  = (12, 12);     // middle
    let mut infected = nodes.iter()
        .map(|pt| (*pt, Infected))
        .collect::<HashMap<_, _>>();

    let mut infections = 0;
    (0..10_000_000).for_each(|_| {
        if let Some(&st) = infected.get(&carrier) {
            match st {
                Flagged => {
                    // Clean and reverse direction
                    facing = (facing + 2) % 4;
                    infected.remove(&carrier)
                },
                Weakened => {
                    // Infect and keep going
                    infections += 1;
                    infected.insert(carrier, Infected)
                },
                Infected => {
                    // Flag for cleaning and turn right
                    facing = (facing + 1) % 4;
                    infected.insert(carrier, Flagged)
                },
            };
        } else {
            // Weaken and turn left
            infected.insert(carrier, Weakened);
            facing = if facing == 0 { 3 } else { facing - 1 };
        }
        carrier = (carrier.0 + dir[facing].0, carrier.1 + dir[facing].1)
    });

    infections
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let nodes = load(&input);
    
        let infections = part_one(&nodes);
        assert_eq!(infections, 5575);
    
        let infections = part_two(&nodes);
        assert_eq!(infections, 2511991);
    }
}
