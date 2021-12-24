use std::ops::BitOr;

fn main() {
    use std::fs;
    use std::time::Instant;
    
    let input = fs::read_to_string("./input.txt").unwrap();
    let steps = load(&input);

    let t1 = Instant::now();
    let cubes = part_one(&steps);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", cubes, t2 - t1);

    let t1 = Instant::now();
    let cubes = part_two(&steps);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", cubes, t2 - t1);
}

#[derive(Debug)]
struct Step {
    a: bool,
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}

#[derive(Clone, Copy, Debug)]
struct Cuboid {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
    z1: i32,
    z2: i32,
}

impl Cuboid {
    fn new(x1: i32, x2: i32, y1: i32, y2: i32, z1: i32, z2: i32) -> Cuboid {
        Cuboid { x1, x2, y1, y2, z1, z2 }
    }

    fn is_valid(&self) -> bool {
        self.x2 > self.x1 && 
        self.y2 > self.y1 &&
        self.z2 > self.z1
    }

    fn volume(&self) -> u64 {
        (self.x2 - self.x1) as u64 *
        (self.y2 - self.y1) as u64 *
        (self.z2 - self.z1) as u64
    }

    fn punch(&self, other: &Cuboid) -> Vec<Cuboid> {
        let a = self;
        let b = other;

        [
            Cuboid::new(a.x1, b.x1, a.y1, a.y2, a.z1, a.z2),
            Cuboid::new(b.x2, a.x2, a.y1, a.y2, a.z1, a.z2),
            Cuboid::new(b.x1, b.x2, a.y1, b.y1, a.z1, a.z2),
            Cuboid::new(b.x1, b.x2, b.y2, a.y2, a.z1, a.z2),
            Cuboid::new(b.x1, b.x2, b.y1, b.y2, a.z1, b.z1),
            Cuboid::new(b.x1, b.x2, b.y1, b.y2, b.z2, a.z2),
        ]
        .iter()
        .filter_map(|c| if c.is_valid() { Some(*c) } else { None })
        .collect()
    }
}

impl BitOr for Cuboid {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Cuboid {
            x1: rhs.x1.max(self.x1).min(self.x2),
            x2: rhs.x2.max(self.x1).min(self.x2),
            y1: rhs.y1.max(self.y1).min(self.y2),
            y2: rhs.y2.max(self.y1).min(self.y2),
            z1: rhs.z1.max(self.z1).min(self.z2),
            z2: rhs.z2.max(self.z1).min(self.z2),
        }
    }
}

fn load(input: &str) -> Vec<Step> {
    use regex::Regex;

    let re = Regex::new(concat!(
        r"^(?P<action>(on|off)) ",
        r"x=(?P<x1>\-?\d+)..(?P<x2>\-?\d+),",
        r"y=(?P<y1>\-?\d+)..(?P<y2>\-?\d+),",
        r"z=(?P<z1>\-?\d+)..(?P<z2>\-?\d+)$",
    )).unwrap();

    input.lines().map(|s| {
        let cap = re.captures(s).unwrap();
        Step {
            a: cap["action"].eq("on"),
            x: (cap["x1"].parse().unwrap(), cap["x2"].parse().unwrap()),
            y: (cap["y1"].parse().unwrap(), cap["y2"].parse().unwrap()),
            z: (cap["z1"].parse().unwrap(), cap["z2"].parse().unwrap()),
        }
    }).collect()
}

fn part_one(steps: &[Step]) -> usize {
    use std::collections::HashSet;

    let on = steps.iter().take(20).fold(HashSet::new(), |mut pts, s| {
        (s.x.0..=s.x.1).for_each(|x|
            (s.y.0..=s.y.1).for_each(|y|
                (s.z.0..=s.z.1).for_each(|z| {
                    let pt = (x, y, z);
                    if s.a { pts.insert(pt); } else { pts.remove(&pt); }
                })
            )
        );
        pts
    });

    on.len()
}

fn part_two(steps: &[Step]) -> u64 {
    // For each step, create a cuboid and intersect it (bit |) with each cube
    // in the list. If the result is valid, punch out the intersection from
    // the cube in the list. This results in 6 more "on" cubes. Stuff the valid
    // ones back list for the next step. If the new cubiod (a) is an "on" step,
    // add it to the list. When we're done, we have a list of distinct cubes
    // for "on" cubes so add up their volumes.
    let s1 = &steps[0];
    let a = Cuboid::new(s1.x.0, s1.x.1+1, s1.y.0, s1.y.1+1, s1.z.0, s1.z.1+1);
    steps.iter().skip(1).fold(vec![a], |v, s| {
        let a = Cuboid::new(s.x.0, s.x.1+1, s.y.0, s.y.1+1, s.z.0, s.z.1+1);
        let mut n = v.iter().map(|b| {
            let c = a | *b;
            if c.is_valid() { b.punch(&c) } else { vec![*b] }
        }).flatten().collect::<Vec<_>>();
        if s.a { n.push(a) }
        n
    })
    .iter()
    .map(|c| c.volume() as u64)
    .sum()
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let steps = load(&input);

        let cubes = part_one(&steps);
        assert_eq!(cubes, 596598);

        let cubes = part_two(&steps);
        assert_eq!(cubes, 1199121349148621);
    }

    #[test]
    fn example1() {
        let input = fs::read_to_string("./test1.txt").unwrap();
        let steps = load(&input);

        let cubes = part_one(&steps);
        assert_eq!(cubes, 590784);
    }

    #[test]
    fn example2() {
        let input = fs::read_to_string("./test2.txt").unwrap();
        let steps = load(&input);

        let cubes = part_two(&steps);
        assert_eq!(cubes, 2758514936282235);
    }
}