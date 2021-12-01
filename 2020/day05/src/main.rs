fn main() {
    let passes = load(include_str!("./passes.txt"));
    
    let ticket = part_one(&passes).unwrap();
    println!("Part1: {}", ticket);
    
    let ticket = part_two(&passes);
    println!("Part2: {}", ticket);
}

fn load(passes: &str) -> Vec<&str> {
    passes.lines().collect()
}

fn part_one(passes: &[&str]) -> Option<u32> {
    let rows: Vec<_> = passes.iter().map(|s| find_row(&s[0..7])).collect();
    let cols: Vec<_> = passes.iter().map(|s| find_col(&s[7..])).collect();

    rows.iter().zip(cols.iter()).map(|(r, c)| r * 8 + c).max()
}

fn part_two(passes: &[&str]) -> u32 {
    let rows: Vec<_> = passes.iter().map(|s| find_row(&s[0..7])).collect();
    let cols: Vec<_> = passes.iter().map(|s| find_col(&s[7..])).collect();
    let mut sids: Vec<_> = rows.iter().zip(cols.iter()).map(|(r, c)| r * 8 + c).collect();
    sids.sort();

    let mut prev = sids[0];
    for &id in sids.iter().skip(1) {
        if id - prev == 2 {
            return id - 1
        }
        prev = id
    }

    0
}

fn find_row(pass: &str) -> u32 {
    let mut rows = (0..128).map(|r| r).collect::<Vec<u32>>();

    for &c in pass.as_bytes() {
        let zones = rows.split_at(rows.len()/2);
        rows = if c == b'F' { zones.0.into() } else { zones.1.into() }
    }

    rows[0]
}

fn find_col(pass: &str) -> u32 {
    let mut cols = (0..8).map(|r| r).collect::<Vec<u32>>();

    for &c in pass.as_bytes() {
        let zones = cols.split_at(cols.len()/2);
        cols = if c == b'L' { zones.0.into() } else { zones.1.into() };
    }

    cols[0]
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let passes = load(include_str!("./passes.txt"));
    
    let ticket = part_one(&passes).unwrap();
    assert_eq!(ticket, 998);

    let ticket = part_two(&passes);
    assert_eq!(ticket, 676);
  }
}