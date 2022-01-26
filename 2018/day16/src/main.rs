fn main() {
    use std::time::Instant;

    let (samples, program) = load(include_str!("./input.txt"));

    let t1 = Instant::now();
    let count = part_one(&samples);
    let t2 = Instant::now();
    println!("Part 1: {}  ({:?})", count, t2 - t1);

    let t1 = Instant::now();
    let value = part_two(&samples, &program);
    let t2 = Instant::now();
    println!("Part 2: {}  ({:?})", value, t2 - t1);
}

#[derive(Clone, Copy, Debug)]
struct Sample {
    inst: [i32;4],
    reg_a: [i32;4],
    reg_b: [i32;4],
}

type Samples = Vec<Sample>;
type Program = Vec<[i32;4]>;

fn load(input: &str) -> (Samples, Program) {
    let sections = input.split("\n\n\n\n").collect::<Vec<_>>();

    let samples = sections[0].split("\n\n")
        .map(|s| {
            let v = s.lines().collect::<Vec<_>>();
            make_sample(&v)
        })
        .collect::<Vec<_>>();

    let program = sections[1].lines()
        .map(|s| {
            let mut inst = [0i32;4];
            let it = s.split(" ");
            fill(it, &mut inst);
            inst
        })
        .collect::<Vec<_>>();

    (samples, program)
}

fn part_one(samples: &Samples) -> usize {
    use Opcode::*;

    let opcodes = [
        setr, seti,
        addr, addi,
        mulr, muli,
        banr, bani,
        borr, bori,
        gtir, gtri, gtrr,
        eqir, eqri, eqrr
    ];

    samples.iter()
        .filter(|s|
            opcodes.iter()
                .filter(|opc| opc.exec(&s.inst, &s.reg_a) == s.reg_b)
                .count() >= 3
        )
        .count()
}

fn part_two(samples: &Samples, program: &Program) -> i32 {
    use Opcode::*;

    let mut opcodes = [
        setr, seti, addr, addi, mulr, muli, banr, bani, borr, bori,
        gtir, gtri, gtrr, eqir, eqri, eqrr
    ].to_vec();

    let mut cmds = [setr;16];
    let mut samples = samples.clone();

    // Find the samples where only one opcode works and widdle
    // down the number of opcodes and samples as we find the singles.
    while opcodes.len() > 0 {
        let mut used_opcodes = vec![];
        let mut used_samples = vec![];

        samples.iter()
            .enumerate()
            .for_each(|(i, s)| {
                let v = opcodes.iter()
                    .filter_map(|opc| 
                        if opc.exec(&s.inst, &s.reg_a) == s.reg_b {
                            Some(*opc)
                        } else {
                            None
                        }
                    )
                    .collect::<Vec<_>>();

                if v.len() == 1 {
                    used_opcodes.push(v[0]);
                    used_samples.push(i);
                    cmds[s.inst[0] as usize] = v[0];
                }
            });

        for opc in used_opcodes { 
            if let Some(i) = opcodes.iter().position(|op| *op == opc) {
                opcodes.remove(i);
            }
        }

        used_samples.sort();
        for i in used_samples.iter().rev() { 
            samples.remove(*i);
        }
    }

    let mut reg = [0i32;4];
    for inst in program {
        reg = cmds[inst[0] as usize].exec(inst, &reg);
    }

    reg[0]
}

fn make_sample(v: &[&str]) -> Sample {
    let mut inst  = [0i32;4];
    let mut reg_a = [0i32;4];
    let mut reg_b = [0i32;4];

    let it = v[0][9..19].split(", ");
    fill(it, &mut reg_a);
    let it = v[1].split(" ");
    fill(it, &mut inst);
    let it = v[2][9..19].split(", ");
    fill(it, &mut reg_b);

    Sample { inst, reg_a, reg_b }
}

fn fill(it: std::str::Split<&str>, buf: &mut [i32;4]) {
    it.enumerate().for_each(|(i, s)| {
        let n = s.parse::<i32>().unwrap();
        buf[i] = n
    });
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Opcode {
    setr, seti,
    addr, addi,
    mulr, muli,
    banr, bani,
    borr, bori,
    gtir, gtri, gtrr,
    eqir, eqri, eqrr,
}

impl Opcode {
    fn exec(&self, v: &[i32;4], reg: &[i32;4]) -> [i32;4] {
        use Opcode::*;

        let a = v[1] as usize;
        let b = v[2] as usize;
        let c = v[3] as usize;

        let mut r = *reg;
        match self {
            setr => r[c] = r[a],
            seti => r[c] = a as i32,
            addr => r[c] = r[a] + r[b],
            addi => r[c] = r[a] + b as i32,
            mulr => r[c] = r[a] * r[b],
            muli => r[c] = r[a] * b as i32,
            banr => r[c] = r[a] & r[b],
            bani => r[c] = r[a] & b as i32,
            borr => r[c] = r[a] | r[b],
            bori => r[c] = r[a] | b as i32,
            gtir => r[c] = (a as i32 > r[b]) as i32,
            gtri => r[c] = (r[a] > b as i32) as i32,
            gtrr => r[c] = (r[a] > r[b]) as i32,
            eqir => r[c] = (a as i32 == r[b]) as i32,
            eqri => r[c] = (r[a] == b as i32) as i32,
            eqrr => r[c] = (r[a] == r[b]) as i32,
        }

        r
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (samples, program) = load(include_str!("./input.txt"));

        let count = part_one(&samples);
        assert_eq!(count, 521);

        let value = part_two(&samples, &program);
        assert_eq!(value, 594);
    }
}
