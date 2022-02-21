use std::collections::HashMap;

type Bags1<'a> = HashMap<(&'a str, &'a str), Vec<(&'a str, &'a str)>>;
type Bags2<'a> = HashMap<(&'a str, &'a str), Vec<((&'a str, &'a str), u32)>>;

fn main() {
    let bags = load_one(include_str!("./input.txt"));
    let colors = part_one(&bags);
    println!("Part 1: {}", colors);

    let bags = load_two(include_str!("./input.txt"));
    let total = part_two(&bags);
    println!("Part 2: {}", total);
}

fn part_one(bags: &Bags1) -> usize {
    use std::collections::{HashSet, VecDeque};

    let sg = bags.get(&("shiny", "gold")).unwrap();
    let mut q: VecDeque<_> = sg.iter().collect();
    let mut colors = HashSet::new();

    while let Some(&b) = q.pop_front() {
        colors.insert(b);
        if let Some(v) = bags.get(&b) {
            v.iter().for_each(|c| q.push_back(c))
        }
    };

    colors.len()
}

fn part_two(bags: &Bags2) -> u32 {
    use std::collections::VecDeque;

    let sg = bags.get(&("shiny", "gold")).unwrap();
    let mut q: VecDeque<_> = sg.iter().copied().collect();
    let mut total_bags = 0;

    while let Some(b) = q.pop_front() {
        if let Some(v) = bags.get(&b.0) {
            total_bags += b.1;
            v.iter().for_each(|c| q.push_back((c.0, c.1 * b.1)));
        } else {
            total_bags += b.1;
        }
    }

    total_bags
}

fn load_one(input: &str) -> Bags1 {
    let mut bags = HashMap::new();
    input.lines()
        .map(|s| s.split(" bags contain ").collect::<Vec<_>>())
        .map(|v| (v[0], v[1].split(", ").collect::<Vec<_>>()))
        .for_each(|(b, v)| v.iter().for_each(|&s| {
            let b = b.split(' ').collect::<Vec<_>>();
            let c = s.split(' ').collect::<Vec<_>>();
            bags.entry((c[1], c[2])).or_insert_with(Vec::new).push((b[0], b[1]));
        }));

    bags
}

fn load_two(input: &str) -> Bags2 {
    let mut bags = HashMap::new();
    input.lines()
        .map(|s| s.split(" bags contain ").collect::<Vec<_>>())
        .map(|v| (v[0], v[1].split(", ").collect::<Vec<_>>()))
        .for_each(|(b, v)| v.iter().for_each(|&s| {
            let b = b.split(' ').collect::<Vec<_>>();
            let c = s.split(' ').collect::<Vec<_>>();
            if c[0] != "no" {
                bags.entry((b[0], b[1])).or_insert_with(Vec::new).push(
                    ((c[1], c[2]), c[0].parse::<u32>().unwrap())
                );
            }
        }));

    bags
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let bags = load_one(include_str!("./input.txt"));

    let colors = part_one(&bags);
    assert_eq!(colors, 142);

    let bags = load_two(include_str!("./input.txt"));
    let total = part_two(&bags);
    assert_eq!(total, 10219);
  }

  #[test]
  fn example_1() {
    let bags = load_two("\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.");

    let total = part_two(&bags);
    assert_eq!(total, 32)
  }

  #[test]
  fn example_2() {
    let bags = load_two("\
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.");

    let total = part_two(&bags);
    assert_eq!(total, 126)
  }
}
