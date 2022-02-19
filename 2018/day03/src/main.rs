use std::str::FromStr;

fn main() {
    use std::time::Instant;

    let claims = load(include_str!("./input.txt"));

    let t1 = Instant::now();
    let overlaps = part_one(&claims);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", overlaps, t2 - t1);

    let t1 = Instant::now();
    let fabric = part_two(&claims);
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", fabric, t2 - t1);
}

fn part_one(claims: &[Claim]) -> usize {
    use ndarray::{s, Array2};
    use std::cmp::max;

    let (rows, cols) = claims.iter().fold((0, 0), |br, claim|
        (max(br.0, claim.rect.2), max(br.1, claim.rect.3))
    );
    let mut heatmap = Array2::<usize>::zeros((rows as usize, cols as usize));
    claims.iter().for_each(|c| {
        let (t, l, b, r) = c.rect;
        heatmap.slice_mut(s![t..b, l..r]).iter_mut().for_each(|v| *v += 1);
    });

    heatmap.iter().filter(|&v| *v > 1).count()
}

fn part_two(claims: &[Claim]) -> usize {
    use ndarray::{s, Array2};
    use std::cmp::max;

    let (rows, cols) = claims.iter().fold((0, 0), |br, claim|
        (max(br.0, claim.rect.2), max(br.1, claim.rect.3))
    );
    let mut heatmap = Array2::<usize>::zeros((rows as usize, cols as usize));
    claims.iter().for_each(|c| {
        let (t, l, b, r) = c.rect;
        heatmap.slice_mut(s![t..b, l..r]).iter_mut().for_each(|v| *v += 1);
    });

    let mut iter = claims.iter().skip_while(|c| {
        let (t, l, b, r) = c.rect;
        !heatmap.slice(s![t..b, l..r]).iter().all(|v| *v == 1)
    });

    iter.next().unwrap().id
}

fn load(input: &str) -> Vec<Claim> {
    input.lines()
        .map(|s| s.split(' ').collect::<Vec<_>>())
        .map(|v| {
            let id = to_usize(&v[0][1..]);
            let lt: Vec<_> = v[2].split(',').map(decolonize).map(to_usize).collect();
            let wh: Vec<_> = v[3].split('x').map(to_usize).collect();

            let br = [lt[1] + wh[1], lt[0] + wh[0]];
            Claim { id, rect: (lt[1], lt[0], br[0], br[1]) }
        })
        .collect()
}

fn to_usize(n: &str) -> usize {
    usize::from_str(n).unwrap()
}

fn decolonize(s: &str) -> &str {
    if let Some(stripped) = s.strip_suffix(':') { stripped } else { s }
}

struct Claim {
    id: usize,
    rect: (usize, usize, usize, usize), // t, l, b, r
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let claims = load(include_str!("./input.txt"));

    let overlaps = part_one(&claims);
    assert_eq!(overlaps, 104241);

    let fabric = part_two(&claims);
    assert_eq!(fabric, 806);
  }
}
