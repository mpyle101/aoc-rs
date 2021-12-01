use std::str;

fn main() {
    let moves = include_str!("./moves.txt");

    let dist = part_one(moves);
    println!("Part1: {}", dist);

    let dist = part_two(moves);
    println!("Part2: {}", dist);
}

fn part_one(moves: &str) -> i32 {
    use Direction::*;

    let mut facing = Facing::East;

    let dest = moves.lines()
        .fold((0i32, 0i32), |acc, line| {
            let cmd = line.as_bytes()[0];
            let val = (&line[1..]).parse::<i32>().unwrap();

            match cmd {
                b'N' => (acc.0, acc.1 - val),
                b'S' => (acc.0, acc.1 + val),
                b'E' => (acc.0 + val, acc.1),
                b'W' => (acc.0 - val, acc.1),
                b'F' => advance(&facing, acc, val),
                b'L' => { facing = rotate(&facing, Left, val); acc },
                b'R' => { facing = rotate(&facing, Right, val); acc },
                _    => panic!("Invalid command")
            }
        });
    
    dest.0.abs() + dest.1.abs()
}

fn part_two(moves: &str) -> i32 {
    let (ship, _) = moves.lines()
        .fold(((0, 0), (10, -1)), |(ship, wp), line| {
            let cmd = line.as_bytes()[0];
            let val = (&line[1..]).parse::<i32>().unwrap();

            match cmd {
                b'N' => (ship, (wp.0, wp.1 - val)),
                b'S' => (ship, (wp.0, wp.1 + val)),
                b'E' => (ship, (wp.0 + val, wp.1)),
                b'W' => (ship, (wp.0 - val, wp.1)),
                b'R' => (ship, rotate_waypoint(wp, val)),
                b'L' => (ship, rotate_waypoint(wp, 360 - val)),
                b'F' => ((ship.0 + wp.0 * val, ship.1 + wp.1 * val), wp),
                _    => panic!("Invalid command")
            }
        });
    
    ship.0.abs() + ship.1.abs()
}

fn advance(facing: &Facing, pos: (i32, i32), dist: i32) -> (i32, i32) {
    match facing {
        Facing::North => (pos.0, pos.1 - dist),
        Facing::South => (pos.0, pos.1 + dist),
        Facing::East  => (pos.0 + dist, pos.1),
        Facing::West  => (pos.0 - dist, pos.1),
    }
}

fn rotate(facing: &Facing, dir: Direction, deg: i32) -> Facing {
    use Direction::*;

    let d = match dir {
        Right => (*facing as u8).rotate_right((deg as u32 / 90) * 2),
        Left  => (*facing as u8).rotate_left((deg as u32/ 90) * 2),
    };

    match d {
        0b10000000 => Facing::North,
        0b00100000 => Facing::East,
        0b00001000 => Facing::South,
        0b00000010 => Facing::West,
        _ => panic!("Unknown rotate result: {:#010b}", d)
    }
}

fn rotate_waypoint(wp: (i32, i32), deg: i32) -> (i32, i32) {
    let steps = deg / 90;
    (0..steps).fold(wp, |acc, _| (-acc.1, acc.0))
}

// E10  N 4  10   -4
// E 4  S10   4   10
// W10  S 4 -10    4
// W 4  N10  -4  -10

#[derive(Debug)]
enum Direction {
    Right,
    Left,
}

#[derive(Clone, Copy)]
enum Facing {
    North = 0b10000000,
    East  = 0b00100000,
    South = 0b00001000,
    West  = 0b00000010,
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let moves = include_str!("./moves.txt");

    let dist = part_one(moves);
    assert_eq!(dist, 562);

    let dist = part_two(moves);
    assert_eq!(dist, 101860);
  }
}