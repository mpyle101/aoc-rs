
#[derive(Clone, Copy, Debug)]
enum Value {
    Number(i32),
    Register(char)
}

type Registers = [i32;4];

impl Value {
    fn inc(&self, reg: &mut Registers) {
        match self {
            Value::Number(_)   => panic!("Can only 'inc' register"),
            Value::Register(c) => {
                let r = (*c as u8 - b'a') as usize;
                reg[r] += 1
            }
        }
    }

    fn dec(&self, reg: &mut Registers) {
        match self {
            Value::Number(_)   => panic!("Can only 'dec' register"),
            Value::Register(c) => {
                let r = (*c as u8 - b'a') as usize;
                reg[r] -= 1
            }
        }
    }

    fn set(&self, reg: &mut Registers, n: i32) {
        match self {
            Value::Number(_)   => panic!("Can only 'set' register"),
            Value::Register(c) => {
                let r = (*c as u8 - b'a') as usize;
                reg[r] = n
            }
        }
    }

    fn get(&self, reg: &Registers) -> i32 {
        match self {
            Value::Number(n)   => *n,
            Value::Register(c) => {
                let r = (*c as u8 - b'a') as usize;
                reg[r]
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
enum Cmd {
    cpy(Value, Value),
    dec(Value),
    inc(Value),
    jnz(Value, Value),
    tgl(Value)
}

impl Cmd {
    fn exec(&self, ip: usize, reg: &mut Registers, cmds: &mut [Cmd]) -> usize {
        use Cmd::*;

        match self {
            cpy(v, n) => { n.set(reg, v.get(reg)); ip+1 },
            dec(v) => { v.dec(reg); ip+1 },
            inc(v) => { v.inc(reg); ip+1 },
            jnz(v, n) => {
                if v.get(reg) != 0 { 
                    (ip as i32 + n.get(reg)) as usize
                } else {
                    ip+1
                }
            },
            tgl(v) => {
                let idx = (ip as i32 + v.get(reg)) as usize;
                if idx < cmds.len() {
                    let cmd = cmds[idx];
                    cmds[idx] = match cmd {
                        inc(r) => dec(r),
                        dec(r) => inc(r),
                        tgl(r) => inc(r),
                        jnz(v1, v2) => cpy(v1, v2),
                        cpy(v1, v2) => jnz(v1, v2),
                    };
                }
                ip + 1
            }
        }
    }
}

fn main() {
    use std::{fs, time::Instant};

    let input = fs::read_to_string("./input.txt").unwrap();
    let program = load(&input);

    let t1 = Instant::now();
    let reg = part_one(&program);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", reg, t2 - t1);

    let t1 = Instant::now();
    let reg = part_two(&program);
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", reg, t2 - t1);
}

fn load(input: &str) -> Vec<Cmd> {
    use Cmd::*;

    input.lines().map(|s| {
        let mut it = s.split(' ');
        let cmd = it.next().unwrap();
        let p1  = get_value(it.next().unwrap());

        match cmd {
            "cpy" => cpy(p1, get_value(it.next().unwrap())),
            "dec" => dec(p1),
            "inc" => inc(p1),
            "jnz" => jnz(p1, get_value(it.next().unwrap())),
            "tgl" => tgl(p1),
            _ => panic!("Unknown command: {}", cmd)
        }
    })
    .collect()
}

fn part_one(program: &[Cmd]) -> i32 {
    let mut ip  = 0;
    let mut reg = [7, 0, 0, 0];

    let mut cmds = program.to_vec();
    while ip < program.len() {
        let cmd = cmds[ip];
        ip = cmd.exec(ip, &mut reg, &mut cmds);
    }

    reg[0]
}

fn part_two(program: &[Cmd]) -> i32 {
    let mut ip  = 0;
    let mut reg = [12, 0, 0, 0];

    let mut cmds = program.to_vec();
    while ip < program.len() {
        let cmd = cmds[ip];
        ip = cmd.exec(ip, &mut reg, &mut cmds);
    }

    reg[0]
}

fn get_value(s: &str) -> Value {
    if let Ok(n) = s.parse::<i32>() {
        Value::Number(n)
    } else {
        Value::Register(s.chars().next().unwrap())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let program = load(&input);
    
        let reg = part_one(&program);
        assert_eq!(reg, 11200);
    
        let reg = part_two(&program);
        assert_eq!(reg, 479007760);
    }
}