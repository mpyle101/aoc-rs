use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

fn main() {
    use std::time::Instant;

    let map = load(include_str!("./samples/sample4.txt"));
    let t1 = Instant::now();
    let steps = part_one(&map);
    let t2 = Instant::now();
    println!("Part 1: {steps} {:?}", t2 - t1);
}

fn part_one(map: &Map) -> usize {
    let state = State::new(map.robot);
    let mut heap = BinaryHeap::from([state]);

    loop {
        if let Some(mut st) = heap.pop() {
            map.keys.get(&st.robot()).map(|c| st.keys.push(*c));
            if st.keys.len() == map.keys.len() {
                return st.path.len() - 1
            } else {
                // Get the shortest paths from the current position to any
                // keys we don't have. Filter out the blocked ones, create a
                // new state incorporating the new segment and put it on the
                // heap. The heap garuntees we'll always get the shortest path
                // with the most keys to work on next.
                map.keys.iter()
                    .filter(|(_, v)| !st.keys.contains(v))
                    .filter_map(|(k, _)| bfs(k, &st, map))
                    .map(|v| st.extend(&v))
                    .for_each(|st| heap.push(st))
            }
        } else {
            return 0
        }
    }
}

fn load(input: &str) -> Map {
    let mut robot = (0, 0);
    let mut keys  = Keys::new();
    let mut doors = Doors::new();
    let mut tiles = Tiles::new();

    for (y, s) in input.lines().enumerate() {
        for (x, c) in s.chars().enumerate() {
            let tile = (x as i32, y as i32);
            match c {
                '.' => { tiles.insert(tile); },
                'a'..='z' => {
                    tiles.insert(tile);
                    keys.insert(tile, c);
                },
                'A'..='Z' => {
                    tiles.insert(tile);
                    doors.insert(tile, c.to_ascii_lowercase());
                },
                '@' => {
                    tiles.insert(tile);
                    robot = tile;
                },
                _ => { /* ignore walls */ }
            }
        }
    }

    Map { keys, doors, tiles, robot }
}

fn bfs(goal: &Tile, st: &State, map: &Map) -> Option<Vec<Tile>> {
    pathfinding::prelude::bfs(
        &st.robot(),
        |p| open_tiles(p, &map.tiles, &map.doors, &st.keys),
        |p| p == goal
    )
}

const DELTA: [Tile;4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

fn open_tiles((x, y): &Tile, tiles: &Tiles, doors: &Doors, keys: &[char]) -> Vec<Tile> {
    DELTA.iter()
        .map(|(dx, dy)| (x + dx, y + dy))
        .filter(|p| 
            tiles.contains(&p) && doors.get(&p).map_or(true, |v| keys.contains(v))
        )
        .collect()
}

type Tile = (i32, i32);
type Keys = HashMap<Tile, char>;
type Tiles = HashSet<Tile>;
type Doors = HashMap<Tile, char>;

struct Map {
    keys: Keys,
    doors: Doors,
    tiles: Tiles,
    robot: Tile,
}

#[derive(Clone, Debug, Eq)]
struct State {
    path: Vec<Tile>,
    keys: Vec<char>,
}

impl State {
    fn new(robot: Tile) -> State {
        State { path: vec![robot], keys: vec![] }
    }

    fn robot(&self) -> Tile {
        *self.path.last().unwrap()
    }

    fn extend(&self, path: &[Tile]) -> State {
        let mut st = self.clone();
        st.path.extend(&path[1..]);
        st
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap
        if self.path.len() == other.path.len() {
            self.keys.len().cmp(&other.keys.len())
        } else {
            other.path.len().cmp(&self.path.len())
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.path.len() == other.path.len() &&
        self.keys.len() == other.keys.len()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let map = load(include_str!("./input/part1.txt"));
        let steps = part_one(&map);
        assert_eq!(steps, 5450);

        // let map = load(include_str!("./part2.txt"));
        // let steps = part_two(&map);
        // assert_eq!(steps, 2020);
    }

    #[test]
    fn sample1() {
        let map = load(include_str!("./samples/sample1.txt"));
        let steps = part_one(&map);
        assert_eq!(steps, 8);
    }

    #[test]
    fn sample2() {
        let map = load(include_str!("./samples/sample2.txt"));
        let steps = part_one(&map);
        assert_eq!(steps, 86);
    }

    #[test]
    fn sample3() {
        let map = load(include_str!("./samples/sample3.txt"));
        let steps = part_one(&map);
        assert_eq!(steps, 132);
    }

    #[test]
    fn sample4() {
        let map = load(include_str!("./samples/sample4.txt"));
        let steps = part_one(&map);
        assert_eq!(steps, 136);
    }

    #[test]
    fn sample5() {
        let map = load(include_str!("./samples/sample5.txt"));
        let steps = part_one(&map);
        assert_eq!(steps, 81);
    }
}