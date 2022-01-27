use std::collections::{BTreeMap, HashSet};
use lazy_static::lazy_static;

lazy_static! {
    // (row, col) => above, right, left, below
    static ref DELTA: [(i32, i32);4] = [
        (-1, 0), (0, 1), (0, -1), (1, 0)
    ];
}

fn main() {
    use std::time::Instant;

    let (units, tiles) = load(include_str!("./input.txt"));

    let t1 = Instant::now();
    let score = part_one(&units, &tiles);
    let t2 = Instant::now();
    println!("Part 1: {}  ({:?})", score, t2 - t1);
}

type Tile = (i32, i32);             // (row, col)
type Unit = (i32, bool);            // (hp, is_elf)
type Tiles = HashSet<Tile>;         // all open tiles
type Units = BTreeMap<Tile, Unit>;  // tiles to units in "reading order"

fn load(input: &str) -> (Units, Tiles) {
    let mut units = Units::new();
    let mut tiles = Tiles::new();

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let tile = (row as i32, col as i32);
            match c {
                '.' => { tiles.insert(tile); },
                'E' => { tiles.insert(tile); units.insert(tile, (200, true)); },
                'G' => { tiles.insert(tile); units.insert(tile, (200, false)); },
                 _  => {},
            }
        }
    }
    
    (units, tiles)
}

fn part_one(units: &Units, tiles: &Tiles) -> i32 {
    let mut round = -1;
    let mut units = units.clone();

    'outer: loop {
        round += 1;
        // println!("Round: {}", round);
        // println!("Units: {:?}", units);
        // print(&units, tiles, 7);

        let keys = units.keys().cloned().collect::<Vec<_>>();
        for pos in keys {
            if let Some(&actor) = units.get(&pos) {
                let targets = enemies(&actor, &units);
                if targets.len() == 0 {
                    break 'outer
                } else {
                    do_turn(&pos, &actor, &targets, &mut units, tiles)
                }
            }
        }
    }
    // print(&units, tiles, 7);
    // println!("{} {:?}", round, units);

    round * units.values().map(|(hp, _)| hp).sum::<i32>()
}

fn do_turn(
    pos: &Tile,
    actor: &Unit,
    targets: &[(Tile, i32)],
    units: &mut Units,
    tiles: &Tiles)
{
    // If we can attack someone we're done.
    if !do_attack(pos, targets, units) {
        let p = do_move(pos, targets, units, tiles);
        units.remove(pos);
        units.insert(p, *actor);
        do_attack(&p, targets, units);
    }
}

fn do_attack(pos: &Tile, targets: &[(Tile, i32)], units: &mut Units) -> bool {
    let mut opponents = in_range(pos, targets);
    if opponents.len() > 0 {
        // Sort by hit points, then row, then column so we get the
        // lowest hit point opponents first in "reading order".
        opponents.sort_by_key(|&((r, c), hp)| (hp, r, c));
        let (tile, hp) = opponents[0];
        if hp <= 3 {
            units.remove(&tile);
        } else {
            units.get_mut(&tile).unwrap().0 -= 3;
        }
    }

    opponents.len() > 0
}

fn do_move(
    pos: &Tile,
    targets: &[(Tile, i32)],
    units: &Units,
    tiles: &Tiles
) -> Tile {
    // Find all the adjacent / "in range" tiles
    let adjacent = find_adjacent(targets, units, tiles);

    // Get the shortest paths to all reachable target adjacent tiles
    // and find the shortest of those to get the nearest tiles.
    let mut paths = find_reachable(pos, &adjacent, units, tiles);
    if paths.len() > 0 {
        paths.sort_by_key(|v| v.len());
        paths[0][1]
    } else {
        *pos
    }
}

fn enemies(actor: &Unit, units: &Units) -> Vec<(Tile, i32)> {
    units.iter().filter_map(|(k, v)|
        if v.1 != actor.1 { Some((*k, v.0)) } else { None }
    )
    .collect()
}

fn in_range(pos: &Tile, targets: &[(Tile, i32)]) -> Vec<(Tile, i32)> {
    targets.iter()
        .filter(|((r, c), _)| (pos.0 - r).abs() + (pos.1 - c).abs() == 1)
        .cloned()
        .collect()
}

fn find_adjacent(targets: &[(Tile, i32)], units: &Units, tiles: &Tiles) -> Vec<Tile> {
    let mut adjacent = targets.iter()
        .map(|(tile, _)| open_tiles(tile, units, tiles))
        .flatten()
        .collect::<Vec<_>>();

    // Sort so tiles are in "reading order".
    adjacent.sort();

    adjacent
}

fn find_reachable(pos: &Tile, adjacent: &[Tile], units: &Units, tiles: &Tiles) -> Vec<Vec<Tile>> {
    use pathfinding::prelude::bfs;

    adjacent.iter()
        .filter_map(|tile| 
            bfs(pos, |p| open_tiles(p, units, tiles), |p| p == tile)
        )
        .collect()
}

fn open_tiles((r, c): &Tile, units: &Units, tiles: &Tiles) -> Vec<Tile> {
    DELTA.iter().filter_map(move |(dr, dc)| {
        let tile = (r + dr, c + dc);
        if tiles.contains(&tile) && !units.contains_key(&tile) {
            Some(tile)
        } else {
            None
        }
    })
    .collect()
}

#[allow(dead_code)]
fn print(units: &Units, tiles: &Tiles, n: i32) {
    for row in 0..n {
        for col in 0..n {
            let tile = (row, col);
            if let Some(unit) = units.get(&tile) {
                print!("{}", if unit.1 { 'E' } else { 'G' })
            } else if tiles.contains(&tile) {
                print!(".")
            } else {
                print!("#")
            }
        }
        println!()
    }
    println!()
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let (units, tiles) = load(include_str!("./input.txt"));

    let score = part_one(&units, &tiles);
    assert_eq!(score, 181952);
  }
}
