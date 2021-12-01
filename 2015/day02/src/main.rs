fn main() {
    let gifts = load(include_str!("./input.txt"));

    let paper = part_one(&gifts);
    println!("Part 1: {}", paper);

    let ribbon = part_two(&gifts);
    println!("Part 2: {}", ribbon);
}

fn part_one(gifts: &[(u32, u32, u32)]) -> u32 {
    gifts.iter()
        .map(|t| [2*t.0+t.1, 2*t.1+t.2, 2*t.0+t.2])
        .map(|v| v.iter().sum::<u32>() + v.iter().min().unwrap() / 2)
        .fold(0, |acc, v: u32| acc + v)
}

fn part_two(gifts: &[(u32, u32, u32)]) -> u32 {
    gifts.iter()
        .map(|t| (t.0*t.1*t.2, [2*(t.0+t.1), 2*(t.1+t.2), 2*(t.0+t.2)]))
        .map(|(p, v)| p + v.iter().min().unwrap())
        .fold(0, |acc, v: u32| acc + v)
}

fn load(input: &str) -> Vec<(u32, u32, u32)> {
    input.lines()
        .map(|s| s.split('x').collect::<Vec<_>>())
        .map(|v| (
            v[0].parse::<u32>().unwrap(),
            v[1].parse::<u32>().unwrap(),
            v[2].parse::<u32>().unwrap()
        ))
        .collect()
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let gifts = load(include_str!("./input.txt"));

    let paper = part_one(&gifts);
    assert_eq!(paper, 1586300);

    let ribbon = part_two(&gifts);
    assert_eq!(ribbon, 3737498);
  }
}