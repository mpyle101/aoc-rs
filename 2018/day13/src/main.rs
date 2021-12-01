use std::cmp::Ordering;
use std::collections::HashMap;

type Track = HashMap<(u32, u32), char>;

fn main() {
    use std::time::Instant;

    let (carts, track) = load(include_str!("./input.txt"));

    let t1 = Instant::now();
    let (x, y) = part_one(&carts, &track);
    let t2 = Instant::now();
    println!("Part 1: {},{}  ({:?})", x, y, t2 - t1);

    let t1 = Instant::now();
    let (x, y) = part_two(&carts, &track);
    let t2 = Instant::now();
    println!("Part 2: {},{}  ({:?})", x, y, t2 - t1);
}

fn part_one(starting: &[Cart], track: &Track) -> (u32, u32) {
    let mut carts: Vec<_> = starting.iter().cloned().collect();
    loop {
        let (mut updated, collision) = update_collision(&carts);
        if let Some((x, y)) = collision {
            return (x, y)
        }
        updated.sort();
        carts = update_carts(&updated, track);
    }
}

fn part_two(starting: &[Cart], track: &Track) -> (u32, u32) {
    let mut carts: Vec<_> = starting.iter().cloned().collect();
    loop {
        let updated = update_crashed(&carts);
        if updated.len() == 1 {
            return updated[0].pos
        }
        carts = update_carts(&updated, track);
        carts.sort();
    }
}

fn update_collision(carts: &[Cart]) -> (Vec<Cart>, Option<(u32, u32)>) {
    use std::collections::VecDeque;

    let mut updated: VecDeque<_> = carts.iter().cloned().collect();
    for _ in 0..carts.len() {
        let cart = updated.pop_front().unwrap();
        let cart = step(&cart);
        if updated.contains(&cart) {
            return (carts.to_vec(), Some(cart.pos))
        } else {
            updated.push_back(cart)
        }
    }

    (updated.iter().map(|c| *c).collect(), None)
}

fn update_crashed(carts: &[Cart]) -> Vec<Cart> {
    use std::collections::VecDeque;

    let mut convoy: VecDeque<_> = carts.iter().cloned().collect();
    for _ in 0..carts.len() {
        let cart = convoy.pop_front().unwrap();
        if !cart.crashed {
            let cart = step(&cart);
            if let Some(idx) = convoy.iter().position(|c| *c == cart) {
                convoy[idx].crashed = true;
            } else {
                convoy.push_back(cart)
            }
        }
    }

    convoy.iter().filter(|c| !c.crashed).map(|c| *c).collect()
}

fn update_carts(carts: &[Cart], track: &Track) -> Vec<Cart> {
    use Action::*;
    use Direction::*;

    carts.iter()
        .map(|c| {
            match track.get(&c.pos) {
                Some('|') => *c,
                Some('-') => *c,
                Some('/')  if c.dir == West  => c.turn(South),
                Some('/')  if c.dir == East  => c.turn(North),
                Some('/')  if c.dir == North => c.turn(East),
                Some('/')  if c.dir == South => c.turn(West),
                Some('\\') if c.dir == West  => c.turn(North),
                Some('\\') if c.dir == East  => c.turn(South),
                Some('\\') if c.dir == North => c.turn(West),
                Some('\\') if c.dir == South => c.turn(East),
                Some('+') => {
                        let dir = if c.action == GoStraight {
                            c.dir
                        } else {
                            match c.dir {
                                North => if c.action == TurnLeft { West } else { East },
                                South => if c.action == TurnLeft { East } else { West },
                                East  => if c.action == TurnLeft { North } else { South },
                                West  => if c.action == TurnLeft { South } else { North },
                            }
                        };
                        Cart { dir, pos: c.pos, action: c.action.next(), crashed: false }
                    },
                None => panic!("Off the track!"),
                   _ => panic!("Unknown track segment")
            }
        })
        .collect()
}

