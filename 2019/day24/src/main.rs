use std::collections::{BTreeSet, HashSet};
use std::hash::{Hash, Hasher};

type Tile = (i32, i32);
type Pt = (i32, i32, i32);

fn main() {
    let map = load(include_str!("./eris.txt"));

    let bio = part_one(&map);
    println!("Part 1: {}", bio);

    let bugs = part_two(&map);
    println!("Part 2: {}", bugs);
}

fn part_one(eris: &MapState) -> u64 {
    let mut maps = HashSet::new();
    let mut state = eris.clone();
    
    maps.insert(eris.clone());
    let tiles = loop {
        state = cycle(&state);
        if maps.contains(&state) {
            break state
        } else {
            maps.insert(state.clone());
        }
    };

    let a = if tiles.contains(&(0, 0)) { 1 } else { 0 };
    tiles.0.iter().skip(1).fold(a, |n, t| n + (2 << ((t.0 * 5) + t.1 - 1)))
}

fn part_two(eris: &MapState) -> usize {
    let bugs: HashSet<_> = eris.0.iter().map(|t| (t.0, t.1, 0)).collect();
    (0..200).fold(bugs, 
        |infested, _| cycle_with_recursion(&infested)
    ).len()
}

fn load(input: &str) -> MapState {
    MapState(input.lines().enumerate().flat_map(|(r, s)|
        s.as_bytes().iter().enumerate().filter_map(move |(c, b)|
            match b {
                b'#' => Some((r as i32, c as i32)),
                _ => None
            }
        )
    ).collect())
}

#[allow(dead_code)]
fn draw(state: &MapState) {
    (0..5).for_each(|row| {
        (0..5).for_each(|col| {
            let c = if state.contains(&(row, col)) { '#' } else { '.' };
            print!("{}", c);
        });
        println!();
    });
    println!();
}

#[derive(Clone, Eq, PartialEq)]
struct MapState(BTreeSet<Tile>);
impl MapState {
    fn contains(&self, tile: &Tile) -> bool {
        self.0.contains(tile)
    }
}

impl Hash for MapState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.iter().for_each(|t| t.hash(state))
    }
}

const DELTAS: [Tile;4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

fn cycle(bugs: &MapState) -> MapState {
    use std::collections::VecDeque;

    let mut infested = BTreeSet::new();
    let mut queue: VecDeque<_> = bugs.0.iter().cloned().collect();
    while let Some(tile) = queue.pop_back() {
        let nearby: Vec<_> = DELTAS.iter().map(|d|
            (tile.0 + d.0, tile.1 + d.1)
        ).collect();

        let count = nearby.iter().filter(|&p| bugs.contains(p)).count();
        if bugs.contains(&tile) {
            nearby.iter().for_each(|t| 
                if !bugs.contains(t) && in_bounds(t) { queue.push_back(*t)}
            );
            if count == 1 {
                infested.insert(tile);
            }
        } else if count == 1 || count == 2 {
            infested.insert(tile);
        }
    }
    
    MapState(infested)
}

fn in_bounds(tile: &Tile) -> bool {
    tile.0 >=0 && tile.0 < 5 && tile.1 >= 0 && tile.1 < 5
}

fn cycle_with_recursion(bugs: &HashSet<Pt>) -> HashSet<Pt> {
    use std::collections::VecDeque;

    let mut infested = HashSet::new();
    let mut queue: VecDeque<_> = bugs.iter().cloned().collect();
    while let Some(pt) = queue.pop_back() {
        let nearby = nearby_tiles(pt);
        let count = nearby.iter().filter(|&p| bugs.contains(p)).count();
        if bugs.contains(&pt) {
            nearby.iter().for_each(|t| if !bugs.contains(t) { queue.push_back(*t)});
            if count == 1 {
                infested.insert(pt);
            }
        } else if count == 1 || count == 2 {
            infested.insert(pt);
        }
    }
    
    infested
}

fn nearby_tiles(pt: Pt) -> Vec<Pt> {
    match pt {
        (0, 0, z) => vec![(1, 2, z-1), (2, 1, z-1), (0, 1, z), (1, 0, z)],
        (0, 4, z) => vec![(1, 2, z-1), (2, 3, z-1), (0, 3, z), (1, 4, z)],
        (4, 0, z) => vec![(3, 2, z-1), (2, 1, z-1), (3, 0, z), (4, 1, z)],
        (4, 4, z) => vec![(3, 2, z-1), (2, 3, z-1), (4, 3, z), (3, 4, z)],
        (0, c, z) => vec![(1, 2, z-1), (0, c-1, z), (1, c, z), (0, c+1, z)],
        (4, c, z) => vec![(3, 2, z-1), (4, c-1, z), (3, c, z), (4, c+1, z)],
        (r, 0, z) => vec![(2, 1, z-1), (r-1, 0, z), (r, 1, z), (r+1, 0, z)],
        (r, 4, z) => vec![(2, 3, z-1), (r-1, 4, z), (r, 3, z), (r+1, 4, z)],
        (1, 2, z) => {
            let mut v: Vec<_> = (0..5).map(|c| (0, c, z+1)).collect();
            v.extend_from_slice(&[(1, 1, z), (0, 2, z), (1, 3, z)]);
            v
        },
        (2, 1, z) => {
            let mut v: Vec<_> = (0..5).map(|r| (r, 0, z+1)).collect();
            v.extend_from_slice(&[(1, 1, z), (2, 0, z), (3, 1, z)]);
            v
        },
        (2, 3, z) => {
            let mut v: Vec<_> = (0..5).map(|r| (r, 4, z+1)).collect();
            v.extend_from_slice(&[(1, 3, z), (2, 4, z), (3, 3, z)]);
            v
        },
        (3, 2, z) => {
            let mut v: Vec<_> = (0..5).map(|c| (4, c, z+1)).collect();
            v.extend_from_slice(&[(3, 1, z), (4, 2, z), (3, 3, z)]);
            v
        },
        (r, c, z) => vec![(r-1, c, z), (r+1, c, z), (r, c-1, z), (r, c+1, z)],
   }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let map = load(include_str!("./eris.txt"));

    let bio = part_one(&map);
    assert_eq!(bio, 10282017);

    let bugs = part_two(&map);
    assert_eq!(bugs, 2065);
  }
}