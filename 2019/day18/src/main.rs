// part 1 5450
// part 2 2020

use itertools::Itertools;
use pathfinding::prelude::{absdiff, astar};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt;

use std::time::Instant;

fn main() {
  let mut map = Map::load(include_str!("./part1.txt"));
  let t1 = Instant::now();
  let steps = part_one(&mut map);
  let t2 = Instant::now();
  println!("Part 1: {} {:?}", steps, t2 - t1);

  let mut map = Map::load(include_str!("./part2.txt"));
  let t1 = Instant::now();
  let steps = part_two(&mut map);
  let t2 = Instant::now();
  println!("Part 2: {} {:?}", steps, t2 - t1);
}

fn part_one(map: &mut Map) -> i32 {
  let cache = build_cache(map);
  let mut heap = build_heap(map, &[(0, 0)]);
  let all_keys = (0..26).fold(0, |acc, key| acc | (1 << key));

  let mut visited = HashSet::new();
  while let Some(Reverse((steps, key, found))) = heap.pop() {
    if found == all_keys {
      return steps
    }
    if visited.insert((key, found)) {
      (0..26).filter(|k| (found & (1 << k)) == 0)
        .map(|k| if k > key { (k, (key, k)) } else { (k, (k, key)) })
        .map(|(k, t)| (k, cache.get(&t).unwrap()))
        .filter(|(_, (_, doors))| (doors | found) == found)
        .for_each(|(k, (p, _))| {
          heap.push(Reverse((steps + p, k, found | (1 << k))))
        });
    }
  }

  0
}

fn part_two(map:&mut Map) -> i32 {
  let cache = build_cache(map);
  let offsets  = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
  let mut heap = build_heap(map, &offsets);
  let all_keys = (0..26).fold(0, |acc, key| acc | (1 << key));

  let mut visited = HashSet::new();
  while let Some(Reverse((steps, key, found))) = heap.pop() {
    if found == all_keys {
      return steps
    }
    if visited.insert((key, found)) {
      (0..26).filter(|k| (found & (1 << k)) == 0)
        .map(|k| if k > key { (k, (key, k)) } else { (k, (k, key)) })
        .map(|(k, t)| (k, cache.get(&t)))
        .filter(|(_, opt)| opt.is_some())
        .map(|(k, opt)| (k, opt.unwrap()))
        .filter(|(_, (_, doors))| (doors | found) == found)
        .for_each(|(k, (p, _))| {
          heap.push(Reverse((steps + p, k, found | (1 << k))))
        });
    }
  }

  0
}

fn build_cache(map: &Map) -> HashMap<(u8, u8),(i32, u32)> {
  (0..26).combinations(2)
    .map(|v| (v[0], v[1], map.key_path(v[0], v[1])))
    .filter(|(_, _, opt)| opt.is_some())
    .map(|(a, b, opt)| (a, b, opt.unwrap()))
    .map(|(a, b, (path, steps))| ((a, b), (steps, map.get_doors(&path))))
    .collect()
}

fn build_heap(map: &Map, offsets: &[(i32, i32)]) -> BinaryHeap<Reverse<(i32, u8, u32)>> {
  let mut paths = HashMap::new();
  for offset in offsets {
    (0..26)
      .map(|k| (k, map.robot_path(&offset, k)))
      .filter(|(_, opt)| opt.is_some())
      .map(|(k, opt)| (k, opt.unwrap()))
      .map(|(k, (path, steps))| (k, (steps, map.get_doors(&path))))
      .filter(|(_, (_, doors))| *doors == 0)
      .for_each(|(k, path)| { paths.insert(k, path); });
  }

  println!("{:?}", paths);
  
  let mut heap = BinaryHeap::new();
  paths.iter().for_each(
    |(k, (steps, _))| heap.push(Reverse((*steps, *k, 1u32 << k)))
  );

  heap
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Pos {
  x: i32,
  y: i32,
}

impl fmt::Debug for Pos {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

struct Map {
  keys: HashMap<u8, Pos>,
  doors: HashMap<Pos, u8>,
  walls: HashSet<Pos>,
  robot: Pos,
}

impl Map {
  fn load(map: &str) -> Self {
    let mut robot = Pos { x: 0, y: 0 };
    let mut keys  = HashMap::new();
    let mut doors = HashMap::new();
    let mut walls = HashSet::new();

    map.lines()
      .enumerate()
      .for_each(|(y, l)| l.chars()
        .enumerate()
        .for_each(|(x, c)| {
          let pt = Pos { x: x as i32, y: y as i32 };
          match c {
            '.' => {},
            '#' => { walls.insert(pt); },
            '@' => robot = pt,
            'a'..='z' => { keys.insert(c as u8 - b'a', pt); },
            'A'..='Z' => { doors.insert(pt, c.to_ascii_lowercase() as u8 - b'a'); },
             _ => {}
          }
        })
      );

    Map {
      keys,
      doors,
      walls,
      robot,
    }
  }

  fn key_path(&self, from: u8, to: u8) -> Option<(Vec<Pos>, i32)> {
    let start = self.keys.get(&from)?;
    let end   = self.keys.get(&to)?;
    self.calc_path(start, end)
  }

  fn get_doors(&self, path: &[Pos]) -> u32 {
    let mut doors = 0;
    for pos in path {
      if let Some(d) = self.doors.get(pos) {
        doors |= 1 << d;
      }
    }

    doors
  }

  fn calc_path(&self, start: &Pos, end: &Pos) -> Option<(Vec<Pos>, i32)> {
    astar(
      start,
      |p| self.open_tiles(p).into_iter().map(|p| (p, 1)),
      |&Pos { x, y }| absdiff(x, end.x) + absdiff(y, end.y),
      |p| p == end
    )
  }

  fn robot_path(&self, offset: &(i32, i32), key: u8) -> Option<(Vec<Pos>, i32)> {
    let end = self.keys.get(&key)?;
    let robot = Pos { x: self.robot.x + offset.0, y: self.robot.y + offset.1 };
    self.calc_path(&robot, end)
  }

  fn open_tiles(&self, pos: &Pos) -> Vec<Pos> {
    let v = [
      Pos { x: pos.x, y: pos.y - 1 },
      Pos { x: pos.x, y: pos.y + 1 },
      Pos { x: pos.x - 1, y: pos.y },
      Pos { x: pos.x + 1, y: pos.y }
    ];

    v.iter()
      .filter(|p| self.walls.get(p).is_none())
      .copied().collect()
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let mut map = Map::load(include_str!("./part1.txt"));

    let steps = part_one(&mut map);
    assert_eq!(steps, 5450);
  }
}