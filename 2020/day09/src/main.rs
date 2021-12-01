fn main() {
    let xmas = load(include_str!("./xmas.txt"));

    let num = part_one(&xmas);
    println!("Part 1: {}", num);

    let num = part_two(&xmas, num);
    println!("Part 2: {}", num);
}

fn load(input: &str) -> Vec<u64> {
    input.lines().map(|l| l.parse::<u64>().unwrap()).collect()
}

fn part_one(xmas: &[u64]) -> u64 {
    use itertools::Itertools;

    let mut iter = xmas.iter().enumerate().skip(25)
        .skip_while(|(i, n)| xmas[i - 25..*i].iter()
            .combinations(2).any(|v| v[0] + v[1] == **n));

    *(iter.next().unwrap().1)
}

fn part_two(xmas: &[u64], n: u64) -> u64 {
    let mut s = 0;
    let mut e = 0;
    let mut sum = xmas[e];
    while sum != n {
        while sum < n {
            e += 1;
            sum += xmas[e];
        }
        while sum > n {
            sum -= xmas[s];
            s += 1;
        }
    }

    let min = xmas[s..e].iter().min().unwrap();
    let max = xmas[s..e].iter().max().unwrap();
    min + max
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let xmas = load(include_str!("./xmas.txt"));

        let num = part_one(&xmas);
        assert_eq!(num, 1124361034);

        let num = part_two(&xmas, num);
        assert_eq!(num, 129444555);
    }

    #[test]
    fn small_weakness() {
        let xmas = [35,20,15,25,47,40,62,55,65,95];

        let weakness = part_two(&xmas, 127);
        assert_eq!(weakness, 62);
    }
}