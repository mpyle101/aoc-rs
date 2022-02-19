use serde_json::{ Map, Value };

fn main() {
    use std::time::Instant;

    let input = include_str!("./input.txt");

    let t1 = Instant::now();
    let sum = part_one(input);
    let t2 = Instant::now();
    println!("Part 1: {sum} ({:?})", t2 - t1);

    let t1 = Instant::now();
    let sum = part_two(input);
    let t2 = Instant::now();
    println!("Part 2: {sum} ({:?})", t2 - t1);
}

fn part_one(s: &str) -> i32 {
    use regex::Regex;

    let re = Regex::new(r"(?-u:\-?\d+)").unwrap();
    re.find_iter(s)
        .map(|v| v.as_str().parse::<i32>().unwrap())
        .sum()
}

fn part_two(s: &str) -> i64 {
    let json: Vec<Value> = serde_json::from_str(s).unwrap();
    json.iter().map(process).sum()
}

fn process(v: &Value) -> i64 {
    match v {
        Value::Number(n)   => n.as_i64().unwrap(),
        Value::Array(arr)  => arr.iter().map(process).sum(),
        Value::Object(obj) => process_attributes(obj),
        _ => 0,
    }
}

fn process_attributes(obj: &Map<String, Value>) -> i64 {
    let red = obj.values().any(|v| v.as_str() == Some("red"));
    if red { 0 } else { obj.values().map(process).sum() }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = include_str!("./input.txt");

    let sum = part_one(&input);
    assert_eq!(sum, 191164);

    let sum = part_two(&input);
    assert_eq!(sum, 87842);
  }
}