fn step(cart: &Cart) -> Cart {
    use Direction::*;

    match cart.dir {
        North => cart.step((cart.pos.0, cart.pos.1 - 1)),
        South => cart.step((cart.pos.0, cart.pos.1 + 1)),
        East  => cart.step((cart.pos.0 + 1, cart.pos.1)),
        West  => cart.step((cart.pos.0 - 1, cart.pos.1)),
    }
}

fn load(input: &str) -> (Vec<Cart>, Track) {
    use Direction::*;

    let mut carts = Vec::new();
    let mut track = Track::new();

    for (y, s) in input.lines().enumerate() {
        for (x, c) in s.chars().enumerate() {
            let pos = (x as u32, y as u32);
            match c {
                '^' => {
                    carts.push(Cart::new(pos, North));
                    track.insert(pos, '|')
                },
                'v' => {
                    carts.push(Cart::new(pos, South));
                    track.insert(pos, '|')
                },
                '<' => {
                    carts.push(Cart::new(pos, West));
                    track.insert(pos, '-')
                },
                '>' => {
                    carts.push(Cart::new(pos, East));
                    track.insert(pos, '-')
                },
                ' ' => None,
                  _ => track.insert(pos, c),
           };
        }
    }
    carts.sort();

    (carts, track)
}

#[allow(dead_code)]
fn write(carts: &[Cart], track: &Track, collision: (u32, u32)) {
    use std::fs::OpenOptions;
    use std::io::prelude::*;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("ticks.txt")
        .unwrap();

    let map: HashMap<_,_> = carts.iter().map(|c| (c.pos, c)).collect();
    let v: Vec<String> = 
        (0..150).map(|y|
            (0..150).map(|x|
                if collision == (x, y) {
                    'X'
                } else if let Some(c) = map.get(&(x, y)) {
                    c.dir as u8 as char
                } else {
                    *track.get(&(x, y)).unwrap_or(&' ')
                }
            ).collect()
        ).collect();
    writeln!(file, "{}\n\n", v.join("\n")).unwrap();
}

#[derive(Clone, Copy, Debug)]
struct Cart {
    pos: (u32, u32),
    dir: Direction,
    action: Action,
    crashed: bool,
}

impl Cart {
    fn new(pos: (u32, u32), dir: Direction) -> Self {
        Cart { pos, dir, action: Action::TurnLeft, crashed: false }
    }

    fn step(&self, pos: (u32, u32)) -> Self {
        Cart { pos, dir: self.dir, action: self.action, crashed: false }
    }

    fn turn(&self, dir: Direction) -> Self {
        Cart { pos: self.pos, dir, action: self.action, crashed: false }
    }
}

impl Eq for Cart { }

impl PartialEq for Cart {
    fn eq(&self, other: &Self) -> bool {
        if self.crashed || other.crashed {
            // Crashed carts don't exist
            false
        } else {
            self.pos == other.pos
        }
    }
}

impl Ord for Cart {
    fn cmp(&self, other: &Self) -> Ordering {
        // Order by y then x
        let a = (self.pos.1, self.pos.0);
        let b = (other.pos.1, other.pos.0);
        a.cmp(&b)
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Order by y then x
        let a = (self.pos.1, self.pos.0);
        let b = (other.pos.1, other.pos.0);
        Some(a.cmp(&b))
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    North = b'^',
    South = b'v',
    East  = b'>',
    West  = b'<',
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Action {
    TurnLeft,
    TurnRight,
    GoStraight,
}

impl Action {
    fn next(&self) -> Self {
        use Action::*;
        match self {
            TurnLeft   => GoStraight,
            TurnRight  => TurnLeft,
            GoStraight => TurnRight,
        }
    }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let (carts, track) = load(include_str!("./input.txt"));

    let (x, y) = part_one(&carts, &track);
    assert_eq!((x, y), (39, 52));

    let (x, y) = part_two(&carts, &track);
    assert_eq!((x, y), (133, 146));
  }
}
