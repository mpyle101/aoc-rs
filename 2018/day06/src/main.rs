fn main() {
    use std::time::Instant;

    let coords = load(include_str!("./input.txt"));

    let t1 = Instant::now();
    let area = part_one(&coords);
    let t2 = Instant::now();
    println!("Part 1: {}  ({:?})", area, t2 - t1);

    let t1 = Instant::now();
    let area = part_two(&coords);
    let t2 = Instant::now();
    println!("Part 2: {}  ({:?})", area, t2 - t1);
}

fn part_one(coords: &Coords) -> u32 {
    use std::collections::HashMap;
    use ndarray::Array;
    use State::*;

    let v = &coords.coords;
    let mut grid = Array::from_elem((coords.rows as usize, coords.cols as usize), State::Unknown);
    grid.indexed_iter_mut().for_each(|(pt, state)| {
        let p1 = (pt.0 as u32, pt.1 as u32);
        *state = v.iter().fold(State::Unknown, |st, c|
            match st {
                Unknown => Owned(mdist(p1, *c), *c),
                Tied(d) => {
                    let md = mdist(p1, *c);
                    if md < d { Owned(md, *c) } else { Tied(d) }
                },
                Owned(d, p) => {
                    match mdist(p1, *c) {
                        md if md < d => Owned(md, *c),
                        md if md > d => Owned(d, p),
                                  md => Tied(md)
                    }
                }
            }
        )
    });

    let area = grid.indexed_iter().fold(HashMap::new(), |mut m, (_, st)| {
        match st {
            Unknown | Tied(_) => (),
            Owned(_, p) => *m.entry(p).or_insert(0) += 1,
        };
        m
    });

    *area.values().max().unwrap()
}

fn part_two(coords: &Coords) -> usize {
    use ndarray::Array2;

    let v = &coords.coords;
    let mut grid = Array2::<u32>::zeros((coords.rows as usize, coords.cols as usize));
    grid.indexed_iter_mut().for_each(|(pt, d)| {
        let p = (pt.0 as u32, pt.1 as u32);
        v.iter().for_each(|c| *d += mdist(p, *c));
    });

    grid.iter().filter(|&v| *v < 10000).count()
}

fn mdist(p1: (u32, u32), p2: (u32, u32)) -> u32 {
    (i32::abs(p1.0 as i32 - p2.0 as i32) + i32::abs(p1.1 as i32 - p2.1 as i32)) as u32
}

fn load(input: &str) -> Coords {
    use std::cmp::{max, min};

    let coords: Vec<_> = input.lines()
        .map(|s| s.split(", ").collect::<Vec<_>>())
        .map(|v| (v[0].parse::<u32>().unwrap(), v[1].parse::<u32>().unwrap()))
        .collect();

    let (top, bottom, left, right) = coords.iter()
        .fold((u32::MAX, 0, u32::MAX, 0), |acc, c|
            (min(acc.0, c.1), max(acc.1, c.1), min(acc.2, c.0), max(acc.3, c.0))
        );

    Coords {
        cols: (right - left) as u32,
        rows: (bottom - top) as u32,
        coords: coords.iter().map(|(x, y)| (x - left, y - top)).collect()
    }
}

#[derive(Debug)]
struct Coords {
    rows: u32,
    cols: u32,
    coords: Vec<(u32, u32)>,
}

#[derive(Clone)]
enum State {
    Unknown,
    Tied(u32),
    Owned(u32, (u32, u32)),
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let coords = load(include_str!("./input.txt"));

    let area = part_one(&coords);
    assert_eq!(area, 3290);

    let area = part_two(&coords);
    assert_eq!(area, 45602);
  }
}
