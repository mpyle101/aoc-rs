use std::hash::{Hash, Hasher};

fn main() {
    use std::{fs, time::Instant};
    
    let input = fs::read_to_string("./input.txt").unwrap();
    let particles = load(&input);

    let t1 = Instant::now();
    let particle = part_one(&particles);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", particle, t2 - t1);

    let t1 = Instant::now();
    let left = part_two(&particles);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", left, t2 - t1);
}

#[derive(Clone, Copy, Debug)]
struct Particle {
    pos: [i64;3],
    vel: [i64;3],
    acc: [i64;3],
}

impl Particle {
    fn new() -> Particle {
        Particle { pos: [0;3], vel: [0;3], acc: [0;3] }
    }

    fn update(&mut self) {
        (0..3).for_each(|i| self.vel[i] += self.acc[i]);
        (0..3).for_each(|i| self.pos[i] += self.vel[i]);
    }
}

impl Eq for Particle {}

impl PartialEq for Particle {
    fn eq(&self, other: &Particle) -> bool {
        self.pos == other.pos
    }
}

impl Hash for Particle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i64(self.pos[0]);
        state.write_i64(self.pos[1]);
        state.write_i64(self.pos[2]);
        state.finish();
    }
}

fn load(input: &str) -> Vec<Particle> {
    input.lines().map(|l| {
        let mut p = Particle::new();
        let v = l.split(", ").map(|s| &s[3..s.len()-1]).collect::<Vec<_>>();
        v[0].split(',').enumerate()
            .for_each(|(i, s)| p.pos[i] = s.parse::<i64>().unwrap());
        v[1].split(',').enumerate()
            .for_each(|(i, s)| p.vel[i] = s.parse::<i64>().unwrap());
        v[2].split(',').enumerate()
            .for_each(|(i, s)| p.acc[i] = s.parse::<i64>().unwrap());
        p
    })
    .collect()
}

fn part_one(particles: &[Particle]) -> usize {
    // Calculate the overall accleration for each particle.
    let acc = particles.iter()
        .map(|p| p.acc.iter().map(|n| n.abs()).sum::<i64>())
        .collect::<Vec<_>>();

    // Get the lowest acceleration value and all the particles with it.
    let n = *acc.iter().min().unwrap();
    let v = acc.iter()
        .enumerate()
        .filter_map(|(i, m)| if *m == n { Some(i) } else { None })
        .collect::<Vec<_>>();

    // The closest one to begin with will always be the closest.
    *v.iter()
        .min_by_key(|&i| particles[*i].pos.iter().map(|n| n.abs()).sum::<i64>())
        .unwrap()
}

fn part_two(particles: &[Particle]) -> usize {
    use std::collections::HashSet;

    let mut arr = particles.iter().cloned().collect::<Vec<_>>();

    (0..50).for_each(|_| {
        arr.iter_mut().for_each(|p| p.update());

        // If we can't insert it into the hash set then it's a dupe.
        let mut h = HashSet::new();
        let dupes = arr.iter().filter(|p| !h.insert(*p)).cloned().collect::<Vec<_>>();
        dupes.iter().for_each(|p| {
            // Find the indices for each dupe and remove them from
            // back to front.
            let ix = arr.iter().enumerate()
                .filter(|(_, q)| p == *q).map(|(i, _)| i)
                .collect::<Vec<_>>();
            ix.iter().rev().for_each(|i| { arr.remove(*i); });
        })
    });

    arr.len()
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let particles = load(&input);
    
        let particle = part_one(&particles);
        assert_eq!(particle, 243);
    
        let left = part_two(&particles);
        assert_eq!(left, 648);
    }
}
