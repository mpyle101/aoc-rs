
fn main() {
    use std::time::Instant;

    let t1 = Instant::now();
    let value = part_one(328);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", value, t2 - t1);

    let t1 = Instant::now();
    let value = part_two(328);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", value, t2 - t1);
}

fn part_one(steps: usize) -> i32 {
    use std::collections::VecDeque;
    
    let mut q = VecDeque::from([0]);
    (1..2018).for_each(|n| {
        q.rotate_left(steps % q.len());
        q.push_back(n);
    });

    q[0]
}

fn part_two(steps: usize) -> i32 {
    use std::collections::VecDeque;
    
    let mut q = VecDeque::from([0]);
    (1..50_000_000).for_each(|n| {
        q.rotate_left(steps % q.len());
        q.push_back(n);
    });

    let i = q.iter().position(|v| *v == 0).unwrap();
    q[i + 1]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let value = part_one(328);
        assert_eq!(value, 1670);

        let value = part_two(328);
        assert_eq!(value, 2316253);
    }
}
