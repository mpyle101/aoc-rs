use std::collections::HashMap;

fn main() {
    use std::time::Instant;

    let happiness = load(include_str!("./input.txt"));

    let t1 = Instant::now();
    let delta = part_one(&happiness);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", delta, t2 - t1);

    let t1 = Instant::now();
    let delta = part_two(&happiness);
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", delta, t2 - t1);
}

fn load(input: &str) -> HashMap<(&str, &str), i32> {
    input.lines().map(|l| {
        let v = l.split(' ').collect::<Vec<_>>();
        let n1 = v[0];
        let n2 = v[10].trim_end_matches('.');
        let mut units = v[3].parse::<i32>().unwrap();
        if v[2] == "lose" { units *= -1 }
        ((n1, n2), units)
    })
    .collect()
}

fn part_one(map: &HashMap<(&str, &str), i32>) -> i32 {
    use std::collections::HashSet;
    use itertools::Itertools;

    let names = map.keys().map(|(n, _)| *n).collect::<HashSet<_>>();
    names.iter().permutations(names.len())
        .map(|v| {
            (0..v.len()).fold(0, |acc, i| {
                let left  = if i == 0 { v.last().unwrap() } else { v[i-1] };
                let right = v[(i + 1) % v.len()];

                acc + 
                *map.get(&(v[i], left)).unwrap() +
                *map.get(&(v[i], right)).unwrap()
            })
        })
        .max()
        .unwrap()
}

fn part_two(map: &HashMap<(&str, &str), i32>) -> i32 {
    use std::collections::HashSet;
    use itertools::Itertools;

    let mut names = map.keys().map(|(n, _)| *n).collect::<HashSet<_>>();
    let mut map2 = map.clone();
    names.iter().for_each(|n| {
        map2.insert((n, "me"), 0);
        map2.insert(("me", n), 0);
    });
    names.insert("me");

    names.iter().permutations(names.len())
        .map(|v| {
            (0..v.len()).fold(0, |acc, i| {
                let left  = if i == 0 { v.last().unwrap() } else { v[i-1] };
                let right = v[(i + 1) % v.len()];

                acc + 
                *map2.get(&(v[i], left)).unwrap() +
                *map2.get(&(v[i], right)).unwrap()
            })
        })
        .max()
        .unwrap()
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let happiness = load(include_str!("./input.txt"));

    let delta = part_one(&happiness);
    assert_eq!(delta, 733);

    let delta = part_two(&happiness);
    assert_eq!(delta, 725);
  }
}