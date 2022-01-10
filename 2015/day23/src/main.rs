
fn main() {
    use std::time::Instant;

    let program = load(include_str!("./input.txt"));

    let t1 = Instant::now();
    let rb = part_one(&program);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", rb, t2 - t1);

    let t1 = Instant::now();
    let rb = part_two(&program);
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", rb, t2 - t1);
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum Cmd {
    hlf(usize),
    tpl(usize),
    inc(usize),
    jmp(i32),
    jie(usize, i32),
    jio(usize, i32),
}

impl Cmd {
    fn exec(&self, ip: usize, reg: &mut [u32;2]) -> usize {
        use Cmd::*;

        match self {
            hlf(r) => { reg[*r] /= 2; ip+1 },
            tpl(r) => { reg[*r] *= 3; ip+1 },
            inc(r) => { reg[*r] += 1; ip+1 },
            jmp(n) => (ip as i32 + n) as usize,
            jie(r, n) => if reg[*r] % 2 == 0 { (ip as i32 + n) as usize } else { ip+1 },
            jio(r, n) => if reg[*r] == 1 { (ip as i32 + n) as usize } else { ip+1 },
        }
    }
}

fn load(input: &str) -> Vec<Cmd> {
    use Cmd::*;

    input.lines().map(|l| {
        let mut it = l.split(' ');
        let cmd = it.next().unwrap();
        let reg = it.next().unwrap();
        if cmd == "jmp" {
            let offset = reg.parse::<i32>().unwrap();
            jmp(offset)
        } else {
            let reg = if reg.chars().nth(0) == Some('a') { 0 } else { 1 };
            if cmd == "jie" || cmd == "jio" {
                let offset = it.next().unwrap().parse::<i32>().unwrap();
                if cmd == "jie" { jie(reg, offset) } else { jio(reg, offset) }
            } else if cmd == "hlf" {
                hlf(reg)
            } else if cmd == "tpl" {
                tpl(reg)
            } else {
                inc(reg)
            }
        }
    })
    .collect()
}

fn part_one(program: &[Cmd]) -> u32 {
    let mut ip  = 0;
    let mut reg = [0u32;2];
    while ip < program.len() {
        ip = program[ip].exec(ip, &mut reg);
    }

    reg[1]
}

fn part_two(program: &[Cmd]) -> u32 {
    let mut ip  = 0;
    let mut reg: [u32;2] = [1, 0];
    while ip < program.len() {
        ip = program[ip].exec(ip, &mut reg);
    }

    reg[1]
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let program = load(include_str!("./input.txt"));

    let regb = part_one(&program);
    assert_eq!(regb, 255);

    let regb = part_two(&program);
    assert_eq!(regb, 334);
  }
}