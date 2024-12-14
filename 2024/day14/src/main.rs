use std::{collections::HashSet, hash::Hash};

#[derive(Eq, Clone, Copy, Debug, Hash, PartialEq)]
struct Robot {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input, 103, 101);
    println!("Part 1: {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two(input, 103, 101);
    println!("Part 2: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str, nrows: i32, ncols: i32) -> usize
{
    let mut robots = load(input);
    (0..100).for_each(|_| 
        robots.iter_mut()
            .for_each(|robot| { *robot = move_robot(*robot, nrows, ncols); })
    );
    
    let quadrants = [
        (0..ncols / 2, 0..nrows / 2),
        (ncols / 2 + 1..ncols, 0..nrows / 2),
        (0..ncols / 2, nrows / 2 + 1..nrows),
        (ncols / 2 + 1..ncols, nrows / 2 + 1..nrows)
    ];
    quadrants.iter()
        .map(|(x, y)| robots.iter()
            .filter(|r| x.contains(&r.x) && y.contains(&r.y))
            .count())
        .product()
}

fn part_two(input: &str, nrows: i32, ncols: i32) -> usize
{
    use std::hash::{DefaultHasher, Hasher};

    // For looking for the top of a tree anywhere in the data.
    // Yes, we don't need the (0, 0) but it looks nicer. :)
    let tree_top = [
                          (0, 0),
                 (-1, 1), (0, 1), (1, 1),
        (-2, 2), (-1, 2), (0, 2), (1, 2), (2, 2)
    ];

    let mut robots = load(input);
    let mut hashes = HashSet::new();

    let mut steps = 1;
    'outer: loop {
        robots.iter_mut()
            .for_each(|robot| { *robot = move_robot(*robot, nrows, ncols); });

        // Hash sets are faster to search than vectors.
        let pos = robots.iter().map(|r| (r.x, r.y)).collect::<HashSet<_>>();
        for p in &pos {
            if tree_top.iter().all(|d| pos.contains(&(p.0 + d.0, p.1 + d.1))) {
                break 'outer
            }
        }

        // Stop when we start to cycle in case we need to start again
        // with a larger tree top.
        let mut hasher = DefaultHasher::new();
        robots.hash(&mut hasher);
        if !hashes.insert(hasher.finish()) {
            break;
        }
        steps += 1
    };

    steps
}

fn move_robot(robot: Robot, nrows: i32, ncols: i32) -> Robot
{
    let col = robot.x + robot.dx;
    let x = if col < 0 { 
        ncols + col
    } else if col >= ncols {
        col - ncols
    } else {
        col
    };

    let row = robot.y + robot.dy;
    let y = if row < 0 {
        nrows + row
    } else if row >= nrows {
        row - nrows
    } else {
        row
    };

    Robot { x, y, ..robot }
}

fn load(input: &str) -> Vec<Robot>
{
    input.lines()
        .map(|line| {
            let (p, v) = line.split_once(' ').unwrap();
            let (sx, sy) = p[2..].split_once(',').unwrap();
            let x = sx.parse::<i32>().unwrap();
            let y = sy.parse::<i32>().unwrap();

            let (sx, sy) = v[2..].split_once(',').unwrap();
            let dx = sx.parse::<i32>().unwrap();
            let dy = sy.parse::<i32>().unwrap();

            Robot { x, y, dx, dy }
        })
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input, 103, 101), 230900224);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input, 103, 101), 6532);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input, 7, 11), 12);
    }
}
