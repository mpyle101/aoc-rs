fn main() {
    use std::time::Instant;

    let program = load(include_str!("./input.txt"));

    let t1 = Instant::now();
    let value = part_one(&program);
    let t2 = Instant::now();
    println!("Part 1: {}  ({:?})", value, t2 - t1);

    let t1 = Instant::now();
    let value = part_two(&program);
    let t2 = Instant::now();
    println!("Part 2: {}  ({:?})", value, t2 - t1);
}

fn part_one(program: &[Opcode]) -> i32 {
    let mut program = program.to_vec();

    // Run the program and print out the registers
    // for ip == 28 then match the first value in
    // reg 3 => 3909249.
    let mut reg: [Reg;6] = [0;6];
    reg[0] = 3909249 as Reg;

    let ipr = program.remove(0).eval();
    let mut ip = 0;
    while ip < program.len() {
        // if ip == 28 {
        //     println!("{:?}", reg)
        // }
        ip = program[ip].exec(ip, ipr, &mut reg);
    }
    
    3909249
}

fn part_two(program: &[Opcode]) -> i64 {
    use std::collections::HashSet;

    let mut program = program.to_vec();
    let mut reg: [Reg;6] = [0;6];

    let ipr = program.remove(0).eval();
    let mut ip = 0;
    let mut last = 0;
    let mut seen = HashSet::new();
    loop {
        if ip == 28 {
            if !seen.insert(reg[3]) {
                // Found a cycle so return the previous value since
                // it'll be the first time it's been seen and will
                // have taken the most instructions to produce.
                break last
            } else {
                last = reg[3]
            }
        }
        ip = program[ip].exec(ip, ipr, &mut reg);
    }

    // 12333799
}

type Reg = i64;
type Registers = [Reg;6];
type Program = Vec<Opcode>;

fn load(input: &str) -> Program {
    use Opcode::*;

    input.lines().map(|s| {
        let mut it = s.split(" ");
        let opc = it.next().unwrap();
        let a = read_n(&mut it);
        let b = read_n(&mut it);
        let c = read_n(&mut it);

        match opc {
            "#ip"  => ipx(a),
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
    .collect::<Vec<_>>()
}

fn read_n(it: &mut std::str::Split<&str>) -> Reg {
    it.next().map_or(0, |v| v.parse::<u32>().map_or(0, |n| n as u32)) as Reg
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
enum Opcode {
    ipx(Reg),
    setr(Reg, Reg, Reg), seti(Reg, Reg, Reg),
    addr(Reg, Reg, Reg), addi(Reg, Reg, Reg),
    mulr(Reg, Reg, Reg), muli(Reg, Reg, Reg),
    banr(Reg, Reg, Reg), bani(Reg, Reg, Reg),
    borr(Reg, Reg, Reg), bori(Reg, Reg, Reg),
    gtir(Reg, Reg, Reg), gtri(Reg, Reg, Reg), gtrr(Reg, Reg, Reg),
    eqir(Reg, Reg, Reg), eqri(Reg, Reg, Reg), eqrr(Reg, Reg, Reg),
}

impl Opcode {
    fn exec(&self, ip: usize, ipr: usize, reg: &mut Registers) -> usize{
        use Opcode::*;

        let r = reg;
        r[ipr] = ip as Reg;
        match self {
            ipx(_) => {},
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
            gtir(a, b, c) => r[*c as usize] = (*a > r[*b as usize]) as Reg,
            gtri(a, b, c) => r[*c as usize] = (r[*a as usize] > *b) as Reg,
            gtrr(a, b, c) => r[*c as usize] = (r[*a as usize] > r[*b as usize]) as Reg,
            eqir(a, b, c) => r[*c as usize] = (*a == r[*b as usize]) as Reg,
            eqri(a, b, c) => r[*c as usize] = (r[*a as usize] == *b) as Reg,
            eqrr(a, b, c) => r[*c as usize] = (r[*a as usize] == r[*b as usize]) as Reg,
        }

        (r[ipr] + 1) as usize
    }

    fn eval(&self) -> usize {
        use Opcode::*;

        match self {
            ipx(a) => *a as usize,
            _ => panic!("eval unsupported for {:?}", self)
        }
    }
}
