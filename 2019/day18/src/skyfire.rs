use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::read_to_string;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = {
        let input_file = Path::new(env!("CARGO_MANIFEST_DIR")).join("src").join("map.txt");
        read_to_string(&input_file)
            .map_err(|_| format!("Cannot find input file {}", input_file.display()))?
    };

    println!("part1: {}", part1(&input)?);
    println!("part2: {}", part2(&input)?);

    Ok(())
}

fn part1(input: &str) -> Result<u32> {
    let (start, walls, keys, doors) = parse_map(input)?;
    solve(start, &walls, &keys, &doors)
}

fn part2(input: &str) -> Result<u32> {
    let (start, mut walls, keys, doors) = parse_map(input)?;

    walls.extend(
        [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)].iter().cloned()
            .map(|(dx, dy)| (start.0 + dx, start.1 + dy))
    );

    let robot_relative_positions = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

    let mut total = 0;
    for &(dx, dy) in robot_relative_positions.iter() {
        let keys = keys.iter()
            .map(|(&k, &v)| (k, v))
            .filter(|((x,y), _)| dx * x > dx * start.0 && dy * y > dy * start.1)
            .collect::<KeyDoorMap>();
        let keys_set = keys.values().collect::<HashSet<_>>();
        let doors = doors.iter()
            .map(|(&k, &v)| (k, v))
            .filter(|((x,y), _)| dx * x > dx * start.0 && dy * y > dy * start.1)
            .filter(|(_, k)| keys_set.contains(&k))
            .collect::<KeyDoorMap>();
        
        let distance = solve((start.0 + dx, start.1 + dy), &walls, &keys, &doors)?;
        total += distance;
    }

    return Ok(total)
}

type Point = (isize, isize);
type KeyDoorMap = HashMap<Point, u32>;

fn solve(
    start: Point,
    walls: &HashSet<Point>,
    keys: &KeyDoorMap,
    doors: &KeyDoorMap
) -> Result<u32> {
    let mut key_solver = KeySolver::new(walls, keys, doors);
    let keys_cache = keys.keys().chain([start].iter())
        .map(|&point| (point, key_solver.reach_keys(point)))
        .collect::<HashMap<_, _>>();

    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::new();
    queue.push(Reverse((0, start, 0)));

    let keys_u32 = keys.values().fold(0, |acc, key| acc | key);

    while let Some(Reverse((distance, robot, coll_keys))) = queue.pop() {
        if coll_keys == keys_u32 {
            return Ok(distance)
        }
        if seen.insert((robot, coll_keys)) {
            let accessible_keys = keys_cache[&robot].iter()
                .filter(|(_, _, new_key, _)| coll_keys & new_key == 0)
                .filter(|(_, _, _, needed_doors)| needed_doors & !coll_keys == 0);

            for &(rel_distance, new_robot, new_key, _) in accessible_keys {
                queue.push(Reverse((distance + rel_distance, new_robot, coll_keys | new_key)));
            }
        }
    }

    return Err("There's no way to get all the keys".into())
}

struct KeySolver<'a> {
    walls: &'a HashSet<Point>,
    keys: &'a KeyDoorMap,
    doors: &'a KeyDoorMap,
    queue: VecDeque<(u32, Point, u32)>,
    seen: HashSet<Point>
}

impl<'a> KeySolver<'a> {
    fn new(walls: &'a HashSet<Point>, keys: &'a KeyDoorMap, doors: &'a KeyDoorMap) -> Self {
        Self { walls, keys, doors, queue: VecDeque::new(), seen: HashSet::new() }
    }
    fn reach_keys(&mut self, current_position: Point) -> Vec<(u32, Point, u32, u32)> {
        let mut vec = Vec::new();
        let Self { queue, seen, walls, keys, doors } = self;
        seen.clear();
        queue.clear();
        queue.push_back((0, current_position, 0));

        while let Some((distance, robot, doors_seen)) = queue.pop_front() {
            if seen.insert(robot) {
                if let Some(&key) = keys.get(&robot) {
                    vec.push((distance, robot, key, doors_seen));
                }
                queue.extend([(1, 0), (-1, 0), (0, 1), (0, -1)].iter().cloned()
                    .map(|(dx, dy)| (robot.0 + dx, robot.1 + dy))
                    .filter(|new_robot| !walls.contains(&new_robot))
                    .map(|new_robot| (distance + 1, new_robot, doors_seen | doors.get(&new_robot).unwrap_or(&0)))
                );
            }
        }

        return vec
    }
}

fn parse_map(input: &str) -> Result<(Point, HashSet<Point>, KeyDoorMap, KeyDoorMap)> {
    let mut start = None;
    let mut walls = HashSet::new();
    let mut keys = KeyDoorMap::new();
    let mut doors = KeyDoorMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let (x, y) = (x as isize, y as isize);
            match c {
                '.' => {},
                '#' => { walls.insert((x, y)); },
                '@' => start = Some(start.xor(Some((x, y))).ok_or_else(|| "Multiple starting points found")?),
                c if c.is_ascii_uppercase() => { doors.insert((x, y), 1 << (c.to_ascii_lowercase() as u8 - b'a')); },
                c if c.is_ascii_lowercase()=> { keys.insert((x, y), 1 << (c as u8 - b'a')); },
                _ => return Err("Found invalid char in input".into())
            }
        }
    }

    Ok((start.ok_or_else(|| "No starting point found")?, walls, keys, doors))
}
