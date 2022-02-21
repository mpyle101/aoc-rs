use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::collections::{BinaryHeap, HashMap, HashSet};

type Tile = (i32, i32);
type Keys = HashMap<Tile, u32>;
type Paths = HashMap<u32, (u32, u32)>;
type Tiles = HashSet<Tile>;
type Doors = HashMap<Tile, u32>;
type Cache = HashMap<State, u32>;

fn main() {
    use std::time::Instant;

    let map = load(include_str!("./input/part1.txt"));
    let t1 = Instant::now();
    let steps = solver(&map);
    let t2 = Instant::now();
    println!("Part 1: {steps} ({:?})", t2 - t1);

    let map = load(include_str!("./input/part2.txt"));
    let t1 = Instant::now();
    let steps = solver(&map);
    let t2 = Instant::now();
    println!("Part 2: {steps} ({:?})", t2 - t1);
}

fn load(input: &str) -> Map {
    let mut keys  = Keys::new();
    let mut doors = Doors::new();
    let mut tiles = Tiles::new();
    let mut robots = Keys::new();

    let mut robot = 28;
    for (y, s) in input.lines().enumerate() {
        for (x, c) in s.chars().enumerate() {
            let tile = (x as i32, y as i32);
            match c {
                '.' => { tiles.insert(tile); },
                'a'..='z' => {
                    tiles.insert(tile);
                    keys.insert(tile, 1 << c as u8 - b'a');
                },
                'A'..='Z' => {
                    let c = c.to_ascii_lowercase();
                    tiles.insert(tile);
                    doors.insert(tile, 1 << c as u8 - b'a');
                },
                '@' => {
                    tiles.insert(tile);
                    robots.insert(tile, 1 << robot);
                    robot += 1;
                },
                _ => { /* ignore walls */ }
            }
        }
    }

    Map { keys, doors, tiles, robots }
}

fn solver(map: &Map) -> u32 {
    let paths = calc_paths(map);
    let mut heap = init_heap(map);
    let mut cache = Cache::new();

    while let Some(st) = heap.pop() {
        if st.keys == 0 {
            return st.steps
        } else if cache.get(&st).is_none() {
            cache.insert(st.clone(), st.steps);
            for state in update(&st, &paths) { heap.push(state) }
        }
    }

    0
}

fn update(st: &State, paths: &Paths) -> Vec<State> {
    // Get the shortest paths from the current position to any
    // keys we don't have. Filter out the blocked ones, create a
    // new state incorporating the new segment.
    let mut keys = st.keys;
    let mut states = vec![];

    while keys > 0 {
        let key = 1 << keys.trailing_zeros();

        for i in 0..st.count {
            let route = st.robots[i] | key;
            if let Some((steps, doors)) = paths.get(&route) {
                if st.found & doors == *doors {
                    states.push(st.update(i, key, *steps))
                }
            }    
        }

        keys ^= key;
    }

    states
}

const DELTA: [Tile;4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

fn calc_paths(map: &Map) -> HashMap<u32, (u32, u32)> {
    use itertools::Itertools;
    use pathfinding::prelude::bfs;

    let open = |(x, y): &Tile| DELTA.iter()
        .map(|(dx, dy)| (x + dx, y + dy))
        .filter(|p| map.tiles.contains(p))
        .collect::<Vec<_>>();

    // Calculate the shortest path between any two keys along
    // with the number of steps it takes and the keys needed
    // including the starting locations of the robots.
    map.keys.iter().chain(map.robots.iter())
        .combinations(2)
        .filter_map(|v| {
            bfs(v[0].0, |p| open(p), |p| p == v[1].0)
                .map(|path| {
                    let doors = find_doors(&path, map);
                    let steps = path.len() as u32 - 1;
                    let keys  = v.iter().fold(0u32, |n, (_, &k)| n | k);
                    (keys, (steps, doors))
                })
        })
        .collect()
}

fn init_heap(map: &Map) -> BinaryHeap<State> {
    let keys = map.keys.values().fold(0u32, |n, k| n | k);
    let mut robots = [0;4];
    map.robots.values().enumerate().for_each(|(i, k)| robots[i] = *k);

    let state = State { 
        keys,
        robots,
        found: 0,
        steps: 0,
        count: map.robots.len(),
    };

    BinaryHeap::from([state])
}

fn find_doors(path: &[(i32, i32)], map: &Map) -> u32 {
    path.iter()
        .filter_map(|p| map.doors.get(p))
        .fold(0u32, |n, k| n | k)
}

#[derive(Debug)]
struct Map {
    keys: Keys,
    doors: Doors,
    tiles: Tiles,
    robots: Keys,
}

#[derive(Clone, Debug, Eq)]
struct State {
    keys:  u32,         // bits representing keys left to find
    found: u32,         // bits representing found keys
    steps: u32,         // number of steps take so far
    count: usize,       // number of robots
    robots: [u32;4],    // bit representing location as current key
}

impl State {
    fn update(&self, robot: usize, key: u32, steps: u32) -> State {
        let mut robots = self.robots;
        robots[robot] = key;

        State {
            robots,
            keys:  self.keys & !key,
            found: self.found | key,
            steps: self.steps + steps,
            count: self.count,
        }
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.found.hash(state);
        self.robots.hash(state);
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
        self.robots == other.robots
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let map = load(include_str!("./input/part1.txt"));
        let steps = solver(&map);
        assert_eq!(steps, 5450);

        let map = load(include_str!("./input/part2.txt"));
        let steps = solver(&map);
        assert_eq!(steps, 2020);
    }

    #[test]
    fn sample1() {
        let map = load(include_str!("./samples/sample1.txt"));
        let steps = solver(&map);
        assert_eq!(steps, 8);
    }

    #[test]
    fn sample2() {
        let map = load(include_str!("./samples/sample2.txt"));
        let steps = solver(&map);
        assert_eq!(steps, 86);
    }

    #[test]
    fn sample3() {
        let map = load(include_str!("./samples/sample3.txt"));
        let steps = solver(&map);
        assert_eq!(steps, 132);
    }

    #[test]
    fn sample4() {
        let map = load(include_str!("./samples/sample4.txt"));
        let steps = solver(&map);
        assert_eq!(steps, 136);
    }

    #[test]
    fn sample5() {
        let map = load(include_str!("./samples/sample5.txt"));
        let steps = solver(&map);
        assert_eq!(steps, 81);
    }
}