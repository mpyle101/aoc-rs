type Brick = [[u32;3];2];

fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str) -> u32
{
    use std::collections::HashMap;

    let mut bricks: Vec<_> = input.lines()
        .map(|line| {
            let (l, r) = line.split_once('~').unwrap();
            let p1 = parse_coords(l);
            let p2 = parse_coords(r);

            [p1, p2]
        })
        .collect();

    // Sort into ascending z order.
    bricks.sort_by(|a, b| a[0][2].cmp(&b[0][2]));

    // Map of brick to minimum group of bricks supporting
    // other bricks.
    let mut supports: HashMap<_,_> = bricks.iter()
        .filter(|brick| brick[0][2] == 1)
        .map(|brick| (*brick, u32::MAX))
        .collect();

    // Settle the bricks from the bottom up and
    // resort into z order.
    for i in 0..bricks.len() {
        let (brick, supported_by) = fall(&bricks[i], &bricks);
        bricks[i] = brick;
        supports.insert(brick, u32::MAX);

        let n = supported_by.len() as u32;
        supported_by.iter()
            .for_each(|brick| {
                supports.entry(*brick).and_modify(|b| *b = (*b).min(n));
            })
    }

    supports.values().filter(|n| **n > 1).count() as u32
}

fn parse_coords(s: &str) -> [u32;3]
{
    let mut p = [0;3];
    s.split(',')
        .enumerate()
        .for_each(|(i, n)| p[i] = n.parse().unwrap());

    p
}

fn fall(brick: &Brick, bricks: &[Brick]) -> (Brick, Vec<Brick>)
{
    if brick[0][2] == 1 {
        (*brick, vec![])
    } else {
        let mut b1 = *brick;
        let (z1, z2) = (b1[0][2], b1[1][2]);
        for i in 1..brick[0][2] {
            b1[0][2] = z1 - i; b1[1][2] = z2 - i;
            let v: Vec<_> = bricks.iter()
                .filter(|b2| *b2 != brick && intersect(&b1, b2))
                .cloned()
                .collect();
            if !v.is_empty() {
                b1[0][2] += 1; b1[1][2] += 1;
                return (b1, v)
            }
        }

        (b1, vec![])
    }
}

fn intersect(a: &Brick, b: &Brick) -> bool
{
    a[0][2] <= b[1][2] &&   // a.minZ <= b.maxZ
    a[1][2] >= b[0][2] &&   // a.maxZ >= b.minZ
    a[0][0] <= b[1][0] &&   // a.minX <= b.maxX
    a[1][0] >= b[0][0] &&   // a.maxX >= b.minX
    a[0][1] <= b[1][1] &&   // a.minY <= b.maxY
    a[1][1] >= b[0][1]      // a.max& >= b.minY
}

#[allow(dead_code)]
fn intersection(a: &Brick, b: &Brick) -> u32
{
    use std::cmp::{min, max};

    max(min(a[1][0], b[1][0]) - max(a[0][0], b[0][0]), 0)
        + max(min(a[1][1], b[1][1]) - max(a[0][1], b[0][1]), 0)
        + max(min(a[1][2], b[1][2]) - max(a[0][2], b[0][2]), 0)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 463);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 5);
    }
}
