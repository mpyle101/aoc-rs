use std::collections::{HashMap, HashSet};

fn main() {
    let recipes = load(include_str!("./input.txt"));
    let allergens = find_allergens(&recipes);

    let count = part_one(&recipes, &allergens);
    println!("Part 1: {}", count);

    let ingredients = part_two(&allergens);
    println!("Part 2: {}", ingredients);
}

fn part_one(recipes: &[Recipe], allergens: &HashMap<&str, &str>) -> usize {
    recipes.iter()
        .map(|r| r.ingredients.iter().filter(|&e| !allergens.contains_key(e)).count())
        .sum()
}

fn part_two(allergens: &HashMap<&str, &str>) -> String {
    let mut values: Vec<_> = allergens.iter().map(|(&k, &v)| (v, k)).collect();
    values.sort();
    values.iter().map(|(_, k)| *k).collect::<Vec<_>>().join(",")
}

fn find_allergens<'a>(recipes: &'a [Recipe]) -> HashMap<&'a str, &'a str> {
    let mut allergens: HashMap<&str, HashSet<&str>> = HashMap::new();
    for r in recipes {
        let i: HashSet<_> = r.ingredients.iter().map(|e| *e).collect();
        for a in r.allergens.clone() {
            let mut s = allergens.get(a).unwrap_or(&i).clone();
            s = s.intersection(&i).map(|e| *e).collect();
            allergens.insert(a, s);
        }
    }

    let mut found: HashMap<&str, &str> = HashMap::new();

    while allergens.len() > 0 {
        let mut remove = Vec::new();
        let mut insert = Vec::new();

        for (&k, v) in allergens.iter() {
            if v.len() == 1 {
                found.insert(v.iter().nth(0).unwrap(), k);
                remove.push(k);
            } else {
                let l: HashSet<_> = v.iter()
                    .filter(|&&e| !found.contains_key(e))
                    .map(|e| *e).collect();
                insert.push((k, l));
            }
        }

        remove.iter().for_each(|k| { allergens.remove(k); });
        insert.iter().for_each(|(k, v)| { allergens.insert(k, v.clone()); });
    }

    found
}

fn load(input: &str) -> Vec<Recipe> {
    input.lines()
        .map(|s| s[..s.len()-1].split(" (contains ").collect::<Vec<_>>())
        .map(|v| {
            let allergens: Vec<_> = v[1].split(", ").collect();
            let ingredients: Vec<_> = v[0].split(' ').collect();

            Recipe { allergens, ingredients }
        })
        .collect()
}

#[derive(Debug)]
struct Recipe<'a> {
    allergens: Vec<&'a str>,
    ingredients: Vec<&'a str>,
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let recipes = load(include_str!("./input.txt"));
    let allergens = find_allergens(&recipes);

    let count = part_one(&recipes, &allergens);
    assert_eq!(count, 2125);

    let ingredients = part_two(&allergens);
    assert_eq!(ingredients, "phc,spnd,zmsdzh,pdt,fqqcnm,lsgqf,rjc,lzvh");
  }
}