fn main() {
    use std::time::Instant;

    let (ipr, program) = load(include_str!("./input.txt"));

    let t1 = Instant::now();
    let value = part_one(ipr, &program);
    let t2 = Instant::now();
    println!("Part 1: {}  ({:?})", value, t2 - t1);

    let t1 = Instant::now();
    let value = part_two();
    let t2 = Instant::now();
    println!("Part 2: {}  ({:?})", value, t2 - t1);
}

type Registers = [i32;6];
type Program = Vec<Opcode>;

fn load(input: &str) -> (usize, Program) {
    use Opcode::*;

    let mut it = input.lines();
    let line = it.next().unwrap();
    let mut iter = line.split(" ");
    iter.next();
    let ipr = iter.next().unwrap().parse::<usize>().unwrap();

    let program = it.map(|s| {
        let mut iter = s.split(" ");
        let opc = iter.next().unwrap();
        let a = iter.next().unwrap().parse::<i32>().unwrap();
        let b = iter.next().unwrap().parse::<i32>().unwrap();
        let c = iter.next().unwrap().parse::<i32>().unwrap();

        match opc {
            "setr" => setr(a, b, c),
            "seti" => seti(a, b, c),
            "addr" => addr(a, b, c),
            "addi" => addi(a, b, c),
            "mulr" => mulr(a, b, c),
            "muli" => muli(a, b, c),
            "banr" => banr(a, b, c),
            "bani" => bani(a, b, c),
            "boor" => borr(a, b, c),
            "bori" => bori(a, b, c),
            "gtir" => gtir(a, b, c),
            "gtri" => gtri(a, b, c),
            "gtrr" => gtrr(a, b, c),
            "eqir" => eqir(a, b, c),
            "eqri" => eqri(a, b, c),
            "eqrr" => eqrr(a, b, c),
            _ => panic!("Unknown opcode: {}", opc)
        }
    })
    .collect::<Vec<_>>();

    (ipr, program)
}

fn part_one(ipr: usize, program: &[Opcode]) -> i32 {
    let mut ip = 0;
    
    let mut reg = [0i32;6];
    while ip < program.len() {
        ip = program[ip].exec(ip, ipr, &mut reg);
    }
    
    reg[0]
}

fn part_two() -> i64 {
    // From looking at a dump out of the register contents
    // after setting reg[0] to 1 and running the program, we
    // see reg[1] gets set to 10551311. The sum of divisors
    // for that number will wind up in reg[0].
    let n = 10551311;

    let mut sum = 0;
    for i in 1..((n as f64).sqrt() as i64) {
        if n % i == 0 {
            sum += i;
            sum += n / i;
        }
    }

    sum
}


#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Opcode {
    setr(i32, i32, i32), seti(i32, i32, i32),
    addr(i32, i32, i32), addi(i32, i32, i32),
    mulr(i32, i32, i32), muli(i32, i32, i32),
    banr(i32, i32, i32), bani(i32, i32, i32),
    borr(i32, i32, i32), bori(i32, i32, i32),
    gtir(i32, i32, i32), gtri(i32, i32, i32), gtrr(i32, i32, i32),
    eqir(i32, i32, i32), eqri(i32, i32, i32), eqrr(i32, i32, i32),
}

impl Opcode {
    fn exec(&self, ip: usize, ipr: usize, reg: &mut Registers) -> usize{
        use Opcode::*;

        let r = reg;
        r[ipr] = ip as i32;
        match self {
            setr(a, _, c) => r[*c as usize] = r[*a as usize],
            seti(a, _, c) => r[*c as usize] = *a,
            addr(a, b, c) => r[*c as usize] = r[*a as usize] + r[*b as usize],
            addi(a, b, c) => r[*c as usize] = r[*a as usize] + b,
            mulr(a, b, c) => r[*c as usize] = r[*a as usize] * r[*b as usize],
            muli(a, b, c) => r[*c as usize] = r[*a as usize] * b,
            banr(a, b, c) => r[*c as usize] = r[*a as usize] & r[*b as usize],
            bani(a, b, c) => r[*c as usize] = r[*a as usize] & b,
            borr(a, b, c) => r[*c as usize] = r[*a as usize] | r[*b as usize],
            bori(a, b, c) => r[*c as usize] = r[*a as usize] | b,
            gtir(a, b, c) => r[*c as usize] = (*a > r[*b as usize]) as i32,
            gtri(a, b, c) => r[*c as usize] = (r[*a as usize] > *b) as i32,
            gtrr(a, b, c) => r[*c as usize] = (r[*a as usize] > r[*b as usize]) as i32,
            eqir(a, b, c) => r[*c as usize] = (*a == r[*b as usize]) as i32,
            eqri(a, b, c) => r[*c as usize] = (r[*a as usize] == *b) as i32,
            eqrr(a, b, c) => r[*c as usize] = (r[*a as usize] == r[*b as usize]) as i32,
        }

        (r[ipr] + 1) as usize
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (ip_reg, program) = load(include_str!("./input.txt"));

        let value = part_one(ip_reg, &program);
        assert_eq!(value, 912);

        let value = part_two();
        assert_eq!(value, 10576224);
    }
}
