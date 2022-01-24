use std::collections::HashMap;

fn main() {
    use std::{fs, time::Instant};
    
    let input = fs::read_to_string("./input.txt").unwrap();
    let path  = load(&input);

    let t1 = Instant::now();
    let letters = part_one(&path);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", letters, t2 - t1);

    let t1 = Instant::now();
    let steps = part_two(&path);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", steps, t2 - t1);
}

enum Segment {
    Line,
    Corner,
    Letter(char),
}

type Segments = HashMap<(i32, i32), Segment>;

fn load(input: &str) -> Segments {
    use Segment::*;

    input.lines().enumerate()
        .map(|(y, s)| {
            s.chars().enumerate().filter_map(move |(x, c)| {
                let pt = (x as i32, y as i32);
                match c {
                    ' ' => None,
                    '|' => Some((pt, Line)),
                    '-' => Some((pt, Line)),
                    '+' => Some((pt, Corner)),
                     c  => Some((pt, Letter(c)))
                }
            })
        })
        .flatten()
        .collect()
}

fn part_one(path: &Segments) -> String {
    use Segment::*;

    // Find the starting point: (x, 0)
    let (mut x, mut y) = *path.keys().find(|key| key.1 == 0).unwrap();

    // north, south, west, east;
    let dxdy = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    let mut dir = 1;

    let mut letters = vec![];

    loop {
        x += dxdy[dir].0;
        y += dxdy[dir].1;

        if let Some(seg) = path.get(&(x, y)) {
            match seg {
                Line => { /* keep going */},
                Corner => {
                    if dir > 1 {
                        // turn north / south
                        let south = (x + dxdy[1].0, y + dxdy[1].1);
                        dir = path.contains_key(&south) as usize;
                    } else {
                        let east = (x + dxdy[3].0, y + dxdy[3].1);
                        dir = 2 + path.contains_key(&east) as usize;
                    }
                },
                Letter(c) => letters.push(*c),
            }
        } else {
            break
        }
    }

    letters.iter().collect::<String>()
}

fn part_two(path: &Segments) -> i32 {
    use Segment::*;

    // Find the starting point: (x, 0)
    let (mut x, mut y) = *path.keys().find(|key| key.1 == 0).unwrap();

    // north, south, west, east;
    let dxdy = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    let mut dir = 1;

    let mut steps = 1;

    loop {
        x += dxdy[dir].0;
        y += dxdy[dir].1;

        if let Some(seg) = path.get(&(x, y)) {
            steps += 1;
            match seg {
                Corner => {
                    if dir > 1 {
                        // turn north / south
                        let south = (x + dxdy[1].0, y + dxdy[1].1);
                        dir = path.contains_key(&south) as usize;
                    } else {
                        let east = (x + dxdy[3].0, y + dxdy[3].1);
                        dir = 2 + path.contains_key(&east) as usize;
                    }
                },
                _ => { /* ignore lines & letters */ },
            }
        } else {
            break
        }
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let path  = load(&input);
    
        let letters = part_one(&path);
        assert_eq!(letters, "PBAZYFMHT");
    
        let steps = part_two(&path);
        assert_eq!(steps, 16072);
    }
}
