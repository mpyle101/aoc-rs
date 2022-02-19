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

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum Cmd {
    cpy(i32, usize, bool),
    inc(usize),
    dec(usize),
    jnz(i32, i32, bool),
}

impl Cmd {
    fn exec(&self, ip: usize, reg: &mut [i32;4]) -> usize {
        use Cmd::*;

        match self {
            inc(r) => { reg[*r] += 1; ip+1 },
            dec(r) => { reg[*r] -= 1; ip+1 },
            jnz(r, n, direct) => {
                let x = if *direct { *r } else { reg[*r as usize] as i32 };
                if x != 0 { (ip as i32 + n) as usize } else { ip+1 }
            },
            cpy(n, r, direct) => {
                if *direct {
                    reg[*r] = *n
                } else {
                    reg[*r] = reg[*n as usize]
                }
                ip+1
            },
        }
    }
}


fn load(input: &str) -> Vec<Cmd> {
    use Cmd::*;

    input.lines().map(|s| {
        let mut it = s.split(' ');
        let cmd = it.next().unwrap();
        let reg = it.next().unwrap();
        match cmd {
            "cpy" => {
                let y = it.next().unwrap().chars().next().unwrap();
                let r = (y as u8 - b'a') as usize;
                if let Ok(x) = reg.parse::<i32>() {
                    cpy(x, r, true)
                } else {
                    let x = reg.chars().next().unwrap();
                    cpy(x as i32 - 'a' as i32, r, false)
                }
            },
            "inc" => {
                let x = reg.chars().next().unwrap();
                let r = (x as u8 - b'a') as usize;
                inc(r)
            },
            "dec" => {
                let x = reg.chars().next().unwrap();
                let r = (x as u8 - b'a') as usize;
                dec(r)
            },
            "jnz" => {
                let y = it.next().unwrap().parse::<i32>().unwrap();
                if let Ok(x) = reg.parse::<i32>() {
                    jnz(x, y, true)
                } else {
                    let x = reg.chars().next().unwrap();
                    jnz(x as i32 - 'a' as i32, y, false)
                }
            }
            _ => panic!("Unknown command: {}", cmd)
        }
        
    })
    .collect()
}

fn part_one(program: &[Cmd]) -> i32 {
    let mut ip  = 0;
    let mut reg = [0i32;4];

    while ip < program.len() {
        ip = program[ip].exec(ip, &mut reg);
    }

    reg[0]
}

fn part_two(program: &[Cmd]) -> i32 {
    let mut ip  = 0;
    let mut reg = [0, 0, 1, 0];

    while ip < program.len() {
        ip = program[ip].exec(ip, &mut reg);
    }

    reg[0]
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
        assert_eq!(reg, 318083);
    
        let reg = part_two(&program);
        assert_eq!(reg, 9227737);
    }
}