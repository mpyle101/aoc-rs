use core::iter::Iterator;
use pathfinding::prelude::{Matrix, Weights};
use std::collections::{HashMap, HashSet};

type Tiles = HashMap<Tile, Matrix<char>>;

enum Edge {
    North,
    South,
    East,
    West,
}

// 3x20
// ..................#.
// #....##....##....###
// .#..#..#..#..#..#...
const SEA_MONSTER: [(i32, i32); 15] = [
    (0, 0), (0, -1), (0, -2), (-1, -1), (1, -3),
    (1, -6), (0, -7), (0, -8), (1, -9), (1, -12),
    (0, -13), (0, -14), (1, -15), (1, -18), (0, -19)
];

fn main() {
    let tiles = load(include_str!("./input.txt"));
    let image = find_image(&tiles).unwrap();

    let corners = part_one(&image);
    println!("Part 1: {}", corners);

    let rough = part_two(&image);
    println!("Part 2: {}", rough);
}

fn part_one(image: &Image) -> u64 {
    let i = image.dim;
    [(0, 0), (i-1, 0), (0, i-1), (i-1, i-1)]
        .iter()
        .map(|pos| image.get(pos).unwrap())
        .map(|tile| tile.id as u64)
        .product()
}

fn part_two(image: &Image) -> usize {
    let xforms = [
        (true, 0), (true, 1), (true, 2), (true, 3),
        (false, 0), (false, 1), (false, 2), (false, 3),
    ];
    let mut it = xforms.iter();

    let base = image.build();
    let last = base.rows() - 2;

    let mut sea_monsters = 0;
    while sea_monsters == 0 {
        let xf = it.next().unwrap();
        let im = xform(&base, xf.0, xf.1);
        sea_monsters = im.iter().enumerate().skip(1)
            .map(|(row, data)| data.iter()
                .enumerate().skip(18)
                .filter(|(col, &c)| row < last && c == '#' && sea_monster(&im, (row, *col)))
                .count()
            ).sum();
    }

    let hashes = base.values().filter(|&c| *c == '#').count();
    hashes - sea_monsters * SEA_MONSTER.len()
}

fn sea_monster(im: &Matrix<char>, pos: (usize, usize)) -> bool {
    SEA_MONSTER.iter().all(|d| {
        let i = ((pos.0 as i32 + d.0) as usize, (pos.1 as i32+ d.1) as usize);
        let c = im.get(im.idx(&i)).unwrap();
        *c == '#'
    })
}

fn find_image(tiles: &Tiles) -> Option<Image> {

    let w = ((tiles.len() / 8) as f64).sqrt() as i32;
    let all: Vec<_> = tiles.keys().collect();

    let pos = (0, 0);
    for tile in all {
        let mut image = Image::new(w, tiles);
        image.insert(pos, *tile);

        if solve(pos, tiles, &mut image) {
            return Some(image)
        }
    }

    None
}

fn solve(
    pos: (i32, i32),
    tiles: &Tiles,
    image: &mut Image
) -> bool {
    use Edge::*;

    if image.len() == tiles.len() / 8 {
        return true
    }

    let used = image.used();
    for (tile, _) in tiles {
        if used.contains(&tile.id) {
            continue;
        }

        if let Some(neighbor) = image.get(&(pos.0 - 1, pos.1)) {
            if tile.edge(tiles, North) != neighbor.edge(tiles, South) {
                continue;
            }
        }
        if let Some(neighbor) = image.get(&(pos.0, pos.1 - 1)) {
            if tile.edge(tiles, West) != neighbor.edge(tiles, East) {
                continue;
            }
        }

        image.insert(pos, *tile);
        let p = if pos.1 < image.dim - 1 { (pos.0, pos.1 + 1) } else { (pos.0 + 1, 0) };
        if solve(p, tiles, image) {
            return true
        }
        image.remove(&pos);
    }

    false
}

#[derive(Debug)]
struct Image<'a> {
    dim: i32,
    data: HashMap<(i32, i32), Tile>,
    tiles: &'a Tiles,
}

impl<'a> Image<'a> {
    fn new(dim: i32, tiles: &Tiles) -> Image {
        Image { dim, tiles, data: HashMap::new() }
    }

