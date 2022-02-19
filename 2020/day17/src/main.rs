use std::collections::HashSet;

fn main() {
    let cubes = load(include_str!("./cubes.txt"));

    let active_count = part_one(&cubes);
    println!("Part 1: {}", active_count);

    let active_count = part_two(&cubes);
    println!("Part 2: {}", active_count);
}

fn load(input: &str) -> HashSet<Point> {
    input.lines()
        .enumerate()
        .flat_map(|(y, l)| l.as_bytes().iter()
            .enumerate()
            .filter_map(move |(x, &b)|
                (b == b'#').then(|| Point(x as i32, y as i32, 0, 0))
            )
        ).collect()
}

fn part_one(cubes: &HashSet<Point>) -> usize {
    use itertools::Itertools;

    let mut deltas: Vec<_> = (-1..=1).map(|_| -1..=1)
        .multi_cartesian_product()
        .map(|v| { 
            let mut w = v.clone();
            w.push(0);
            w
        }).collect();
    let home = deltas.iter().find_position(|v| is_home(v)).unwrap().0;
    deltas.remove(home);

    (0..6).fold(cubes.clone(), |acc, _| cycle(&acc, &deltas)).len()
}

fn part_two(cubes: &HashSet<Point>) -> usize {
    use itertools::Itertools;

    let mut deltas: Vec<_> = (-1..=1).flat_map(|w| 
        (-1..=1).map(|_| -1..=1)
            .multi_cartesian_product()
            .map(|p| { 
                let mut v = p.clone();
                v.push(w);
                v
            }).collect::<Vec<_>>()
        ).collect();
    let home = deltas.iter().find_position(|v| is_home(v)).unwrap().0;
    deltas.remove(home);

    (0..6).fold(cubes.clone(), |acc, _| cycle(&acc, &deltas)).len()
}

fn is_home(v: &Vec<i32>) -> bool {
    v.iter().all(|&n| n == 0)
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point(i32, i32, i32, i32);

fn cycle(cubes: &HashSet<Point>, deltas: &[Vec<i32>]) -> HashSet<Point> {
    use std::collections::VecDeque;

    let mut active = HashSet::new();
    let mut queue: VecDeque<_> = cubes.iter().cloned().collect();
    while let Some(pt) = queue.pop_back() {
        let nearby: Vec<_> = deltas.iter().map(|v|
            Point(pt.0 + v[0], pt.1 + v[1], pt.2 + v[2], pt.3 + v[3])
        ).collect();

        let count = nearby.iter().filter(|&p| cubes.contains(p)).count();
        if cubes.contains(&pt) {
            nearby.iter().for_each(|p| if !cubes.contains(p) { queue.push_back(*p)});
 
            if count == 2 || count == 3 {
                active.insert(pt);
            }
        } else if count == 3 {
            active.insert(pt);
        }
    }
    
    active
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let cubes = load(include_str!("./cubes.txt"));

    let active_count = part_one(&cubes);
    assert_eq!(active_count, 319);

    let active_count = part_two(&cubes);
    assert_eq!(active_count, 2324);
  }
}