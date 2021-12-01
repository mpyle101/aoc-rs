fn main() {
    let words = include_str!("./input.txt");

    let count = part_one(words);
    println!("Part 1: {}", count);

    let count = part_two(words);
    println!("Part 2: {}", count);
}

fn part_one(words: &str) -> u32 {
    words.lines().fold(0, |acc, word| acc + nice(word) as u32)
}

fn part_two(words: &str) -> u32 {
    words.lines().fold(0, |acc, word| acc + nicer(word) as u32)
}

fn nice(word: &str) -> bool {
    let (vowels, double, bad, _) = word.chars()
        .fold((0, 0, 0, '^'), 
            |(v, d, b, l), c| {
                let v = v + "aeiou".contains(c) as i32;
                match (l, c) {
                    (l, c) if l == c => (v, d+1, b, c),
                    ('a', 'b') => (v, d, b+1, c),
                    ('c', 'd') => (v, d, b+1, c),
                    ('p', 'q') => (v, d, b+1, c),
                    ('x', 'y') => (v, d, b+1, c),
                    _ => (v, d, b, c)
                }
            }
        );

    vowels > 2 && double > 0 && bad == 0
}

fn nicer(word: &str) -> bool {
    use std::collections::HashMap;

    let mut pairs = HashMap::new();
    let (repeats, _, _) = word.chars()
        .fold((0, '^', '^'),
            |(r, c0, c1), c| {
                if !(c0 == c1 && c1 == c) {
                    *pairs.entry((c1, c)).or_insert(0) += 1;
                }
                (r + (c0 == c) as i32, c1, c)
            }
        );

    *pairs.values().max().unwrap() > 1 && repeats > 0
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let words = include_str!("./input.txt");

    let count = part_one(words);
    assert_eq!(count, 258);

    let count = part_two(words);
    assert_eq!(count, 53);
  }

  #[test]
  fn example_1() {
    assert!(nicer("qjhvhtzxzqqjkmpb"));
  }

  #[test]
  fn example_2() {
    assert!(nicer("xxyxx"));
  }

  #[test]
  fn example_3() {
    assert!(!nicer("uurcxstgmygtbstg"));
  }

  #[test]
  fn example_4() {
    assert!(!nicer("ieodomkazucvgmuy"));
  }
}