use std::collections::HashMap;

fn main() {
    let mut wires = load(include_str!("./input.txt"));

    let signal = part_one(&wires);
    println!("Part 1: {}", signal);

    let signal = part_two(&mut wires);
    println!("Part 2: {}", signal);
}

fn part_one(wires: &HashMap<&str, Wire>) -> u32 {
    process(wires)
}

fn part_two(wires: &mut HashMap<&str, Wire>) -> u32 {
    let b = wires.get_mut("b").unwrap();
    *b = Wire::Set(16076);

    process(wires)
}

fn process(wires: &HashMap<&str, Wire>) -> u32 {
    let mut signals = HashMap::new();
    let mut q = Vec::new();
    q.push("a");

    while let Some(w) = q.pop() {
        let wire = wires.get(w).unwrap();
        match wire {
            Wire::Set(v) => { signals.insert(w, *v); },
            Wire::Not(a) => 
                if let Some(&v) = signals.get(a) {
                    signals.insert(w, !v);
                } else {
                    q.push(w);
                    q.push(a);
                },
            Wire::SetW(a) =>                 
                if let Some(&v) = signals.get(a) {
                    signals.insert(w, v);
                } else {
                    q.push(w);
                    q.push(a);
                },
            Wire::And1(a) =>
                if let Some(&v) = signals.get(a) {
                    signals.insert(w, v & 1);
                } else {
                    q.push(w);
                    q.push(a);
                },
            Wire::Or(a, b) => {
                    let w_a = signals.get(a).cloned();
                    let w_b = signals.get(b).cloned();
                    if w_a.is_some() && w_b.is_some() {
                        signals.insert(w, w_a.unwrap() | w_b.unwrap());
                    } else {
                        q.push(w);
                        if w_a.is_none() { q.push(a); }
                        if w_b.is_none() { q.push(b); }
                    }
                },
            Wire::And(a, b) =>{
                    let w_a = signals.get(a).cloned();
                    let w_b = signals.get(b).cloned();
                    if w_a.is_some() && w_b.is_some() {
                        signals.insert(w, w_a.unwrap() & w_b.unwrap());
                    } else {
                        q.push(w);
                        if w_a.is_none() { q.push(a); }
                        if w_b.is_none() { q.push(b); }
                    }
                },
            Wire::LShift(a, c) =>
                if let Some(&v) = signals.get(a) {
                    signals.insert(w, v << c);
                } else {
                    q.push(w);
                    q.push(a);
                },
            Wire::RShift(a, c) => 
                if let Some(&v) = signals.get(a) {
                    signals.insert(w, v >> c);
                } else {
                    q.push(w);
                    q.push(a);
                },
        };
    }

    *signals.get("a").unwrap()
}

#[derive(Debug)]
enum Wire<'a> {
    Set(u32),
    Not(&'a str),
    SetW(&'a str),
    Or(&'a str, &'a str),
    And(&'a str, &'a str),
    And1(&'a str),
    LShift(&'a str, u32),
    RShift(&'a str, u32),
}

fn load(input: &str) -> HashMap<&str, Wire> {
    input.lines()
        .map(|s| s.split(' ').collect::<Vec<_>>())
        .map(|v| match (v[0], v[1]) {
            ("NOT", _)    => (v[3], Wire::Not(v[1])),
            ("1", "AND")  => (v[4], Wire::And1(v[2])),
            (_, "OR")     => (v[4], Wire::Or(v[0], v[2])),
            (_, "AND")    => (v[4], Wire::And(v[0], v[2])),
            (_, "LSHIFT") => (v[4], Wire::LShift(v[0], v[2].parse::<u32>().unwrap())),
            (_, "RSHIFT") => (v[4], Wire::RShift(v[0], v[2].parse::<u32>().unwrap())),
                        _ => (v[2], v[0].parse::<u32>().map_or(
                                Wire::SetW(v[0]), |v| Wire::Set(v))),
        })
        .collect()
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let mut wires = load(include_str!("./input.txt"));

    let signal = part_one(&wires);
    assert_eq!(signal, 16076);

    let signal = part_two(&mut wires);
    assert_eq!(signal, 2797);
  }
}
