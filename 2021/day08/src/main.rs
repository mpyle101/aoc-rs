
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

fn load(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    input.lines().map(|l| {
        let v = l.split(" | ").collect::<Vec<_>>();
        (split(v[0]), split(v[1]))
    }).collect()
}

fn part_one(signals: &[(Vec<String>, Vec<String>)]) -> i32 {
    signals.iter().map(|(_, outputs)| {
        outputs.iter().filter(|v|
            v.len() == 2 || v.len() == 3 || v.len() == 4 || v.len() == 7
        ).count() as i32
    }).sum()
}

fn part_two(signals: &[(Vec<String>, Vec<String>)]) -> i32 {
    use std::collections::HashMap;
    use itertools::Itertools;

    let display = [
        ("cf",      '1'),
        ("acf",     '7'),
        ("bcdf",    '4'),
        ("acdeg",   '2'),
        ("acdfg",   '3'),
        ("abdfg",   '5'),
        ("abcefg",  '0'),
        ("abdefg",  '6'),
        ("abcdfg",  '9'),
        ("abcdefg", '8'),
    ];

    // Brute force using what amounts to a rainbow table.
    // Generate a vector of all the possible wire mappings.
    // Use that vector to generate the set of resulting display
    // values for each wire mapping and capture the mapping,
    // the display results and the reverse mapping.
    let base = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    let keys = base.iter()
        .permutations(7)
        .map(|v| base.iter().zip(v.iter())
            .map(|(b, k)| (*b, **k)).collect::<HashMap<_, _>>()
        )
        .collect::<Vec<_>>();
    let digits = keys.iter()
        .map(|k| {
            let d = display.iter().map(|(s, _)| {
                let mut d = s.chars().map(|c| k.get(&c).unwrap()).collect::<Vec<_>>();
                d.sort();
                String::from_iter(d)
            })
            .collect::<Vec<_>>();
            (
                k, 
                k.keys().map(|key| (k.get(key).unwrap(), key)).collect::<HashMap<_, _>>(),
                d
            )
        })
        .collect::<Vec<_>>();

    // For each signal find the set of "encrypted" display
    // values which contain all the signal values. That gives
    // us a valid key. Take the inverse of the key and use it
    // "decrypt" the output values and look them up in the base
    // display map to get the "numeric" value. Combine those and
    // parse the result into an i32.
    signals.iter().map(|(wires, outputs)| {
        let (_, d, _) = digits.iter()
            .find(|(_, _, v)| wires.iter().all(|w| v.contains(w)))
            .unwrap();
        let v = outputs.iter()
            .map(|s| {
                let mut chars = s.chars().map(|c| **d.get(&c).unwrap()).collect::<Vec<char>>();
                chars.sort();
                String::from_iter(chars)
            })
            .map(|s| display.iter().find(|(k, _)| s == *k).unwrap())
            .map(|(_, v)| v)
            .collect::<String>();
        v.parse::<i32>().unwrap()
    }).sum()
}

fn split(line: &str) -> Vec<String> {
    line.split(' ').map(|s| {
        let mut chars = s.chars().collect::<Vec<char>>();
        chars.sort();
        String::from_iter(chars)
    }).collect()
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