    fn insert(&mut self, pos: (i32, i32), tile: Tile) -> Option<Tile> {
        self.data.insert(pos, tile)
    }

    fn remove(&mut self, pos: &(i32, i32)) -> Option<Tile> {
        self.data.remove(pos)
    }

    fn get(&self, pos: &(i32, i32)) -> Option<&Tile> {
        self.data.get(pos)
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn used(&self) -> HashSet<u32> {
        self.data.values().map(|t| t.id).collect()
    }

    fn build(&self) -> Matrix<char> {
        let data = self.trimmed_row(0);
        (1..self.dim).fold(data, |mut m, r| {
            let d = self.trimmed_row(r);
            d.iter().for_each(|r| m.extend(r).unwrap());
            m
        })
    }

    fn trimmed_row(&self, row: i32) -> Matrix<char> {
        let tile = self.data.get(&(row, 0)).unwrap();
        let data = tile.trim(self.tiles);

        (1..self.dim).fold(data, |m, c| {
            let t = self.data.get(&(row, c)).unwrap();
            let d = t.trim(self.tiles);
            combine(&m, &d)
        })
    }
}

fn combine(t1: &Matrix<char>, t2: &Matrix<char>) -> Matrix<char> {
    let iter1 = t1.iter();
    let iter2 = t2.iter();

    Matrix::from_rows(
        iter1.zip(iter2).map(|(r1, r2)| r1.iter().chain(r2).map(|c| *c))
    ).unwrap()
}


#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Tile {
    id: u32,
    flip: bool,
    rotate: usize,
}

impl Tile {
    fn edge(&self, tiles: &Tiles, edge: Edge) -> Matrix<char> {
        let data = tiles.get(self).unwrap();

        let rows = data.rows();
        let cols = data.columns();

        use Edge::*;
        match edge {
            North => data.slice(0..1, 0..cols).unwrap(),
            South => data.slice(rows-1..rows, 0..cols).unwrap(),
            East  => data.slice(0..rows, cols-1..cols).unwrap(),
            West  => data.slice(0..rows, 0..1).unwrap()
        }
    }

    fn trim(&self, tiles:&Tiles) -> Matrix<char> {
        let data = tiles.get(self).unwrap();
        data.slice(1..data.rows()-1, 1..data.columns()-1).unwrap()
    }
}

fn load(input: &str) -> Tiles {
    let xforms = [
        (true, 0), (true, 1), (true, 2), (true, 3),
        (false, 0), (false, 1), (false, 2), (false, 3),
    ];

    input.split("\n\n").flat_map(|s| {
        let mut it = s.lines();
        let v: Vec<_> = it.next().unwrap().split(' ').collect();
        let id = v[1][..v[1].len() - 1].parse::<u32>().unwrap();
        let data = Matrix::from_rows(it.map(|s| s.chars())).unwrap();

        xforms.iter().map(|x| (
            Tile { id, flip: x.0, rotate: x.1 },
            xform(&data, x.0, x.1))
        ).collect::<Vec<_>>()
    })
    .collect()
}

fn xform(data: &Matrix<char>, flip: bool, rotate: usize) -> Matrix<char> {
    let mut m = if flip { data.flipped_lr() } else { data.clone() };
    if rotate > 0 {
        m.rotate_cw(rotate)
    }

    m
}

#[allow(dead_code)]
fn draw(image: &Matrix<char>) {
    println!("rows: {}, cols: {}", image.rows(), image.columns());
    for row in image.iter() {
        println!("{}", row.iter().collect::<String>());
    }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let tiles = load(include_str!("./input.txt"));
    let image = find_image(&tiles).unwrap();

    let corners = part_one(&image);
    assert_eq!(corners, 18482479935793);

    let rough = part_two(&image);
    assert_eq!(rough, 2118);
  }

  #[test]
  fn small() {
    let tiles = load(include_str!("./test.txt"));
    let image = find_image(&tiles).unwrap();

    let corners = part_one(&image);
    assert_eq!(corners, 20899048083289);

    let rough = part_two(&image);
    assert_eq!(rough, 273);
  }
}