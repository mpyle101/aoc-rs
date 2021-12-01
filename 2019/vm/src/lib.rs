use anyhow::{bail, Result};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct Pipe {
  q: Rc<RefCell<VecDeque<i64>>>
}

impl Pipe {
  pub fn new() -> Self {
    Pipe { q: Rc::new(RefCell::new(VecDeque::new())) }
  }

  pub fn from(other: &Pipe) -> Self {
    Pipe { q: Rc::clone(&other.q) }
  }

  pub fn read(&mut self) -> Option<i64> {
    (*self.q).borrow_mut().pop_front()
  }

  pub fn write<T>(&mut self, v: T) 
    where T: std::convert::Into<i64>
  {
    (*self.q).borrow_mut().push_back(v.into())
  }

  pub fn flush(&mut self) -> i64 {
    let mut q = (*self.q).borrow_mut();
    let v = q.pop_back().unwrap_or(-1i64);
    q.clear();
    v
  }

  pub fn drain(&mut self) -> Vec<i64> {
    let mut q = (*self.q).borrow_mut();
    q.drain(..).collect::<Vec<i64>>()
  }

  pub fn connect(&mut self, other: &Pipe) {
    self.q = Rc::clone(&other.q)
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
  Ready,
  Running,
  Waiting,
  Done,
}

#[derive(Debug)]
pub struct Vm {
  ip: usize,
  base: i64,
  state: State,
  input: Pipe,
  output: Pipe,
  memory: [i64; 8192],
  backup: [i64; 8192],
}

impl Vm {
  pub fn new(program: &str) -> Result<Self> {
    let mut vm = Vm { 
      memory: [0; 8192],
      backup: [0; 8192],
      ip: 0,
      base: 0,
      state: State::Ready,
      input: Pipe::new(),
      output: Pipe::new()
    };

    // Copy instructions into backup memory
    program
      .split(',')
      .enumerate()
      .try_for_each(|(i, s)| -> Result<()> {
        let v = s.parse::<i64>()?;
        vm.backup[i] = v;
        Ok(())
      })?;

    Ok(vm)
  }

  pub fn pipes(&self) -> (Pipe, Pipe) {
    (Pipe::from(&self.input), Pipe::from(&self.output))
  }

  pub fn is_done(&self) -> bool {
    match self.state {
      State::Done => true,
      _ => false
    }
  }

  pub fn is_running(&self) -> bool {
    match self.state {
      State::Running | State::Waiting => true,
      _ => false
    }
  }

  pub fn write<T>(&mut self, v: T) 
    where T: std::convert::Into<i64>
  {
    self.input.write(v)
  }

  pub fn read(&mut self) -> Option<i64> {
    self.output.read()
  }

  pub fn drain(&mut self) -> Vec<i64> {
    self.output.drain()
  }

  pub fn connect(&mut self, other: &mut Vm) {
    other.input.connect(&self.output)
  }

  pub fn exec(&mut self) -> Result<State> {
    // Restore the program to original state
    self.memory = self.backup;
    self.ip = 0;
    self.base = 0;
    self.run()?;

    Ok(self.state)
  }

  pub fn cont(&mut self) -> Result<State> {
    if self.is_running() {
      self.run()?;
    }

    Ok(self.state)
  }

  pub fn set_addr(&mut self, addr: usize, v: i64) {
    self.memory[addr] = v
  }

  pub fn edit_program(&mut self, addr: usize, v: i64) {
    self.backup[addr] = v
  }

