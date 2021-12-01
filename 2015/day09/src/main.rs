use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let routes = load(include_str!("./input.txt"));

    let shortest = part_one(&routes);
    println!("Part 1: {}", shortest);

    let longest = part_two(&routes);
    println!("Part 2: {}", longest);
}

fn part_one(routes: &Routes) -> u32 {
    let cities: HashSet<_> = routes.keys().map(|(a, _)| a).collect();
    cities.iter().permutations(cities.len())
        .map(|v| {
            (0..v.len()-1).fold(0, |acc, i| {
                let r = if let Some(d) = routes.get(&(v[i], v[i+1])) { *d } else { 1000 };
                acc + r
            })
        }).min().unwrap()
}


fn part_two(routes: &Routes) -> u32 {
    let cities: HashSet<_> = routes.keys().map(|(a, _)| a).collect();
    cities.iter().permutations(cities.len())
        .map(|v| {
            (0..v.len()-1).fold(0, |acc, i| {
                let r = if let Some(d) = routes.get(&(v[i], v[i+1])) { *d } else { 0 };
                acc + r
            })
        }).max().unwrap()
}

type Route<'a> = (&'a str, &'a str);
type Routes<'a> = HashMap<Route<'a>, u32>;

fn load(input: &str) -> Routes {
    input.lines()
        .map(|s| s.split(' ').collect::<Vec<_>>())
        .flat_map(|v| vec![
            ((v[0], v[2]), v[4].parse::<u32>().unwrap()),
            ((v[2], v[0]), v[4].parse::<u32>().unwrap()),
        ])
        .collect()
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let routes = load(include_str!("./input.txt"));

    let shortest = part_one(&routes);
    assert_eq!(shortest, 141);

    let longest = part_two(&routes);
    assert_eq!(longest, 736);
  }
}
