use pathfinding::matrix::Matrix;

fn main() {
    use std::time::Instant;

    let acres = load(include_str!("./input.txt"));

    let t1 = Instant::now();
    let resource = part_one(&acres);
    let t2 = Instant::now();
    println!("Part 1: {}  ({:?})", resource, t2 - t1);

    let t1 = Instant::now();
    let resource = part_two(&acres);
    let t2 = Instant::now();
    println!("Part 2: {}  ({:?})", resource, t2 - t1);
}

fn load(input: &str) -> Matrix<char> {
    Matrix::from_rows(input.lines().map(|s| s.chars())).unwrap()
}

fn part_one(acres: &Matrix<char>) -> i32 {
    let mut m = update(&acres);
    for _ in 0..9 { m = update(&m) }

    resources(&m)
}

fn part_two(acres: &Matrix<char>) -> i32 {
    use std::collections::HashMap;

    let mut m = update(acres);
    let mut seen = HashMap::from([(m.clone(), 1)]);

    // Look for a cycle.
    let mut count = 1;
    let (n, c) = loop {
        count += 1;
        m = update(&m);
        if let Some(n) = seen.get(&m) {
            break (*n, count)
        } else {
            seen.insert(m.clone(), count);
        }
    };

    // Once we have a cycle, subtract the current count
    // to get the total remaining number of iterations
    // and then mod that by the cycle length to get the
    // number of iterations left in the last cycle. Do
    // those and calculate the resource score.
    let cycle = c - n;
    let end = (1_000_000_000 - c) % cycle;
    for _ in 0..end { m = update(&m) }

    resources(&m)
}


fn update(acres: &Matrix<char>) -> Matrix<char> {
    let mut m = Matrix::new(acres.rows, acres.columns, '.');

    acres.indices().for_each(|p| {
        let v = acres.neighbours(p, true);
        let (trees, lumber) = v.fold((0, 0), |acc, p1| {
            match acres.get(p1).unwrap() {
                '|' => (acc.0 + 1, acc.1),
                '#' => (acc.0, acc.1 + 1),
                 _  => (acc.0, acc.1),  // we don't track open
            }
        });

        let c = *acres.get(p).unwrap();
        if c == '.' {
            if trees >= 3 {
                *m.get_mut(p).unwrap() = '|'
            }
        } else if c == '|' {
            if lumber >= 3 {
                *m.get_mut(p).unwrap() = '#'
            } else {
                *m.get_mut(p).unwrap() = '|'
            }
        } else {
            if trees > 0 && lumber > 0 {
                *m.get_mut(p).unwrap() = '#'
            }
        }
    });

    m
}

fn resources(acres: &Matrix<char>) -> i32 {
    let (wooded, lumber) = acres.values().fold((0, 0), |acc, c| 
        match c {
            '|' => (acc.0 + 1, acc.1),
            '#' => (acc.0, acc.1 + 1),
                _  => (acc.0, acc.1),  // we don't track open
        });

    wooded * lumber
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let acres = load(include_str!("./input.txt"));

    let resource = part_one(&acres);
    assert_eq!(resource, 536370);

    let resource = part_two(&acres);
    assert_eq!(resource, 190512);
  }
}
