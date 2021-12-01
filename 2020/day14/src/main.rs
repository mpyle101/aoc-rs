fn main() {
    let cmds = load(include_str!("./program.txt"));

    let checksum = part_one(&cmds);
    println!("Part 1: {}", checksum);

    let checksum = part_two(&cmds);
    println!("Part 2: {}", checksum);
}

fn part_one(cmds: &[Cmd]) -> u64 {
    use std::collections::HashMap;

    let mut memory = HashMap::new();

    let mut mask = (0u64, 0u64);
    for cmd in cmds {
        match cmd {
            Cmd::Mem { addr, val } => {
                memory.insert(addr, (val | mask.1) & mask.0);
            },
            Cmd::Mask { pattern } => {
                mask = generate_mask(pattern);
            }
        }
    }

    memory.values().sum()
}

fn part_two(cmds: &[Cmd]) -> u64 {
    use std::collections::HashMap;

    let mut memory = HashMap::new();

    let mut mask = "";
    for cmd in cmds {
        match cmd {
            Cmd::Mem { addr, val } => {
                let addrs = apply_mask(mask, addr);
                addrs.iter().for_each(|a| { memory.insert(*a, *val); })
            },
            Cmd::Mask { pattern } => mask = pattern,
        }
    }

    memory.values().sum()
}

enum Cmd<'a> {
    Mem { addr: u64, val: u64 },
    Mask { pattern: &'a str },
}

fn load(input: &str) -> Vec<Cmd> {
    input.lines()
        .map(|s| s.split(" = ").collect::<Vec<&str>>())
        .map(|v| if v[0] == "mask" { 
            Cmd::Mask { pattern: v[1] }
        } else {
            Cmd::Mem {
                addr: v[0][4..v[0].len() - 1].parse::<u64>().unwrap(),
                val: v[1].parse::<u64>().unwrap()
            }
        })
        .collect()
}

fn generate_mask(pattern: &str) -> (u64, u64) {
    let (mut zeros, ones) = pattern.as_bytes().iter().rev()
        .enumerate()
        .fold((0, 0), |(zeros, ones), (i, b)|
            match b {
                b'1' => (zeros, ones | 1 << i),
                b'0' => (zeros | 1 << i, ones),
                _    => (zeros, ones)
            }
        );

    // Flip the bits and clear those beyond 36
    zeros = !zeros;
    (36..64).for_each(|n| zeros &= !(1 << n));

    (zeros, ones)
}

fn apply_mask(pattern: &str, addr: &u64) -> Vec<u64> {
    use std::collections::HashMap;
    let mut cache = HashMap::new();

    let addr_bits = format!("{:036b}", addr);
    let result = pattern.as_bytes().iter().zip(
        addr_bits.as_bytes().iter())
        .map(|(p, a)| match p {
            b'0' => *a,
            b'1' => b'1',
            _    => b'X'
        })
        .collect::<Vec<_>>();
        
    let pos: Vec<_> = result.iter().enumerate()
        .filter(|(_, &v)| v == b'X').map(|(i, _)| i)
        .collect();
    let cmb = cache.entry(pos.len()).or_insert_with_key(|&k| combinations(k));

    cmb.iter().map(|v| {
        let mut m = result.clone();
        v.iter().zip(pos.iter()).for_each(|(&b, &i)| m[i] = b);
        m
    })
    .map(|v| generate_value(&v))
    .collect()
}

fn combinations(len: usize) -> Vec<Vec<u8>> {
    let v = Vec::new();
    let mut c = Vec::new();
    generate_combination(len, v, &mut c);

    c
}

fn generate_value(bits: &[u8]) -> u64 {
    bits.iter().rev()
        .enumerate()
        .fold(0, |acc, (i, b)|
            match b {
                b'1' => acc | 1 << i,
                _    => acc
            }
        )
}

fn generate_combination(len: usize, v: Vec<u8>, c: &mut Vec<Vec<u8>>) {
    if len == 0 {
        c.push(v);
    } else {
        let mut w = v.clone(); w.push(b'0');
        generate_combination(len - 1, w, c);
        let mut w = v.clone(); w.push(b'1');
        generate_combination(len - 1, w, c);
    }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let cmds = load(include_str!("./program.txt"));

    let checksum = part_one(&cmds);
    assert_eq!(checksum, 12512013221615);

    let checksum = part_two(&cmds);
    assert_eq!(checksum, 3905642473893);
  }

  #[test]
  fn example_1() {
    let cmds = load("\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0");

    let checksum = part_one(&cmds);
    assert_eq!(checksum, 165);
  }
}
