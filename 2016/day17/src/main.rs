use lazy_static::lazy_static;

lazy_static! {
    static ref DELTA: [((i32, i32), char);4] = [
        (( 0, -1), 'U'),
        (( 0,  1), 'D'),
        ((-1,  0), 'L'),
        (( 1,  0), 'R')
    ];
}

fn main() {
    use std::time::Instant;

    let t1 = Instant::now();
    let path = part_one("veumntbg");
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", path, t2 - t1);

    let t1 = Instant::now();
    let steps = part_two("veumntbg");
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", steps, t2 - t1);

    // 536
}

type State = ((i32, i32), String);

fn part_one(passcode: &str) -> String {
    use pathfinding::prelude::bfs;
    
    let goal = (3, 3);
    let steps = bfs(
        &((0, 0), passcode.to_string()),
        |st| doors(st),
        |st| st.0 == goal
    );

    let s = steps.unwrap().last().unwrap().1.clone();
    s[passcode.len()..].to_string()
}

fn part_two(passcode: &str) -> usize {
    use pathfinding::prelude::yen;
    
    let goal = (3, 3);
    let steps: Vec<(Vec<State>, usize)> = yen(
        &((0, 0), passcode.to_string()),
        |st| doors(st).iter().map(|st| (st.clone(), 1)).collect::<Vec<_>>(),
        |st| st.0 == goal,
        1500    // manually increase until longest path doesn't change
    );

    let mut paths = steps.iter().map(|v| v.0.last().unwrap().1.clone()).collect::<Vec<_>>();
    paths.sort_by(|a, b| a.len().cmp(&b.len()));
    let longest = paths.last().unwrap().len() - passcode.len();

    longest
}

fn doors(((x, y), passcode): &State) -> Vec<State> {
    let hash  = format!("{:x}", md5::compute(passcode));
    let doors = hash[0..4].chars().map(|c| c as u8).collect::<Vec<_>>();

    DELTA.iter().enumerate()
        .filter_map(|(i, ((dx, dy), c))| {
            let d = doors[i] - 'a' as u8;
            if d > 0 && d < 'g' as u8 {
                let pt = (x + dx, y + dy);
                if pt.0 >= 0 && pt.0 < 4 && pt.1 >= 0 && pt.1 < 4 {
                    Some((pt, format!("{}{}", passcode, c)))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let path = part_one("veumntbg");
        assert_eq!(path, "DDRRULRDRD");
    }
}