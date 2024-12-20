use std::collections::HashSet;

fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input, 100);
    println!("Part 1: {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two(input, 100);
    println!("Part 2: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str, limit: usize) -> usize
{
    use std::collections::HashMap;
    use pathfinding::prelude::dijkstra;

    let (start, goal, ncols, maze) = load(input);
    let (path, _) = dijkstra(
        &start,
        |&p| do_moves(p, ncols, &maze).into_iter().map(|p| (p, 1)),
        |&p| p == goal
    ).unwrap();
    let tiles = path.iter()
        .enumerate()
        .map(|(i, p)| (*p, i))
        .collect::<HashMap<_,_>>();

    // We know there's only one path from the problem statement so all we
    // really need to do is for each step find steps through walls which
    // are farther along the path. The tiles map gives us maze position to
    // index into the path which tells us if a given tile is farther along
    // and, thus, saves steps. Count how many are at or over the limit.
    path.iter()
        .enumerate()
        .fold(0, |acc, (i, p)| {
            acc + [p - 1, p + 1, p - ncols, p + ncols].iter()
                .filter(|q| maze[**q] == '#')
                .filter_map(|q| tiles.get(&((q + q).wrapping_sub(*p))))
                .filter(|&&j| j > i && j - i - 2 >= limit)
                .count()
        })
}

fn part_two(input: &str, limit: usize) -> usize
{
    use std::collections::HashMap;
    use pathfinding::prelude::dfs;

    let (start, goal, ncols, maze) = load(input);
    let path = dfs(
        start,
        |&p| do_moves(p, ncols, &maze),
        |&p| p == goal
    ).unwrap();
    let tiles = path.iter()
        .enumerate()
        .map(|(i, p)| (*p, i))
        .collect::<HashMap<_,_>>();

    path.iter()
        .enumerate()
        .fold(0, |acc,  (i, &p)| {
            acc + do_cheats(p, ncols, &maze).iter()
                .filter_map(|q| tiles.get(q).map(|j| (*q, *j)))
                .filter(|(_, j)| *j > i)
                .map(|(q, j)| j - i - md(p, q, ncols))
                .filter(|n| *n >= limit)
                .count()
        })
}

fn md(p: usize, q: usize, ncols: usize) -> usize
{
    let p_row = p / ncols;
    let p_col = p % ncols;
    let q_row = q / ncols;
    let q_col = q % ncols;

    p_row.abs_diff(q_row) + p_col.abs_diff(q_col)
}

fn do_cheats(p: usize, ncols: usize, maze: &[char]) -> HashSet<usize>
{
    // Find all positions within a manhattan distance of 20 that
    // are also within the walls of the maze and return the ones
    // which are open. The MD from a point in a grid is going to
    // be a star with tips straight up, down, left and right.
    let p = p as i32;
    let ncols = ncols as i32;

    let row = p / ncols;
    let col = p % ncols;
    let nrows = maze.len() as i32 / ncols;

    let mut positions = HashSet::new();
    for r in 0..=20 {
        for c in 0..=20 - r {
            let (rt, rb) = (row - r, row + r);
            let (cl, cr) = (col - c, col + c);

            if rt > 0 {
                if cl > 0 {
                    let q = (rt * ncols + cl) as usize;
                    if maze[q] == '.' { positions.insert(q); }
                }
                if cr < ncols { 
                    let q = (rt * ncols + cr) as usize;
                    if maze[q] == '.' { positions.insert(q); }
                }
            }
            if rb < nrows {
                if cl > 0 {
                    let q = (rb * ncols + cl) as usize;
                    if maze[q] == '.' { positions.insert(q); }
                }
                if cr < ncols {
                    let q = (rb * ncols + cr) as usize;
                    if maze[q] == '.' { positions.insert(q); }
                }
            }
        }
    }

    positions
}

fn do_moves(p: usize, ncols: usize, maze: &[char]) -> Vec<usize>
{
    let mut moves = Vec::with_capacity(4);
    if maze[p - 1] == '.' { moves.push(p - 1) }
    if maze[p + 1] == '.' { moves.push(p + 1) }
    if maze[p - ncols] == '.' { moves.push(p - ncols)}
    if maze[p + ncols] == '.' { moves.push(p + ncols) }

    moves
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
        assert_eq!(part_one(input, 100), 1372);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input, 100), 979014);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input, 2), 44);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input, 50), 285);
    }
}
