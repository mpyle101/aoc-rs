use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::collections::{BinaryHeap, HashMap, HashSet};

type Tile = (i32, i32);
type Keys = HashMap<Tile, char>;
type Paths = HashMap<u32, (u32, u32)>;
type Tiles = HashSet<Tile>;
type Doors = HashMap<Tile, char>;
type Cache = HashMap<State, u32>;

fn main() {
    use std::time::Instant;

    let map = load(include_str!("./input/part1.txt"));
    let t1 = Instant::now();
    let steps = part_one(&map);
    let t2 = Instant::now();
    println!("Part 1: {steps} ({:?})", t2 - t1);
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

fn part_one(map: &Map) -> u32 {
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

        let route = st.robot | key;
        if let Some((steps, doors)) = paths.get(&route) {
            if st.found & doors == *doors {
                states.push(st.update(key, *steps))
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
    // with the number of steps it takes and the keys needed.
    map.keys.iter()
        .combinations(2)
        .filter_map(|v| {
            bfs(v[0].0, |p| open(p), |p| p == v[1].0)
                .map(|path| {
                    let doors = find_doors(&path, map);
                    let steps = path.len() as u32 - 1;
                    let keys  = v.iter().fold(0u32, |n, (_, &c)| n | 1 << c as u8 - b'a');
                    (keys, (steps, doors))
                })
        })
        .collect()
}

fn init_heap(map: &Map) -> BinaryHeap<State> {
    use pathfinding::prelude::bfs;

    let open = |(x, y): &Tile| DELTA.iter()
        .map(|(dx, dy)| (x + dx, y + dy))
        .filter(|p| map.tiles.contains(p))
        .collect::<Vec<_>>();
        
    // Initialize a binary heap with states representing the robot
    // moving from it's initial position to keys with no intermediate
    // doors.
    let keys = map.keys.values().fold(0u32, |n, c| n | 1 << *c as u8 - b'a');
    map.keys.iter()
        .filter_map(|(goal, c)| 
            bfs(&map.robot, |p| open(p), |p| p == goal).map(|path| (path, c))
        )
        .filter_map(|(path, c)| {
            let doors = find_doors(&path, map);
            let steps = path.len() as u32 - 1;
            (doors == 0).then(|| (steps, 1 << *c as u8 - b'a'))
        })
        .map(|(steps, k)| State { steps, found: k, robot: k, keys: keys & !k } )
        .collect()
}

fn find_doors(path: &[(i32, i32)], map: &Map) -> u32 {
    path.iter()
        .filter_map(|p| map.doors.get(p))
        .fold(0u32, |n, c| n | 1 << *c as u8 - b'a')
}

struct Map {
    keys: Keys,
    doors: Doors,
    tiles: Tiles,
    robot: Tile,
}

#[derive(Clone, Debug, Eq)]
struct State {
    keys:  u32,     // bits representing keys left to find
    found: u32,     // bits representing found keys
    robot: u32,     // bit representing location as current key
    steps: u32,     // number of steps take so far
}

impl State {
    fn update(&self, key: u32, steps: u32) -> State {
        State { 
            keys:  self.keys & !key,
            found: self.found | key,
            robot: key,
            steps: self.steps + steps,
        }
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