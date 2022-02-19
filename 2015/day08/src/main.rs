fn main() {
    let strings = include_str!("./input.txt");

    let diff = part_one(strings);
    println!("Part 1: {}", diff);

    let diff = part_two(strings);
    println!("Part 2: {}", diff);
}

fn part_one(strings: &str) -> usize {
    strings.lines().fold(0, |acc, s| {
        let bytes = s.as_bytes();
        let mut chars = 0;
        let mut i = 1;
        while i < bytes.len() - 1 {
            i = match (bytes[i] as char, bytes[i+1] as char) {
                ('\\', '"')  => { chars += 1; i + 2 },
                ('\\', '\\') => { chars += 1; i + 2 },
                ('\\', 'x')  => { chars += 1; i + 4 },
                           _ => { chars += 1; i + 1 },
            }
        }

        acc + bytes.len() - chars
    })
}

fn part_two(strings: &str) -> usize {
    strings.lines().fold(0, |acc, s| {
        let bytes = s.as_bytes();
        let chars = bytes.iter().flat_map(|&b| 
            match b as char {
                '"'  => vec!['\\', '"'],
                '\\' => vec!['\\', '\\'],
                  c  => vec![c]
            });

        acc + chars.count() - bytes.len() + 2
    })
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let strings = include_str!("./input.txt");

    let diff = part_one(strings);
    assert_eq!(diff, 1342);

    let diff = part_two(strings);
    assert_eq!(diff, 2074);
  }
}
