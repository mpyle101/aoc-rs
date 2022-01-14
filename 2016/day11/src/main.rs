use lazy_static::lazy_static;

lazy_static! {
    // Cobalt, Polonium, Promethium, Ruthenium, Thulium
    static ref ISOTOPES: [usize; 5] = [1, 3, 5, 7, 9];
}

fn main() {
    use std::time::Instant;

    // Don't feel like parsing the input (really? sentences?).

    // State values represent the floor a given object is on.
    // state[0] is the elevator; after that each pair of
    // values is the microchip and generator location for a
    // given isotope. The Isotope enum specifies the offset
    // in the state for a given isotope's floor values. From
    // this, the goal state consists of an array of all 4's.

    let t1 = Instant::now();
    let steps = part_one();
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", steps, t2 - t1);

    let t1 = Instant::now();
    let steps = part_two();
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", steps, t2 - t1);
}

fn part_one() -> usize {
    use pathfinding::prelude::bfs;

    let state = [1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1];
    let isotopes = [1, 3, 5, 7, 9];

    let goal = [4u8;11];
    let steps = bfs(&state, |st| next_states(st, &isotopes), |&st| st == goal);

    let v = steps.unwrap();

    // The vector contains the initial state.
    v.len() - 1
}

fn part_two() -> usize {
    use pathfinding::prelude::bfs;

    let state = [1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    let isotopes = [1, 3, 5, 7, 9];

    let goal = [4u8;15];
    let steps = bfs(&state, |st| next_states(st, &isotopes), |&st| st == goal);

    let v = steps.unwrap();

    // The vector contains the initial state.
    v.len() - 1
}

fn next_states<const N: usize>(state: &[u8;N], isotopes: &[usize]) -> Vec<[u8;N]> {
    // Get the floors the elevator can move to.
    let elevator = state[0];
    let floors = match elevator {
        1 => vec![2],
        2 => vec![1, 3],
        3 => vec![2, 4],
        4 => vec![3],
        _ => panic!("How did we get here?!?! {}", elevator)
    };

    // Get the objects on the current floor (skip the elevator).
    let objects = state.iter().enumerate().skip(1)
        .filter_map(|(i, &n)| if n == elevator { Some(i) } else { None } )
        .collect::<Vec<_>>();

    // Get all possible states of moving one or two objects
    // to the available floors and filter out the ones with
    // unprotected microchips on the same floor as generators.
    get_all(state, &objects, &floors)
        .iter()
        .filter_map(|st| 
            if invalid(st, isotopes) { None } else { Some(*st) }
        )
        .collect()
}

fn get_all<const N: usize>(state: &[u8;N], objects: &[usize], floors: &[u8]) -> Vec<[u8;N]> {
    use itertools::Itertools;

    let states = floors.iter().map(|&floor| {
        let mut states = objects.iter().map(|&i| {
            let mut st = *state;
            st[0] = floor;
            st[i] = floor;
            st
        })
        .collect::<Vec<_>>();

        objects.iter().combinations(2).for_each(|v| {
            let mut st = *state;
            st[0] = floor;
            st[*v[0]] = floor;
            st[*v[1]] = floor;
            states.push(st)
        });

        states
    })
    .flatten()
    .collect();

    states
}

fn invalid(state: &[u8], isotopes: &[usize]) -> bool {
    // A state is invalid if there are unprotected microchips
    // on the same floor as a generator for another isotope.
    isotopes.iter()
        .filter(|&i| state[*i] != state[i + 1])
        .any(|&i| isotopes.iter().any(|n| state[i] == state[n + 1]))
}

#[allow(dead_code)]
fn print(state: &[u8]) {
    let symbols = [
        "E ",
        "Cm", "Cg",
        "Pm", "Pg",
        "Qm", "Qg",
        "Rm", "Rg",
        "Tm", "Tg",
        "Em", "Eg",
        "Dm", "Dg"
    ];
    
    let mut floor = 4;
    while floor > 0 {
        print!("F{} ", floor);
        state.iter().enumerate()
            .for_each(|(i, n)| if *n == floor {
                    print!("{} ", symbols[i])
                } else {
                    print!(".  ")
                }
            );
        println!();
        floor -= 1;
    }    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let steps = part_one();
        assert_eq!(steps, 47);

        // slow, ~100 seconds in release
        let steps = part_two();
        assert_eq!(steps, 71);
    }
}