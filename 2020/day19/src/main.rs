use regex::Regex;
use std::collections::HashMap;

type Rules<'a> = HashMap<u32, Rule<'a>>;

fn main() {
    let (mut rules, msgs) = load(include_str!("./input.txt"));

    let valid = part_one(&rules, msgs);
    println!("Part 1: {}", valid);

    let valid = part_two(&mut rules, msgs);
    println!("Part 2: {}", valid);
}

fn part_one(rules: &Rules, msgs: &str) -> usize {
    let rs = build_regex(rules, 0);
    let re = Regex::new(&rs).unwrap();
    
    msgs.lines().filter(|msg| re.is_match(msg)).count()
}

fn part_two(rules: &mut Rules, msgs: &str) -> usize {
    rules.insert(8, Rule::Either(vec![42], vec![42, 8]));
    rules.insert(11, Rule::Either(vec![42, 31], vec![42, 11, 31]));

    let rs = build_regex(rules, 0);
    let re = Regex::new(&rs).unwrap();
    
    msgs.lines()
        .filter(|msg| re.is_match(msg)).count()
}


fn load(input: &str) -> (Rules, &str) {
    let v: Vec<_> = input.split("\n\n").collect();
    let rules: HashMap<u32, Rule> = v[0].lines()
        .map(|s| {
            let r: Vec<_> = s.split(' ').collect();
            let mut chars = r[0].chars();
            chars.next_back();
            let id = chars.as_str().parse::<u32>().unwrap();

            let rule = match r.len() {
                2 => if r[1].starts_with('"') {
                    let mut chars = r[1].chars();
                    chars.next(); chars.next_back();
                    Rule::Literal(chars.as_str())
                } else {
                    Rule::Concat(vec![r[1].parse::<u32>().unwrap()])
                },
                3 => Rule::Concat(vec![
                    r[1].parse::<u32>().unwrap(),
                    r[2].parse::<u32>().unwrap(),
                ]),
                4 => Rule::Either(
                    vec![r[1].parse::<u32>().unwrap()],
                    vec![r[3].parse::<u32>().unwrap()],
                ),
                _ => Rule::Either(
                    vec![r[1].parse::<u32>().unwrap(), r[2].parse::<u32>().unwrap()],
                    vec![r[4].parse::<u32>().unwrap(), r[5].parse::<u32>().unwrap()]
                ),
            };

            (id, rule)
        })
        .collect();

    (rules, v[1])
}

#[derive(Debug)]
enum Rule<'a> {
    Concat(Vec<u32>),
    Either(Vec<u32>, Vec<u32>),
    Literal(&'a str),
}

fn build_regex(rules: &Rules, r: u32) -> String {
    format!("^{}$", clause(rules, r, 0))
}

fn clause(rules: &Rules, id: u32, depth: u32) -> String {
    // The strings are only so big, so cap the depth recursive rules
    // can descend to. 15 gives us enough for the right answers to
    // both part 1 and part 2.
    if depth == 15 {
        return "".into()
    }

    let rule = rules.get(&id).unwrap();

    match rule {
        Rule::Concat(v)  => v.iter().map(|&r| clause(rules, r, depth + 1)).collect(),
        Rule::Literal(s) => s.to_string(),
        Rule::Either(v1, v2) => 
            format!(
                "({}|{})",
                v1.iter().map(|&r| clause(rules, r, depth + 1)).collect::<String>(),
                v2.iter().map(|&r| clause(rules, r, depth + 1)).collect::<String>()
            )
    }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let (mut rules, msgs) = load(include_str!("./input.txt"));

    let valid = part_one(&rules, msgs);
    assert_eq!(valid, 210);

    let valid = part_two(&mut rules, msgs);
    assert_eq!(valid, 422);
  }
}