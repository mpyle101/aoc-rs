
#[derive(Clone, Copy, Debug)]
enum Value {
    Number(i64),
    Register(char)
}

type Registers = [i64;5];

impl Value {
    fn add(&self, reg: &mut Registers, n: i64) {
        match self {
            Value::Number(_)   => panic!("Can only 'add' register"),
            Value::Register(c) => {
                let r = (*c as u8 - 'a' as u8) as usize;
                reg[r] += n
            }
        }
    }

    fn mul(&self, reg: &mut Registers, n: i64) {
        match self {
            Value::Number(_)   => panic!("Can only 'mul' register"),
            Value::Register(c) => {
                let r = (*c as u8 - 'a' as u8) as usize;
                reg[r] *= n
            }
        }
    }

    fn rem(&self, reg: &mut Registers, n: i64) {
        match self {
            Value::Number(_)   => panic!("Can only 'rem' register"),
            Value::Register(c) => {
                let r = (*c as u8 - 'a' as u8) as usize;
                reg[r] %= n
            }
        }
    }

    fn set(&self, reg: &mut Registers, n: i64) {
        match self {
            Value::Number(_)   => panic!("Can only 'set' register"),
            Value::Register(c) => {
                let r = (*c as u8 - 'a' as u8) as usize;
                reg[r] = n
            }
        }
    }

    fn get(&self, reg: &Registers) -> i64 {
        match self {
            Value::Number(n)   => *n,
            Value::Register(c) => {
                let r = (*c as u8 - 'a' as u8) as usize;
                reg[r]
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
enum Cmd {
    snd(Value),
    set(Value, Value),
    add(Value, Value),
    mul(Value, Value),
    rem(Value, Value),
    rcv(Value),
    jgz(Value, Value),
}

impl Cmd {
    fn exec(&self, ip: usize, reg: &mut Registers, freq: &mut i64, out: &mut i64) -> usize {
        use Cmd::*;

        match self {
            snd(a) => { *freq = a.get(reg); ip+1 },
            rcv(a) => { if a.get(reg) != 0 { *out = *freq }; ip+1 },
            set(a, b) => { a.set(reg, b.get(reg)); ip+1 },
            add(a, b) => { a.add(reg, b.get(reg)); ip+1 },
            mul(a, b) => { a.mul(reg, b.get(reg)); ip+1 },
            rem(a, b) => { a.rem(reg, b.get(reg)); ip+1 },
            jgz(a, b) => {
                if a.get(reg) > 0 { 
                    (ip as i64 + b.get(reg)) as usize
                } else {
                    ip+1
                }
            },
        }
    }
}


fn main() {
    use std::{fs, time::Instant};
    
    let input = fs::read_to_string("./input.txt").unwrap();
    let program = load(&input);

    let t1 = Instant::now();
    let frequency = part_one(&program);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", frequency, t2 - t1);
}

fn load(input: &str) -> Vec<Cmd> {
    use Cmd::*;

    input.lines().map(|s| {
        let mut it = s.split(' ');
        let cmd = it.next().unwrap();
        let p1  = get_value(it.next().unwrap());

        match cmd {
            "snd" => snd(p1),
            "set" => set(p1, get_value(it.next().unwrap())),
            "add" => add(p1, get_value(it.next().unwrap())),
            "mul" => mul(p1, get_value(it.next().unwrap())),
            "mod" => rem(p1, get_value(it.next().unwrap())),
            "rcv" => rcv(p1),
            "jgz" => jgz(p1, get_value(it.next().unwrap())),
            _ => panic!("Unknown command: {}", cmd)
        }
    })
    .collect()
}

fn part_one(program: &[Cmd]) -> i64 {
    let mut ip   = 0;
    let mut out  = 0;
    let mut reg  = [0i64;5];
    let mut freq = 0;

    while ip < program.len() && out == 0 {
        let cmd = program[ip];
        ip = cmd.exec(ip, &mut reg, &mut freq, &mut out);
    }

    out
}

fn get_value(s: &str) -> Value {
    if let Ok(n) = s.parse::<i64>() {
        Value::Number(n)
    } else {
        Value::Register(s.chars().nth(0).unwrap())
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
    
        let frequency = part_one(&program);
        assert_eq!(frequency, 4601);
    }

    #[test]
    fn examples() {
        let input = fs::read_to_string("./example.txt").unwrap();
        let program = load(&input);
    
        let frequency = part_one(&program);
        assert_eq!(frequency, 4);
    }
}
