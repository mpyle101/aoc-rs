use anyhow::Result;
use itertools::Itertools;
use std::cell::RefCell;
use vm::Vm;

fn main() {
  let program = include_str!("./program.txt");

  let signal = (0..=4).permutations(5)
    .map(|phases| run(program, &phases))
    .map(Result::unwrap)
    .max().unwrap();
  println!("One pass signal: {}", signal);

  let signal = (5..=9).permutations(5)
    .map(|phases| feedback(program, &phases))
    .map(Result::unwrap)
    .max().unwrap();
  println!("Feedback signal: {}", signal);
}

struct Amp {
  vm: RefCell<Vm>,
  phase: i64,
}

impl Amp {
  pub fn from(program: &str, phase: i64) -> Result<Self> {
    let vm = RefCell::new(Vm::new(program)?);
    Ok(Amp { vm, phase })
  }

  pub fn boot(&mut self) -> Result<()> {
    let mut vm = self.vm.borrow_mut();
    let (mut pipe, _) = vm.pipes();
    pipe.write(self.phase);
    vm.exec()?;

    Ok(())
  }

  pub fn cont(&mut self) -> Result<()> {
    let mut vm = self.vm.borrow_mut();
    vm.cont()?;

    Ok(())
  }

  pub fn is_running(&self) -> bool {
    self.vm.borrow().is_running()
  }

  pub fn write(&self, v: i64) {
    let vm = self.vm.borrow_mut();
    let (mut pipe, _) = vm.pipes();
    pipe.write(v)
  }

  pub fn signal(&self) -> i64 {
    let vm = self.vm.borrow_mut();
    let (_, mut pipe) = vm.pipes();
    pipe.flush()
  }

  pub fn connect(&self, other: &Amp) {
    let mut a = self.vm.borrow_mut();
    let mut b = other.vm.borrow_mut();
    a.connect(&mut b);
  }
}

fn load(program: &str, phases: &[i64]) -> Result<Vec<Amp>> {
  let amps = phases.iter().map(|&p| Amp::from(program, p))
    .collect::<Result<Vec<Amp>>>()?;
  for pos in 0..phases.len() - 1 {
    amps[pos].connect(&amps[pos + 1])
  }

  Ok(amps)
}

fn run(program: &str, phases: &[i64]) -> Result<i64> {
  let mut amps = load(program, phases)?;
  amps.iter_mut().try_for_each(|amp| amp.boot())?;

  // Set initial signal value
  amps[0].write(0);
  amps.iter_mut().try_for_each(|amp| amp.cont())?;
  let signal = amps.last().unwrap().signal();

  Ok(signal)
}

fn feedback(program: &str, phases: &[i64]) -> Result<i64> {
  let mut amps = load(program, phases)?;

  // Setup the feedback loop
  amps[4].connect(&amps[0]);
  amps.iter_mut().try_for_each(|amp| amp.boot())?;

  // Set initial signal value
  amps[0].write(0);
  while amps[4].is_running() {
    amps.iter_mut().try_for_each(|amp| amp.cont())?;
  }
  let signal = amps.last().unwrap().signal();
  
  Ok(signal)
}


/** Unit Tests */
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let program = include_str!("./program.txt");
    let mut amp = Amp::from(program, 3).unwrap();
    amp.boot().unwrap();
    amp.write(0);
    amp.cont().unwrap();

    assert_eq!(amp.signal(), 4);
  }

  #[test]
  fn test_simple() {
    let program = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
    let mut amp = Amp::from(program, 3).unwrap();
    amp.boot().unwrap();
    amp.write(0);
    amp.cont().unwrap();

    assert_eq!(amp.signal(), 3);
  }

  #[test]
  fn test_43210() {
    let program = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
    let signal = run(program, &[4,3,2,1,0]).unwrap();

    assert_eq!(signal, 43210);
  }

  #[test]
  fn test_01234() {
    let program = "3,23,3,24,1002,24,10,24,1002,23,-1,23,\
                   101,5,23,23,1,24,23,23,4,23,99,0,0";
    let signal = run(program, &[0,1,2,3,4]).unwrap();

    assert_eq!(signal, 54321);
  }

  #[test]
  fn test_10432() {
    let program = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,\
                   1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
    let signal = run(program, &[1,0,4,3,2]).unwrap();

    assert_eq!(signal, 65210);
  }

  #[test]
  fn test_98765() {
    let program = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,\
                   27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
    let signal = feedback(program, &[9,8,7,6,5]).unwrap();

    assert_eq!(signal, 139629729);
  }

  #[test]
  fn test_97856() {
    let program = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,\
                   -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,\
                   53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
    let signal = feedback(program, &[9,7,8,5,6]).unwrap();

    assert_eq!(signal, 18216);
  }

  #[test]
  fn max_run() {
    let program = include_str!("./program.txt");
    let signal = (0..=4).permutations(5)
      .map(|phases| run(program, &phases))
      .map(Result::unwrap)
      .max().unwrap();

    assert_eq!(signal, 844468);
  }

  #[test]
  fn max_feedback() {
    let program = include_str!("./program.txt");
    let signal = (5..=9).permutations(5)
      .map(|phases| feedback(program, &phases))
      .map(Result::unwrap)
      .max().unwrap();

    assert_eq!(signal, 4215746);
  }
}