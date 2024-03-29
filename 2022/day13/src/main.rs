use std::cmp::Ordering;

fn main() {
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    println!("Part 1: {} ({:?})", part_one(input), t.elapsed());

    let t = Instant::now();
    println!("Part 2: {} ({:?})", part_two(input), t.elapsed());
}

fn part_one(input: &str) -> usize {
    use std::cmp::Ordering::Greater;

    input.split("\n\n")
        .enumerate()
        .filter_map(|(i, packets)| packets.split_once('\n').map(|v| (i, v)))
        .filter_map(|(i, (a, b))| (compare_packets(a, b) != Greater).then_some(i+1))
        .sum()
}

fn part_two(input: &str) -> usize {
    let markers = "[[2]]\n[[6]]";

    let mut packets: Vec<_> = markers.split('\n')
        .chain(input.split('\n'))
        .filter(|s| !s.is_empty())
        .collect();
    packets.sort_by(|a, b| compare_packets(a, b));
    
    let i2 = packets.iter().position(|&s| s == "[[2]]").unwrap() + 1;
    let i6 = packets.iter().position(|&s| s == "[[6]]").unwrap() + 1;

    i2 * i6
}

fn compare_packets(a: &str, b: &str) -> Ordering {
    compare_lists(
        &mut a.chars().skip(1).peekable(),
        &mut b.chars().skip(1).peekable()
    )
}

fn compare_lists<I1, I2>(
    a: &mut std::iter::Peekable<I1>,
    b: &mut std::iter::Peekable<I2>
) -> Ordering
    where I1: Iterator<Item=char>, I2: Iterator<Item=char>
{
    loop {
        let mut ca = a.next().unwrap();
        let mut cb = b.next().unwrap();
        if ca == ',' { ca = a.next().unwrap() }
        if cb == ',' { cb = b.next().unwrap() }

        if ca == ']' && cb == ']' {
            break Ordering::Equal
        } else if ca == ']' || cb == ']' {
            break if ca == ']' { Ordering::Less } else { Ordering::Greater }
        } else if ca == '[' && cb == '[' {
            let v = compare_lists(a, b);
            if v != Ordering::Equal { break v }
        } else if ca == '[' {
            let s = format!("{}]", number(cb, b));
            let v = compare_lists(a, &mut s.chars().peekable());
            if v != Ordering::Equal { break v }
        } else if cb == '[' {
            let s = format!("{}]", number(ca, a));
            let v = compare_lists(&mut s.chars().peekable(), b);
            if v != Ordering::Equal { break v }
        } else {
            match (number(ca, a), number(cb, b)) {
                (va, vb) if va > vb => break Ordering::Greater,
                (va, vb) if va < vb => break Ordering::Less,
                _ => ()
            }
        }
    }
}

fn number<I>(c1: char, iter: &mut std::iter::Peekable<I>) -> u8 
    where I: Iterator<Item=char>    
{
    let mut v = c1 as u8 - b'0';
    if let Some(&c2) = iter.peek() {
        if c2.is_ascii_digit() {
            iter.next(); 
            v *= 10;
            v += c2 as u8 - b'0'
        }
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one() {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 5555);
    }

    #[test]
    fn input_part_two() {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 22852);
    }

    #[test]
    fn example_part_one() {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 13);
    }

    #[test]
    fn example_part_two() {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 140);
    }
}
