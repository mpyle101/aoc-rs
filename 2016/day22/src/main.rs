
fn main() {
    use std::{fs, time::Instant};

    let input = fs::read_to_string("./input.txt").unwrap();
    let disks = load(&input);

    let t1 = Instant::now();
    let pairs = part_one(&disks);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", pairs, t2 - t1);

    let t1 = Instant::now();
    let steps = part_two(&disks);
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", steps, t2 - t1);
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Disk {
    x: usize,
    y: usize,
    size: i32,
    used: i32,
}

impl Disk {
    fn can_hold(&self, d: &Disk) -> bool {
        (self.size - self.used) >= d.used
    }
}

type Disks = Vec<Disk>;

fn load(input: &str) -> Disks {
    input.lines().skip(2).map(|s| {
        let v = s.split_ascii_whitespace().collect::<Vec<_>>();
        let size = v[1][0..v[1].len()-1].parse::<i32>().unwrap();
        let used = v[2][0..v[2].len()-1].parse::<i32>().unwrap();
        let v = v[0].split('-').collect::<Vec<_>>();
        let x = v[1][1..].parse::<usize>().unwrap();
        let y = v[2][1..].parse::<usize>().unwrap();
        Disk { x, y, size, used }
    })
    .collect()
}

fn part_one(disks: &Disks) -> usize {
    use itertools::Itertools;

    disks.iter().permutations(2)
        .filter(|v| v[0].used != 0 && v[1].can_hold(v[0]))
        .count()
}

fn part_two(disks: &Disks) -> usize {
    (0..29).for_each(|y| {
        (0..35).for_each(|x| {
            let disk = disks[x*29+y];
            if x == 0 && y == 0 {
                print!("(.)")
            } else if x == 34 && y == 0 {
                print!(" G ")
            } else if disk.used == 0 {
                print!(" - ")
            } else if disk.size > 500 {
                print!(" # ")
            } else {
                print!(" . ")
            }
        });
        println!();
    });

    // Manually look at the print out. It takes 26 moves to get the
    // empty disk to the left of G. Then it'll take 34 moves to get
    // G to (0, 0) and it'll take 4 moves to get the free disk in
    // front of G each time and we need to do that one less than G
    // moves so 33 times => 26 + 34 + (33 * 4) = 192
    192
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let disks = load(&input);
        
        let pairs = part_one(&disks);
        assert_eq!(pairs, 1003);
    }
}