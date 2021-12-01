use anyhow::Result;
use std::cmp::max;
use std::collections::HashSet;
use vm::Vm;

type Skaffold = HashSet<(i32, i32)>;

fn main() {
  let program = include_str!("./program.txt");
  let calibration = part_one(program).unwrap();
  println!("Part 1: {}", calibration);

  let dust = part_two(program).unwrap();
  println!("Part 2: {}", dust);
}

fn part_one(program: &str) -> Result<i32> {
  let mut vm = Vm::new(program)?;
  let (_, mut stdout) = vm.pipes();
  vm.exec()?;

  let mut x = 0;
  let mut y = 0;
  let mut cols = 0;
  let mut robot = (0, 0);
  let mut facing = Direction::Up;
  let mut skaffold = HashSet::new();
  stdout.drain().iter().for_each(|v| {
    match v {
      10  => { cols = max(x, cols); y += 1; x = -1; },
      35  => { skaffold.insert((x, y)); },
      60  => { robot = (x, y); facing = Direction::Left; },
      62  => { robot = (x, y); facing = Direction::Right; },
      94  => { robot = (x, y); facing = Direction::Up; },
      118 => { robot = (x, y); facing = Direction::Down; },
      _   => ()
    };
    x += 1;
  });
  let _rows = y - 1;

  let calibration: i32 = skaffold.iter()
    .filter(|p| is_intersection(p, &skaffold))
    .map(|p| p.0 * p.1)
    .sum();

  Ok(calibration)
}

fn part_two(program: &str) -> Result<i64, &str> {
  // M: A,C,C,B,B,A,C,C
  // A: L,12,R,4,R,4
  // B: R,12,R,4,L,6,L,8,L,8
  // C: R,12,R,4,L,12
  let rules = b"\
    A,C,C,B,B,A,A,C,C,B\n\
    L,12,R,4,R,4\n\
    R,12,R,4,L,6,L,8,L,8\n\
    R,12,R,4,L,12\n";

  let mut vm = Vm::new(program).or(Err("Load failed"))?;

  vm.edit_program(0, 2);
  rules.iter().for_each(|&v| vm.write(v as i64));
  vm.write(b'n'); vm.write(b'\n'); // feed
  vm.exec().or(Err("Exec failed"))?;

  let screen = vm.drain();
  let dust = screen.last().ok_or("No output")?;

  Ok(*dust)
}

#[allow(dead_code)]
fn draw(screen: &[i64]) {
  let s: String = screen.iter().map(|b| (*b as u8) as char).collect();
  println!("{}", s);
}

#[allow(dead_code)]
fn print_screen(
  rows: i32,
  cols: i32,
  robot: &(i32, i32),
  facing: Direction,
  skaffold: &Skaffold
) {
  for y in 0..rows {
    for x in 0..cols {
      let pos = (x, y);
      let c = if skaffold.contains(&pos) {
        if is_intersection(&pos, &skaffold) { 'O' } else { '#' }
      } else if robot == &pos {
        facing_char(facing)
      } else {
        '.'
      };
      print!("{}", c);
    }
    print!("\n");
  }
}

#[allow(dead_code)]
fn facing_char(dir: Direction) -> char {
  use Direction::*;

  match dir {
    Up    => '^',
    Down  => 'v',
    Left  => '<',
    Right => '>',
  }
}

fn is_intersection(pos: &(i32, i32), skaffold: &Skaffold) -> bool {
  skaffold.contains(&(pos.0, pos.1 - 1)) &&  // above
  skaffold.contains(&(pos.0, pos.1 + 1)) &&  // below
  skaffold.contains(&(pos.0 - 1, pos.1)) &&  // before
  skaffold.contains(&(pos.0 + 1, pos.1))     // after
}

#[derive(Clone, Copy)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let program = include_str!("./program.txt");

    let calibration = part_one(program).unwrap();
    assert_eq!(calibration, 6672);

    let dust = part_two(program).unwrap();
    assert_eq!(dust, 923017);
  }
}