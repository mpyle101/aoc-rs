fn main() {
    use std::{fs, time::Instant};
    
    let input = fs::read_to_string("./input.txt").unwrap();

    let t1 = Instant::now();
    let steps = part_one(&input);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", steps, t2 - t1);

    let t1 = Instant::now();
    let steps = part_two(&input);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", steps, t2 - t1);
}

fn part_one(input: &str) -> i32 {
    let (x, y) = input.split(',')
        .fold((0i32, 0i32), |tile, s| step(s, tile));

    let dx = x.abs();
    let dy = y.abs();

    if dy >= dx { dy / 2 } else { (dx + dy) / 4 }
}

fn part_two(input: &str) -> i32 {
    let mut farthest = 0;

    let (mut x, mut y) = (0i32, 0i32);
    input.split(',').for_each(|s| {
        let (x1, y1) = step(s, (x, y));
        let dx = x1.abs();
        let dy = y1.abs();
        let steps = if dy >= dx { dy / 2 } else { (dx + dy) / 4 };
        farthest = farthest.max(steps);

        x = x1;
        y = y1;
    });

    farthest
}

fn step(s: &str, (x, y): (i32, i32)) -> (i32, i32) {
    match s {
        "n"  => (x + 2, y + 2),
        "ne" => (x + 4, y + 0),
        "se" => (x + 2, y - 2),
        "s"  => (x - 2, y - 2),
        "sw" => (x - 4, y - 0),
        "nw" => (x - 2, y + 2),
        _ => panic!("Unknown direction: {}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();

        let steps = part_one(&input);
        assert_eq!(steps, 685);

        let steps = part_two(&input);
        assert_eq!(steps, 1457);
    }
}