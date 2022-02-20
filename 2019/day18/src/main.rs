use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

fn main() {
    use std::time::Instant;

    let map = load(include_str!("./samples/sample5.txt"));
    let t1 = Instant::now();
    let steps = part_one(&map);
    let t2 = Instant::now();
    println!("Part 1: {steps} {:?}", t2 - t1);
}

fn part_one(map: &Map) -> u32 {
    let state = State::new(map);
    let mut heap = BinaryHeap::from([state]);
    let mut cache: Cache = HashMap::new();

    while let Some(st) = heap.pop() {
        if st.keys_left.is_empty() {
            return st.steps
        } else {
            // Get the shortest paths from the current position to any
            // keys we don't have. Filter out the blocked ones, create a
            // new state incorporating the new segment and put it on the
            // heap. The heap garuntees we'll always get the shortest path
            // with the most keys to work on next.
            let robot = st.robot;
            for tile in st.keys_left.values() {
                let pts = if robot < *tile { (robot, *tile) } else { (*tile, robot) }; 
                let cache_key = (st.found, pts);
                if let Some((steps, keys)) = cache.get(&cache_key) {
                    heap.push(st.extend(tile, *steps, *keys))
                } else if let Some(path) = bfs(tile, &st, map) {
                    let keys = path.iter()
                        .skip(1)
                        .filter_map(|t| map.keys.get(t))
                        .filter_map(|c| st.needs(c))
                        .fold(0u32, |n, k| n | k);
                    let steps = path.len() as u32 - 1;
                    cache.insert(cache_key, (steps, keys));
                    heap.push(st.extend(tile, steps, keys))
                }
            }
        }
    }

    0
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
        &st.robot,
        |p| open_tiles(p, &map.tiles, &map.doors, st.found),
        |p| p == goal
    )
}

const DELTA: [Tile;4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

fn open_tiles((x, y): &Tile, tiles: &Tiles, doors: &Doors, keys: u32) -> Vec<Tile> {
    let found = |c: &char| keys & 1 << *c as u8 - b'a' > 0;

    DELTA.iter()
        .map(|(dx, dy)| (x + dx, y + dy))
        .filter(|p| 
            tiles.contains(&p) && doors.get(&p).map_or(true, found)
        )
        .collect()
}

type Tile = (i32, i32);
type Keys = HashMap<Tile, char>;
type Tiles = HashSet<Tile>;
type Doors = HashMap<Tile, char>;
type Cache = HashMap<(u32, (Tile, Tile)), (u32, u32)>;

struct Map {
    keys: Keys,
    doors: Doors,
    tiles: Tiles,
    robot: Tile,
}

#[derive(Clone, Debug, Eq)]
struct State {
    steps: u32,
    found: u32,
    robot: Tile,
    keys_left: HashMap<u32, Tile>,
}

impl State {
    fn new(map: &Map) -> State {
        State { 
            steps: 0,
            found: 0,
            robot: map.robot,
            keys_left: map.keys.iter()
                .map(|(k, c)| (0u32 | 1 << *c as u8 - b'a', *k))
                .collect(),

        }
    }

    fn needs(&self, c: &char) -> Option<u32> {
        let key = 1 << *c as u8 - b'a';
        (self.found & key == 0).then(|| key)
    }

    fn extend(&self, robot: &Tile, steps: u32, keys: u32) -> State {
        let mut st = self.clone();
        st.robot = *robot;
        st.steps += steps;
        st.found |= keys;

        let mut keys = keys;
        while keys > 0 {
            let i = keys.trailing_zeros();
            keys ^= 1 << i;
            st.keys_left.remove(&(1 << i));
        }

        st
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap
        if self.steps == other.steps {
            other.keys_left.len().cmp(&self.keys_left.len())
        } else {
            other.steps.cmp(&self.steps)
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
        self.steps == other.steps &&
        self.keys_left.len() == other.keys_left.len()
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