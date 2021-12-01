fn main() {
    let len = part_one(&[3,1,1,3,3,2,2,1,1,3]);
    println!("Part 1: {}", len);

    let len = part_two(&[3,1,1,3,3,2,2,1,1,3]);
    println!("Part 2: {}", len);
}

fn part_one(digits: &[u8]) -> usize {
    (0..40).fold(digits.to_vec(), |v, _| cycle(&v)).len()
}

fn part_two(digits: &[u8]) -> usize {
    (0..50).fold(digits.to_vec(), |v, _| cycle(&v)).len()
}

fn cycle(digits: &[u8]) -> Vec<u8> {
    let mut run = 1;
    let mut curr = digits[0];

    let mut v = digits.iter().skip(1).fold(
        Vec::new(),
        |mut v, &d| {
        if d == curr { 
            run += 1;
        } else {
            v.push(run);
            v.push(curr);
            run = 1;
            curr = d;
        };
        v
    });
    v.push(run);
    v.push(curr);

    v
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let len = part_one(&[3,1,1,3,3,2,2,1,1,3]);
    assert_eq!(len, 329356);

    let len = part_two(&[3,1,1,3,3,2,2,1,1,3]);
    assert_eq!(len, 4666278);
  }
}
