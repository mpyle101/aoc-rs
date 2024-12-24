use std::collections::HashMap;

type Wires<'a> = HashMap<&'a str, u64>;
type Gates<'a> = HashMap<&'a str, (&'a str, &'a str, char)>;

fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());
}

#[allow(clippy::manual_strip)]
fn part_one(input: &str) -> u64
{
    let (s1, s2) = input.split_once("\n\n").unwrap();
    let mut wires = s1.lines()
        .filter_map(|line| line.split_once(": "))
        .filter_map(|(w, v)| v.parse::<u64>().ok().map(|n| (w, n)))
        .collect::<Wires>();

    let mut z_wires = Vec::new();
    let gates = s2.lines()
        .fold(Gates::new(), |mut m, line| {
            let mut it = line.split(' ');
            let a  = it.next().unwrap();
            let op = match it.next().unwrap() {
                "AND" => '&',
                "OR"  => '|',
                "XOR" => '^',
                _ => unreachable!()
            };
    
            let b  = it.next().unwrap();
            it.next();
            let c  = it.next().unwrap();

            if c.starts_with('z') { z_wires.push(c) }
            m.insert(c, (a, b, op));
            m
        });

    let mut z = 0;
    for w in z_wires {
        let n = evaluate(w, &mut wires, &gates);
        let i = w[1..].parse::<u64>().unwrap();
        if n == 1 { z |= 1 << i } else { z &= !(1 << i) }
    }

    z
}

fn evaluate<'a>(w: &'a str, wires: &mut Wires<'a>, gates: &Gates<'a>) -> u64
{
    if let Some(n) = wires.get(w) {
        *n
    } else if let Some(&(a, b, op)) = gates.get(w) {
        let a = evaluate(a, wires, gates);
        let b = evaluate(b, wires, gates);
        let n = match op {
            '&' => a & b,
            '|' => a | b,
            '^' => a ^ b,
            _ => unreachable!()
        };

        wires.insert(w, n);
        n
    } else {
        unreachable!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 46463754151024);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example1.txt");
        assert_eq!(part_one(input), 4);

        let input = include_str!("../example2.txt");
        assert_eq!(part_one(input), 2024);
    }
}
