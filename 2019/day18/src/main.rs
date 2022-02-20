use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::collections::{BinaryHeap, HashMap, HashSet};

fn main() {
    use std::time::Instant;

    let map = load(include_str!("./input/part1.txt"));
    let t1 = Instant::now();
    let steps = part_one(&map);
    let t2 = Instant::now();
    println!("Part 1: {steps} ({:?})", t2 - t1);
}

fn part_one(map: &Map) -> u32 {
    let state = State::new(map);
    let mut heap = BinaryHeap::from([state]);
    let mut cache: Cache = HashMap::new();

    while let Some(st) = heap.pop() {
        if st.keys_left.is_empty() {
            return st.steps
        } else if cache.get(&st).is_none() {
            cache.insert(st.clone(), st.steps);
            update(&st, map).iter()
                .for_each(|st| heap.push(st.clone()))
        }
    }

    0
}

fn update(st: &State, map: &Map) -> Vec<State> {
    // Get the shortest paths from the current position to any
    // keys we don't have. Filter out the blocked ones, create a
    // new state incorporating the new segment.
    st.keys_left.values()
        .filter_map(|tile| bfs(tile, st, map))
        .map(|path| {
            let keys = path.iter().skip(1)
                .enumerate()
                .filter_map(|(i, t)| map.keys.get(t).map(|c| (t, i + 1, c)))
                .filter_map(|(t, s, c)| st.needs(c).map(|k| (t, s, k)))
                .collect::<Vec<_>>();
            let (tile, steps, key) = keys[0];
            st.update(tile, steps as u32, key)
        })
        .collect()
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
type Cache = HashMap<State, u32>;

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

    fn update(&self, robot: &Tile, steps: u32, key: u32) -> State {
        let mut st = self.clone();
        st.robot = *robot;
        st.steps += steps;
        st.found |= key;
        st.keys_left.remove(&key);

        st
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.found.hash(state);
        self.robot.hash(state);
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap
        if self.steps == other.steps {
            self.found.count_ones().cmp(&other.found.count_ones())
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
        self.found == other.found &&
        self.robot == other.robot
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