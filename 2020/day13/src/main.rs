
fn main() {
    let (ts, shuttles) = load_notes(include_str!("./notes.txt"));

    let bus = part_one(ts, &shuttles);
    println!("Part1: {}", bus);

    let shuttles = load_shuttles(include_str!("./notes.txt"));
    let ts = part_two(&shuttles);
    println!("Part2: {}", ts);
}

fn load_notes(notes: &str) -> (i32, Vec<i32>) {
    let mut lines = notes.lines();
    let ts = lines.next().unwrap().parse::<i32>().unwrap();
    let shuttles = lines.next().unwrap().split(',')
        .filter_map(|v| match v {
            "x" => None,
             s  => s.parse::<i32>().ok()
        })
        .collect::<Vec<i32>>();

    (ts, shuttles)
}

fn load_shuttles(notes: &str) -> Vec<(u64, u64)> {
    let mut line = notes.lines().skip(1);
    let mut v: Vec<_> = line.next().unwrap().split(',')
        .enumerate()
        .filter_map(|(i, v)| match v {
            "x" => None,
             s  => s.parse::<u64>().and_then(|v| Ok((i as u64, v))).ok()
        })
        .collect();

    // Generate remainder / lookback values
    let last = v.last().unwrap().0;
    v.iter_mut().for_each(|(wait, _)| *wait = last - *wait );
    v
}

fn part_one(ts: i32, shuttles: &[i32]) -> i32 {
    let (wait, id) = shuttles.iter().map(|id| {
        let wait = (ts / id + 1) * id - ts;
        (wait, id)
    }).min().unwrap();
    
    wait * id
}

fn part_two(shuttles: &[(u64, u64)]) -> u64 {
    let f = shuttles[0].0;
    let m = chinese_remainder(shuttles);
    let mut ts = m;

    loop {
        if shuttles.iter().all(|(lb, id)| (ts - lb) % id == 0) {
            return ts - f
        }
        ts += m;
    }
}

fn chinese_remainder(shuttles: &[(u64, u64)]) -> u64 {
    let prod = shuttles.iter().fold(1, |acc, (_, id)| acc * id);
    let pp: Vec<_>  = shuttles.iter().map(|(_, id)| prod / id).collect();
    let inv: Vec<_> = shuttles.iter().enumerate()
        .map(|(i, (_, n))| modinv(pp[i], *n)).collect();
    
    shuttles.iter().enumerate()
        .fold(0, |acc, (i, (r, _))| acc + r * pp[i] * inv[i]) % prod
}

// Modulo inverse of a with respect to m
fn modinv(a: u64, m: u64) -> u64 {
    let mut aa = a as i64;
    let mut ma = m as i64;
    let mut x0 = 0;
    let mut x1 = 1;

    if m == 1 { return 0 }

    // Extended Euclid Algorithm
    while aa > 1 {
        let q = aa / ma;
        let t0 = ma;

        ma = aa % ma;
        aa = t0;

        let t1 = x0;
        x0 = x1 - q * x0;
        x1 = t1;
    }

    if x1 < 0 { x1 += m as i64 };
    x1 as u64
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let (ts, shuttles) = load_notes(include_str!("./notes.txt"));
    let bus = part_one(ts, &shuttles);
    assert_eq!(bus, 203);

    let shuttles = load_shuttles(include_str!("./notes.txt"));
    let ts = part_two(&shuttles);
    assert_eq!(ts, 905694340256752);
  }

  #[test]
  fn example_1() {
    let shuttles = load_shuttles("0\n7,13,x,x,59,x,31,19");

    let ts = part_two(&shuttles);
    assert_eq!(ts, 1068781);
  }

  #[test]
  fn example_2() {
    let shuttles = load_shuttles("0\n17,x,13,19");

    let ts = part_two(&shuttles);
    assert_eq!(ts, 3417);
  }

  #[test]
  fn example_3() {
    let shuttles = load_shuttles("0\n67,7,59,61");
    println!("{:?}", shuttles);
    let ts = part_two(&shuttles);
    assert_eq!(ts, 754018);
  }

  #[test]
  fn example_4() {
    let shuttles = load_shuttles("0\n67,x,7,59,61");

    let ts = part_two(&shuttles);
    assert_eq!(ts, 779210);
  }

  #[test]
  fn example_5() {
    let shuttles = load_shuttles("0\n67,7,x,59,61");
    let ts = part_two(&shuttles);

    assert_eq!(ts, 1261476);
  }

  #[test]
  fn example_6() {
    let shuttles = load_shuttles("0\n1789,37,47,1889");
    let ts = part_two(&shuttles);

    assert_eq!(ts, 1202161486);
  }
}