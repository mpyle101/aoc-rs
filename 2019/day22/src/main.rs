// Didn't get Part 2
// https://github.com/sasa1977/aoc/blob/master/lib/2019/201922.ex

fn main() {
    let shuffles = load(include_str!("./shuffles.txt"));

    let pos = part_one(&shuffles);
    println!("Part 1: {}", pos);

    let pos = part_two(&shuffles);
    println!("Part 2: {}", pos);
}

fn part_one(shuffles: &[Shuffle]) -> i128 {
    const COUNT: i128 = 10_007;
    shuffles.iter().fold(2019, |pos, s| match s {
        Shuffle::New    => COUNT - pos - 1,
        Shuffle::Cut(n) => (pos + COUNT - n) % COUNT,
        Shuffle::Inc(n) => (pos * n) % COUNT,
    })
}

fn part_two(shuffles: &[Shuffle]) -> i128 {
    use mod_exp::mod_exp;

    const M: i128 = 119_315_717_514_047;
    const N: i128 = 101_741_582_076_661;

    // Convert to linear equation: ax + b
    // Run the command in reverse order.
    let (a, b) = shuffles.iter().rev().fold((1, 0), |(a, b), s| {
        let (a_prime, b_prime) = match s {
            Shuffle::New    => (-a, -b - 1),
            Shuffle::Cut(n) => ( a,  b + n),
            Shuffle::Inc(n) => {
                let n = mod_exp(*n, M - 2, M);
                (a * n, b * n)
            },
        };
        (a_prime % M, b_prime % M)
    });

    // Cycling the function n times devolves to:
    // x * a^n + b * (a^n - 1) / (a - 1)
    let t1 = 2020 * mod_exp(a, N, M) % M;
    let tmp = (mod_exp(a, N, M) - 1) * mod_exp(a - 1, M - 2, M) % M;
    let t2 = b * tmp % M;

    (t1 + t2) % M
}

enum Shuffle {
    New,
    Cut(i128),
    Inc(i128),
}

fn load(input: &str) -> Vec<Shuffle> {
    input.lines().map(|l| {
        let s: Vec<_> = l.split(' ').collect();
        match s[1] {
            "into" => Shuffle::New,
            "with" => Shuffle::Inc(s[3].parse::<i128>().unwrap()),
                _  => Shuffle::Cut(s[1].parse::<i128>().unwrap()),
        }
    })
    .collect()
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let shuffles = load(include_str!("./shuffles.txt"));

    let pos = part_one(&shuffles);
    assert_eq!(pos, 6526);

    let card = part_two(&shuffles);
    assert_eq!(card, 79855812422607);
  }
}