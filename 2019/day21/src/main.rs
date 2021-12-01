use vm::Vm;

fn main() {
  let program = include_str!("./program.txt");

  let damage = part_one(program);
  println!("Part 1: {}", damage);

  let damage = part_two(program);
  println!("Part 2: {}", damage);
}

fn part_one(program: &str) -> i64 {
  let rules = b"\
NOT C J
AND D J
NOT A T
OR T J 
WALK
";

  let mut vm = Vm::new(program).unwrap();

  rules.iter().for_each(|&v| vm.write(v as i64));
  vm.exec().unwrap();

  let screen = vm.drain();
  let dust = screen.last().unwrap();

  *dust
}

fn part_two(program: &str) -> i64 {
  let rules = b"\
NOT H T
OR C T
AND B T
AND A T
NOT T J
AND D J
RUN
";

  let mut vm = Vm::new(program).unwrap();

  rules.iter().for_each(|&v| vm.write(v as i64));
  vm.exec().unwrap();

  let screen = vm.drain();
  let dust = screen.last().unwrap();

  *dust
}


#[allow(dead_code)]
fn draw(screen: &[i64]) {
  let s: String = screen.iter().map(|b| (*b as u8) as char).collect();
  println!("{}", s);
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let program = include_str!("./program.txt");

    let damage = part_one(program);
    assert_eq!(damage, 19349722);

    let damage = part_two(program);
    assert_eq!(damage, 1141685254);
  }
}