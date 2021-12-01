fn main() {
    let public_keys: Vec<_> = include_str!("./input.txt").lines()
        .map(|l| l.parse::<u64>().unwrap()).collect();

    let enc_key = part_one(public_keys[0], public_keys[1]);
    println!("Part 1: {}", enc_key);
}

fn part_one(card: u64, door: u64) -> u64 {
    let mut loops = 0;
    let mut value = 1;
    while value != card {
        value = (value * 7) % 20201227;
        loops += 1;
    }

    (0..loops).fold(1, |v, _| (v * door) % 20201227)
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let public_keys: Vec<_> = include_str!("./input.txt").lines()
        .map(|l| l.parse::<u64>().unwrap()).collect();

    let enc_key = part_one(public_keys[0], public_keys[1]);
    assert_eq!(enc_key, 12227206);
  }
}