// Create a list of x,y locations containing asteriods.
// For each element in the list, create a "line" to every other asteriod.
// Each line contains a reduced slope value (rise over run reduced by gcd)
// and put them into a hash set. This gives us the number of asteroids
// visible from each potential base since multiple lines with the same
// slope mean multiple asteroids on the same line and only one would be
// visible.
// Use the line count for each site to compare them and take the one with
// the most lines as the max. This is the answer to Part 1.
// For part 2, create a queue of vectors with each vector containing all
// asteroids with the same slope in vaporization order (distance from site).
// Run through the queue popping vectors off the front, grabbing the first
// element as the vaporized rock and adding the vector onto the back of the
// queue if there are any more rocks in it. When the queue is finally empty,
// you've vaporized all the asteriods in order.

use gcd::Gcd;
use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};
use std::f64::consts::PI;
use std::hash::{Hash, Hasher};

fn main() {
  let coords = load(include_str!("./asteroids.txt"));
  let site = process(&coords).unwrap();
  println!("{:?}", site);

  let mut lines = coords.iter()
    .map(|dst| Line::new(&site.loc, dst))
    .collect::<Vec<Line>>();
  // sort by angle from x axis and then length
  radsort::sort_by_key(&mut lines, |l| (l.angle, l.length));
  lines.remove(0);

  let target = vaporize(&lines)[200];
  println!("{} {:#?}", target.dst.x * 100 + target.dst.y, target);
}

fn load(program: &str) -> Vec<Coord> {
  program.lines()
    .enumerate()
    .flat_map(|(y, s)| s.as_bytes().iter()
      .enumerate()
      .filter(|(_, &c)| c as char == '#')
      .map(|(x, _)| Coord { x, y })
      .collect::<Vec<Coord>>())
    .collect::<Vec<Coord>>()
}

fn process(coords: &[Coord]) -> Option<Site> {
  coords.iter().map(|src| {
    let lines = coords.iter()
      .map(|dst| Line::new(src, dst))
      .fold(HashSet::new(), |mut lines, line| { lines.insert(line); lines });
    // Minus 1 for same location
    Site { loc: *src, lines: lines.len() - 1 }
  }).max()
}

fn vaporize(lines: &[Line]) -> Vec<Line> {
  // So the indexes match the count
  let mut vaporized = vec![Line {
    src: Coord { x: 0, y: 0, },
    dst: Coord { x: 0, y: 0,  },
    slope: Slope { rise: 0, run: 0, },
    angle: 0f64,
    length: 0f64,
  }];

  let mut idx = 1;
  let mut rocks = VecDeque::<Vec<Line>>::new();
  while idx < lines.len() {
    let mut v = Vec::<Line>::new();
    let slope = lines[idx].slope;
    while idx < lines.len() && lines[idx].slope == slope {
      v.push(lines[idx]);
      idx += 1;
    }
    rocks.push_back(v);
  }

  while let Some(mut v) = rocks.pop_front() {
    vaporized.push(v.remove(0));
    if v.len() > 0 {
      rocks.push_back(v);
    }
  }

  vaporized
}

fn angle(x: f64, y: f64) -> f64 {
  let mut degrees = x.atan2(y) * 180f64 / PI;
  if x < 0f64 { degrees += 360f64 }
  degrees
}

#[derive(Debug)]
struct Site {
  pub loc: Coord,
  lines: usize,
}

impl Eq for Site {}

impl PartialEq for Site {
  fn eq(&self, other: &Self) -> bool {
    self.lines.eq(&other.lines)
  }
}

impl PartialOrd for Site {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.lines.cmp(&other.lines))
  }
}

