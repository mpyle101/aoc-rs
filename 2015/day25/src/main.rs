
fn main() {
    use std::time::Instant;

    let t1 = Instant::now();
    let code = part_one();
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", code, t2 - t1);
}

fn part_one() -> u64 {

    let target = (2978, 3083);

    let mut last = 1;
    let mut cell = (1, 1);
    let mut code: u64 = 20151125;

    while cell != target {
        code *= 252533;
        code %= 33554393;

        if cell.0 == 1 {
            cell.0 = last + 1;
            cell.1 = 1;
            last += 1;
        } else {
            cell.0 -= 1;
            cell.1 += 1;
        }
    }

    code
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let code = part_one();
    assert_eq!(code, 2650453);
  }
}