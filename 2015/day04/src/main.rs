fn main() {
    let hash = compute("yzbqklnj", "00000");
    println!("Part 1: {}", hash);

    let hash = compute("yzbqklnj", "000000");
    println!("Part 2: {}", hash);
}

fn compute(secret: &str, tag: &str) -> u32 {
    let mut n = 0;
    loop {
        let key = secret.to_owned() + &n.to_string();
        let digest = md5::compute(key);
        if format!("{:x}", digest).starts_with(tag) {
            break n
        }
        n += 1
    }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let hash = compute("yzbqklnj", "00000");
    assert_eq!(hash, 282749);

    let hash = compute("yzbqklnj", "000000");
    assert_eq!(hash, 9962624);
  }
}