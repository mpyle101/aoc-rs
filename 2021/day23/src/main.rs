
fn main() {
    use std::time::Instant;

    let t1 = Instant::now();
    let energy = part_one();
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", energy, t2 - t1);

    let t1 = Instant::now();
    let energy = part_two();
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", energy, t2 - t1);
}

type Shrimp = ((i32, i32), char);
type State  = Vec<Shrimp>;

fn part_one() -> i32 {
    use pathfinding::prelude::dijkstra;
    
    let amphipods = vec![
        ((2, 3), 'C'), ((2, 5), 'A'), ((2, 7), 'D'), ((2, 9), 'D'),
        ((3, 3), 'B'), ((3, 5), 'A'), ((3, 7), 'B'), ((3, 9), 'C'),
    ];

    let goal = vec![
        ((2, 3), 'A'), ((2, 5), 'B'), ((2, 7), 'C'), ((2, 9), 'D'),
        ((3, 3), 'A'), ((3, 5), 'B'), ((3, 7), 'C'), ((3, 9), 'D'),
    ];

    dijkstra(&amphipods,
        |p| p.iter().filter_map(|s| states(s, p, &goal, 3)).flatten().collect::<Vec<_>>(), 
        |p| *p == goal
    ).unwrap().1
}

fn part_two() -> i32 {
    use pathfinding::prelude::dijkstra;
    
    let amphipods = vec![
        ((2, 3), 'C'), ((2, 5), 'A'), ((2, 7), 'D'), ((2, 9), 'D'),
        ((3, 3), 'D'), ((3, 5), 'C'), ((3, 7), 'B'), ((3, 9), 'A'),
        ((4, 3), 'D'), ((4, 5), 'B'), ((4, 7), 'A'), ((4, 9), 'C'),
        ((5, 3), 'B'), ((5, 5), 'A'), ((5, 7), 'B'), ((5, 9), 'C'),
    ];

    let goal = vec![
        ((2, 3), 'A'), ((2, 5), 'B'), ((2, 7), 'C'), ((2, 9), 'D'),
        ((3, 3), 'A'), ((3, 5), 'B'), ((3, 7), 'C'), ((3, 9), 'D'),
        ((4, 3), 'A'), ((4, 5), 'B'), ((4, 7), 'C'), ((4, 9), 'D'),
        ((5, 3), 'A'), ((5, 5), 'B'), ((5, 7), 'C'), ((5, 9), 'D'),
    ];

    dijkstra(&amphipods,
        |p| p.iter().filter_map(|s| states(s, p, &goal, 5)).flatten().collect::<Vec<_>>(), 
        |p| *p == goal
    ).unwrap().1
}

fn get(state: &State, r: i32, c: i32) -> Option<char> {
    state.iter().find(|s| s.0.0 == r && s.0.1 == c).map(|s| s.1)
}

fn row(s: &Shrimp) -> i32 { s.0.0 }
fn col(s: &Shrimp) -> i32 { s.0.1 }

fn states(s: &Shrimp, state: &State, goal: &State, bottom: i32) -> Option<Vec<(State, i32)>> {
    let rooms = [3, 5, 7, 9];

    if row(s) > 1 {
        // In a room, get the genus for that room.
        let c = get(goal, row(s), col(s)).unwrap();

        // In the right room and the same genus in the slots
        // below (if any). Don't move.
        if s.1 == c {
            let mut okay = true;
            for r in row(s)+1..=bottom {
                let a = get(state, r, col(s));
                okay &= a.unwrap() == c;
            }

            if okay { return None }
        }

        // In a lower slot of the wrong room and another amphipod is
        // in the slot above. Can't move.
        if row(s) > 2 && get(state, row(s) - 1, col(s)).is_some() { return None }
    }

    // Possible hallway (baring being blocked by other amphipods).
    let mut possible = vec![(1, 1), (1, 2), (1, 4), (1, 6), (1, 8), (1, 10), (1, 11)];

    let i = s.1 as usize - 'A' as usize;
    let col = rooms[i];

    // Can only move into the target room as long as that room doesn't
    // contain an amphipod of the wrong genus.
    if let Some(r) = check_room(state, col, s.1, bottom) {
        // If we're in the Hallway, we can only move to the target room.
        if row(s) == 1 {
            possible = vec![(r, col)]
        } else {
            possible.push((r, col))
        }
    } else if row(s) == 1 {
        return None
    }

    // State without the current shrimp.
    let s2 = state.iter().filter(|t| t.0 != s.0).cloned().collect::<Vec<_>>();
    Some(possible.iter()
        .filter_map(|&rc| cost(s, rc, state).map(|n| (rc, n)))
        .map(|(rc, n)| {
            let mut v = s2.clone();
            v.push((rc, s.1));
            v.sort_unstable();
            (v, n)
        })
        .collect())
}

fn check_room(state: &State, c: i32, genus: char, bottom: i32) -> Option<i32> {
    let mut open = 0;

    for r in 2..=bottom {
        if let Some(a) = get(state, r, c) {
            if a != genus { return None }
        } else {
            open = r
        }
    }

    Some(open)
}

fn cost(
    s: &Shrimp,
    (r1, c1): (i32, i32),
    state: &State
) -> Option<i32> {
    let cost  = [1, 10, 100, 1000];

    let mut c = col(s);
    let dir = (c1 - c).signum();
    while c != c1 {
        c += dir;
        if get(state, 1, c).is_some() {
            return None
        }
    }

    let i = s.1 as usize - 'A' as usize;
    Some(
        cost[i] * (c1 - col(s)).abs()   // Hallway moves
        + (r1 - 1) * cost[i]            // Entering room
        + (row(s) - 1) * cost[i]        // Exitting room
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let energy = part_one();
        assert_eq!(energy, 10526);

        let energy = part_two();
        assert_eq!(energy, 41284);
    }
}