use std::collections::HashMap;

type Rules<'a> = HashMap<&'a str, Vec<&'a str>>;

fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two(input);
    println!("Part 2: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str) -> u32
{
    use std::str::FromStr;

    let (s1, s2) = input.split_once("\n\n").unwrap();
    let rules = s1.lines()
        .filter_map(|line| line.split_once('|'))
        .fold(Rules::new(), |mut m, (a, b)| {
            m.entry(a).or_default().push(b);
            m
        });

    s2.lines()
        .map(|line| line.split(',').collect::<Vec<_>>())
        .filter(|v| is_ordered(&rules, v))
        .map(|v| v[v.len() / 2])
        .filter_map(|s| u32::from_str(s).ok())
        .sum()
}

fn part_two(input: &str) -> u32
{
    use std::str::FromStr;

    let (s1, s2) = input.split_once("\n\n").unwrap();
    let rules = s1.lines()
        .filter_map(|line| line.split_once('|'))
        .fold(Rules::new(), |mut m, (a, b)| {
            m.entry(a).or_default().push(b);
            m
        });

    s2.lines()
        .map(|line| line.split(',').collect::<Vec<_>>())
        .filter(|v| !is_ordered(&rules, v))
        .map(|mut v| { reorder(&rules, &mut v); v })
        .map(|v| v[v.len() / 2])
        .filter_map(|s| u32::from_str(s).ok())
        .sum()
}

fn is_ordered(rules: &Rules, v: &[&str]) -> bool
{
    for i in 0..v.len()-1 {
        for j in i+1..v.len() {
            if let Some(r) = rules.get(v[j]) {
                if r.contains(&v[i]) {
                    return false
                }
            }
        }
    }

    true
}

fn reorder(rules: &Rules, v: &mut [&str])
{
    for i in 0..v.len()-1 {
        for j in i+1..v.len() {
            if let Some(r) = rules.get(v[j]) {
                if r.contains(&v[i]) {
                    v.swap(i, j);
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 4281);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 5466);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 143);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 123);
    }
}
