
fn main() {
    use std::fs;
    use std::time::Instant;
    
    let input   = fs::read_to_string("./input.txt").unwrap();
    let heights = load(&input);

    let t1 = Instant::now();
    let risk = part_one(&heights);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", risk, t2 - t1);

    let t1 = Instant::now();
    let basins = part_two(&heights);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", basins, t2 - t1);
}

fn load(input: &str) -> Vec<u32> {
    input.lines().flat_map(|l|
        l.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>()
    ).collect()
}

fn part_one(heights: &[u32]) -> u32 {
    heights.iter().enumerate()
        .filter_map(|(i, n)| is_lowest(&heights, i).then(|| n+1))
        .sum()
}

fn part_two(heights: &[u32]) -> u32 {
    let mut basins = (0..heights.len())
        .filter(|i| is_lowest(heights, *i))
        .map(|i| basin_size(heights, i))
        .collect::<Vec<u32>>();
    basins.sort_by(|a, b| b.cmp(a));

    basins.iter().take(3).product()
}

fn is_lowest(heights: &[u32], pos: usize) -> bool {
    let value = heights[pos as usize];
    neighbors(heights, pos).iter().all(|(_, n)| *n > value)
}

fn neighbors(heights: &[u32], pos: usize) -> [(usize, u32);4] {
    const XDIM: usize = 100;

    let above = if pos < XDIM { (usize::MAX, 9) } else { (pos - XDIM, heights[pos - XDIM]) };
    let below = if pos > heights.len() - XDIM - 1 { (usize::MAX, 9) } else { (pos + XDIM, heights[pos + XDIM]) };
    let left  = if pos % XDIM == 0 { (usize::MAX, 9) } else { (pos - 1, heights[pos - 1]) };
    let right = if (pos + 1) % XDIM == 0 { (usize::MAX, 9) } else { (pos + 1, heights[pos + 1]) };

    [above, below, left, right]
}

fn basin_size(heights: &[u32], pos: usize) -> u32 {
    use std::collections::{HashSet, VecDeque};

    let mut q = VecDeque::from([pos]);
    let mut visited = HashSet::new();
    while let Some(p) = q.pop_front() {
        visited.insert(p);
        neighbors(heights, p).iter()
            .filter(|(i, n)| *n < 9 && !visited.contains(i))
            .for_each(|(i, _)| q.push_back(*i));
    }

    visited.len() as u32
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input   = fs::read_to_string("./input.txt").unwrap();
        let heights = load(&input);

        let risk = part_one(&heights);
        assert_eq!(risk, 633);

        let basins = part_two(&heights);
        assert_eq!(basins, 1050192);
    }
}