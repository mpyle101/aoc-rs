use std::collections::HashMap;

fn main() {
    use std::time::Instant;

    let regex = load(include_str!("./input.txt"));

    let t1 = Instant::now();
    let doors = part_one(&regex);
    let t2 = Instant::now();
    println!("Part 1: {}  ({:?})", doors, t2 - t1);
}

type Tile  = (i32, i32);
type Tiles = HashMap<Tile, char>;

fn load(input: &str) -> Vec<char> {
    // We don't need the begining and end characters (^, $).
    input.chars().skip(1).take(input.len() - 2).collect()
}

fn part_one(regex: &[char]) -> usize {
    use pathfinding::prelude::bfs;

    // Build the map.
    let mut i = 0;
    let mut tiles = traverse(&mut i, (0, 0), regex);
    tiles.insert((0, 0), 'X');

    // Find the dead ends
    let dead_ends = tiles.iter()
        .filter(|(&k, &v)| v == '.' && neighbors(k, &tiles).len() == 1);

    // Find the paths to the dead ends
    let mut paths = dead_ends
        .filter_map(|(goal, _)| {
            bfs(&(0, 0), 
                |p| neighbors(*p, &tiles).iter().map(|(pt, _)| *pt).collect::<Vec<_>>(),
                |p| p == goal)
        })
        .collect::<Vec<_>>();

    // Sort to get the longest.
    paths.sort_by(|a, b| b.len().cmp(&a.len()));

    paths[0].len() / 2
}

fn traverse(i: &mut usize, pos: Tile, regex: &[char]) -> Tiles {
    let mut pt = pos;
    let mut tiles = Tiles::new();

    while *i < regex.len() {
        let c = regex[*i];
        *i += 1;
        if c == '(' {
            let map = traverse(i, pt, regex);
            map.iter().for_each(|(k, v)| { tiles.insert(*k, *v); })
        } else if c == ')' {
            return tiles
        } else if c == '|' {
            pt = pos
        } else {
            let (dx, dy, door) = delta(c);
            pt = (pt.0 + dx, pt.1 + dy);
            tiles.insert(pt, door);
            pt = (pt.0 + dx, pt.1 + dy);
            tiles.insert(pt, '.');
        }
    }

    tiles
}

fn delta(c: char) -> (i32, i32, char) {
    match c {
        'N' => ( 0, -1, '-'),
        'S' => ( 0,  1, '-'),
        'E' => ( 1,  0, '|'),
        'W' => (-1,  0, '|'),
        _ => panic!("Unknown direction: {}", c)
    }
}

const DELTAS: [Tile;4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];

fn neighbors((x, y): Tile, tiles: &Tiles) -> Vec<(Tile, char)> {
    DELTAS.iter().filter_map(|(dx, dy)| {
        let p = (x + dx, y + dy);
        if let Some(c) = tiles.get(&p) {
            Some((p, *c))
        } else {
            None
        }
    })
    .collect()
}

#[allow(dead_code)]
fn print(tiles: &Tiles) {
    let mut arr = tiles.keys().collect::<Vec<_>>();

    arr.sort();
    let min_x = arr[0].0 - 1;
    let max_x = arr[arr.len()-1].0 + 1;

    arr.sort_by_key(|(_, y)| y);
    let min_y = arr[0].1 - 1;
    let max_y = arr[arr.len()-1].1 + 1;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if let Some(c) = tiles.get(&(x, y)) {
                print!("{}", c)
            } else {
                print!("#")
            }
        }
        println!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let regex = load(include_str!("./input.txt"));

        let doors = part_one(&regex);
        assert_eq!(doors, 4018);
    }
}
