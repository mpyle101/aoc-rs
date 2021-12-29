
fn main() {
    use std::time::Instant;
    
    let amphipods = vec![
        ((2, 3), 'C'), ((2, 5), 'A'), ((2, 7), 'D'), ((2, 9), 'D'),
        ((3, 3), 'B'), ((3, 5), 'A'), ((3, 7), 'B'), ((3, 9), 'C'),
    ];

    let t1 = Instant::now();
    let energy = part_one(&amphipods);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", energy, t2 - t1);
}

type Shrimp = ((i32, i32), char);
type State  = Vec<Shrimp>;

fn part_one(amphipods: &State) -> i32 {
    use pathfinding::prelude::dijkstra;

    let goal = vec![
        ((2, 3), 'A'), ((2, 5), 'B'), ((2, 7), 'C'), ((2, 9), 'D'),
        ((3, 3), 'A'), ((3, 5), 'B'), ((3, 7), 'C'), ((3, 9), 'D'),
    ];

    dijkstra(amphipods,
        |p| p.iter().map(|s| states(s, p, &goal)).flatten().collect::<Vec<_>>(), 
        |p| *p == goal
    ).unwrap().1
}

fn get(state: &State, r: i32, c: i32) -> Option<char> {
    state.iter().find(|s| s.0.0 == r && s.0.1 == c).map(|s| s.1)
}

fn row(s: &Shrimp) -> i32 { s.0.0 }
fn col(s: &Shrimp) -> i32 { s.0.1 }

fn states(s: &Shrimp, state: &State, goal: &State) -> Vec<(State, i32)> {
    let rooms = [3, 5, 7, 9];

    if row(s) > 1 {
        // In a room, get the goal genus for that room.
        let c = get(&goal, row(s), col(s)).unwrap();

        // In the bottom slot of the correct room. Don't move.
        if s.1 == c && row(s) == 3 { return vec![] }

        // In the top slot of the correct room and the same genus
        // in the bottom slot. Don't move.
        let a = get(state, 3, col(s));
        if s.1 == c && row(s) == 2 && a.is_some() && a.unwrap() == c { return vec![] }

        // In the bottom slot of the wrong room and another amphipod
        // in the top slot. Can't move.
        if row(s) == 3 && get(state, 2, col(s)).is_some() { return vec![] }
    }

    // Possible hallway (baring being blocked by other amphipods).
    let mut possible = vec![(1, 1), (1, 2), (1, 4), (1, 6), (1, 8), (1, 10), (1, 11)];

    let i = s.1 as usize - 'A' as usize;
    let col = rooms[i];

    // Can only move into the correct room as long as that room doesn't
    // contain an amphipod of the wrong genus.
    let c2 = get(state, 2, col);
    let c3 = get(state, 3, col);
    if (c3.is_none() || c3.unwrap() == s.1) && c2.is_none() {
        // If we're in the Hallway, we can only move to the target room.
        if row(s) == 1 {
            possible = vec![]
        }
        
        // Prefer the bottom slot.
        if c3.is_none() { 
            possible.push((3, col))
        } else {
            possible.push((2, col))
        }
    }

    // State without the current shrimp.
    let s2 = state.iter().filter(|t| t.0 != s.0).cloned().collect::<Vec<_>>();
    possible.iter()
        .filter_map(|&rc| cost(s, rc, state).map(|n| (rc, n)))
        .map(|(rc, n)| {
            let mut v = s2.clone();
            v.push((rc, s.1));
            v.sort();
            (v, n)
        })
        .collect()
}

fn cost(
    s: &Shrimp,
    (r1, c1): (i32, i32),
    state: &State
) -> Option<i32> {
    let cost  = [1, 10, 100, 1000];

    // In the bottom slot and blocked by an amphipod in the
    // top slot.
    if row(s) == 3 && get(state, 2, col(s)).is_some() {
        return None
    }

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
        let amphipods = vec![
            ((2, 3), 'C'), ((2, 5), 'A'), ((2, 7), 'D'), ((2, 9), 'D'),
            ((3, 3), 'B'), ((3, 5), 'A'), ((3, 7), 'B'), ((3, 9), 'C'),
        ];
    
        let energy = part_one(&amphipods);
        assert_eq!(energy, 10526);
    }
}