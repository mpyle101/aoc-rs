fn main() {
    let numbers = [1, 0, 15, 2, 10, 13];

    let n = part_one(&numbers);
    println!("Part 1: {}", n);

    let n = part_two(&numbers);
    println!("Part 2: {}", n);
}

fn part_one(nums: &[usize]) -> usize {
    memory_game(nums, 2020)
}

fn part_two(nums: &[usize]) -> usize {
    memory_game(nums, 30000000)
}

fn memory_game(nums: &[usize], iteration: usize) -> usize {
    use std::collections::HashMap;

    let mut spoken: HashMap<_,_> = nums.iter().enumerate()
        .map(|(i, n)| (*n, (i+1, i+1)))
        .collect();

    let mut last  = nums[nums.len() - 1];
    let mut count = spoken.len() + 1;
    while count <= iteration {
        let t = spoken.entry(last).or_insert((count, count));
        last  = t.1 - t.0;
        let n = spoken.entry(last).or_insert((count, count));
        *n = (n.1, count);
        count += 1;
    };

    last
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let numbers = [1, 0, 15, 2, 10, 13];
    
    let n = part_one(&numbers);
    assert_eq!(n, 211);
    
    let n = part_two(&numbers);
    assert_eq!(n, 2159626);
  }

  #[test]
  fn examples_2020() {
    let numbers = [0,3,6];
    let n = part_one(&numbers);
    assert_eq!(n, 436);

    let numbers = [1,3,2];
    let n = part_one(&numbers);
    assert_eq!(n, 1);

    let numbers = [2,1,3];
    let n = part_one(&numbers);
    assert_eq!(n, 10);

    let numbers = [1,2,3];
    let n = part_one(&numbers);
    assert_eq!(n, 27);

    let numbers = [2,3,1];
    let n = part_one(&numbers);
    assert_eq!(n, 78);

    let numbers = [3,2,1];
    let n = part_one(&numbers);
    assert_eq!(n, 438);

    let numbers = [3,1,2];
    let n = part_one(&numbers);
    assert_eq!(n, 1836);
  }
}
