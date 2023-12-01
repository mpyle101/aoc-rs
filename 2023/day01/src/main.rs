fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let calibration = part_one(input);
    println!("Part 1: {} ({:?})", calibration, t.elapsed());

    let t = Instant::now();
    let calibration = part_two(input);
    println!("Part 2: {} ({:?})", calibration, t.elapsed());
}

fn part_one(input: &str) -> u32
{
    input.lines()
        .map(|s| s.as_bytes().iter()
            .fold((0, 0), |acc, &c| {
                if c.is_ascii_digit() {
                    if acc.0 == 0 {
                        return (c - b'0', c - b'0')
                    } else {
                        return (acc.0, c - b'0')
                    }
                };
                acc
            }))
        .map(|(d1, d2)| d1 as u32 * 10 + d2 as u32)
        .sum()
}

fn part_two(input: &str) -> u32
{
    let nums = vec![
        ("one".as_bytes(), 1),
        ("two".as_bytes(), 2),
        ("three".as_bytes(), 3),
        ("four".as_bytes(), 4),
        ("five".as_bytes(), 5),
        ("six".as_bytes(), 6),
        ("seven".as_bytes(), 7),
        ("eight".as_bytes(), 8),
        ("nine".as_bytes(), 9),
    ];

    input.lines()
        .map(|s| {
            let mut d1 = 0;
            let mut d2 = 0;

            let bytes = s.as_bytes();
            (0..s.len()).for_each(|n| {
                let mut digit = 0;
                let v = &bytes[n..];
                if v[0].is_ascii_digit() {
                    digit = v[0] - b'0';
                } else {
                    for (key, value) in nums.iter() {
                        if v.starts_with(key) {
                            digit = *value;
                            break;
                        }
                    }
                }

                if digit > 0 {
                    if d1 == 0 {
                        d1 = digit;
                        d2 = digit;
                    } else {
                        d2 = digit;
                    }
                }
            });

            d1 as u32 * 10 + d2 as u32
        })
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 53974);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 52840);
    }

    #[test]
    fn example_part_two() {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 281);
    }
}
