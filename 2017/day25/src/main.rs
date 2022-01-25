
fn main() {
    use std::time::Instant;

    let t1 = Instant::now();
    let checksum = part_one(12425180);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", checksum, t2 - t1);
}

fn part_one(steps: i32) -> usize {
    use std::collections::HashMap;

    let mut tape   = HashMap::new();
    let mut state  = 'A';
    let mut cursor = 0;

    for _ in 0..steps {
        let v = tape.entry(cursor).or_insert(0);

        let (dx, st) = match state {
            'A' => if *v == 0 { *v = 1; ( 1, 'B') } else { *v = 0; ( 1, 'F') },
            'B' => if *v == 0 { *v = 0; (-1, 'B') } else { *v = 1; (-1, 'C') },
            'C' => if *v == 0 { *v = 1; (-1, 'D') } else { *v = 0; ( 1, 'C') },
            'D' => if *v == 0 { *v = 1; (-1, 'E') } else { *v = 1; ( 1, 'A') },
            'E' => if *v == 0 { *v = 1; (-1, 'F') } else { *v = 0; (-1, 'D') },
            'F' => if *v == 0 { *v = 1; ( 1, 'A') } else { *v = 0; (-1, 'E') },
            _ => panic!("Unknown state: {}", state)
        };

        state = st;
        cursor += dx;
    }

    tape.values().filter(|&v| *v == 1).count()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let checksum = part_one(12425180);
        assert_eq!(checksum, 3099);
    }
}
