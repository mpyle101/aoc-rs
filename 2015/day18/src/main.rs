use pathfinding::matrix::Matrix;

fn main() {
    use std::time::Instant;

    let map = load(include_str!("./input.txt"));

    let t1 = Instant::now();
    let count = part_one(&map);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", count, t2 - t1);

    let t1 = Instant::now();
    let count = part_two(&map);
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", count, t2 - t1);
}

fn load(input: &str) -> Matrix<char> {
    Matrix::from_rows(input.lines().map(|l| l.chars())).unwrap()
}

fn part_one(map: &Matrix<char>) -> i32 {
    (0..100).fold(map.clone(), |mat, _| {
        let mut m = Matrix::new(mat.rows, mat.columns, '.');
        mat.indices().for_each(|p| {
            let cnt = mat.neighbours(p, true)
                .filter(|p| *mat.get(*p).unwrap() == '#')
                .count();
            if cnt == 3 {
                *m.get_mut(p).unwrap() = '#';
            } else if cnt == 2 {
                if *mat.get(p).unwrap() == '#' {
                    *m.get_mut(p).unwrap() = '#'
                }
            }
        });
        m
    }).values().filter(|&c| *c == '#').count() as i32
}

fn part_two(map: &Matrix<char>) -> i32 {
    let corners = [
        (0, 0),
        (0, map.rows - 1),
        (map.columns - 1, 0),
        (map.rows - 1, map.columns - 1)
    ];

    // The lights in the corners are stuck on.
    let mut m0 = map.clone();
    corners.iter().for_each(|p| *m0.get_mut(*p).unwrap() = '#');

    (0..100).fold(m0, |m1, _| {
        let mut m = Matrix::new(m1.rows, m1.columns, '.');
        m1.indices().for_each(|p| {
            let cnt = m1.neighbours(p, true)
                .filter(|p| *m1.get(*p).unwrap() == '#')
                .count();
            if cnt == 3 {
                *m.get_mut(p).unwrap() = '#';
            } else if cnt == 2 {
                if *m1.get(p).unwrap() == '#' {
                    *m.get_mut(p).unwrap() = '#'
                }
            }
        });

        // The lights in the corners are stuck on.
        corners.iter().for_each(|p| *m.get_mut(*p).unwrap() = '#');
        m
    }).values().filter(|&c| *c == '#').count() as i32
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let map = load(include_str!("./input.txt"));

    let count = part_one(&map);
    assert_eq!(count, 1061);

    let count = part_two(&map);
    assert_eq!(count, 1006);
  }
}