fn main() {
    use std::{fs, time::Instant};
    
    let input = fs::read_to_string("./input.txt").unwrap();
    let program = load(&input);

    let t1 = Instant::now();
    let calls = part_one(&program);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", calls, t2 - t1);

    let t1 = Instant::now();
    let count = part_two();
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", count, t2 - t1);
}

#[derive(Clone, Copy, Debug)]
enum Value {
    Number(i64),
    Register(char)
}

type Registers = [i64;8];

impl Value {
    fn sub(&self, reg: &mut Registers, n: i64) {
        match self {
            Value::Number(_)   => panic!("Can only 'sub' register"),
            Value::Register(c) => {
                let r = (*c as u8 - b'a') as usize;
                reg[r] -= n
            }
        }
    }

    fn mul(&self, reg: &mut Registers, n: i64) {
        match self {
            Value::Number(_)   => panic!("Can only 'mul' register"),
            Value::Register(c) => {
                let r = (*c as u8 - b'a') as usize;
                reg[r] *= n
            }
        }
    }

    fn set(&self, reg: &mut Registers, n: i64) {
        match self {
            Value::Number(_)   => panic!("Can only 'set' register"),
            Value::Register(c) => {
                let r = (*c as u8 - b'a') as usize;
                reg[r] = n
            }
        }
    }

    fn get(&self, reg: &Registers) -> i64 {
        match self {
            Value::Number(n)   => *n,
            Value::Register(c) => {
                let r = (*c as u8 - b'a') as usize;
                reg[r]
            }
        }
    }
}

struct State {
    ip: usize,
    reg: Registers,
    mul: i64,
}

impl State {
    fn new() -> State {
        State { ip: 0, reg: [0i64;8], mul: 0 }
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
enum Cmd {
    set(Value, Value),
    sub(Value, Value),
    mul(Value, Value),
    jnz(Value, Value),
}

impl Cmd {
    fn exec(&self, st: &mut State) -> usize {
        use Cmd::*;

        let reg = st.reg;
        match self {
            set(a, b) => { a.set(&mut st.reg, b.get(&reg)); st.ip + 1 },
            sub(a, b) => { a.sub(&mut st.reg, b.get(&reg)); st.ip + 1 },
            mul(a, b) => {
                a.mul(&mut st.reg, b.get(&reg));
                st.mul += 1;
                st.ip + 1
            },
            jnz(a, b) => {
                if a.get(&st.reg) != 0 { 
                    st.ip + b.get(&st.reg) as usize
                } else {
                    st.ip + 1
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
        let p1 = get_value(it.next().unwrap());
        let p2 = get_value(it.next().unwrap());

        match cmd {
            "set" => set(p1, p2),
            "sub" => sub(p1, p2),
            "mul" => mul(p1, p2),
            "jnz" => jnz(p1, p2),
            _ => panic!("Unknown command: {}", cmd)
        }
    })
    .collect()
}

fn part_one(program: &[Cmd]) -> i64 {
    let mut st = State::new();

    while st.ip < program.len() {
        st.ip = program[st.ip].exec(&mut st);
    }

    st.mul
}

fn part_two() -> usize {
    let sqrt = |n: i64| (n as f64).sqrt() as i64;
    let not_prime = |n: i64| (2..sqrt(n) + 1).any(|v| n % v == 0);
    
    // From notes & internet...sigh
    (106700..123717)
        .filter(|n| (n - 106700) % 17 == 0)
        .filter(|n| not_prime(*n))
        .count()
}

fn get_value(s: &str) -> Value {
    if let Ok(n) = s.parse::<i64>() {
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
    
        let calls = part_one(&program);
        assert_eq!(calls, 4225);
    
        let count = part_two();
        assert_eq!(count, 905);
    }
}
