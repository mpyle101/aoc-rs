use std::collections::HashSet;

fn main() {
    let map = load(include_str!("./map.txt"));

    let trees = part_one(&map);
    println!("Part1: {}", trees);

    let trees = part_two(&map);
    println!("Part2: {}", trees);
}

#[derive(Debug)]
struct Map {
    rows: usize,
    cols: usize,
    trees: HashSet<(usize, usize)>
}

impl Map {
    fn is_tree(&self, pos: &(usize, usize)) -> bool {
        self.trees.contains(&(pos.0, pos.1 % self.cols))
    }
}

fn load(map: &str) -> Map {
    let v: Vec<_> = map.lines().map(|l| l.as_bytes().iter().count()).collect();
    let rows = v.len();
    let cols = v[0];

    let trees: HashSet<_> = map.lines().enumerate()
        .flat_map(|(r, l)| {
            l.as_bytes().iter().enumerate().filter(|(_, &c)| c == b'#')
                .map(|(c, _)| (r, c) ).collect::<Vec<(usize, usize)>>()
        }).collect();

    Map { rows, cols, trees }
}

fn part_one(map: &Map) -> u32 {
    trees(map, &(1, 3))
}

fn part_two(map: &Map) -> u32 {
    [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)].iter()
        .fold(1, |acc, slope| acc * trees(map, slope))
}

fn trees(map: &Map, slope: &(usize, usize)) -> u32 {
    let mut pos = (0, 0);
    (0..map.rows).fold(0, |acc, _| {
        pos = (pos.0 + slope.0, pos.1 + slope.1);
        acc + map.is_tree(&pos) as u32 
    })
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let map = load(include_str!("./map.txt"));

    let trees = part_one(&map);
    assert_eq!(trees, 259);

    let trees = part_two(&map);
    assert_eq!(trees, 2224913600);
  }
}