use std::collections::HashSet;

fn main() {
    let (seats, rows, cols) = load(include_str!("./seats.txt"));

    let occupied = part_one(&seats);
    println!("Part1: {}", occupied);

    let occupied = part_two(&seats, rows, cols);
    println!("Part2: {}", occupied);
}

type Pos = (i32, i32);

fn load(input: &str) -> (HashSet<Pos>, usize, usize) {
    let rows  = input.lines().count();
    let cols  = input.as_bytes().iter().take_while(|&b| *b != b'\n').count();

    let seats = input.lines()
        .enumerate()
        .flat_map(|(y, s)| s.as_bytes().iter().enumerate()
            .filter_map(|(x, c)| match c {
                b'L' => Some((x as i32, y as i32)),
                _ => None,
            }).collect::<Vec<_>>())
        .collect::<HashSet<_>>();
    
    (seats, rows, cols)
}

fn part_one(seats: &HashSet<Pos>) -> usize {
    let mut occupied = HashSet::new();

    loop {
        let mut taken = Vec::new();
        for seat in seats {
            let adjacent = adjacent_taken(seat, &occupied);
            if occupied.contains(seat) {
                if adjacent < 4 {
                    taken.push(seat);
                }
            } else if adjacent == 0 {
                taken.push(seat);
            }
        }
        if taken.len() == occupied.len() {
            break occupied.len()
        } else {
            occupied = taken.iter().map(|s| (s.0, s.1)).collect::<HashSet<_>>();
        }
    }
}

fn part_two(seats: &HashSet<Pos>, rows: usize, cols: usize) -> usize {
    let mut occupied = HashSet::new();

    loop {
        let mut taken = Vec::new();
        for seat in seats {
            let adjacent = adjacent_visible(seat, seats, &occupied, rows, cols);
            if occupied.contains(seat) {
                if adjacent < 5 {
                    taken.push(seat);
                }
            } else if adjacent == 0 {
                taken.push(seat);
            }
        }
        if taken.len() == occupied.len() {
            break occupied.len()
        } else {
            occupied = taken.iter().map(|s| (s.0, s.1)).collect::<HashSet<_>>();
        }
    }
}


const DELTA: [Pos; 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1)
];
fn adjacent_taken(seat: &Pos, occupied: &HashSet<Pos>) -> usize {
    DELTA.iter()
        .map(|d| (seat.0 + d.0, seat.1 + d.1))
        .filter(|s| occupied.contains(s))
        .count()
}

fn adjacent_visible(
    seat: &Pos,
    seats: &HashSet<Pos>,
    occupied: &HashSet<Pos>,
    rows: usize,
    cols: usize
) -> usize {
    let in_bounds = |t:(i32, i32)| t.0 >= 0 && t.1 >=0 && t.0 < cols as i32 && t.1 < rows as i32;
    
    DELTA.iter()
        .filter_map(|d| {
            let mut check = (seat.0 + d.0, seat.1 + d.1);
            while in_bounds(check) {
                if occupied.contains(&check) {
                    return Some(())
                } else if seats.contains(&check) {
                    return None
                }
                check = (check.0 + d.0, check.1 + d.1);
            };
            None
        })
        .count()
}

#[allow(dead_code)]
fn draw(seats: &HashSet<Pos>, taken: &HashSet<Pos>, rows: usize, cols: usize) {
    (0..cols).for_each(|y| {
        (0..rows).for_each(|x| {
            let seat = (x as i32, y as i32);
            let c = match seat {
                _ if taken.contains(&seat) => '#',
                _ if seats.contains(&seat) => 'L',
                _ => '.'
            };
            print!("{c}");
        });
        println!();
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (seats, rows, cols) = load(include_str!("./seats.txt"));
        
        let taken = part_one(&seats);
        assert_eq!(taken, 2344);
        
        let taken = part_two(&seats, rows, cols);
        assert_eq!(taken, 2076);
    }


    #[test]
    fn small_works() {
        let (seats, rows, cols) = load(include_str!("./test_s.txt"));
        
        let taken = part_one(&seats);
        assert_eq!(taken, 37);
        
        let taken = part_two(&seats, rows, cols);
        assert_eq!(taken, 26);
    }
}