use chrono::prelude::*;

fn main() {
    use std::time::Instant;

    let events = load(include_str!("./input.txt"));

    let t1 = Instant::now();
    let guard = part_one(&events);
    let t2 = Instant::now();
    println!("Part 1: {guard}  ({:?})", t2 - t1);

    let t1 = Instant::now();
    let guard = part_two(&events);
    let t2 = Instant::now();
    println!("Part 2: {guard}  ({:?})", t2 - t1);
}

fn part_one(events: &[Event]) -> u32 {
    use ndarray::{Array2, Axis};
    use std::collections::HashMap;
    use Event::*;

    let mut gids: Vec<_> = events.iter()
        .filter_map(|e| e.is_guard().then(|| e.get_id()))
        .collect();
    gids.sort();
    gids.dedup();

    let guards: HashMap<_,_> = gids.iter()
        .enumerate()
        .map(|(i, e)| (e, i))
        .collect();

    let mut guard = 0;
    let mut sleep = 0;
    let mut heatmap = Array2::<usize>::zeros((guards.len(), 60));
    for evt in events {
        match evt {
            Guard(_, id) => guard = *id,
            Sleep(dt)    => sleep = dt.minute() as usize,
            Wake(dt)     => {
                let wake = dt.minute() as usize;
                let row = guards.get(&guard).unwrap();
                let mut minutes = heatmap.row_mut(*row);
                (sleep..wake).for_each(|m| { minutes[m] += 1; })
            }
        }
    }

    let row = heatmap.sum_axis(Axis(1)).iter()
        .enumerate()
        .max_by_key(|a| a.1)
        .map(|(i, _)| i)
        .unwrap();
    let col = heatmap.row(row).iter()
        .enumerate()
        .max_by_key(|a| a.1)
        .map(|(i, _)| i)
        .unwrap();

    let gid = *guards.iter()
        .find_map(|(k, &v)| (v == row).then(||k))
        .unwrap();

    col as u32 * gid
}

fn part_two(events: &[Event]) -> u32 {
    use ndarray::Array2;
    use std::collections::HashMap;
    use Event::*;

    let mut gids: Vec<_> = events.iter()
        .filter_map(|e| e.is_guard().then(|| e.get_id()))
        .collect();
    gids.sort();
    gids.dedup();

    let guards: HashMap<_,_> = gids.iter()
        .enumerate()
        .map(|(i, e)| (e, i))
        .collect();

    let mut guard = 0;
    let mut sleep = 0;
    let mut heatmap = Array2::<usize>::zeros((guards.len(), 60));
    for evt in events {
        match evt {
            Guard(_, id) => guard = *id,
            Sleep(dt)    => sleep = dt.minute() as usize,
            Wake(dt)     => {
                let wake = dt.minute() as usize;
                let row = guards.get(&guard).unwrap();
                let mut minutes = heatmap.row_mut(*row);
                (sleep..wake).for_each(|m| { minutes[m] += 1; })
            }
        }
    }

    let ((row, col), _) = heatmap.indexed_iter()
        .fold(((0, 0), &0), |acc, g| if *g.1 > *acc.1 { g } else { acc });
    let gid = *guards.iter()
        .find_map(|(k, &v)| (v == row).then(||k))
        .unwrap();

    col as u32 * gid
}

fn load(input: &str) -> Vec<Event> {
    let mut events: Vec<_> = input.lines().map(|s| {
        let dt = Utc.datetime_from_str(&s[1..17], "%Y-%m-%d %H:%M").unwrap();
        let ev: Vec<_> = s[19..].split(' ').collect();
        match ev[0] {
            "wakes" => Event::Wake(dt),
            "falls" => Event::Sleep(dt),
            "Guard" => {
                    let id = ev[1][1..].parse::<u32>().unwrap();
                    Event::Guard(dt, id)
                },
                  _ => unreachable!(),
        }
    })
    .collect();

    events.sort();
    events
}

#[derive(Debug, Eq, Ord, PartialEq)]
enum Event {
    Wake(DateTime<Utc>),
    Sleep(DateTime<Utc>),
    Guard(DateTime<Utc>, u32),
}

impl Event {
    fn is_guard(&self) -> bool {
        use Event::*;
        match self {
            Guard(_,_) => true,
                     _ => false,
        }
    }

    fn get_id(&self) -> u32 {
        use Event::*;
        match self {
            Guard(_, id) => *id,
                       _ => panic!("Not a guard: {:?}", self)
        }
    }
}
impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use Event::*;

        let dt1 = match self {
            Wake(dt) | Sleep(dt) | Guard(dt, _) => dt
        };
        let dt2 = match other {
            Wake(dt) | Sleep(dt) | Guard(dt, _) => dt
        };

        dt1.partial_cmp(dt2)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let events = load(include_str!("./input.txt"));

        let guard = part_one(&events);
        assert_eq!(guard, 104764);

        let guard = part_two(&events);
        assert_eq!(guard, 128617);
    }
}
