use super::*;

static PROGRAM: &str = include_str!("./program.txt");

#[test]
fn it_works() {
  // test program
  let mut vm = Vm::new(PROGRAM).unwrap();
  let (mut stdin, mut stdout) = vm.pipes();

  stdin.write(1);
  assert_eq!(vm.exec().unwrap(), State::Done);
  assert_eq!(stdout.flush(), 13787043);
}

#[test]
fn trc() {
  // thermal radiator controller
  let mut vm = Vm::new(PROGRAM).unwrap();
  let (mut stdin, mut stdout) = vm.pipes();

  stdin.write(5);
  assert_eq!(vm.exec().unwrap(), State::Done);
  assert_eq!(stdout.flush(), 3892695);
}

#[test]
fn jt_eq_pos_true() {
  let program = "3,9,8,9,10,9,4,9,99,-1,8";
  let mut vm = Vm::new(program).unwrap();
  let (mut stdin, mut stdout) = vm.pipes();

  stdin.write(8);
  assert_eq!(vm.exec().unwrap(), State::Done);
  assert_eq!(stdout.flush(), 1);
}

#[test]
fn jt_eq_pos_false() {
  let program = "3,9,8,9,10,9,4,9,99,-1,8";
  let mut vm = Vm::new(program).unwrap();
  let (mut stdin, mut stdout) = vm.pipes();

  stdin.write(2);
  assert_eq!(vm.exec().unwrap(), State::Done);
  assert_eq!(stdout.flush(), 0);
}

#[test]
fn jt_lt_pos_true() {
  let program = "3,9,7,9,10,9,4,9,99,-1,8";
  let mut vm = Vm::new(program).unwrap();
  let (mut stdin, mut stdout) = vm.pipes();

  stdin.write(7);
  assert_eq!(vm.exec().unwrap(), State::Done);
  assert_eq!(stdout.flush(), 1);
}

#[test]
fn jt_lt_pos_false() {
  let program = "3,9,7,9,10,9,4,9,99,-1,8";
  let mut vm = Vm::new(program).unwrap();
  let (mut stdin, mut stdout) = vm.pipes();

  stdin.write(8);
  assert_eq!(vm.exec().unwrap(), State::Done);
  assert_eq!(stdout.flush(), 0);
}

#[test]
fn jt_eq_imm_true() {
  let program = "3,3,1108,-1,8,3,4,3,99";
  let mut vm = Vm::new(program).unwrap();
  let (mut stdin, mut stdout) = vm.pipes();

  stdin.write(8);
  assert_eq!(vm.exec().unwrap(), State::Done);
  assert_eq!(stdout.flush(), 1);
}

#[test]
fn jt_eq_imm_false() {
  let program = "3,3,1108,-1,8,3,4,3,99";
  let mut vm = Vm::new(program).unwrap();
  let (mut stdin, mut stdout) = vm.pipes();

  stdin.write(2);
  assert_eq!(vm.exec().unwrap(), State::Done);
  assert_eq!(stdout.flush(), 0);
}

#[test]
fn jt_lt_imm_true() {
  let program = "3,3,1107,-1,8,3,4,3,99";
  let mut vm = Vm::new(program).unwrap();
  let (mut stdin, mut stdout) = vm.pipes();

  stdin.write(7);
  assert_eq!(vm.exec().unwrap(), State::Done);
  assert_eq!(stdout.flush(), 1);
}

#[test]
fn jt_lt_imm_false() {
  let program = "3,3,1107,-1,8,3,4,3,99";
  let mut vm = Vm::new(program).unwrap();
  let (mut stdin, mut stdout) = vm.pipes();

  stdin.write(8);
  assert_eq!(vm.exec().unwrap(), State::Done);
  assert_eq!(stdout.flush(), 0);
}

#[test]
fn jump_test_pos_zero() {
  let program = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
  let mut vm = Vm::new(program).unwrap();
  let (mut stdin, mut stdout) = vm.pipes();

  stdin.write(0);
  assert_eq!(vm.exec().unwrap(), State::Done);
  assert_eq!(stdout.flush(), 0);
}

#[test]
fn jump_test_pos_nonzero() {
  let program = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
  let mut vm = Vm::new(program).unwrap();
  let (mut stdin, mut stdout) = vm.pipes();

  stdin.write(5);
  assert_eq!(vm.exec().unwrap(), State::Done);
  assert_eq!(stdout.flush(), 1);
}

#[test]
fn jump_test_imm_zero() {
  let program = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
  let mut vm = Vm::new(program).unwrap();
  let (mut stdin, mut stdout) = vm.pipes();

  stdin.write(0);
  assert_eq!(vm.exec().unwrap(), State::Done);
  assert_eq!(stdout.flush(), 0);
}

#[test]
fn jump_test_imm_nonzero() {
  let program = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
  let mut vm = Vm::new(program).unwrap();
  let (mut stdin, mut stdout) = vm.pipes();

  stdin.write(5);
  assert_eq!(vm.exec().unwrap(), State::Done);
  assert_eq!(stdout.flush(), 1);
}

#[test]
fn jump_test_lt_8() {
  let program = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,\
                  1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,\
                  999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
  let mut vm = Vm::new(program).unwrap();
  let (mut stdin, mut stdout) = vm.pipes();

  stdin.write(7);
  assert_eq!(vm.exec().unwrap(), State::Done);
  assert_eq!(stdout.flush(), 999);
}

#[test]
fn jump_test_eq_8() {
  let program = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,\
                  1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,\
                  999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
  let mut vm = Vm::new(program).unwrap();
  let (mut stdin, mut stdout) = vm.pipes();

  stdin.write(8);
  assert_eq!(vm.exec().unwrap(), State::Done);
  assert_eq!(stdout.flush(), 1000);
}

#[test]
fn jump_test_gt_8() {
  let program = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,\
                  1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,\
                  999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
  let mut vm = Vm::new(program).unwrap();
  let (mut stdin, mut stdout) = vm.pipes();

  stdin.write(9);
  assert_eq!(vm.exec().unwrap(), State::Done);
  assert_eq!(stdout.flush(), 1001);
}

#[test]
fn quine() {
  let result: [i64; 16] = [109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
  let program = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
  let mut vm = Vm::new(program).unwrap();
  let (_, mut stdout) = vm.pipes();

  assert_eq!(vm.exec().unwrap(), State::Done);
  assert_eq!(stdout.drain(), result);
}

#[test]
fn large_values() {
  let program = "104,1125899906842624,99";
  let mut vm = Vm::new(program).unwrap();
  let (_, mut stdout) = vm.pipes();

  assert_eq!(vm.exec().unwrap(), State::Done);
  assert_eq!(stdout.flush(), 1125899906842624);
}

#[test]
fn large_output() {
  let program = "1102,34915192,34915192,7,4,7,99,0";
  let mut vm = Vm::new(program).unwrap();
  let (_, mut stdout) = vm.pipes();

  assert_eq!(vm.exec().unwrap(), State::Done);
  assert_eq!(stdout.flush(), 1219070632396864);
}
