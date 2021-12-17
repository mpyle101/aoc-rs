fn main() {
    use std::fs;
    use std::time::Instant;
    
    let input = fs::read_to_string("./input.txt").unwrap();
    let target = load(&input);

    let t1 = Instant::now();
    let max_y = part_one(&target);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", max_y, t2 - t1);

    let t1 = Instant::now();
    let hits = part_two(&target);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", hits, t2 - t1);
}

#[derive(Debug)]
struct Target {
    tl: (i32, i32),
    br: (i32, i32),
}

#[derive(Debug)]
struct Probe {
    pos: (i32, i32),
    vel: (i32, i32),
    init: (i32, i32),
}

impl Probe {
    fn new(vel: (i32, i32)) -> Probe {
        Probe { vel, init: vel, pos: (0, 0) }
    }

    fn x(&self)  -> i32 { self.pos.0 }
    fn y(&self)  -> i32 { self.pos.1 }
    fn dx(&self) -> i32 { self.vel.0 }
    fn dy(&self) -> i32 { self.vel.1 }

    fn step(&mut self) {
        let pos_x = self.x();
        let pos_y = self.y();
        let vel_x = self.dx();
        let vel_y = self.dy();

        let dx = -(vel_x.cmp(&0) as i32);
    
        self.pos = (pos_x + vel_x, pos_y + vel_y);
        self.vel = (vel_x + dx, vel_y - 1);
    }

    fn hit(&self, target: &Target) -> bool {
        self.pos.0 >= target.tl.0 &&
        self.pos.0 <= target.br.0 &&
        self.pos.1 <= target.tl.1 &&
        self.pos.1 >= target.br.1
    }

    fn missed(&self, target: &Target) -> bool {
        (self.x() > target.br.0) ||
        (self.y() < target.br.1) ||
        self.stalled(target)
    }

    fn stalled(&self, target: &Target) -> bool {
        self.dx() == 0 && (self.x() < target.tl.0 || self.x() > target.br.0)
    }
}

fn load(input: &str) -> Target {
    let mut it = input.split(' ').skip(2);
    let (x1, x2) = it.next().map(|s| {
        let v = s.split("..").collect::<Vec<_>>();
        (
            v[0][2..].parse::<i32>().unwrap(),
            v[1][..v[1].len()-1].parse::<i32>().unwrap()
        )
    }).unwrap();
    let (y1, y2) = it.next().map(|s| {
        let v = s.split("..").collect::<Vec<_>>();
        (
            v[0][2..].parse::<i32>().unwrap(),
            v[1].parse::<i32>().unwrap()
        )
    }).unwrap();

    Target { 
        tl: (x1.min(x2), y1.max(y2)),
        br: (x1.max(x2), y1.min(y2))
    }
}

fn part_one(target: &Target) -> i32 {
    let mut vel_y = 0;
    let mut max_y = i32::MIN;
    while vel_y < 100 {
        vel_y += 1;
        if let Some(y) = (1..=target.br.0)
            .filter_map(|vel_x| fire((vel_x, vel_y), target).map(|(_, y)| y))
            .max() 
        {
            max_y = max_y.max(y)
        }
    }

    max_y
}

fn part_two(target: &Target) -> u32 {
    use std::collections::HashSet;

    let mut hits = HashSet::new();
    let mut vel_y = -100;
    while vel_y < 100 {
        vel_y += 1;
        (1..=target.br.0)
            .filter_map(|vel_x| fire((vel_x, vel_y), target))
            .for_each(|(probe, _)| { hits.insert(probe.init); });
    }

    hits.len() as u32
}

fn fire(vel: (i32, i32), target: &Target) -> Option<(Probe, i32)> {
    let mut probe = Probe::new(vel);

    let mut max_y = 0;
    let res = loop {
        probe.step();
        max_y = max_y.max(probe.y());

        if probe.hit(target) {
            break Some((probe, max_y))
        } else if probe.missed(target) {
            break None
        }
    };

    res
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let target = load(&input);

        let max_y = part_one(&target);
        assert_eq!(max_y, 3003);

        let hits = part_two(&target);
        assert_eq!(hits, 940);
    }
}