impl Ord for Site {
  fn cmp(&self, other: &Self) -> Ordering {
    self.lines.cmp(&other.lines)
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Coord {
  x: usize,
  y: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Slope {
  rise: i8,
  run: i8,
}

impl Slope {
  pub fn new(rise: i8, run: i8) -> Self {
    let p_run  = run.abs() as u8;
    let p_rise = rise.abs() as u8;
    let gcd = p_run.gcd(p_rise) as i8;

    if gcd == 0 {
      Slope { rise, run }
    } else {
      Slope { rise: rise / gcd, run: run / gcd }
    }
  }

  #[allow(dead_code)]
  pub fn from(src: &Coord, dst: &Coord) ->Self {
    let run  = dst.x as i8 - src.x as i8;
    let rise = dst.y as i8 - src.y as i8;
    Slope::new(rise, run)
  }
}

impl Hash for Slope {
  fn hash<H: Hasher>(&self, hasher: &mut H) {
    self.run.hash(hasher);
    self.rise.hash(hasher);
  }
}

#[derive(Clone, Copy, Debug)]
struct Line {
  src: Coord,
  dst: Coord,
  slope: Slope,
  angle: f64,
  length: f64,
}

impl Line {
  pub fn new(src: &Coord, dst: &Coord) -> Self {
    let run  = dst.x as i8 - src.x as i8;
    let rise = src.y as i8 - dst.y as i8;
    let angle = angle(run as f64, rise as f64);
    let slope = Slope::new(rise, run);
    let length = (run as f64).powi(2) + (rise as f64).powi(2);
    let length = (length as f64).sqrt();

    Line { src: *src, dst: *dst, slope, angle, length }
  }
}

impl Eq for Line {}

impl PartialEq for Line {
  fn eq(&self, other: &Self) -> bool {
    self.angle.eq(&other.angle)
  }
}

impl Hash for Line {
  fn hash<H: Hasher>(&self, hasher: &mut H) {
    self.slope.hash(hasher);
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let asteroids = load(include_str!("./asteroids.txt"));
    let site = process(&asteroids);

    let best = Site {
      loc: Coord { x: 17, y: 22 },
      lines: 288,
    };

    assert_eq!(site, Some(best));
  }

  #[test]
  fn it_works2() {
    let coords = load(include_str!("./asteroids.txt"));
    let site = process(&coords).unwrap();
    let mut lines = coords.iter()
      .map(|dst| Line::new(&site.loc, dst))
      .collect::<Vec<Line>>();
    // sort by angle from x axis and then length
    radsort::sort_by_key(&mut lines, |l| (l.angle, l.length));
    lines.remove(0);

    let target = vaporize(&lines)[200];
    let two_hundred = Coord { x: 6, y: 16 };

    assert_eq!(target.dst, two_hundred);
  }

  #[test]
  fn small() {
    let program = load(
     ".#..#\n\
      .....\n\
      #####\n\
      ....#\n\
      ...##",
    );
    let site = process(&program);
    let best = Site {
      loc: Coord { x: 3, y: 4 },
      lines: 8,
    };

    assert_eq!(site, Some(best));
  }

  #[test]
  fn medium1() {
    let program = load(
     "......#.#.\n\
      #..#.#....\n\
      ..#######.\n\
      .#.#.###..\n\
      .#..#.....\n\
      ..#....#.#\n\
      #..#....#.\n\
      .##.#..###\n\
      ##...#..#.\n\
      .#....####",
    );
    let site = process(&program);
    let best = Site {
      loc: Coord { x: 5, y: 8 },
      lines: 33,
    };

    assert_eq!(site, Some(best));
  }

  #[test]
  fn medium2() {
    let program = load(
     "#.#...#.#.\n\
      .###....#.\n\
      .#....#...\n\
      ##.#.#.#.#\n\
      ....#.#.#.\n\
      .##..###.#\n\
      ..#...##..\n\
      ..##....##\n\
      ......#...\n\
      .####.###."
    );
    let site = process(&program);
    let best = Site {
      loc: Coord { x: 1, y: 2 },
      lines: 35,
    };

    assert_eq!(site, Some(best));
  }

  #[test]
  fn medium3() {
    let program = load(
     ".#..#..###\n\
      ####.###.#\n\
      ....###.#.\n\
      ..###.##.#\n\
      ##.##.#.#.\n\
      ....###..#\n\
      ..#.#..#.#\n\
      #..#.#.###\n\
      .##...##.#\n\
      .....#.#.."
    );
    let site = process(&program);
    let best = Site {
      loc: Coord { x: 6, y: 3 },
      lines: 41,
    };

    assert_eq!(site, Some(best));
  }

  #[test]
  fn large_vaporize() {
    let program = 
     ".#..##.###...#######\n\
      ##.############..##.\n\
      .#.######.########.#\n\
      .###.#######.####.#.\n\
      #####.##.#.##.###.##\n\
      ..#####..#.#########\n\
      ####################\n\
      #.####....###.#.#.##\n\
      ##.#################\n\
      #####.##.###..####..\n\
      ..######..##.#######\n\
      ####.##.####...##..#\n\
      .#####..#.######.###\n\
      ##...#.##########...\n\
      #.##########.#######\n\
      .####.#.###.###.#.##\n\
      ....##.##.###..#####\n\
      .#.#.###########.###\n\
      #.#.#.#####.####.###\n\
      ###.##.####.##.#..##";
    let coords = load(program);
    let site = process(&coords).unwrap();
    let mut lines = coords.iter()
      .map(|dst| Line::new(&site.loc, dst))
      .collect::<Vec<Line>>();

      // sort by angle from x axis and then length
    radsort::sort_by_key(&mut lines, |l| (l.angle, l.length));

    let order = vaporize(&lines);
    println!("  1: {:?}", order[1].dst);
    println!("  2: {:?}", order[2].dst);
    println!("  3: {:?}", order[3].dst);
    println!(" 10: {:?}", order[10].dst);
    println!(" 20: {:?}", order[20].dst);
    println!(" 50: {:?}", order[50].dst);
    println!("100: {:?}", order[100].dst);
    println!("199: {:?}", order[199].dst);
    println!("200: {:?}", order[200].dst);
    println!("201: {:?}", order[201].dst);

    assert_eq!(order[1].dst, Coord { x: 11, y: 12, });
    assert_eq!(order[2].dst, Coord { x: 12, y: 1, });
    assert_eq!(order[3].dst, Coord { x: 12, y: 2, });
    assert_eq!(order[10].dst, Coord { x: 12, y: 8, });
    assert_eq!(order[20].dst, Coord { x: 16, y: 0, });
    assert_eq!(order[50].dst, Coord { x: 16, y: 9, });
    assert_eq!(order[100].dst, Coord { x: 10, y: 16, });
    assert_eq!(order[199].dst, Coord { x: 9, y: 6, });
    assert_eq!(order[200].dst, Coord { x: 8, y: 2, });
    assert_eq!(order[201].dst, Coord { x: 10, y: 9, });
    assert_eq!(order[299].dst, Coord { x: 11, y: 1, });
  }

  #[test]
  fn large() {
    let program = load(
     ".#..##.###...#######\n\
      ##.############..##.\n\
      .#.######.########.#\n\
      .###.#######.####.#.\n\
      #####.##.#.##.###.##\n\
      ..#####..#.#########\n\
      ####################\n\
      #.####....###.#.#.##\n\
      ##.#################\n\
      #####.##.###..####..\n\
      ..######..##.#######\n\
      ####.##.####...##..#\n\
      .#####..#.######.###\n\
      ##...#.##########...\n\
      #.##########.#######\n\
      .####.#.###.###.#.##\n\
      ....##.##.###..#####\n\
      .#.#.###########.###\n\
      #.#.#.#####.####.###\n\
      ###.##.####.##.#..##"
    );
    let site = process(&program);
    let best = Site {
      loc: Coord { x: 11, y: 13 },
      lines: 210,
    };

    assert_eq!(site, Some(best));
  }

  #[test]
  fn compare_slopes1() {
    let b1 = Coord { x: 0, y: 0 };
    let d1 = Coord { x: 3, y: 9 };
    let d2 = Coord { x: 6, y: 18 };
    let s1 = Slope::from(&b1, &d1);
    let s2 = Slope::from(&b1, &d2);

    assert_eq!(s1, s2);
  }

  #[test]
  fn compare_slopes2() {
    let b1 = Coord { x: 1, y: 1 };
    let d1 = Coord { x: 3, y: 9 };
    let d2 = Coord { x: 5, y: 17 };
    let s1 = Slope::from(&b1, &d1);
    let s2 = Slope::from(&b1, &d2);

    assert_eq!(s1, s2);
  }

  #[test]
  fn compare_slopes3() {
    let b1 = Coord { x: 5, y: 17 };
    let d1 = Coord { x: 3, y: 9 };
    let d2 = Coord { x: 1, y: 1 };
    let s1 = Slope::from(&b1, &d1);
    let s2 = Slope::from(&b1, &d2);

    assert_eq!(s1, s2);
  }

  #[test]
  fn compare_lines1() {
    let b1 = Coord { x: 0, y: 0 };
    let d1 = Coord { x: 3, y: 9 };
    let d2 = Coord { x: 6, y: 18 };
    let l1 = Line::new(&b1, &d1);
    let l2 = Line::new(&b1, &d2);

    assert_eq!(l1, l2);
  }

  #[test]
  fn compare_lines2() {
    let b1 = Coord { x: 1, y: 1 };
    let d1 = Coord { x: 3, y: 9 };
    let d2 = Coord { x: 5, y: 17 };
    let l1 = Line::new(&b1, &d1);
    let l2 = Line::new(&b1, &d2);

    assert_eq!(l1, l2);
  }

  #[test]
  fn compare_lines3() {
    let b1 = Coord { x: 5, y: 17 };
    let d2 = Coord { x: 1, y: 1 };
    let d1 = Coord { x: 3, y: 9 };
    let l1 = Line::new(&b1, &d1);
    let l2 = Line::new(&b1, &d2);

    assert_eq!(l1, l2);
  }
}
