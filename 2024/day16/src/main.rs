type Step = ((usize, char), usize);

fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two(input);
    println!("Part 2: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str) -> usize
{
    use pathfinding::prelude::dijkstra;

    let (start, goal, ncols, maze) = load(input);
    let (_, cost) = dijkstra(
        &(start, '>'),
        |&(p, d)| do_moves(p, d, ncols, &maze),
        |&(p, _)| p == goal
    ).unwrap();

    cost
}

fn part_two(input: &str) -> usize
{
    use std::collections::HashSet;
    use pathfinding::prelude::yen;

    let (start, goal, ncols, maze) = load(input);

    // Yen finds the 'k' shortest paths. Manually move
    // the value up from 2 until the unique set of tiles
    // across all the paths stops progressing. For our
    // input data, that was 9. There are mmore paths of
    // the shortest cost through the maze but they are
    // just variations on traversing the tiles covered
    // by the top 9.
    let k = 9;
    let paths = yen(
        &(start, '>'),
        |&(p, d)| do_moves(p, d, ncols, &maze),
        |&(p, _)| p == goal,
        k
    );

    let cost  = paths[0].1;
    let tiles = paths.iter()
        .filter(|(_, c)| *c == cost)
        .flat_map(|(v, _)| v.iter().map(|p| p.0))
        .collect::<HashSet<_>>();
    tiles.len()
}

fn do_moves(p: usize, d: char, ncols: usize, maze: &[char]) -> Vec<Step>
{
    let pos = match d {
        '>' => p + 1,
        '<' => p - 1,
        'v' => p + ncols,
        '^' => p - ncols,
         _  => unreachable!()
    };
    let mut steps = if maze[pos] == '.' { 
        vec![((pos, d), 1)]
    } else {
        vec![]
    };

    match d {
        '^' | 'v' => {
            if maze[p - 1] == '.' { steps.push(((p, '<'), 1000)) }
            if maze[p + 1] == '.' { steps.push(((p, '>'), 1000)) }
        },
        '>' | '<' => {
            if maze[p - ncols] == '.' { steps.push(((p, '^'), 1000)) }
            if maze[p + ncols] == '.' { steps.push(((p, 'v'), 1000)) }
        },
         _  => unreachable!()
    };

    steps
}

fn load(input: &str) -> (usize, usize, usize, Vec<char>)
{
    let mut start = 0;
    let mut goal  = 0;
    let mut ncols = 0;

    let mut maze = input.lines()
        .enumerate()
        .fold(vec![], |mut v, (row, line)| {
            ncols = line.len();
            line.chars()
                .enumerate()
                .for_each(|(col, c)| {
                    if c == 'E' {
                        goal = row * ncols + col
                    } else if c == 'S' {
                        start = row * ncols + col
                    }
                });
            v.extend(line.chars());
            v
        });
    maze[goal]  = '.';
    maze[start] = '.';

    (start, goal, ncols, maze)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 134588);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 631);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example1.txt");
        assert_eq!(part_one(input), 7036);

        let input = include_str!("../example2.txt");
        assert_eq!(part_one(input), 11048);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example1.txt");
        assert_eq!(part_two(input), 45);

        let input = include_str!("../example2.txt");
        assert_eq!(part_two(input), 64);
    }
}
