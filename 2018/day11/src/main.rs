use ndarray::Array2;

fn main() {
    use std::time::Instant;

    let t1 = Instant::now();
    let (x, y) = part_one(18);
    let t2 = Instant::now();
    println!("Part 1: {},{}  ({:?})", x, y, t2 - t1);

    let t1 = Instant::now();
    let (x, y, d) = part_two(9424);
    let t2 = Instant::now();
    println!("Part 2: {},{},{}  ({:?})", x, y, d, t2 - t1);
}

fn part_one(serial_no: i32) -> (usize, usize) {
    let mat = calc_sums(serial_no);
    let (_, x, y) = 
        (0..297).map(|x| 
            (0..297).map(|y| {
                (sum_at(&mat, x, y, 3), x, y)
            }).max().unwrap()
        ).max().unwrap();

    (x + 1, y + 1)
}

fn part_two(serial_no: i32) -> (usize, usize, usize) {
    let mat = calc_sums(serial_no);
    let (_, x, y, d) = 
        (1..300).map(|d|
            (0..300 - d).map(|x| 
                (0..300 - d).map(|y| 
                    (sum_at(&mat, x, y, d), x, y, d)
                ).max().unwrap()
            ).max().unwrap()
        ).max().unwrap();

    (x + 1, y + 1, d)
}

fn sum_at(mat: &Array2<i32>, x: usize, y: usize, d: usize) -> i32 {
    let r = x + (d - 1);
    let s = y + (d - 1);
    let rs = *mat.get((r, s)).unwrap();
    let rq = *mat.get((r, y - 1)).unwrap_or(&0);
    let ps = *mat.get((x - 1, s)).unwrap_or(&0);
    let pq = *mat.get((x - 1, y - 1)).unwrap_or(&0);

    rs - rq - ps + pq
}

fn calc_sums(serial_no: i32) -> Array2<i32> {
    let mut above = 0;
    let mut cache: [i32;300] = [0;300];
    let mut grid = Array2::<i32>::zeros((300, 300));
    for ((x, y), v) in grid.indexed_iter_mut() {
        let mut power = calc_power(serial_no, x, y) + cache[y];
        if y > 0 {
            power += above - cache[y - 1];
            cache[y - 1] = above;
        }
        above = power;
        *v = power;
    }

    grid
}

#[inline]
fn calc_power(serial_no: i32, x: usize, y: usize) -> i32 {
    let rid   = x as i32 + 10 + 1;
    let power = (rid * (y as i32 + 1) + serial_no) * rid;
    (if power < 100 { 0 } else { (power / 100) % 10 }) - 5
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let (x, y) = part_one(9424);
    assert_eq!((x, y), (243, 72));

    let (x, y, d) = part_two(9424);
    assert_eq!((x, y, d), (229,192,11));
  }
}
