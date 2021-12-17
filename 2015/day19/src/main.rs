use std::collections::HashMap;

type Rules<'a> = HashMap<&'a str, Vec<&'a str>>;

fn main() {
    use std::time::Instant;

    let (rules, molecule) = load(include_str!("./input.txt"));

    let t1 = Instant::now();
    let count = part_one(molecule, &rules);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", count, t2 - t1);
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
    use std::collections::HashSet;

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