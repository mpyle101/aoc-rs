// For part 2, the position and velocity variables are independent so you
// can find where each set of x's, y's and z's repeat on their own. Also,
// since you get to a value via stepping, you know the first repeat has to
// be when the velocities are all zero.
// Finding when each set of velocity values hit zero together is waaaay faster
// then trying to find when all the velocities are zero across the board. Once
// you find each cycle point, you then find the least common multiple which will
// tell you the iteration when all of them have cycled to zero together.

use itertools::Itertools;
use num::integer::Integer;
use regex::Regex;
use std::hash::Hash;
use std::ops::{AddAssign, Neg};

fn main() {
  let mut moons = load(include_str!("./moons.txt"));
  step(1000, &mut moons);
  println!("Total energy after 1000 steps: {}", total_energy(&moons));

  let moons = load(include_str!("./moons.txt"));
  let vx = find_cycle(&mut moons.clone(), 0);
  let vy = find_cycle(&mut moons.clone(), 1);
  let vz = find_cycle(&mut moons.clone(), 2);

  let iterations = vx.lcm(&vy).lcm(&vz);
  println!("Iterations: {} {} {} {}", vx, vy, vz, iterations)
}

fn load(moons: &str) -> Vec<Moon> {
  let re = Regex::new(r"<x=(.*), y=(.*), z=(.*)>").unwrap();
  moons
    .lines()
    .map(|l| re.captures(l).unwrap())
    .map(|c| Moon::new( 
      c.get(1).unwrap().as_str().parse::<i32>().unwrap(), 
      c.get(2).unwrap().as_str().parse::<i32>().unwrap(),
      c.get(3).unwrap().as_str().parse::<i32>().unwrap()
    ))
    .collect::<Vec<Moon>>()
}

fn step(iterations: u32, moons: &mut [Moon]) {
  // Apply gravity
  for _ in 0..iterations {
    apply_gravity(moons);
    apply_velocity(moons);
  }
}

fn find_cycle(moons: &mut [Moon], index: usize) -> u64 {
  let mut iterations = 1u64;

  step(1, moons);
  while !(0..4).all(|i| moons[i].vel.get(index) == 0) {
    apply_gravity(moons);
    apply_velocity(moons);
    iterations += 1;

    if iterations % 1_000_000 == 0 {
      println!("Iterations: {}", iterations)
    }
  }

  iterations * 2
}

fn apply_gravity(moons: &mut [Moon]) {
  (0..moons.len()).combinations(2)
    .for_each(|v| {
      let delta = velocity_delta(&moons[v[0]].pos, &moons[v[1]].pos);
      // Only one mutable reference allowed at a time
      let moon: &mut Moon = &mut moons[v[0]];
      moon.vel += delta.clone();

      let moon: &mut Moon = &mut moons[v[1]];
      moon.vel += -delta;
    })
}

fn apply_velocity(moons: &mut [Moon]) {
  moons.iter_mut().for_each(|m| m.update());
}

fn total_energy(moons: &[Moon]) -> i32 {
  moons.iter().map(|m| m.energy()).sum()
}

fn velocity_delta(a: &Triplet, b: &Triplet) -> Triplet {
  let delta = |a, b| if a < b { 1 } else if a > b { -1 } else { 0 };
  Triplet {
    x: delta(a.x, b.x),
    y: delta(a.y, b.y),
    z: delta(a.z, b.z),
  }
}

#[derive(Clone, Debug, Hash, PartialEq)]
struct Triplet {
  x: i32,
  y: i32,
  z: i32,
}
impl Triplet {
  fn get(&self, index: usize) -> i32 {
    match index {
      0 => self.x,
      1 => self.y,
      2 => self.z,
      _ => panic!()
    }
  }
}
impl AddAssign for Triplet {
  fn add_assign(&mut self, other: Self) {
    *self = Self {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    };
  }
}
impl Neg for Triplet {
  type Output = Self;

  fn neg(self) -> Self::Output {
    Self {
      x: -self.x,
      y: -self.y,
      z: -self.z,
    }
  }
}

#[derive(Clone, Debug, Hash, PartialEq)]
struct Moon {
  pos: Triplet,
  vel: Triplet,
}

impl Moon {
  fn new(x: i32, y: i32, z: i32) -> Self {
    Moon {
      pos: Triplet { x, y, z },
      vel: Triplet { x: 0, y: 0, z: 0 }
    }
  }

  fn update(&mut self) {
    self.pos += self.vel.clone()
  }

