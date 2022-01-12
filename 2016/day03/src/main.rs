fn main() {
    use std::fs;
    use std::time::Instant;

    let input = fs::read_to_string("./input.txt").unwrap();
    let rows = load(&input);

    let t1 = Instant::now();
    let valid = part_one(&rows);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", valid, t2 - t1);

    let t1 = Instant::now();
    let valid = part_two(&rows);
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", valid, t2 - t1);
}

fn load(input: &str) -> Vec<[i32;3]> {
    input.lines().map(|l| {
        let mut it = l.split_ascii_whitespace();
        [
            it.next().unwrap().parse::<i32>().unwrap(),
            it.next().unwrap().parse::<i32>().unwrap(),
            it.next().unwrap().parse::<i32>().unwrap(),
        ]
    })
    .collect()
}

fn part_one(rows: &[[i32;3]]) -> i32 {
    rows.iter().map(|[a, b, c]|
        (a + b > *c && a + c > *b && b + c > *a) as i32
    )
    .sum()
}

fn part_two(rows: &[[i32;3]]) -> i32 {
    rows.chunks(3).map(|r|
        (0..3).fold(0, |n, i| {
            let (a, b, c) = (r[0][i], r[1][i], r[2][i]);
            n + (a + b > c && a + c > b && b + c > a) as i32
        })
    )
    .sum()
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let rows = load(&input);
    
        let valid = part_one(&rows);
        assert_eq!(valid, 869);
    
        let valid = part_two(&rows);
        assert_eq!(valid, 1544);
    }
}