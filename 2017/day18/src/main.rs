
fn main() {
    use std::{fs, time::Instant};
    
    let input = fs::read_to_string("./input.txt").unwrap();
    let program = load(&input);

    let t1 = Instant::now();
    let frequency = part_one(&program);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", frequency, t2 - t1);

    let t1 = Instant::now();
    let sent = part_two(&program);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", sent, t2 - t1);
}

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

struct State {
    ip: usize,
    reg: Registers,
}

impl State {
    fn new() -> State {
        State { ip: 0, reg: [0i64;5] }
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
    rec(Value),
    rcv(Value),
    jgz(Value, Value),
}

impl Cmd {
    fn exec(&self, st: &mut State, is: &mut Vec<i64>, os: &mut Vec<i64>) -> Option<usize> {
        use Cmd::*;

        let reg = st.reg;
        match self {
            snd(a) => { os.push(a.get(&st.reg)); Some(st.ip + 1) },
            rec(a) => { if a.get(&st.reg) != 0 { is.push(*os.last().unwrap()) }; Some(st.ip + 1) },
            set(a, b) => { a.set(&mut st.reg, b.get(&reg)); Some(st.ip + 1) },
            add(a, b) => { a.add(&mut st.reg, b.get(&reg)); Some(st.ip + 1) },
            mul(a, b) => { a.mul(&mut st.reg, b.get(&reg)); Some(st.ip + 1) },
            rem(a, b) => { a.rem(&mut st.reg, b.get(&reg)); Some(st.ip + 1) },
            jgz(a, b) => {
                if a.get(&st.reg) > 0 { 
                    Some(st.ip + b.get(&st.reg) as usize)
                } else {
                    Some(st.ip + 1)
                }
            },
            rcv(a) => {
                if is.len() > 0 {
                    let n = is.remove(0);
                    a.set(&mut st.reg, n);
                    Some(st.ip + 1)
                } else {
                    None
                }
            },
        }
    }
}

fn load(input: &str) -> Vec<Cmd> {
    use Cmd::*;

    input.lines().map(|s| {
        let mut it = s.split(' ');
        let cmd = it.next().unwrap();
        let p1  = get_value(it.next().unwrap());

        match cmd {
            "snd" => snd(p1),
            "rcv" => rcv(p1),
            "set" => set(p1, get_value(it.next().unwrap())),
            "add" => add(p1, get_value(it.next().unwrap())),
            "mul" => mul(p1, get_value(it.next().unwrap())),
            "mod" => rem(p1, get_value(it.next().unwrap())),
            "jgz" => jgz(p1, get_value(it.next().unwrap())),
            _ => panic!("Unknown command: {}", cmd)
        }
    })
    .collect()
}

fn part_one(program: &[Cmd]) -> i64 {
    use Cmd::*;

    let mut is = vec![];
    let mut os = vec![];
    let mut st = State::new();

    // Version 1 uses "recover" vs. "receive"
    let cmds = program.iter()
        .map(|cmd| if let rcv(a) = cmd { rec(*a) } else { *cmd })
        .collect::<Vec<_>>();

    while st.ip < cmds.len() && is.len() == 0 {
        if let Some(ip) = cmds[st.ip].exec(&mut st, &mut is, &mut os) {
            st.ip = ip
        }
    }

    *is.last().unwrap()
}

fn part_two(program: &[Cmd]) -> i32 {
    let mut s1 = vec![];
    let mut s2 = vec![];

    let mut sends = [0i32;2];
    let mut state = [State::new(), State::new()];
    let mut p = 0;

    // Set program id to 1
    state[1].reg[2] = 1;

    sends[0] = run(program, &mut state[0], &mut s1, &mut s2);
    while s1.len() > 0 || s2.len() > 0 {
        p = 1 - p;  // switch programs

        // Tired of fighting with the borrow checker so we'll switch
        // the streams back and forth manually.
        if p == 0 {
            sends[p] += run(program, &mut state[p], &mut s1, &mut s2);
        } else {
            sends[p] += run(program, &mut state[p], &mut s2, &mut s1);
        }
    }

    sends[1]
}

fn run(cmds: &[Cmd], st: &mut State, is: &mut Vec<i64>, os: &mut Vec<i64>) -> i32 {
    use Cmd::*;

    // Run until the program is done or we block waiting for input.
    let mut sends = 0;

    loop {
        if let Some(ip) = cmds[st.ip].exec(st, is, os) {
            // Count the number of "send" commands executed.
            if let snd(_) = cmds[st.ip] { sends += 1 }
            st.ip = ip;
            if st.ip >= cmds.len() {
                break sends
            }
        } else {
            break sends
        }
    }
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
    
        let sends = part_two(&program);
        assert_eq!(sends, 6858);
    }

    #[test]
    fn examples() {
        let input = fs::read_to_string("./example.txt").unwrap();
        let program = load(&input);
    
        let frequency = part_one(&program);
        assert_eq!(frequency, 4);
    }
}