  fn kinetic(&self) -> i32 {
    self.vel.x.abs() + self.vel.y.abs() + self.vel.z.abs()
  }

  fn potential(&self) -> i32 {
    self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs()
  }

  fn energy(&self) -> i32 {
    self.potential() * self.kinetic()
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let mut moons = load(include_str!("./moons.txt"));
    step(1000, &mut moons);

    assert_eq!(total_energy(&moons), 8538);
  }

  #[test]
  fn stepping_up() {
    let data = "\
      <x=-1, y=0, z=2>\n\
      <x=2, y=-10, z=-7>\n\
      <x=4, y=-8, z=8>\n\
      <x=3, y=5, z=-1>";
    let mut moons = load(data);

    step(1, &mut moons);
    let expected = vec![
      Moon { pos: Triplet { x: 2, y: -1, z:  1 }, vel: Triplet { x:  3, y: -1, z: -1 } },
      Moon { pos: Triplet { x: 3, y: -7, z: -4 }, vel: Triplet { x:  1, y:  3, z:  3 } },
      Moon { pos: Triplet { x: 1, y: -7, z:  5 }, vel: Triplet { x: -3, y:  1, z: -3 } },
      Moon { pos: Triplet { x: 2, y:  2, z:  0 }, vel: Triplet { x: -1, y: -3, z:  1 } },
    ];
    assert_eq!(moons[0], expected[0]);
    assert_eq!(moons[1], expected[1]);
    assert_eq!(moons[2], expected[2]);
    assert_eq!(moons[3], expected[3]);

    step(9, &mut moons);
    let expected = vec![
      Moon { pos: Triplet { x: 2, y:  1, z: -3 }, vel: Triplet { x: -3, y: -2, z:  1 } },
      Moon { pos: Triplet { x: 1, y: -8, z:  0 }, vel: Triplet { x: -1, y:  1, z:  3 } },
      Moon { pos: Triplet { x: 3, y: -6, z:  1 }, vel: Triplet { x:  3, y:  2, z: -3 } },
      Moon { pos: Triplet { x: 2, y:  0, z:  4 }, vel: Triplet { x:  1, y: -1, z: -1 } },
    ];
    assert_eq!(moons[0], expected[0]);
    assert_eq!(moons[1], expected[1]);
    assert_eq!(moons[2], expected[2]);
    assert_eq!(moons[3], expected[3]);
  }

  #[test]
  fn total_energy_calc() {
    let data = "\
      <x=-8, y=-10, z=0>\n\
      <x=5, y=5, z=10>\n\
      <x=2, y=-7, z=3>\n\
      <x=9, y=-8, z=-3>";
    let mut moons = load(data);

    step(100, &mut moons);
    let expected = vec![
      Moon { pos: Triplet { x:   8, y: -12, z: -9 }, vel: Triplet { x: -7, y:   3, z:  0 } },
      Moon { pos: Triplet { x:  13, y:  16, z: -3 }, vel: Triplet { x:  3, y: -11, z: -5 } },
      Moon { pos: Triplet { x: -29, y: -11, z: -1 }, vel: Triplet { x: -3, y:   7, z:  4 } },
      Moon { pos: Triplet { x:  16, y: -13, z: 23 }, vel: Triplet { x:  7, y:   1, z:  1 } },
    ];
    assert_eq!(moons[0], expected[0]);
    assert_eq!(moons[1], expected[1]);
    assert_eq!(moons[2], expected[2]);
    assert_eq!(moons[3], expected[3]);

    assert_eq!(total_energy(&moons), 1940);
  }

  #[test]
  fn repeat() {
    let data = "\
      <x=-1, y=0, z=2>\n\
      <x=2, y=-10, z=-7>\n\
      <x=4, y=-8, z=8>\n\
      <x=3, y=5, z=-1>";
    let moons = load(data);
    let vx = find_cycle(&mut moons.clone(), 0);
    let vy = find_cycle(&mut moons.clone(), 1);
    let vz = find_cycle(&mut moons.clone(), 2);
    let iterations = vx.lcm(&vy).lcm(&vz);

    assert_eq!(iterations, 2772);
  }


  #[test]
  fn it_works2() {
    let moons = load(include_str!("./moons.txt"));
    let vx = find_cycle(&mut moons.clone(), 0);
    let vy = find_cycle(&mut moons.clone(), 1);
    let vz = find_cycle(&mut moons.clone(), 2);
    let iterations = vx.lcm(&vy).lcm(&vz);

    assert_eq!(iterations, 506_359_021_038_056);
  }

}