  fn run(&mut self) -> Result<State> {
    self.state = State::Running;
    while self.state == State::Running {
      let inst = self.memory[self.ip];
      let (opc, inst) = getop(inst)?;
      let (m_a, inst) = getm(inst)?;
      let (m_b, inst) = getm(inst)?;
      let (m_c, _) = getm(inst)?;

      self.ip = match opc  {
        Op::Add => {
          let a = self.getv(m_a, self.ip + 1);
          let b = self.getv(m_b, self.ip + 2);
          self.setv(m_c, self.ip + 3, a+b)?;
          self.ip + 4
        },
        Op::Mul => {
          let a = self.getv(m_a, self.ip + 1);
          let b = self.getv(m_b, self.ip + 2);
          self.setv(m_c, self.ip + 3, a*b)?;
          self.ip + 4
        },
        Op::Read => {
          // read-from-input
          if let Some(v) = self.input.read() {
            self.setv(m_a, self.ip + 1, v)?;
            self.ip + 2
          } else {
            self.state = State::Waiting;
            self.ip
          }
        },
        Op::Jt => {
          // jump-if-true
          let a = self.getv(m_a, self.ip + 1);
          let b = self.getv(m_b, self.ip + 2);
          if a != 0 { b as usize } else { self.ip + 3 }
        },
        Op::Jf => {
          // jump-if-false
          let a = self.getv(m_a, self.ip + 1);
          let b = self.getv(m_b, self.ip + 2);
          if a == 0 { b as usize } else { self.ip + 3 }
        },
        Op::Lt => {
          let a = self.getv(m_a, self.ip + 1);
          let b = self.getv(m_b, self.ip + 2);
          let v = if a < b { 1 } else { 0 };
          self.setv(m_c, self.ip + 3, v)?;
          self.ip + 4
        },
        Op::Eq => {
          let a = self.getv(m_a, self.ip + 1);
          let b = self.getv(m_b, self.ip + 2);
          let v = if a == b { 1 } else { 0 };
          self.setv(m_c, self.ip + 3, v)?;
          self.ip + 4
        },
        Op::Arb => {
          // adjust-relative-base
          let a = self.getv(m_a, self.ip + 1);
          self.base += a;
          self.ip + 2
        },
        Op::Write => {
          // write-to-output
          let a = self.getv(m_a, self.ip + 1);
          self.output.write(a);
          self.ip + 2
        },
        Op::Halt => {
          self.state = State::Done;
          0
        }
      }
    }

    Ok(self.state)
  }

  fn getv(&self, mode: Mode, pos: usize) -> i64 {
    let v = self.memory[pos];
    match mode {
      Mode::Position  => self.memory[v as usize],
      Mode::Relative  => self.memory[(v + self.base) as usize],
      Mode::Immediate => v,
    }
  }

  fn setv(&mut self, mode: Mode, pos: usize, val: i64) -> Result<()> {
    let v = self.memory[pos];
    Ok(match mode {
      Mode::Position  => self.memory[v as usize] = val,
      Mode::Relative  => self.memory[(v + self.base) as usize] = val,
      Mode::Immediate => bail!("Immediate mode not allowed for setting values"),
    })
  }
}

#[derive(Debug)]
enum Op {
  Add = 1,
  Mul = 2,
  Read = 3,
  Write = 4,
  Jt = 5,
  Jf = 6,
  Lt = 7,
  Eq = 8,
  Arb = 9,
  Halt = 99,
}

#[derive(Clone, Copy, Debug)]
enum Mode {
  Position = 0,
  Immediate = 1,
  Relative = 2,
}

fn getop(inst: i64) -> Result<(Op, i64)> {
  let op = match inst % 100 {
     1 => Op::Add,
     2 => Op::Mul,
     3 => Op::Read,
     4 => Op::Write,
     5 => Op::Jt,
     6 => Op::Jf,
     7 => Op::Lt,
     8 => Op::Eq,
     9 => Op::Arb,
    99 => Op::Halt,
    _  => bail!("Unknown opcode encountered: {}:{}", inst, inst % 100), 
  };

  Ok((op, inst / 100))
}

fn getm(inst: i64) -> Result<(Mode, i64)> {
  let mode = match inst % 10 {
    0 => Mode::Position,
    1 => Mode::Immediate,
    2 => Mode::Relative,
    _ => bail!("Invalid mode encountered: {}:{}", inst, inst % 10),
  };

  Ok((mode, inst / 10))
}
