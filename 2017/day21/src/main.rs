use std::collections::HashMap;
use pathfinding::matrix::Matrix;

fn main() {
    use std::{fs, time::Instant};
    
    let input = fs::read_to_string("./input.txt").unwrap();
    let rules = load(&input);

    let t1 = Instant::now();
    let pixels = part_one(&rules);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", pixels, t2 - t1);

    let t1 = Instant::now();
    let pixels = part_two(&rules);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", pixels, t2 - t1);
}

type Rules = HashMap<Matrix<char>, Matrix<char>>;

fn load(input: &str) -> Rules {
    input.lines().map(|l| {
        let v = l.split(" => ").collect::<Vec<_>>();
        let m = Matrix::from_rows(v[0].split('/').map(|s| s.chars())).unwrap();
        let n = Matrix::from_rows(v[1].split('/').map(|s| s.chars())).unwrap();

        let r1 = m.rotated_cw(1);
        let r2 = m.rotated_cw(2);
        let r3 = m.rotated_cw(3);
        let lr = m.flipped_lr();
        let r4 = lr.rotated_cw(1);
        let r5 = lr.rotated_cw(2);
        let r6 = lr.rotated_cw(3);

        vec![
            (m,  n.clone()), (r1, n.clone()), (r2, n.clone()), (r3, n.clone()),
            (lr, n.clone()), (r4, n.clone()), (r5, n.clone()), (r6, n.clone()),
        ]
    })
    .flatten()
    .collect()
}

fn part_one(rules: &Rules) -> usize {
    let rows = ".#./..#/###".split('/').map(|s| s.chars());
    let pixels = Matrix::from_rows(rows).unwrap();

    enhance(&pixels, rules, 5)
}

fn part_two(rules: &Rules) -> usize {
    let rows = ".#./..#/###".split('/').map(|s| s.chars());
    let pixels = Matrix::from_rows(rows).unwrap();

    enhance(&pixels, rules, 18)
}

fn enhance(pixels: &Matrix<char>, rules: &Rules, iterations: usize) -> usize {
    let mut p = pixels.clone();

    (0..iterations).for_each(|_| {
        let step = if p.rows % 2 == 0 { 2 } else { 3 };
        let subs = p.rows / step;

        let mut m = Matrix::new_square(subs * (step + 1), '_');
        for y in (0..p.rows).step_by(step) {
            for x in (0..p.rows).step_by(step) {
                let slice = p.slice(y..y + step, x..x + step).unwrap();
                let pattern = rules.get(&slice).unwrap();
                m.set_slice((y + (y / step), x + (x / step)), pattern);
                // print(&m);
            }
        }

        p = m;
    });

    p.values().filter(|&c| *c == '#').count()
}

#[allow(dead_code)]
fn print(m: &Matrix<char>) {
    m.iter().for_each(|row| {
        row.iter().for_each(|c| print!("{}", c));
        println!();
    });
    println!();
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let rules = load(&input);
    
        let pixels = part_one(&rules);
        assert_eq!(pixels, 136);
    
        let pixels = part_two(&rules);
        assert_eq!(pixels, 1911767);
    }
}
