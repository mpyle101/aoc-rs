fn main() {
    use std::time::Instant;

    let t1 = Instant::now();
    let scores = part_one(681901);
    let t2 = Instant::now();
    println!("Part 1: {}  ({:?})", scores, t2 - t1);

    let t1 = Instant::now();
    let count = part_two([6,8,1,9,0,1]);
    let t2 = Instant::now();
    println!("Part 2: {}  ({:?})", count, t2 - t1);
}

fn part_one(count: usize) -> String {
    let mut recipes: Vec<u8> = Vec::with_capacity(count + 15);
    recipes.push(3);
    recipes.push(7);

    let mut elf1 = 0;
    let mut elf2 = 1;
    while recipes.len() < count + 10 {
        let score1 = recipes[elf1];
        let score2 = recipes[elf2];
        let n = score1 + score2;
        if n >= 10 { recipes.push(1) }
        recipes.push(n % 10);
        elf1 = (elf1 + 1 + score1 as usize) % recipes.len();
        elf2 = (elf2 + 1 + score2 as usize) % recipes.len();
    }

    recipes[count..count + 10].iter().map(|&n| (n + 48) as char).collect()
}

fn part_two(scores: [u8;6]) -> u32 {
    let mut recipes: Vec<u8> = vec![3,7,1,0,1,0,1];
    let mut len = recipes.len();
    
    let mut elf1 = 6;
    let mut elf2 = 4;
    loop {
        let score1 = recipes[elf1];
        let score2 = recipes[elf2];
        let n = score1 + score2;
        if n >= 10 { 
            len += 1;
            recipes.push(1);
            if recipes[len-6..len] == scores {
                return len as u32 - 6
            }
        }
        len += 1;
        recipes.push(n % 10);
        if recipes[len-6..len] == scores {
            return len as u32 - 6
        }

        elf1 = (elf1 + 1 + score1 as usize) % len;
        elf2 = (elf2 + 1 + score2 as usize) % len;
    }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let scores = part_one(681901);
    assert_eq!(scores, "1617111014");

    let count = part_two([6,8,1,9,0,1]);
    assert_eq!(count, 20321495);
  }
}
