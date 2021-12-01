fn main() {
    let program = load(include_str!("./program.txt"));

    let acc = part_one(&program);
    println!("Part 1: {}", acc);

    let acc = part_two(&program);
    println!("Part 2: {}", acc);
}

#[derive(Clone, Copy, Debug)]
enum Cmd {
    Nop(i32),
    Jmp(i32),
    Acc(i32)
}

fn load(input: &str) -> Vec<Cmd> {
    input.lines()
        .map(|l| l.split(' ').collect::<Vec<&str>>())
        .map(|v| match v[0] {
            "nop" => Cmd::Nop(v[1].parse::<i32>().unwrap()),
            "jmp" => Cmd::Jmp(v[1].parse::<i32>().unwrap()),
            "acc" => Cmd::Acc(v[1].parse::<i32>().unwrap()),
            _ => panic!("Unknown command found")
        })
        .collect()
}

fn part_one(program: &[Cmd]) -> i32 {
    let (_, acc) = run(program);
    acc
}

fn part_two(program: &[Cmd]) -> i32 {

    let len = program.len();
    for (i, cmd) in program.iter().enumerate() {
        let mut v: Vec<_> = program.iter().copied().collect();
        match cmd {
            Cmd::Acc(_) => {},
            Cmd::Nop(n) => { v[i] = Cmd::Jmp(*n) },
            Cmd::Jmp(n) => { v[i] = Cmd::Nop(*n) },
        }
        let (ip, acc) = run(&v);
        if ip == len as i32 {
            return acc
        }
    }

    0
}

fn run(program: &[Cmd]) -> (i32, i32) {
    use std::collections::HashSet;

    let mut ip  = 0i32;
    let mut acc = 0;
    let mut visited = HashSet::new();

    let len = program.len() as i32;
    loop {
        visited.insert(ip);
        let cmd = &program[ip as usize];        
        ip = match cmd {
            Cmd::Nop(_) => ip + 1,
            Cmd::Acc(n) => { acc += n; ip + 1 },
            Cmd::Jmp(n) => {
                let nextip = ip + n;
                if visited.contains(&nextip) {
                    break
                } else {
                    nextip
                }
            }
        };

        if ip == len {
            break;
        }
    }

    (ip, acc)
}


/** Unit Tests */
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let program = load(include_str!("./program.txt"));

    let acc = part_one(&program);
    assert_eq!(acc, 1489);

    let acc = part_two(&program);
    assert_eq!(acc, 1539);
  }


  #[test]
  fn small() {
    let program = load(include_str!("./test_s.txt"));

    let acc = part_one(&program);
    assert_eq!(acc, 5);
  }
}