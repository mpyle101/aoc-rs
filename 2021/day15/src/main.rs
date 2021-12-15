use pathfinding::matrix::Matrix;

fn main() {
    use std::fs;
    use std::time::Instant;
    
    let input = fs::read_to_string("./input.txt").unwrap();
    let map = load(&input);

    let t1 = Instant::now();
    let risk = part_one(&map);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", risk, t2 - t1);

    let t1 = Instant::now();
    let risk = part_two(&map);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", risk, t2 - t1);
}

fn load(input: &str) -> Matrix<u32> {
    Matrix::from_rows(
        input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()))
    ).unwrap()
}

fn part_one(m: &Matrix<u32>) -> u32 {
    use pathfinding::prelude::dijkstra;

    let path = dijkstra(&(0, 0),
        |pos| m.neighbours(*pos, false).map(|p| (p, *m.get(p).unwrap())),
        |pos| *pos == (m.rows - 1, m.columns - 1)
    ).unwrap();

    path.1
}

fn part_two(m: &Matrix<u32>) -> u32 {
    use pathfinding::prelude::dijkstra;

    let mut mat = Matrix::new(m.rows * 5, m.columns * 5, 0u32);
    (0..5).for_each(|y|
        (0..5).for_each(|x| {
            m.indices().for_each(|p| {
                let pos = (p.0 + x * m.rows, p.1 + y * m.columns);
                let v = *m.get(p).unwrap() + x as u32 + y as u32;
                *mat.get_mut(pos).unwrap() = if v > 9 { v - 9 } else { v }
            });
        })
    );

    let path = dijkstra(&(0, 0),
        |pos| mat.neighbours(*pos, false).map(|p| (p, *mat.get(p).unwrap())),
        |pos| *pos == (mat.rows - 1, mat.columns - 1)
    ).unwrap();

    path.1
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let map = load(&input);

        let risk = part_one(&map);
        assert_eq!(risk, 441);

        let risk = part_two(&map);
        assert_eq!(risk, 2849);
    }
}