use lazy_static::lazy_static;

lazy_static! {
    static ref TRAPPED: [[i32;3]; 4] = [
        [1, 1, 0], [0, 1, 1], [1, 0, 0], [0, 0, 1]
    ];
}

fn main() {
    use std::{fs, time::Instant};

    let input = fs::read_to_string("./input.txt").unwrap();

    let t1 = Instant::now();
    let tiles = safe_tiles(&input, 40);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", tiles, t2 - t1);

    let t1 = Instant::now();
    let tiles = safe_tiles(&input, 400000);
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", tiles, t2 - t1);
}

fn safe_tiles(input: &str, rows: usize) -> usize {
    use std::collections::HashSet;

    let cols = input.len();
    let row0 = input.chars()
        .enumerate()
        .filter_map(|(i, c)| (c == '^').then(||i as i32))
        .collect::<HashSet<_>>();

    let n = row0.len();
    let trapped = (1..rows as i32).fold((row0, n), |(row, n), _| {
        let traps = (0..cols as i32).filter_map(|x| {
            let t = [
                row.get(&(x - 1)).is_some() as i32,
                row.get(&x).is_some() as i32,
                row.get(&(x + 1)).is_some() as i32,
            ];
            TRAPPED.contains(&t).then(||x)
        })
        .collect::<HashSet<_>>();

        let l = traps.len();
        (traps, n + l)
    });

    rows * cols - trapped.1
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();

        let tiles = safe_tiles(&input, 40);
        assert_eq!(tiles, 1956);

        let tiles = safe_tiles(&input, 400000);
        assert_eq!(tiles, 19995121);
    }
}