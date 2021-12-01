use vm::Vm;

fn main() {
  let program = include_str!("./program.txt");
  let mut vm = Vm::new(program).unwrap();
  let (mut stdin, mut stdout) = vm.pipes();

  stdin.write(1);
  vm.exec().unwrap();

  println!("Diagnostic Code: {}", stdout.flush());
}


#[cfg(test)]
mod tests {
  use super::*;

  static PROGRAM: &str = include_str!("./program.txt");

  #[test]
  fn it_works() {
    // test program
    let mut vm = Vm::new(PROGRAM).unwrap();
    let (mut stdin, mut stdout) = vm.pipes();

    stdin.write(1);
    assert_eq!(vm.exec().unwrap(), vm::State::Done);
    assert_eq!(stdout.flush(), 13787043);
}

  #[test]
  fn thermal_radiator_controller() {
    // thermal radiator controller
    let mut vm = Vm::new(PROGRAM).unwrap();
    let (mut stdin, mut stdout) = vm.pipes();

    stdin.write(5);
    assert_eq!(vm.exec().unwrap(), vm::State::Done);
    assert_eq!(stdout.flush(), 3892695);
  }
}
