use std::collections::HashMap;

fn main() {
    use std::time::Instant;

    let aunts = load(include_str!("./input.txt"));
    let clues = [
        ("children:", 3),
        ("cats:", 7),
        ("samoyeds:", 2),
        ("pomeranians:", 3),
        ("akitas:", 0),
        ("vizslas:", 0),
        ("goldfish:", 5),
        ("trees:", 3),
        ("cars:", 2),
        ("perfumes:", 1),
    ];

    let t1 = Instant::now();
    let sue = part_one(&aunts, &clues);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", sue, t2 - t1);

    let t1 = Instant::now();
    let sue = part_two(&aunts, &clues);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", sue, t2 - t1);
}

fn load(input: &str) -> Vec<HashMap<&str, i32>> {
    input.lines()
        .map(|l| l.split(' ').collect::<Vec<&str>>())
        .map(|v| HashMap::from([
            ("no", v[1][0..v[1].len() - 1].parse::<i32>().unwrap()),
            (v[2], v[3][0..v[3].len() - 1].parse::<i32>().unwrap()),
            (v[4], v[5][0..v[5].len() - 1].parse::<i32>().unwrap()),
            (v[6], v[7].parse::<i32>().unwrap()),
        ]))
        .collect()
}

fn part_one(aunts: &[HashMap<&str, i32>], clues: &[(&str, i32); 10]) -> i32 {
    let sue = clues.iter().fold(aunts.to_vec(), |vec, (clue, value)|
        vec.iter().filter(|map| {
            let v = map.get(clue);
            v == None || v.unwrap() == value
        })
        .cloned()
        .collect::<Vec<_>>()
    );

    *sue.first().unwrap().get("no").unwrap()
}

fn part_two(aunts: &[HashMap<&str, i32>], clues: &[(&str, i32); 10]) -> i32 {
    let sue = clues.iter().fold(aunts.to_vec(), |vec, (clue, value)|
        vec.iter().filter(|map| {
            let v = map.get(clue);
            if v == None {
                true
            } else if *clue == "cats:" || *clue == "trees:" {
                v.unwrap() > value
            } else if *clue == "pomeranians:" || *clue == "goldfish:" {
                v.unwrap() < value
            } else {
                v.unwrap() == value
            }
        })
        .cloned()
        .collect::<Vec<_>>()
    );

    *sue.first().unwrap().get("no").unwrap()
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let aunts = load(include_str!("./input.txt"));
    let clues = [
        ("children:", 3),
        ("cats:", 7),
        ("samoyeds:", 2),
        ("pomeranians", 3),
        ("akitas:", 0),
        ("vizslas:", 0),
        ("goldfish:", 5),
        ("trees:", 3),
        ("cars:", 2),
        ("perfumes:", 1),
    ];

    let sue = part_one(&aunts, &clues);
    assert_eq!(sue, 103);

    let sue = part_two(&aunts, &clues);
    assert_eq!(sue, 405);
  }
}