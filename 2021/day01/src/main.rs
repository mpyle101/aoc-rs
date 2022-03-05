use std::fs;
use std::error::Error;
use std::num::ParseIntError;

fn main() -> Result<(), Box<dyn Error>> {
    let input = load(&fs::read_to_string("./input.txt")?)?;

    timeit("Part 1", || part_one(&input));
    timeit("Part 2", || part_two(&input));

    Ok(())
}

fn timeit<T>(s: &str, func: impl Fn() -> T)
    where T: std::fmt::Debug
{
    let t = std::time::Instant::now();
    let result = func();
    println!("{s}: {:?} ({:?})", result, t.elapsed());
}

fn load(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(|v| v.parse::<i32>()).collect()
}

fn part_one(depths: &[i32]) -> i32 {
    let mut iter = depths.iter();
    let first = iter.next().unwrap();
    let (_, count) = iter
        .fold((first, 0), |(last, count), v|
            (v, count + ((v > last) as i32))
        );
    
    count
}

fn part_two(depths: &[i32]) -> i32 {
    let mut iter = depths.windows(3);
    let w = iter.next().unwrap();
    let first = w.iter().sum::<i32>();
    let (_, count) = iter
        .fold((first, 0), |(last, count), v| {
            let val = v.iter().sum();
            (val, count + ((val > last) as i32))
        });
    
    count
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = load(include_str!("../input.txt"))
            .expect("Failed to load input data");

        let count = part_one(&input);
        assert_eq!(count, 1676);

        let count = part_two(&input);
        assert_eq!(count, 1706);
    }
}