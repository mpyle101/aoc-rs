
fn main() {
    let policies = load(include_str!("./passwords.txt"));

    let valid = part_one(&policies);
    println!("Part1: {}", valid);

    let valid = part_two(&policies);
    println!("Part2: {}", valid);
}

#[derive(Debug)]
struct Policy<'a> {
    min: usize,
    max: usize,
    letter: u8,
    password: &'a [u8]
}

fn load(passwords: &str) -> Vec<Policy> {
    passwords.lines()
        .map(|l| l.split(' ').collect::<Vec<&str>>())
        .map(|v| (v[0].split('-').collect::<Vec<&str>>(), v[1].as_bytes()[0], v[2]))
        .map(|(v, letter, password)| Policy {
            min: v[0].parse::<usize>().unwrap(),
            max: v[1].parse::<usize>().unwrap(),
            letter,
            password: password.as_bytes()
        })
        .collect()
}

fn part_one(policies: &[Policy]) -> u32 {
    policies.iter().fold(0, |acc, p| {
        let count = p.password.iter()
            .filter(|&c| *c == p.letter).count() as usize;
        if count >= p.min && count <= p.max { acc + 1 } else { acc }
    })
}

fn part_two(policies: &[Policy]) -> u32 {
    policies.iter().fold(0, |acc, p| {
        let s = p.password;
        acc + (((s[p.min - 1] == p.letter) as u8) +
               ((s[p.max - 1] == p.letter) as u8) == 1) as u32
    })
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let policies = load(include_str!("./passwords.txt"));

    let valid = part_one(&policies);
    assert_eq!(valid, 538);

    let valid = part_two(&policies);
    assert_eq!(valid, 489);
  }
}