use pathfinding::matrix::{Matrix, MatrixFormatError};

fn main() {
    use std::time::Instant;

    let mat = load(include_str!("./input.txt"));

    let t1 = Instant::now();
    let count = mat.map_or(0, |m| part_one(&m));
    let t2 = Instant::now();
    println!("Part 1: {count} ({:?})", t2 - t1);

    let mat = load(include_str!("./input.txt"));

    let t1 = Instant::now();
    let count = mat.map_or(0, |m| part_two(&m));
    let t2 = Instant::now();
    println!("Part 2: {count} ({:?})", t2 - t1);
}

fn load(input: &str) -> Result<Matrix<char>, MatrixFormatError> {
    Matrix::from_rows(input.lines().map(|l| l.chars()))
}

fn part_one(map: &Matrix<char>) -> usize {
    (0..100).fold(map.clone(), |mat, _| {
        let mut m = Matrix::new(mat.rows, mat.columns, '.');
        mat.indices().for_each(|p| {
            let cnt = mat.neighbours(p, true)
                .filter_map(|p| mat.get(p).filter(|&v| *v == '#'))
                .count();
            if cnt == 3 || (cnt == 2 && mat.get(p).map_or(false, |v| *v == '#')) {
                m.get_mut(p).map(|v| *v = '#');
            }
        });
        m
    })
    .values()
    .filter(|&c| *c == '#')
    .count()
}

fn part_two(map: &Matrix<char>) -> usize {
    let corners = [
        (0, 0),
        (0, map.rows - 1),
        (map.columns - 1, 0),
        (map.rows - 1, map.columns - 1)
    ];

    // The lights in the corners are stuck on.
    let mut m0 = map.clone();
    for p in corners.iter() { m0.get_mut(*p).map(|v| *v = '#'); }

    (0..100).fold(m0, |m1, _| {
        let mut m = Matrix::new(m1.rows, m1.columns, '.');
        m1.indices().for_each(|p| {
            let cnt = m1.neighbours(p, true)
                .filter_map(|p| m1.get(p).filter(|&v| *v == '#'))
                .count();
            if cnt == 3 || (cnt == 2 && m1.get(p).map_or(false, |v| *v == '#')) {
                m.get_mut(p).map(|v| *v = '#');
            }
        });

        // The lights in the corners are stuck on.
        for p in corners.iter() { m.get_mut(*p).map(|v| *v = '#'); }
        m
    })
    .values()
    .filter(|&c| *c == '#')
    .count()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let mat = load(include_str!("./input.txt"));
    let count = mat.map_or(0, |m| part_one(&m));
    assert_eq!(count, 1061);

    let mat = load(include_str!("./input.txt"));
    let count = mat.map_or(0, |m| part_two(&m));
    assert_eq!(count, 1006);
  }
}