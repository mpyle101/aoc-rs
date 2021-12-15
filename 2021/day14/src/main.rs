use std::collections::{BTreeMap, HashMap};

type Counts = HashMap<u8, i64>;
type Cache  = HashMap<(u16, u8), Counts>;
type Rules  = BTreeMap<u16, u8>;

fn main() {
    use std::fs;
    use std::time::Instant;
    
    let input = fs::read_to_string("./input.txt").unwrap();
    let (template, rules) = load(&input);

    let t1 = Instant::now();
    let diff = part_one(template, &rules);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", diff, t2 - t1);

    let t1 = Instant::now();
    let diff = part_two(template, &rules);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", diff, t2 - t1);
}

fn load(input: &str) -> (&[u8], Rules) {
    let mut iter = input.split("\n\n");
    let template = iter.next().unwrap().as_bytes();

    let rules = iter.next().unwrap().lines()
        .map(|l| {
            let v = l.split(" -> ").collect::<Vec<_>>();
            let c1 = (v[0].as_bytes()[0] as u16) << 8;
            let c2 = v[0].as_bytes()[1] as u16;
            (c1 | c2, v[1].as_bytes()[0])
        })
        .collect::<Rules>();

    (template, rules)
}

fn part_one(template: &[u8], rules: &Rules) -> i64 {
    polymerize(10, template, rules)
}

fn part_two(template: &[u8], rules: &Rules) -> i64 {
    polymerize(40, template, rules)
}

fn polymerize(steps: u8, template: &[u8], rules: &Rules) -> i64 {
    let mut counts = HashMap::new();
    template.iter().for_each(|c| *counts.entry(*c).or_insert(0) += 1);

    let mut cache = HashMap::new();
    template.windows(2).for_each(|v| {
        let tmpl = (v[0] as u16) << 8 | v[1] as u16;
        let cnts = expand(tmpl, steps, rules, &mut cache);
        cnts.iter().for_each(|(k, v)| *counts.entry(*k).or_insert(0) += v);
    });
    
    let mut v = counts.values().cloned().collect::<Vec<_>>();
    v.sort();
    
    v[v.len() - 1] - v[0]
}

fn expand(
    tmpl: u16,
    steps: u8,
    rules: &Rules,
    cache: &mut Cache,
) -> Counts {
    if let Some(counts) = cache.get(&(tmpl, steps)) {
        counts.clone()
    } else if steps > 0 {
        let r = *rules.get(&tmpl).unwrap() as u16;
        let mut counts = HashMap::from([(r as u8, 1)]);
        let c1 = expand(tmpl & 0xFF00 | r, steps - 1, rules, cache);
        let c2 = expand(r << 8 | tmpl & 0xFF, steps - 1, rules, cache);
        c1.iter().for_each(|(k, v)| *counts.entry(*k).or_insert(0) += v);
        c2.iter().for_each(|(k, v)| *counts.entry(*k).or_insert(0) += v);
        cache.insert((tmpl, steps), counts.clone());

        counts
    } else {
        HashMap::new()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let (template, rules) = load(&input);

        let diff = part_one(template, &rules);
        assert_eq!(diff, 3697);

        let diff = part_two(template, &rules);
        assert_eq!(diff, 4371307836157);
    }
}