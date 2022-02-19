fn main() {
    use std::time::Instant;

    let input = include_str!("./input.txt");

    let t1 = Instant::now();
    let checksum = part_one(input);
    let t2 = Instant::now();
    println!("Part 1: {checksum} ({:?})", t2 - t1);

    let t1 = Instant::now();
    let letters = part_two(input);
    let t2 = Instant::now();
    println!("Part 2: {letters} ({:?})", t2 - t1);
}

fn part_one(boxes: &str) -> u32 {
    let other = [3, 2];
    let mut letters: [u8;26] = [0;26];

    let [n2, n3] = boxes.lines()
        .fold([0, 0], |mut cnt, line| {
            letters = [0;26];
            line.chars().for_each(|c| letters[c as usize - 97] += 1);

            let mut iter = letters.iter().skip_while(|&v| *v != 2 && *v != 3);
            if let Some(n) = iter.next() {
                cnt[*n as usize - 2] += 1;
                let i = other[*n as usize - 2];
                let mut iter = iter.skip_while(|&v| *v != i);
                if let Some(m) = iter.next() {
                    cnt[*m as usize - 2] += 1;
                }
            }
            cnt
        });

    n2 * n3
}

fn part_two(boxes: &str) -> String {
    use itertools::Itertools;

    let m: Vec<_> = boxes.lines()
        .combinations(2)
        .map(|v| (v[0].as_bytes(), v[1].as_bytes()))
        .collect();

    let v = (0..26).fold(vec![0;m.len()], |mut v, i| {
        v.iter_mut()
            .enumerate()
            .for_each(|(n, v)| {
                let (v1, v2) = m[n];
                *v += if v1[i] != v2[i] { 1 } else { 0 };
            });
        v
    });

    let (v1, v2) = m[v.iter().position(|n| *n == 1).unwrap()];
    v1.iter().zip(v2.iter())
        .filter_map(|(c1, c2)| (c1 == c2).then(|| *c1 as char))
        .collect()
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = include_str!("./input.txt");

    let checksum = part_one(&input);
    assert_eq!(checksum, 5368);

    let letters = part_two(&input);
    assert_eq!(letters, "cvgywxqubnuaefmsljdrpfzyi");
  }
}
