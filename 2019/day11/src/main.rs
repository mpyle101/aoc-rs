use anyhow::Result;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use vm::{State, Vm};

fn main() {
  let program = include_str!("./program.txt");
  let mut robot = Robot::from(program).unwrap();
  robot.paint(Color::White).unwrap();
  println!("Panels painted: {}", robot.painted().len());

  let (height, width) = robot.dimensions();
  let painted = robot.painted();
  for y in 0..=height as i32 {
    for x in 0..width as i32 {
      let panel = Panel { x, y: -y };
      match painted.get(&panel).unwrap_or(&Color::Black) {
        Color::Black => print!(" "),
        Color::White => print!("#"),
      }
    }
    println!("")
  }

  // CBLPJZCU
}

#[derive(Clone, Copy, Debug)]
enum Color {
  Black = 0,
  White = 1,
}

#[derive(PartialEq)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Clone, Copy, Debug)]
struct Panel {
  x: i32,
  y: i32,
}
impl Panel {
  pub fn new(x: i32, y: i32) -> Self {
    Panel { x, y }
  }
}
impl Eq for Panel {}
impl Hash for Panel {
  fn hash<H: Hasher>(&self, hasher: &mut H) {
    self.x.hash(hasher);
    self.y.hash(hasher);
  }
}
impl PartialEq for Panel {
  fn eq(&self, other: &Self) -> bool {
    self.x.eq(&other.x) && self.y.eq(&other.y)
  }
}

struct Robot {
  vm: Vm,
  facing: Direction,
  painted: HashMap<Panel, Color>,
}

impl Robot {
  fn from(program: &str) -> Result<Self> {
    Ok(Robot {
      vm: Vm::new(program)?,
      facing: Direction::Up,
      painted: HashMap::new(),
    })
  }

  fn painted(&self) -> &HashMap<Panel, Color> {
    &self.painted
  }

  fn paint(&mut self, start: Color) -> Result<(), &str> {
    let mut loc = Panel { x: 0, y: 0 };
    let (mut stdin, mut stdout) = self.vm.pipes();

    self.painted.insert(loc, start);

    self.vm.exec().map_err(|_| "Exec failed")?;
    while self.vm.cont().map_err(|_| "Continue failed")? != State::Done {
      let color = self.painted.get(&loc).unwrap_or(&Color::Black);
      stdin.write(*color as i64);

      if let Ok(state) = self.vm.cont() {
        if state != State::Done {
          let color = match stdout.read().ok_or("No color to paint")? {
            0 => Color::Black,
            1 => Color::White,
            _ => return Err("Invalid color")
          };
          self.painted.insert(loc, color);

          self.facing = match stdout.read().ok_or("No direction to turn")? {
            0 => self.turn_left(),
            1 => self.turn_right(),
            _ => return Err("Invalid direction")
          };
          loc = match self.facing {
            Direction::Up    => Panel::new(loc.x, loc.y + 1),
            Direction::Down  => Panel::new(loc.x, loc.y - 1),
            Direction::Left  => Panel::new(loc.x - 1, loc.y),
            Direction::Right => Panel::new(loc.x + 1, loc.y),
          };
        }
      }
    }

    Ok(())
  }

  pub fn dimensions(&self) -> (i8, i8) {
    let mut min_x = 0i8;
    let mut min_y = 0i8;
    let mut max_x = 0i8;
    let mut max_y = 0i8;

    for panel in self.painted.keys() {
      min_x = i8::min(min_x, panel.x as i8);
      min_y = i8::min(min_y, panel.y as i8);
      max_x = i8::max(max_x, panel.x as i8);
      max_y = i8::max(max_y, panel.y as i8);
    }

    (max_y - min_y, max_x - min_x)
  }

  fn turn_left(&self) -> Direction {
    match self.facing {
      Direction::Up    => Direction::Left,
      Direction::Down  => Direction::Right,
      Direction::Left  => Direction::Down,
      Direction::Right => Direction::Up,
    }
  }

  fn turn_right(&self) -> Direction {
    match self.facing {
      Direction::Up    => Direction::Right,
      Direction::Down  => Direction::Left,
      Direction::Left  => Direction::Up,
      Direction::Right => Direction::Down,
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let program = include_str!("./program.txt");
    let mut robot = Robot::from(program).unwrap();
    robot.paint(Color::Black).unwrap();

    assert_eq!(robot.painted().len(), 2276);
  }
}
