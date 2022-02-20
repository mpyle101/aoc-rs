// For each wire, create an in-order vector of all the points it
// runs through following the direction instructions. Also, for
// each wire, generate a hashset from of the points to remove
// crossovers and have a set object.
// Get the intersection of the sets to find where the wires cross.
// Use the vectors to find the number of steps to the intersection
// points and take the smallest.

use anyhow::{bail, Result};
use std::collections::HashSet;

fn main() {
    let mut wires = include_str!("./wires.txt").lines();
    let a = parse_path(wires.next().unwrap());
    let b = parse_path(wires.next().unwrap());
    let (path_a, pts_a) = calc_steps(&a);
    let (path_b, pts_b) = calc_steps(&b);
    let vec_x = find_x_steps(&path_a, &pts_a, &path_b, &pts_b);
    let min_x = vec_x.iter().min();
    
    println!("{:#?}", min_x);
}

#[derive(Debug, PartialEq)]
enum Path {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

fn parse_path(path: &str) -> Vec<Path> {
    path.split(',').map(to_path).map(Result::unwrap).collect()
}

fn calc_steps(path: &[Path]) -> (Vec<Point>, HashSet<Point>) {
    let mut start = Point { x: 0, y: 0 };
    let steps: Vec<Point> = path.iter().flat_map(|p| calc_points(&mut start, p)).collect();
    let points: HashSet<Point> = steps.clone().into_iter().collect();

    (steps, points)
}

fn find_x_steps(
    path_a: &[Point],
    pts_a: &HashSet<Point>,
    path_b: &[Point],
    pts_b: &HashSet<Point>
) -> Vec<usize> {
    pts_a.intersection(pts_b)
        .map(|pt| {
        let steps_a = path_a.iter().position(|&a| a == *pt).unwrap();
        let steps_b = path_b.iter().position(|&b| b == *pt).unwrap();
            
        // Account for step from central port
        steps_a + steps_b + 2
        })
        .collect()
}

fn to_path(s: &str) -> Result<Path> {
    let dir = s.chars().next();
    if dir.is_none() {
        bail!("Path is empty")
    }
    let len = s[1..].parse::<i32>()?;

    let path = match dir {
        Some('U') => Path::Up(len),
        Some('D') => Path::Down(len),
        Some('L') => Path::Left(len),
        Some('R') => Path::Right(len),
        Some(_)   => bail!("Invalid path direction: {}", s),
        None      => bail!("Path direction not found")
    };

    Ok(path)
}

fn calc_points(start: &mut Point, path: &Path) -> Vec<Point> {
    let points: Vec<_> = match path {
        Path::Up(len)    => (start.y + 1..=start.y + len).map(|y| Point { x:start.x, y }).collect(),
        Path::Down(len)  => (start.y - len..=start.y - 1).map(|y| Point { x:start.x, y }).rev().collect(),
        Path::Left(len)  => (start.x - len..=start.x - 1).map(|x| Point { x, y:start.y }).rev().collect(),
        Path::Right(len) => (start.x + 1..=start.x + len).map(|x| Point { x, y:start.y }).collect(),
    };

    *start = *points.last().unwrap();
    points
}


/** Unit Tests */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(to_path("U5").unwrap(), Path::Up(5));
        assert_eq!(to_path("D243").unwrap(), Path::Down(243));
        assert_eq!(to_path("R1009").unwrap(), Path::Right(1009));
        assert_eq!(to_path("L37").unwrap(), Path::Left(37));
    }

    #[test]
    fn unknown_direction() {
        assert!(to_path("G5").is_err());
    }

    #[test]
    fn empty_path() {
        assert!(to_path("").is_err());
    }

    #[test]
    fn bad_length() {
        assert!(to_path("U5x4").is_err());
    }

    #[test]
    fn path_points_up() {
        let mut start = Point { x: 0, y: 0 };
        let path = to_path("U3").unwrap();
        let pts  = calc_points(&mut start, &path);
        let points = vec![Point { x: 0, y: 1 }, Point { x: 0, y: 2 }, Point { x: 0, y: 3 }];

        assert_eq!(pts, points);
    }

    #[test]
    fn path_points_down() {
        let mut start = Point { x: 0, y: 0 };
        let path = to_path("D3").unwrap();
        let pts  = calc_points(&mut start, &path);
        let points = vec![Point { x: 0, y: -1 }, Point { x: 0, y: -2 }, Point { x: 0, y: -3 }];

        assert_eq!(pts, points);
    }

    #[test]
    fn path_points_left() {
        let mut start = Point { x: 0, y: 0 };
        let path = to_path("L3").unwrap();
        let pts  = calc_points(&mut start, &path);
        let points = vec![Point { x: -1, y: 0 }, Point { x: -2, y: 0 }, Point { x: -3, y: 0 }];

        assert_eq!(pts, points);
    }

    #[test]
    fn path_points_right() {
        let mut start = Point { x: 0, y: 0 };
        let path = to_path("R3").unwrap();
        let pts  = calc_points(&mut start, &path);
        let points = vec![Point { x: 1, y: 0 }, Point { x: 2, y: 0 }, Point { x: 3, y: 0 }];

        assert_eq!(pts, points);
    }

    #[test]
    fn path_points_all() {
        let a = parse_path("U3,R3,D3,L3");
        let (pts, _) = calc_steps(&a);
        let points = vec![
        Point { x: 0, y: 1 }, Point { x: 0, y: 2 }, Point { x: 0, y: 3 },
        Point { x: 1, y: 3 }, Point { x: 2, y: 3 }, Point { x: 3, y: 3 },
        Point { x: 3, y: 2 }, Point { x: 3, y: 1 }, Point { x: 3, y: 0 },
        Point { x: 2, y: 0 }, Point { x: 1, y: 0 }, Point { x: 0, y: 0 }
        ];

        assert_eq!(pts, points);
    }

    #[test]
    fn min_steps1() {
        let a = parse_path("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let b = parse_path("U62,R66,U55,R34,D71,R55,D58,R83");
        let (path_a, pts_a) = calc_steps(&a);
        let (path_b, pts_b) = calc_steps(&b);
        let vec_x = find_x_steps(&path_a, &pts_a, &path_b, &pts_b);
        let min_x = vec_x.iter().min();

        assert_eq!(*min_x.unwrap(), 610 as usize);
    }

    #[test]
    fn min_steps2() {
        let a = parse_path("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let b = parse_path("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        let (path_a, pts_a) = calc_steps(&a);
        let (path_b, pts_b) = calc_steps(&b);
        let vec_x = find_x_steps(&path_a, &pts_a, &path_b, &pts_b);
        let min_x = vec_x.iter().min();

        assert_eq!(*min_x.unwrap(), 410 as usize);
    }
}
