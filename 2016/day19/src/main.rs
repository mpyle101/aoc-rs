
fn main() {
    use std::time::Instant;

    let t1 = Instant::now();
    let elf = part_one(3012210);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", elf, t2 - t1);

    let t1 = Instant::now();
    let elf = part_two(3012210);
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", elf, t2 - t1);
}

fn part_one(mut elves: i32) -> i32 {
    let mut elf = 1;

    elves /= 2;
    let mut offset = 2;
    while elves > 1 {
        offset *= 2;
        if elves % 2 == 1 {
            elf += offset;
        }
        elves /= 2;
    }

    elf
}

fn part_two(elves: usize) -> usize {
    // Needed to look for the pattern for the first 100 inputs or so.
    let mut elf = 1;

    while elf * 3 < elves {
        elf *= 3
    }
    
    elves - elf
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let elf = part_one(3012210);
        assert_eq!(elf, 1830117);

        let elf = part_two(3012210);
        assert_eq!(elf, 1417887);
    }
}
