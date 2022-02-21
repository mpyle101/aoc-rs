use pathfinding::matrix::Matrix;

fn main() {
    use std::fs;
    use std::time::Instant;
    
    let input = fs::read_to_string("./input.txt").unwrap();
    let (algo, image) = load(&input);

    let t1 = Instant::now();
    let pixels = part_one(&algo, &image);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", pixels, t2 - t1);

    let t1 = Instant::now();
    let pixels = part_two(&algo, &image);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", pixels, t2 - t1);
}

fn load(input: &str) -> (Vec<u32>, Matrix<u32>) {
    let mut it = input.split("\n\n");
    let algo  = it.next().unwrap().chars().map(|c| (c == '#') as u32).collect();
    let image = Matrix::from_rows(
        it.next().unwrap().lines().map(|l| l.chars().map(|c| (c == '#') as u32))
    ).unwrap();

    (algo, image)
}

fn part_one(algo: &[u32], image: &Matrix<u32>) -> u32 {
    // If the enhanced value for 0 results in a dark pixel, the
    // infinite plain stays dark, ie 0 (like the test input). However,
    // if the enhanced 0 value is 1 (like the real input), then the
    // infinite plain alternates between 0 and 1.
    let f = if algo[0] == 0 { |_| 0 } else { |i: i32| (i % 2) as u32 };
    (0..2).fold(image.clone(), |m, i| enhance(algo, &m, f(i))).values().sum()
}

fn part_two(algo: &[u32], image: &Matrix<u32>) -> u32 {
    // Set part one.
    let f = if algo[0] == 0 { |_| 0 } else { |i: i32| (i % 2) as u32 };
    (0..50).fold(image.clone(), |m, i| enhance(algo, &m, f(i))).values().sum()
}

fn enhance(algo: &[u32], image: &Matrix<u32>, default: u32) -> Matrix<u32> 
{
    let mut m1 = Matrix::new(image.rows + 2, image.columns + 2, default);
    image.indices().zip(image.values()).for_each(|((r, c), v)| 
        *m1.get_mut((r+1, c+1)).unwrap() = *v
    );
    let mut m2 = Matrix::new(m1.rows, m1.columns, 0);
    m2.indices().zip(m2.values_mut()).for_each(|(rc, v)|
        *v = algo[get_index(rc, &m1, default)]
    );

    m2
}

fn get_index((r, c): (usize, usize), image: &Matrix<u32>, default: u32) -> usize {
    [
        (r.wrapping_sub(1), c.wrapping_sub(1)),
        (r.wrapping_sub(1), c),
        (r.wrapping_sub(1), c + 1),
        (r, c.wrapping_sub(1)), (r, c), (r, c + 1),
        (r + 1, c.wrapping_sub(1)), (r + 1, c), (r + 1, c + 1),
    ].iter()
    .enumerate()
    .fold(0u32, |n, (i, &rc)|
        if let Some(&v) = image.get(rc) { 
            n | v << (8 - i)
        } else { 
            n | default << (8 - i)
        }
    ) as usize
}

#[allow(dead_code)]
fn print(image: &Matrix<u32>) {
    (0..image.rows).for_each(|r| {
        (0..image.columns).for_each(|c| {
            print!("{}", if *image.get((r, c)).unwrap() == 1 { '#' } else { '.' })
        });
        println!();
    });
    println!();
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let (algo, image) = load(&input);

        let pixels = part_one(&algo, &image);
        assert_eq!(pixels, 5347);

        let pixels = part_two(&algo, &image);
        assert_eq!(pixels, 17172);
    }

    #[test]
    fn small() {
        let input = fs::read_to_string("./test.txt").unwrap();
        let (algo, image) = load(&input);

        let pixels = part_one(&algo, &image);
        assert_eq!(pixels, 35);

        let pixels = part_two(&algo, &image);
        assert_eq!(pixels, 3351);
    }
}