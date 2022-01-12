fn main() {
    use std::fs;
    use std::time::Instant;

    let input = fs::read_to_string("./input.txt").unwrap();

    let t1 = Instant::now();
    let code = part_one(&input);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", code, t2 - t1);

    let t1 = Instant::now();
    let code = part_two(&input);
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", code, t2 - t1);
}

fn part_one(input: &str) -> String {
    let mut key = (1i8, 1i8);

    input.lines().map(|l| {
        key = l.chars().fold(key, |p, c|
            match c {
                'U' => (0.max(p.0 - 1), p.1),
                'D' => (2.min(p.0 + 1), p.1),
                'L' => (p.0, 0.max(p.1 - 1)),
                'R' => (p.0, 2.min(p.1 + 1)),
                _ => panic!("Unknown direction: {}", c)
            }
        );
        (key.0 * 3 + key.1 + 1) as u32
    })
    .map(|n| char::from_digit(n, 10).unwrap())
    .collect()
}

fn part_two(input: &str) -> String {
    use std::collections::HashMap;

    let keypad = HashMap::from([
        (('1', 'D'), '3'), (('2', 'D'), '6'), (('2', 'R'), '3'),
        (('3', 'U'), '1'), (('3', 'D'), '7'), (('3', 'L'), '2'),
        (('3', 'R'), '4'), (('4', 'D'), '8'), (('4', 'L'), '3'),
        (('5', 'R'), '6'), (('6', 'U'), '2'), (('6', 'D'), 'A'),
        (('6', 'L'), '5'), (('6', 'R'), '7'), (('7', 'U'), '3'),
        (('7', 'D'), 'B'), (('7', 'L'), '6'), (('7', 'R'), '8'),
        (('8', 'U'), '4'), (('8', 'D'), 'C'), (('8', 'L'), '7'),
        (('8', 'R'), '9'), (('9', 'L'), '8'), (('A', 'U'), '6'),
        (('A', 'R'), 'B'), (('B', 'U'), '7'), (('B', 'D'), 'D'),
        (('B', 'L'), 'A'), (('B', 'R'), 'C'), (('C', 'U'), '8'),
        (('C', 'L'), 'B'), (('D', 'U'), 'B'),
    ]);

    let mut key = '5';
    input.lines().map(|l| {
        key = l.chars().fold(key, |k, c|
            if let Some(k1) = keypad.get(&(k, c)) { *k1 } else { k }
        );
        key
    })
    .collect()
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();

        let code = part_one(&input);
        assert_eq!(code, "12578");

        let code = part_two(&input);
        assert_eq!(code, "516DD");
    }
}