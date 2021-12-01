use std::collections::HashSet;
use vm::Vm;

fn main() {
  let program = include_str!("./program.txt");

  let blocks = part_one(program);
  println!("Block tiles: {}", blocks);

  let score = part_two(program);
  println!("Score: {}", score);
}

fn part_one(program: &str) -> usize {
  let mut vm = Vm::new(program).unwrap();
  let (_, mut stdout) = vm.pipes();
  vm.exec().unwrap();
  stdout.drain().chunks(3).filter(|c| c[2] == 2).count()
}

fn part_two(program: &str) -> i64 {
  let mut vm = Vm::new(program).unwrap();
  let (mut stdin, mut stdout) = vm.pipes();

  let adjust = |offset| if offset < 0 { -1 } else if offset > 0 { 1 } else { 0 };

  vm.set_addr(0, 2);
  vm.exec().unwrap();
  let tiles = stdout.drain();
  let mut blocks: HashSet<(i64, i64)> = tiles.chunks(3)
    .filter(|c| c[2] == 2)
    .map(|c| (c[0], c[1]))
    .collect();

  let mut ball   = find_tile(&tiles, 4).unwrap();
  let mut paddle = find_tile(&tiles, 3).unwrap();
  let offset = ball.0 - paddle.0;

  let joystick = adjust(offset);
  stdin.write(joystick);

  let mut score = 0;
  while blocks.len() > 0 {
    vm.cont().unwrap();
    for tile in stdout.drain().chunks(3) {
      match tile {
        [-1, 0, n] => score = *n,
        [x,  y, 0] => { blocks.remove(&(*x, *y)); () },
        [x,  y, 3] => paddle = (*x, *y),
        [x,  y, 4] => ball = (*x, *y),
        _ => panic!()
      }
    };

    let offset = ball.0 - paddle.0;
    let joystick = if offset < 0 { -1 } else if offset > 0 { 1 } else { 0 };
    stdin.write(joystick);
    vm.cont().unwrap();
  };

  score
}

fn find_tile(screen: &[i64], id: i64) -> Option<(i64, i64)> {
  let tiles = screen.chunks(3).collect::<Vec<&[i64]>>();
  let idx = tiles.iter().position(|t| t[2] == id)?;
  Some((tiles[idx][0], tiles[idx][1]))
}

#[derive(Debug)]
struct Tile(i64, i64, i64);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let program = include_str!("./program.txt");
    let blocks = part_one(program);

    assert_eq!(blocks, 427);
  }

  #[test]
  fn it_works2() {
    let program = include_str!("./program.txt");
    let score = part_two(program);

    assert_eq!(score, 21426);
  }
}