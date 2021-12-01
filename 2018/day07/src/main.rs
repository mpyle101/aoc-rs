use std::collections::BTreeSet;

fn main() {
    use std::time::Instant;

    let steps = load(include_str!("./input.txt"));

    let t1 = Instant::now();
    let dance = part_one(&steps);
    let t2 = Instant::now();
    println!("Part 1: {:?}  ({:?})", dance, t2 - t1);

    let t1 = Instant::now();
    let seconds = part_two(&steps);
    let t2 = Instant::now();
    println!("Part 2: {:?}  ({:?})", seconds, t2 - t1);
}

fn part_one(steps: &[Step]) -> String {
    let mut deps: [u32;26] = [0;26];
    steps.iter().for_each(|step| {
        let row = (step.name - 65) as usize;
        let bit = (step.prev - 65) as usize;
        deps[row] |= 1 << bit;
    });

    let mut dance     = Vec::with_capacity(26);
    let mut staged    = BTreeSet::new();
    let mut available = 0;

    while dance.len() < 26 {
        deps.iter()
            .enumerate()
            .for_each(|(i, &v)| 
                if available & (1 << i) == 0 && v & available == v { 
                    staged.insert(i as u8);
                }
            );
        let step = pop(&mut staged);
        dance.push(step + 65);
        available |= 1 << step;
    }

    unsafe { String::from_utf8_unchecked(dance) }
}

fn part_two(steps: &[Step]) -> u32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut deps: [u32;26] = [0;26];
    steps.iter().for_each(|step| {
        let row = (step.name - 65) as usize;
        let bit = (step.prev - 65) as usize;
        deps[row] |= 1 << bit;
    });

    let mut dance     = Vec::with_capacity(26);
    let mut staged    = BTreeSet::new();
    let mut workers   = BinaryHeap::new();
    let mut seconds   = 0u32;
    let mut available = 0u32;
    let mut scheduled = 0u32;

    while dance.len() < 26 {
        deps.iter()
            .enumerate()
            .for_each(|(i, &v)| 
                if available & (1 << i) == 0 && v & available == v {
                    if workers.len() < 5 && scheduled & (1 << i) == 0 {
                        let ts = seconds + 61 + i as u32;
                        workers.push(Reverse((ts, i)));
                        scheduled |= 1 << i;
                    }
                }
            );
        if let Some(Reverse((ts, v))) = workers.pop() {
            staged.insert(v);
            seconds = ts;
        }
        let step = pop(&mut staged);
        dance.push(step);
        available |= 1 << step;
    }

    seconds
}

#[inline]
fn pop<T>(set: &mut BTreeSet<T>) -> T
    where T: Ord + Copy
{
    let value = *set.iter().next().unwrap();
    set.remove(&value);
    value
}

fn load(input: &str) -> Vec<Step> {
    input.lines()
        .map(|s| s.split(' ').collect::<Vec<_>>())
        .map(|v| Step {
            name: v[7].as_bytes()[0],
            prev: v[1].as_bytes()[0],
        })
        .collect()
}

struct Step {
    name: u8,
    prev: u8,
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let steps = load(include_str!("./input.txt"));

    let dance = part_one(&steps);
    assert_eq!(dance, "PFKQWJSVUXEMNIHGTYDOZACRLB");

    let seconds = part_two(&steps);
    assert_eq!(seconds, 864);
  }
}
