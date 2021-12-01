use vm::Vm;

fn main() {
  let program = include_str!("./boost.txt");
  let mut vm = Vm::new(program).unwrap();
  let (mut stdin, mut stdout) = vm.pipes();

  stdin.write(2);
  vm.exec().unwrap();

  println!("Distress co-ords {:?}", stdout.flush())
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let program = include_str!("./boost.txt");
    let mut vm = Vm::new(program).unwrap();
    let (mut stdin, mut stdout) = vm.pipes();

    stdin.write(1);
    assert_eq!(vm.exec().unwrap(), vm::State::Done);
    assert_eq!(stdout.flush(), 3013554615);
  }

  #[test]
  fn distress_coords() {
    let program = include_str!("./boost.txt");
    let mut vm = Vm::new(program).unwrap();
    let (mut stdin, mut stdout) = vm.pipes();

    stdin.write(2);
    assert_eq!(vm.exec().unwrap(), vm::State::Done);
    assert_eq!(stdout.flush(), 50158);
  }
}
