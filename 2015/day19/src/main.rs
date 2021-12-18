use std::collections::HashSet;

type Rules<'a> = Vec<(&'a str, &'a str)>;

fn main() {
    use std::time::Instant;

    let (rules, molecule) = load(include_str!("./input.txt"));

    let t1 = Instant::now();
    let count = part_one(molecule, &rules);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", count, t2 - t1);

    let t1 = Instant::now();
    let steps = part_two(molecule, &rules);
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", steps, t2 - t1);

    // 207
}

fn load(input: &str) -> (Rules, &str) {
    let mut it = input.split("\n\n");
    let rules = it.next().map(|v| {
        v.lines().fold(Vec::new(), |mut rules, s| {
            let kv = s.split(" => ").collect::<Vec<_>>();
            rules.push((kv[0], kv[1]));
            rules
        })
    }).unwrap();

    (rules, it.next().unwrap())
}

fn part_one(molecule: &str, rules: &Rules) -> i32 {
    let mut molecules = HashSet::new();
    rules.iter().for_each(|(k, s)|
        molecule.match_indices(k).for_each(|(i, _)| {
            let mut m = molecule.to_string();
            m.replace_range(i..i+k.len(), s);
            molecules.insert(m);
        })
    );
    
    molecules.len() as i32
}

fn part_two(molecule: &str, rules: &Rules) -> u32 {
    use rand::seq::SliceRandom;

    let rrules = rules.iter().map(|(k, v)| (*v, *k)).collect::<Vec<_>>();

    let mut cnt = 0;
    let mut m = molecule.to_string();
    while m != "e" {
        // Pick a completely random rule to apply. Sometimes we get stuck
        // sometimes we find the answer: 207.
        let rule = rrules.choose(&mut rand::thread_rng()).unwrap();
        while let Some(i) = m.find(rule.0) {
            m.replace_range(i..(i+rule.0.len()), rule.1);
            cnt += 1;
        }
    }

    cnt
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let (rules, molecule) = load(include_str!("./input.txt"));

    let count = part_one(molecule, &rules);
    assert_eq!(count, 576);
  }
}