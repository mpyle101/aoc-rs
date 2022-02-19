fn main() {
    use std::time::Instant;

    let t1 = Instant::now();
    let steps = part_one(347991);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", steps, t2 - t1);

    let t1 = Instant::now();
    let value = part_two(347991);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", value, t2 - t1);
}

fn part_one(square: i32) -> i32 {
    let mut ring = 1;

    let mut high = 9;
    while high < square {
        ring += 1;
        high += 8 * ring;
    }

    let steps = [1, 3, 5, 7].iter()
        .map(|n| {
            let v = (1..ring).fold(1 + n, |acc, i| acc + n + 8 * i);
            (v - square).abs()
        })
        .min()
        .unwrap();

    ring + steps
}

fn part_two(square: i32) -> i32 {
    use std::collections::HashMap;
    
    let mut squares = HashMap::from([((0, 0), 1)]);

    let mut x = 1;
    let mut y = 0;
    let mut value = 1;
    while value < square {
        for sq in spiral(x, y) {
            value = neighbors(sq).iter()
                .filter_map(|pt| squares.get(pt))
                .sum();

            if value > square { break }
            squares.insert(sq, value);
        }

        x += 1;
        y += 1;
    }

    value
}

fn spiral(x: i32, y: i32) -> Vec<(i32, i32)> {
    // n:0..2x  => (x, y-n)
    // m:1..=2x => (x-m, y-n)
    // p:2y..0  => (x-m, y-p)
    // q:2x..0  => (x-q, y+1)
    let mut squares = vec![];

    let mut n = 0;
    while n < 2 * x { squares.push((x, y-n)); n += 1 }
    n -= 1;

    let mut m = 1;
    while m <= 2 * x { squares.push((x-m, y-n)); m += 1 }
    m -= 1;

    let mut p = 2 * y;
    while p >= 0 { squares.push((x-m, y-p)); p -= 1 }

    let mut q = 2 * x;
    while q >= 0 { squares.push((x-q, y+1)); q -= 1 }

    squares
}

fn neighbors(pt: (i32, i32)) -> Vec<(i32, i32)> {
    let delta = [
        (-1, -1), (0, -1), (1, -1), 
        (-1,  0),          (1,  0), 
        (-1,  1), (0,  1), (1,  1),
    ];

    delta.iter().map(|d| (pt.0 + d.0, pt.1 + d.1)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let steps = part_one(347991);
        assert_eq!(steps, 480);

        let value = part_two(347991);
        assert_eq!(value, 349975);
    }
}