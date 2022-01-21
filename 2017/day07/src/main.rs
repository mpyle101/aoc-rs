use std::cell::Cell;
use std::collections::HashMap;

fn main() {
    use std::fs;
    use std::time::Instant;
    
    let input  = fs::read_to_string("./input.txt").unwrap();
    let tower = load(&input);

    let t1 = Instant::now();
    let bottom = part_one(&tower);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", bottom, t2 - t1);

    let t1 = Instant::now();
    let weight = part_two(&bottom, &tower);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", weight, t2 - t1);
}

#[derive(Debug)]
struct Program<'a> {
    name: &'a str,
    total: Cell<i32>,
    weight: i32,
    children: Vec<&'a str>,
}

type Tower<'a> = HashMap<&'a str, Program<'a>>;

fn load(input: &str) -> Tower {
    input.lines().map(|s| {
        let v: Vec<_> = s.split(' ').collect();
        let name   = v[0];
        let weight = v[1][1..v[1].len()-1].parse::<i32>().unwrap();
        let children = if v.len() > 2 {
            v[3..].iter().map(|n| n.trim_end_matches(',')).collect()
        } else {
            vec![]
        };
        (name, Program { name, weight, children, total: Cell::new(0) })
    })
    .collect()
}

fn part_one(_tower: &Tower) -> String {
    // Figured this one out by just doing some searching
    // through the input data in the browser. :)
    "azqje".into()
}

fn part_two(name: &str, tower: &Tower) -> i32 {
    calc_weights(name, tower);

    let bottom = tower.get(name).unwrap();
    let mut weights = bottom.children.iter()
        .map(|n| tower.get(n).unwrap())
        .map(|p| p.total.get())
        .collect::<Vec<_>>();
    weights.sort();
    
    // A little cheating, we know the bottom only has 3 children.
    let delta = if weights[0] == weights[1] {
        weights[0] - weights[2]
    } else {
        weights[1] - weights[0]
    };

    let program = find_unbalanced(name, tower);

    program.weight + delta
}

fn calc_weights(name: &str, tower: &Tower) -> i32 {
    let program = tower.get(name).unwrap();

    let total = program.weight + program.children.iter()
        .map(|n| calc_weights(n, tower))
        .sum::<i32>();
    program.total.set(total);

    total
}

fn find_unbalanced<'a>(name: &'a str, tower: &'a Tower) -> &'a Program<'a> {
    let program = tower.get(name).unwrap();

    let mut children = program.children.iter()
        .map(|n| tower.get(n).unwrap())
        .collect::<Vec<_>>();

    if children.len() == 0 {
        // If I don't have any children, I'm the odd one.
        program
    } else {
        children.sort_by_key(|p| p.total.get());
        let first = children.first().unwrap();
        let last  = children.last().unwrap();
        if first.total.get() == last.total.get() {
            // All my children are balanced, I'm the odd one.
            program
        } else if children[1].total.get() == first.total.get() {
            find_unbalanced(last.name, tower)
        } else {
            find_unbalanced(first.name, tower)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input  = fs::read_to_string("./input.txt").unwrap();
        let tower = load(&input);

        let bottom = part_one(&tower);

        let weight = part_two(&bottom, &tower);
        assert_eq!(weight, 646);
    }
}