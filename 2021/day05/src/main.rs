use std::collections::HashMap;

fn main() {
    use std::fs;
    use std::time::Instant;

    let lines = load(&fs::read_to_string("./input.txt").unwrap());

    let t1 = Instant::now();
    let (p1, p2) = doit(&lines);
    let t2 = Instant::now();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("{:?}", t2 - t1);
}

enum Line {
    Vert((i32, i32), (i32, i32)),
    Horz((i32, i32), (i32, i32)),
    Diag((i32, i32), (i32, i32)),
}

fn load(input: &str) -> Vec<Line> {
    input.lines().map(|l| l.split(" -> ").collect::<Vec<&str>>())
        .map(|v| {
            let p1: Vec<_> = v[0].split(',').map(|s| s.parse::<i32>().unwrap()).collect();
            let p2: Vec<_> = v[1].split(',').map(|s| s.parse::<i32>().unwrap()).collect();
            if p1[0] == p2[0] {
                Line::Vert((p1[0], p1[1]), (p2[0], p2[1]))
            } else if p1[1] == p2[1] {
                Line::Horz((p1[0], p1[1]), (p2[0], p2[1]))
            } else {
                Line::Diag((p1[0], p1[1]), (p2[0], p2[1]))
            }
        })
        .collect()
}

fn doit(lines: &[Line]) -> (i32, i32) {
    use itertools::zip;
    use num::range_step_inclusive as range;

    let mut pts = HashMap::new();
    let diag = lines.iter().fold(Vec::new(), |mut v, line| {
        match line {
            Line::Vert(p1, p2) => {
                let x = std::iter::repeat(p1.0);
                let step = if p1.1 > p2.1 { -1 } else { 1 };
                let iter = zip(x, range(p1.1, p2.1, step));
                iter.for_each(|pt| *pts.entry(pt).or_insert(-1) += 1 );
            },
            Line::Horz(p1, p2) => {
                let y = std::iter::repeat(p1.1);
                let step = if p1.0 > p2.0 { -1 } else { 1 };
                let iter = zip(range(p1.0, p2.0, step), y);
                iter.for_each(|pt| *pts.entry(pt).or_insert(-1) += 1 );
            },
            Line::Diag(_, _) => { v.push(line); }
        };

        v
    });
    let part1 = pts.values().filter(|&v| *v > 0).count();

    diag.iter().for_each(|l| {
        if let Line::Diag(p1, p2) = l {
            let xs = if p1.0 > p2.0 { -1 } else { 1 };
            let ys = if p1.1 > p2.1 { -1 } else { 1 };
            let it = zip(range(p1.0, p2.0, xs), range(p1.1, p2.1, ys));
            it.for_each(|pt| *pts.entry(pt).or_insert(-1) += 1 );
        }
    });
    let part2 = pts.values().filter(|&v| *v > 0).count();

    (part1 as i32, part2 as i32)
}


#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn it_works() {
    let lines = load(&fs::read_to_string("./input.txt").unwrap());

    let (p1, p2) = doit(&lines);
    assert_eq!(p1, 7085);
    assert_eq!(p2, 20271);
  }
}