use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");

    let black = part_one(input);
    println!("Part 1: {}", black.len());

    let tiles = part_two(&black);
    println!("Part 2: {}", tiles);
}

fn part_one(input: &str) -> HashSet<(i32, i32)> {
    input.lines()
        .fold(HashMap::new(), |mut acc, s| {
            let (tile, _) = s.chars().fold(((0, 0), '_'), |((x, y), p), c|
                (match (p, c) {
                    // Doubled coordinates
                    ('n', 'e') => (x + 1, y - 1),
                    ('s', 'e') => (x + 1, y + 1),
                    ('n', 'w') => (x - 1, y - 1),
                    ('s', 'w') => (x - 1, y + 1),
                    ( _ , 'e') => (x + 2, y),
                    ( _ , 'w') => (x - 2, y),
                             _ => (x, y)
                }, c)
            );
            *acc.entry(tile).or_insert(0) += 1;
            acc
        })
        .iter()
        .filter(|(_, &v)| v % 2 != 0)
        .map(|(p, _)| *p)
        .collect()
}

fn part_two(tiles: &HashSet<(i32, i32)>) -> usize {
    let black = (0..100).fold(tiles.clone(), |blk, _| {
        let mut white = HashSet::new();
        let mut black = HashSet::new();

        blk.iter().for_each(|tile| {
            let adjacent = NEIGHBORS.iter()
                .map(|(dx, dy)| (tile.0 + dx, tile.1 + dy))
                .fold(0, |acc, p|
                    if blk.contains(&p) { acc + 1 } else { white.insert(p); acc }
                );
            if adjacent == 1 || adjacent == 2 {
                black.insert(*tile);
            }
        });
        white.iter()
            .filter(|tile| adjacent(&blk, tile) == 2)
            .for_each(|tile| { black.insert(*tile); });

        black
    });

    black.len()
}

const NEIGHBORS: [(i32, i32); 6] = [
    (2, 0), (-2, 0), (1, -1), (-1, -1), (1, 1), (-1, 1)
];

fn adjacent(black: &HashSet<(i32, i32)>, (x, y): &(i32, i32)) -> usize {
    NEIGHBORS.iter().filter(|(dx, dy)| black.contains(&(x + dx, y + dy))).count()
}



#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = include_str!("./input.txt");

    let black = part_one(input);
    assert_eq!(black.len(), 254);

    let tiles = part_two(&black);
    assert_eq!(tiles, 3697);
  }

  #[test]
  fn small() {
    let input = include_str!("./test.txt");

    let black = part_one(input);
    assert_eq!(black.len(), 10);

    let tiles = part_two(&black);
    assert_eq!(tiles, 2208);
  }
}