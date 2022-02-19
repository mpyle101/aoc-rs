
fn main() {
    use std::{fs, time::Instant};
    
    let input = fs::read_to_string("./input.txt").unwrap();
    let ports = load(&input);

    let t1 = Instant::now();
    let score = part_one(&ports);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", score, t2 - t1);

    let t1 = Instant::now();
    let score = part_two(&ports);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", score, t2 - t1);
}

fn load(input: &str) -> Vec<[i32;2]> {
    input.lines().map(|s| {
        let mut it = s.split('/');
        let a = it.next().unwrap().parse::<i32>().unwrap();
        let b = it.next().unwrap().parse::<i32>().unwrap();

        [a, b]
    })
    .collect()
}

#[derive(Clone)]
struct State {
    port: i32,
    score: i32,
    length: i32,
    components: Vec<[i32;2]>,
}

fn part_one(ports: &[[i32;2]]) -> i32 {
    use std::collections::VecDeque;
    
    let start = State { port: 0, score: 0, length: 0, components: ports.to_vec() };
    
    let mut strongest = start.clone();
    let mut q = VecDeque::from([start]);
    while let Some(st) = q.pop_front() {
        if st.score > strongest.score {
            strongest = st.clone();
        }

        let v = st.components.iter()
            .enumerate()
            .filter_map(|(i, p)|
                if p[0] == st.port || p[1] == st.port {
                    Some(i)
                } else {
                    None
                }
            )
            .collect::<Vec<_>>();

        v.iter().for_each(|i| {
            let mut components = st.components.clone();
            let ports = components.remove(*i);
            let port  = if st.port == ports[0] { ports[1] } else { ports[0] };
            let score = st.score + ports[0] + ports[1];
            let state = State { port, score, components, length: st.length + 1 };
            q.push_back(state)
        })
    }
    
    strongest.score
}

fn part_two(ports: &[[i32;2]]) -> i32 {
    use std::collections::VecDeque;
    
    let start = State { port: 0, score: 0, length: 0, components: ports.to_vec() };
    
    let mut bridges = vec![];
    let mut q = VecDeque::from([start]);
    while let Some(st) = q.pop_front() {
        let v = st.components.iter()
            .enumerate()
            .filter_map(|(i, p)|
                if p[0] == st.port || p[1] == st.port {
                    Some(i)
                } else {
                    None
                }
            )
            .collect::<Vec<_>>();
        if v.is_empty() {
            bridges.push(st)
        } else {
            v.iter().for_each(|i| {
                let mut components = st.components.clone();
                let ports = components.remove(*i);
                let port  = if st.port == ports[0] { ports[1] } else { ports[0] };
                let score = st.score + ports[0] + ports[1];
                let state = State { port, score, components, length: st.length + 1 };
                q.push_back(state)
            })
        }
    }
    
    bridges.sort_by_key(|st| (st.length, st.score));
    bridges.last().unwrap().score
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let ports = load(&input);
    
        let score = part_one(&ports);
        assert_eq!(score, 1656);
    
        let score = part_two(&ports);
        assert_eq!(score, 1642);
    }
}
