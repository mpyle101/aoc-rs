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
    println!("Part 1: {score}  ({:?})", t2 - t1);

    let t1 = Instant::now();
    let score = part_two(&units, &tiles);
    let t2 = Instant::now();
    println!("Part 2: {score}  ({:?})", t2 - t1);
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
    let (_, _, round, hp) = do_game(3, units, tiles);

    round * hp
}

fn part_two(units: &Units, tiles: &Tiles) -> i32 {
    let elves = units.values().filter(|unit| unit.1).count();
    let mut attack_power = 4;

    loop {
        let (race, count, round, hp) = do_game(attack_power, units, tiles);
        if race && count == elves {
            return round * hp
        }
        attack_power += 1
    }
}

fn do_game(ap: i32, units: &Units, tiles: &Tiles) -> (bool, usize, i32, i32) {
    let mut round = 0;
    let mut units = units.clone();

    let winners = loop {
        if let Some(elves) = do_round(ap, &mut units, tiles) {
            break elves
        } else {
            round += 1
        }
    };

    let hp = units.values().map(|(hp, _)| hp).sum::<i32>();

    (winners, units.len(), round, hp)
}

fn do_round(ap: i32, units: &mut Units, tiles: &Tiles) -> Option<bool> {
    let keys = units.keys().cloned().collect::<Vec<_>>();
    for pos in keys {
        if let Some(&actor) = units.get(&pos) {
            let targets = enemies(&actor, &units);
            if targets.len() == 0 {
                return Some(actor.1)
            } else {
                do_turn(ap, &pos, &actor, &targets, units, tiles)
            }
        }
    }

    return None
}

fn do_turn(
    ap: i32,
    pos: &Tile,
    actor: &Unit,
    targets: &[(Tile, i32)],
    units: &mut Units,
    tiles: &Tiles)
{
    // If we can attack someone we're done. Goblins always have an
    // attack power of 3.
    let attack = if actor.1 { ap } else { 3 };
    if !do_attack(attack, pos, targets, units) {
        let p = do_move(pos, targets, units, tiles);
        units.remove(pos);
        units.insert(p, *actor);
        do_attack(attack, &p, targets, units);
    }
}

fn do_attack(ap: i32, pos: &Tile, targets: &[(Tile, i32)], units: &mut Units) -> bool {
    let mut opponents = in_range(pos, targets);
    if opponents.len() > 0 {
        // Sort by hit points, then row, then column so we get the
        // lowest hit point opponents first in "reading order".
        opponents.sort_by_key(|&((r, c), hp)| (hp, r, c));
        let (tile, hp) = opponents[0];
        if hp <= ap {
            units.remove(&tile);
        } else {
            units.get_mut(&tile).unwrap().0 -= ap;
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
    units.iter()
        .filter_map(|(k, v)| (v.1 != actor.1).then(|| (*k, v.0)))
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

    let score = part_two(&units, &tiles);
    assert_eq!(score, 47296);
  }
}
