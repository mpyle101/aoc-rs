use std::collections::{HashMap, HashSet};

type Rules<'a> = HashMap<&'a str, Vec<&'a str>>;

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
}

fn load(input: &str) -> (Rules, &str) {
    let mut it = input.split("\n\n");
    let rules = it.next().map(|v| {
        v.lines().fold(HashMap::new(), |mut rules, s| {
            let kv = s.split(" => ").collect::<Vec<_>>();
            rules.entry(kv[0]).or_insert(Vec::new()).push(kv[1]);
            rules
        })
    }).unwrap();

    (rules, it.next().unwrap())
}

fn part_one(molecule: &str, rules: &Rules) -> i32 {
    let mut molecules = HashSet::new();
    rules.iter().for_each(|(&k, v)|
        v.iter().for_each(|s|
            molecule.match_indices(k).for_each(|(i, _)| {
                let mut m = molecule.to_string();
                m.replace_range(i..i+k.len(), s);
                molecules.insert(m);
            })
        )
    );
    
    molecules.len() as i32
}

fn part_two(molecule: &str, rules: &Rules) -> u32 {
    let rrules = rules.iter().map(|(&k, v)|
        v.iter().map(|&s| (s, k)).collect::<Vec<_>>()
    ).flatten().collect::<HashMap<_, _>>();

    let mut cache = HashMap::new();
    reduce(&mut cache, molecule.to_string(), 0, &rrules)
}

fn reduce(
    cache: &mut HashMap<String, u32>,
    molecule: String,
    steps: u32,
    rules: &HashMap<&str, &str>
) -> u32 {
    if cache.contains_key(&molecule) {
        *cache.get(&molecule).unwrap()
    } else if molecule == "e" {
        println!("found: {}", steps);
        steps
    } else {
        let mut v = rules.iter().filter_map(|(&k, &s)|
            if let Some(i) = molecule.find(k) {
                Some((k, s, i))
            } else {
                None
            })
            .collect::<Vec<_>>();
        if v.len() > 0 {
            v.sort_by(|(a, _, _), (b, _, _)| b.len().cmp(&a.len()));
            v.iter().map(|(k, s, i)| {
                let mut m1 = molecule.clone();
                m1.replace_range(i..&(i+k.len()), s);
                let st = if m1.len() > 1 && m1.contains("e") {
                    // e can't be reduced
                    u32::MAX
                } else {
                    reduce(cache, m1.clone(), steps + 1, rules)
                };
                cache.insert(m1, st);
                st
            }).min().unwrap()
        } else {
            u32::MAX
        }
    }
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