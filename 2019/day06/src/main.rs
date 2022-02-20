// Run through the list of orbits and create a map of each object
// being orbited to a vector objects orbiting it.
// Then start at COM and set all its orbiters to have a segment
// count of 1 and store the name/count association in a queue.
// For each element in the queue, get its oribters from the map,
// create name/count values for them increasing the count by one
// and add them to the back of the queue. Put the processed element
// into the orbits vector.
// When the queue is finally empty, all orbits will have been given
// the appropriate count and put into the orbits vector and you can
// sum the counts to get the checksum for Part 1.
// For part 2 we use the intial map to create a vector of object names
// back to COM for both YOU and SAN. Then reverse the arrays and find
// the index where they don't match. Adding the difference between the
// length of each array and the index gives you the number of segments
// needed to go from YOU to SAN, the answer for Part 2.

use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct Orbit<'a> {
    name: &'a str,
    count: u32,
}

fn main() {
    let mut map = HashMap::new();

    let data = include_str!("./orbits.txt");
    data.lines()
        .map(|o| o.split(')').collect::<Vec<_>>())
        .for_each(|v| insert(&mut map, &v) );

    let mut q = VecDeque::new();
    map.get("COM").unwrap().iter()
        .for_each(|&name| q.push_back(Orbit { name, count: 1 }));

    let mut orbits = Vec::new();
    while let Some(o) = q.pop_front() {
        if let Some(v) = map.get(o.name) { 
            v.iter().for_each(|&name| q.push_back(Orbit { name, count: o.count + 1 }))
        }
        orbits.push(o);
    };

    let checksum: u32 = orbits.into_iter().map(|o| o.count).sum();
    println!("Checksum: {}", checksum);

    let mut xfers = HashMap::new();
    data.lines()
        .map(|o| o.split(')').collect::<Vec<_>>())
        .for_each(|v| { xfers.insert(v[1], v[0]); } );

    let you = to_com("YOU", &xfers);
    let san = to_com("SAN", &xfers);
    let idx = walk_back(&you, &san);
    let min_xfers = (you.len() - idx) + (san.len() - idx);

    println!("Minimum xfers: {}", min_xfers);
}

fn insert<'a>(map: &mut HashMap<&'a str, Vec<&'a str>>, orbit: &[&'a str]) {
    match map.get_mut(orbit[0]) {
        Some(v) => v.push(orbit[1]),
        None => { map.insert(orbit[0], vec![orbit[1]]); }
    };
}

fn to_com<'a>(start: &str, map: &HashMap<&str, &'a str>) -> Vec<&'a str> {
    let mut path = Vec::new();
    let mut name = start;
    while let Some(&o) = map.get(name) {
        path.push(o);
        name = o;
    }

    path.reverse();
    path
}

fn walk_back(v1: &[&str], v2: &[&str]) -> usize {
    for (idx, &v) in v1.iter().enumerate() {
        if v != v2[idx] { return idx }
    }

    0
}