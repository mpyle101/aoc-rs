use std::collections::HashSet;
use vm::{Pipe, Vm};

fn main() {
    let program = include_str!("./program.txt");

    let points = part_one(program);
    println!("Affected points: {}", points);

    let closest = part_two(program);
    println!("Closest point: {}", closest);
}

fn part_one(program: &str) -> i64 {
    let mut points = 0;
    let mut vm = Vm::new(program).unwrap();
    let mut pipes = vm.pipes();

    for y in 0..50 {
        for x in 0..50 {
            points += if check_beam(&mut vm, &mut pipes, &(x, y)) {1} else {0};
        }
    }

    points
}

fn part_two(program: &str) -> i64 {
    let mut y = 100;
    let mut x = 0;
    let mut vm = Vm::new(program).unwrap();
    let mut pipes = vm.pipes();
    let mut points = HashSet::new();
    loop {
        // Look for the start of the beam
        while !check_beam(&mut vm, &mut pipes, &(x, y)) { x += 1 }

        // Save the start of the beam for the next row
        // and only check for the Santa's ship while we're
        // in the beam.
        let mut x1 = x;
        while check_beam(&mut vm, &mut pipes, &(x1, y)) {
            if points.get(&(x1, y - 99)).is_some() &&
               points.get(&(x1 - 99, y)).is_some()
            {
                return ((x1 - 99) * 10000) + (y - 99)
            }
            points.insert((x1, y));
            x1 += 1;
        }
        y += 1
    }
}

fn check_beam(vm: &mut Vm, pipes: &mut (Pipe, Pipe), pt: &(i64, i64)) -> bool {
    let stdin  = &mut pipes.0;
    let stdout = &mut pipes.1;
    vm.exec().unwrap();
    stdin.write(pt.0);
    stdin.write(pt.1);
    vm.cont().unwrap();

    stdout.read().unwrap() == 1
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let program = include_str!("./program.txt");

    let points = part_one(program);
    assert_eq!(points, 152);

    let closest = part_two(program);
    assert_eq!(closest, 10730411);
  }
}