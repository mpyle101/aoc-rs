fn main() {
    let input = [1,3,5,4,6,8,7,2,9];

    let labels = part_one(&input, 100);
    println!("Part 1: {}", labels);

    let stars = part_two(&input);
    println!("Part 2: {}", stars);
}

fn part_one(input: &[i32;9], moves: u32) -> String {
    let mut cups: Vec<_> = input.iter().cloned().collect();

    (0..moves).fold(0, |cup, _| do_move(cup, &mut cups));

    let pos = cups.iter().position(|&n| n == 1).unwrap();
    let l = cups.len();
    (1..l).map(|i| cups[(pos + i) % l].to_string()).collect()
}

fn part_two(input: &[i32;9]) -> u64 {
    // Add a zero so the position values align with the cup label.
    let mut cups = vec![0];
    cups.append(&mut (1..10)
        .map(|n| input.iter().position(|v| *v == n))
        .map(|p| if let Some(n) = p {
            if n == input.len() - 1 { input.len() as i32 + 1 } else { input[n + 1] }
        } else { 0 })
        .collect::<Vec<_>>());
    cups.append(&mut (11..1_000_001).collect::<Vec<_>>());
    cups.push(input[0]);

    let max = 1_000_000;
    let mut cup = input[0];

    for _ in 0..10_000_000 {
        // Extract the next 3 cups
        let p0 = cup as usize;
        let p1 = cups[p0];
        let p2 = cups[p1 as usize];
        let p3 = cups[p2 as usize];
        let mut next = cups[p3 as usize];

        // Stitch the reference chain back together
        cups[p0] = next;

        // Find the destination cup
        let picks = [p1, p2, p3];
        let mut dst = if cup == 1 { max } else { cup - 1 };
        while picks.contains(&dst) {
            dst = if dst == 1 { max } else { dst - 1 };
        }

        // Select the next cup and insert the removed cups
        cup = next;
        next = cups[dst as usize];
        cups[dst as usize] = p1;
        cups[p3 as usize] = next;
    }

    let c1 = cups[1];
    let c2 = cups[c1 as usize];

    c1 as u64 * c2 as u64
}


fn do_move(pos: usize, cups: &mut Vec<i32>) -> usize {
    use std::cmp::max;

    let len = cups.len();
    let cup = cups[pos];
    let picks = take_three(pos, cups);
    let dest  = find_dest(cup - 1, cups, &picks, len as i32) + 1;
    picks.iter().rev().for_each(|n| cups.insert(dest, *n));

    let n = (len - pos) as i32;
    let p = if dest > pos { pos } else { pos + 3 - max(0, 4 - n) as usize };
    
    (p + 1) % len
}

fn take_three(pos: usize, cups: &mut Vec<i32>) -> Vec<i32> {
    let l = cups.len();

    let mut slots = [(pos + 1) % l, (pos + 2) % l, (pos + 3) % l];
    let picks: Vec<_> = slots.iter().map(|i| cups[*i]).collect();
    slots.sort();
    cups.remove(slots[2]);
    cups.remove(slots[1]);
    cups.remove(slots[0]);
    
    picks
}

fn find_dest(label: i32, cups: &[i32], picks: &[i32], max: i32) -> usize {
    let mut lbl = if label < 1 { max } else { label };
    while picks.contains(&lbl) {
        lbl -= 1;
        lbl = if lbl < 1 { max } else { lbl };
    }

    cups.iter().position(|&n| n == lbl).unwrap()
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = [1,3,5,4,6,8,7,2,9];

    let labels = part_one(&input, 100);
    assert_eq!(labels, "32897654");

    let stars = part_two(&input);
    assert_eq!(stars, 186715244496);
  }

  #[test]
  fn small() {
    let input = [3,8,9,1,2,5,4,6,7];

    let labels = part_one(&input, 10);
    assert_eq!(labels, "92658374");

    let labels = part_one(&input, 100);
    assert_eq!(labels, "67384529");

    let stars = part_two(&input);
    assert_eq!(stars, 149245887792);
  }
}