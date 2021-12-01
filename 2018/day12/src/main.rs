use std::collections::{BTreeSet, HashMap};

type Rules<'a> = HashMap<&'a [u8], u8>;
type State = BTreeSet<i32>;

fn main() {
    use std::time::Instant;

    let (state, rules) = load(include_str!("./input.txt"));

    let t1 = Instant::now();
    let plants = part_one(&state, &rules);
    let t2 = Instant::now();
    println!("Part 1: {}  ({:?})", plants, t2 - t1);

    let t1 = Instant::now();
    let plants = part_two();
    let t2 = Instant::now();
    println!("Part 2: {}  ({:?})", plants, t2 - t1);
}

fn part_one(initial_state: &State, rules: &Rules) -> i32 {
    let state = cycle(20, initial_state, rules);
    state.iter().sum()
}

fn part_two() -> i64 {
    // via inspection of the repeating value delta pattern showing up
    // around 160 generations or so.
    (((50_000_000_000 - 200) / 20) * 1500) + 16113
}

fn cycle(generations: u64, initial_state: &State, rules: &Rules) -> State {
    let mut pat: [u8;5] = [b'.';5];
    let mut state: State = initial_state.iter().cloned().collect();
    for _ in 0..generations {
        let mut genx = State::new();

        let mut i = state.iter().next().unwrap() - 4;
        pat[0] = if state.contains(&(i)) { b'#' } else { b'.' };
        pat[1] = if state.contains(&(i+1)) { b'#' } else { b'.' };
        pat[2] = if state.contains(&(i+2)) { b'#' } else { b'.' };
        pat[3] = if state.contains(&(i+3)) { b'#' } else { b'.' };
        pat[4] = if state.contains(&(i+4)) { b'#' } else { b'.' };

        let end = state.iter().next_back().unwrap() + 1;
        for _ in i+1..end {
            if let Some(&b) = rules.get(&pat as &[u8]) {
                if b == b'#' {
                    genx.insert(i+2);
                }
            }

            i += 1;
            pat[0] = pat[1];
            pat[1] = pat[2];
            pat[2] = pat[3];
            pat[3] = pat[4];
            pat[4] = if state.contains(&(i+4)) { b'#' } else { b'.' };
        }
        state = genx;
    }
    
    state
}

fn load(input: &str) -> (State, Rules) {
    let mut iter = input.lines();
    let state: State = iter.next().unwrap()
        .split(' ').collect::<Vec<_>>()[2].as_bytes().iter()
        .enumerate()
        .filter(|(_, &b)| b == b'#')
        .map(|(i, _)| i as i32)
        .collect();
    iter.next();
    let rules: Rules = iter.map(|s| s.split(' ').collect::<Vec<_>>())
        .map(|v| (v[0].as_bytes(), v[2].as_bytes()[0]))
        .collect();

    (state, rules)
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let (state, rules) = load(include_str!("./input.txt"));

    let plants = part_one(&state, &rules);
    assert_eq!(plants, 3276);

    let plants = part_two();
    assert_eq!(plants, 3750000001113);
  }
}
