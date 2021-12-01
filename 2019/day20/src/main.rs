use pathfinding::prelude::bfs;
use std::collections::{HashMap, HashSet};
use std::cmp::max;

// x, y, z
type Pos = ((i32, i32), i32);
type Loc = (i32, i32);

fn main() {
    let maze = load(include_str!("./maze.txt"));

    let path = part_one(&maze).unwrap();
    println!("Part one: {}", path.len() - 1);

    let path = part_two(&maze).unwrap();
    println!("Part two: {}", path.len() - 1);
}

fn part_one(maze: &Maze) -> Option<Vec<Pos>> {
    bfs(&maze.aa, |p| maze.successors(p, false), |&p| p == maze.zz)
}

fn part_two(maze: &Maze) -> Option<Vec<Pos>> {
    bfs(&maze.aa, |p| maze.successors(p, true), |&p| p == maze.zz)
}

fn load(maze: &str) -> Maze {   
    let mut gates = Vec::new();
    let mut doors = HashMap::new();
    let mut walls = HashSet::new();

    let mut max_y = 0;
    maze.lines().enumerate().for_each(|(y, _)| max_y = y as i32);
    
    let mut max_x = 0;
    for (y, line) in maze.lines().enumerate() {
        let mut prev = ['*', '*'];
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => { walls.insert((x as i32, y as i32)); },
                ' '|'.'|'*' => {},
                _ => {  walls.insert((x as i32, y as i32));
                        make_gate(x as i32, y as i32, max_x, max_y, c, &prev, &walls, &mut doors, &mut gates); },
            };
            prev[0] = prev[1];
            prev[1] = c;
            max_x = max(max_x, x as i32);
        }
    }

    let mut portals_by_name: HashMap<[char;2], Portal> = HashMap::new();
    gates.iter().for_each(|&(pos, name)| {
        if let Some(portal) = portals_by_name.get_mut(&name) {
            portal.gate2 = pos;
        } else {
            portals_by_name.insert(name, Portal { name, gate1: pos, gate2: ((0, 0), (0, 0), 0) });
        }
    });

    let aa = portals_by_name.remove(&['A','A']).unwrap().gate1.1;
    let zz = portals_by_name.remove(&['Z','Z']).unwrap().gate1.1;
    let portals: HashMap<_,_> = portals_by_name.iter()
        .flat_map(|(_, p)| vec![
            (p.gate1.0, (p.gate2.1, p.gate1.2)), (p.gate2.0, (p.gate1.1, p.gate2.2))])
        .collect();

    Maze { aa: (aa, 0), zz: (zz, 0), walls, portals }
}

fn make_gate(
    x: i32,
    y: i32,
    m: i32,
    n: i32,
    c: char,
    prev: &[char;2],
    walls: &HashSet<(i32, i32)>,
    doors: &mut HashMap<i32, char>,
    gates: &mut Vec<((Loc, Loc, i32), [char;2])>,
) {
    match prev {
        [_, ' '] => if let Some(g) = doors.remove(&x) {
                        if walls.contains(&(x - 1, y - 2)) {
                            let z = if y == n { -1 } else { 1 };
                            gates.push((((x, y - 1), (x, y - 2), z), [g, c]))
                        } else {
                            let z = if y > 1 { 1 } else { -1 };
                            gates.push((((x, y), (x, y + 1), z), [g, c]))
                        }
                    } else {
                        doors.insert(x, c);
                    },
        [_, '*'] => {},
        [_, '.'] => (),
        ['.', d] => { let z = if x < m { 1 } else { -1 };
                      gates.push((((x - 1, y), (x - 2, y), z), [*d, c]));
                    },
        [' ', d] => { doors.remove(&(x - 1));
                      gates.push((((x, y), (x + 1, y), 1), [*d, c]));
                    },
        ['*', d] => { doors.remove(&(x - 1));
                      gates.push((((x, y), (x + 1, y), -1), [*d, c]));
                    }
        [a , b] => panic!("Bad gate: {} {}", a, b)
    }
}

#[derive(Clone, Copy, Debug)]
struct Portal {
    name: [char;2],
    gate1: (Loc, Loc, i32),
    gate2: (Loc, Loc, i32),
}

struct Maze {
    aa: Pos,
    zz: Pos,
    walls: HashSet<(i32, i32)>,
    portals: HashMap<Loc, (Loc, i32)>,
}

impl Maze {
    fn successors(&self, pt: &Pos, recurse: bool) -> Vec<Pos> {
        let p = pt.0;
        let adjacent = [
            (p.0, p.1 - 1), // above
            (p.0, p.1 + 1), // below
            (p.0 - 1, p.1), // before
            (p.0 + 1, p.1), // after
        ];

        adjacent.iter().fold(vec![], |mut v, pos| {
            if let Some(exit) = self.portals.get(pos) {
                if recurse {
                    if pt.1 > 0 || exit.1 == 1 {
                        v.push((exit.0, pt.1 + exit.1))
                    }
                } else {
                    v.push((exit.0, pt.1))
                }
            } else if !self.walls.contains(pos) {
                v.push((*pos, pt.1))
            };
            v
        })
    }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let maze = load(include_str!("./maze.txt"));

    let path = part_one(&maze).unwrap();
    assert_eq!(path.len() - 1, 548);

    let path = part_two(&maze).unwrap();
    assert_eq!(path.len() - 1, 6452);
  }

  #[test]
  fn small_maze() {
    let maze = load(include_str!("./test_s.txt"));

    let path = part_one(&maze).unwrap();
    assert_eq!(path.len() - 1, 23);
  }

  #[test]
  fn medium_maze() {
    let maze = load(include_str!("./test_m.txt"));

    let path = part_one(&maze).unwrap();
    println!("{:?}", path);
    assert_eq!(path.len() - 1, 58);
  }

  #[test]
  fn recurse_maze() {
    let maze = load(include_str!("./test_r.txt"));

    let path = part_two(&maze).unwrap();
    println!("{:?}", path);
    assert_eq!(path.len() - 1, 396);
  }
}