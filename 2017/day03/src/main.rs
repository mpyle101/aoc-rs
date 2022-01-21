fn main() {
    use std::time::Instant;

    let t1 = Instant::now();
    let steps = part_one(347991);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", steps, t2 - t1);
}

fn part_one(square: i32) -> i32 {
    let mut ring = 1;

    let mut high = 9;
    while high < square {
        ring += 1;
        high += 8 * ring;
    }

    let steps = [1, 3, 5, 7].iter()
        .map(|n| (calc_value(*n, ring) - square).abs())
        .min()
        .unwrap();

    ring + steps
}

fn calc_value(n: i32, ring: i32) -> i32 {
    (1..ring).fold(1 + n, |acc, i| acc + n + 8 * i)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let steps = part_one(347991);
        assert_eq!(steps, 480);
    }
}