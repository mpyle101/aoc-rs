fn main() {
    use std::{fs, time::Instant};

    let input = fs::read_to_string("./input.txt").unwrap();

    let t1 = Instant::now();
    let len = part_one(&input);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", len, t2 - t1);

    let t1 = Instant::now();
    let len = part_two(&input);
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", len, t2 - t1);
}

fn part_one(input: &str) -> u32 {
    let buf = input.chars().collect::<Vec<_>>();

    let mut i = 0;
    let mut count = 0;
    while i < input.len() {
        if buf[i] == '(' {
            let mut j = i + 1;
            while buf[j] != ')' { j+= 1 }
            let (n, x) = parse_marker(&buf[i+1..j]);
            count += (n * x) as u32;
            i = j + n + 1;
        } else {
            count += 1;
            i += 1
        }
    }

    count
}

fn part_two(input: &str) -> u64 {
    let buf = input.chars().collect::<Vec<_>>();
    expand(&buf)
}

fn parse_marker(buf: &[char]) -> (usize, usize) {
    let s = buf.iter().collect::<String>();
    let mut it = s.split('x');
    (
        it.next().unwrap().parse::<usize>().unwrap(),
        it.next().unwrap().parse::<usize>().unwrap(),
    )
}

fn expand(buf: &[char]) -> u64 {
    let mut i = 0;
    let mut count = 0;

    while i < buf.len() {
        if buf[i] == '(' {
            let mut j = i + 1;
            while buf[j] != ')' { j+= 1 }
            let (n, x) = parse_marker(&buf[i+1..j]);
            let c = expand(&buf[j+1..j+n+1]);
            count += x as u64 * c;
            i = j + n + 1;
        } else {
            i += 1;
            count += 1;
        }
    }

    count
}


#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn it_works() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let len = part_one(&input);
    assert_eq!(len, 102239);

    let len = part_two(&input);
    assert_eq!(len, 10780403063);
  }

  #[test]
  fn samples1() {
    let len = part_two("(3x3)XYZ");
    assert_eq!(len, "XYZXYZXYZ".len() as u64);
  }

  #[test]
  fn samples2() {
    let len = part_two("X(8x2)(3x3)ABCY");
    assert_eq!(len, "XABCABCABCABCABCABCY".len() as u64);
  }

  #[test]
  fn samples3() {
    let len = part_two("(27x12)(20x12)(13x14)(7x10)(1x12)A");
    assert_eq!(len, 241920);
  }

  #[test]
  fn samples4() {
    let len = part_two("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN");
    assert_eq!(len, 445);
  }
}