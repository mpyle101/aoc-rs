use std::collections::{BTreeMap, HashMap};

fn main() {
    use std::fs;
    use std::time::Instant;
    
    let input = fs::read_to_string("./test.txt").unwrap();
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

type Rules = BTreeMap<[u8;2], u8>;

fn load(input: &str) -> (&[u8], Rules) {
    let mut iter = input.split("\n\n");
    let template = iter.next().unwrap().as_bytes();

    let rules = iter.next().unwrap().lines()
        .map(|l| {
            let v = l.split(" -> ").collect::<Vec<_>>();
            let c1 = v[0].as_bytes()[0];
            let c2 = v[0].as_bytes()[1];

            ([c1, c2], v[1].as_bytes()[0])
        })
        .collect::<Rules>();

    (template, rules)
}

fn part_one(template: &[u8], rules: &Rules) -> i32 {
    polymerize_a(10, template, rules)
}

fn part_two(template: &[u8], rules: &Rules) -> i32 {
    polymerize_b(10, template, rules)
}

fn polymerize_a(steps: i32, template: &[u8], rules: &Rules) -> i32 {
    let tmpl = (0..steps).fold(template.to_vec(), |s, _| {
        let mut t = s.windows(2).flat_map(|v| {
            let r = rules.get(v).unwrap();
            [v[0], *r]
        })
        .collect::<Vec<_>>();
        t.push(s[s.len() - 1]);
        t
    });

    let mut counts = HashMap::new();
    tmpl.iter().for_each(|c| *counts.entry(c).or_insert(0) += 1);
    let most  = counts.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    let least = counts.iter().max_by(|a, b| b.1.cmp(&a.1)).unwrap();

    most.1 - least.1
}

fn polymerize_b(steps: i32, template: &[u8], rules: &Rules) -> i32 {
    let mut counts = HashMap::new();
    template.iter().for_each(|c| *counts.entry(*c).or_insert(0) += 1);

    template.windows(2).for_each(|v| expand(v, steps, &mut counts, rules));
    
    let most  = counts.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    let least = counts.iter().max_by(|a, b| b.1.cmp(&a.1)).unwrap();

    most.1 - least.1
}

fn expand(
    tmpl: &[u8],
    steps: i32,
    counts: &mut HashMap<u8, i32>,
    rules: &Rules
) {
    if steps > 0 {
        let r = *rules.get(tmpl).unwrap();
        *counts.entry(r).or_insert(0) += 1;
        expand(&[tmpl[0], r], steps - 1, counts, rules);
        expand(&[r, tmpl[1]], steps - 1, counts, rules);
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
    }
}