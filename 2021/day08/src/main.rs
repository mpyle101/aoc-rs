use std::collections::{ HashSet, HashMap };

fn main() {
    use std::fs;
    use std::time::Instant;
    
    let input   = fs::read_to_string("./input.txt").unwrap();
    let signals = load(&input);

    let t1 = Instant::now();
    let count = part_one(&signals);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", count, t2 - t1);

    let t1 = Instant::now();
    let total = part_two(&signals);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", total, t2 - t1);
}

fn load(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    input.lines().map(|l| {
        let v = l.split(" | ").collect::<Vec<_>>();
        (split(v[0]), split(v[1]))
    }).collect()
}

fn part_one(signals: &[(Vec<&str>, Vec<&str>)]) -> i32 {
    signals.iter().map(|(_, outputs)| {
        outputs.iter().filter(|v|
            v.len() == 2 || v.len() == 3 || v.len() == 4 || v.len() == 7
        ).count() as i32
    }).sum()
}

fn part_two(signals: &[(Vec<&str>, Vec<&str>)]) -> i32 {
    let digits = HashMap::from([
        (String::from("cf"),      '1'),
        (String::from("acf"),     '7'),
        (String::from("bcdf"),    '4'),
        (String::from("acdeg"),   '2'),
        (String::from("acdfg"),   '3'),
        (String::from("abdfg"),   '5'),
        (String::from("abcefg"),  '0'),
        (String::from("abdefg"),  '6'),
        (String::from("abcdfg"),  '9'),
        (String::from("abcdefg"), '8'),
    ]);

    signals.iter().map(|(wires, outputs)| {
        let one   = find_by_length(wires, 2);
        let four  = find_by_length(wires, 4);
        let seven = find_by_length(wires, 3);
        let eight = find_by_length(wires, 7);

        let abcdf = &four | &seven;
        let fives = filter_by_length(wires, 5);
        let two   = fives.iter()
            .filter(|v| (*v - &abcdf).len() == 2)
            .collect::<Vec<_>>();
        let tmp   = fives.iter()
            .filter(|v| (*v - &abcdf).len() == 1)
            .collect::<Vec<_>>();
        let two   = *two.first().unwrap();
        let tmp   = *tmp.first().unwrap();

        let bd    = &four - &one;
        let b     = &bd - two;
        let d     = &bd - &b;
        let g     = tmp - &abcdf;
        let f     = &abcdf - &(two | &bd);
        let a     = &seven - &one;
        let nine  = &(&four | &a) | &g;
        let e     = &eight - &nine;
        let c     = &one - &f;

        let key = HashMap::from([
            (as_char(&a), 'a'),
            (as_char(&b), 'b'),
            (as_char(&c), 'c'),
            (as_char(&d), 'd'),
            (as_char(&e), 'e'),
            (as_char(&f), 'f'),
            (as_char(&g), 'g'),
        ]);

        let v = outputs.iter()
            .map(|s| decode(&key, s))
            .map(|s| digits.get(&s).unwrap())
            .collect::<String>();
        v.parse::<i32>().unwrap()
    }).sum()
}

fn split(line: &str) -> Vec<&str> {
    line.split(' ').collect::<Vec<_>>()
}

fn decode(key: &HashMap<char, char>, s: &str) -> String {
    let mut chars = s.chars().map(|c| *key.get(&c).unwrap()).collect::<Vec<char>>();
    chars.sort_unstable();
    String::from_iter(chars)
}

fn find_by_length(wires: &[&str], len: usize) -> HashSet<char> {
    wires.iter().find(|s| s.len() == len).unwrap().chars().collect()
}

fn filter_by_length(wires: &[&str], len: usize) -> Vec<HashSet<char>> {
    wires.iter()
        .filter(|s| s.len() == len)
        .map(|s| s.chars().collect::<HashSet<_>>())
        .collect()
}

fn as_char(set: &HashSet<char>) -> char {
    *set.iter().next().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input   = fs::read_to_string("./input.txt").unwrap();
        let signals = load(&input);

        let count = part_one(&signals);
        assert_eq!(count, 381);

        let total = part_two(&signals);
        assert_eq!(total, 1023686);
    }
}