use itertools::Itertools;
use lazy_static::lazy_static;
use nalgebra::{Matrix3, Point3};

lazy_static! {
    static ref ROTATIONS: [Matrix3<i32>; 24] = [
        Matrix3::new(1, 0, 0, 0, 1, 0, 0, 0, 1),
        Matrix3::new(1, 0, 0, 0, 0, 1, 0, -1, 0),
        Matrix3::new(1, 0, 0, 0, -1, 0, 0, 0, -1),
        Matrix3::new(1, 0, 0, 0, 0, -1, 0, 1, 0),
        Matrix3::new(0, 1, 0, 0, 0, 1, 1, 0, 0),
        Matrix3::new(0, 1, 0, 1, 0, 0, 0, 0, -1),
        Matrix3::new(0, 1, 0, 0, 0, -1, -1, 0, 0),
        Matrix3::new(0, 1, 0, -1, 0, 0, 0, 0, 1),
        Matrix3::new(0, 0, 1, 1, 0, 0, 0, 1, 0),
        Matrix3::new(0, 0, 1, 0, 1, 0, -1, 0, 0),
        Matrix3::new(0, 0, 1, -1, 0, 0, 0, -1, 0),
        Matrix3::new(0, 0, 1, 0, -1, 0, 1, 0, 0),
        Matrix3::new(-1, 0, 0, 0, -1, 0, 0, 0, 1),
        Matrix3::new(-1, 0, 0, 0, 0, 1, 0, 1, 0),
        Matrix3::new(-1, 0, 0, 0, 1, 0, 0, 0, -1),
        Matrix3::new(-1, 0, 0, 0, 0, -1, 0, -1, 0),
        Matrix3::new(0, -1, 0, 0, 0, -1, 1, 0, 0),
        Matrix3::new(0, -1, 0, 1, 0, 0, 0, 0, 1),
        Matrix3::new(0, -1, 0, 0, 0, 1, -1, 0, 0),
        Matrix3::new(0, -1, 0, -1, 0, 0, 0, 0, -1),
        Matrix3::new(0, 0, -1, -1, 0, 0, 0, 1, 0),
        Matrix3::new(0, 0, -1, 0, 1, 0, 1, 0, 0),
        Matrix3::new(0, 0, -1, 1, 0, 0, 0, -1, 0),
        Matrix3::new(0, 0, -1, 0, -1, 0, -1, 0, 0),
    ];
}

fn main() {
    use std::fs;
    use std::time::Instant;
    
    let input = fs::read_to_string("./input.txt").unwrap();
    let reports = load(&input);

    let t1 = Instant::now();
    let (beacons, scanners) = part_one(&reports);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", beacons, t2 - t1);

    let t1 = Instant::now();
    let manhattan = part_two(&scanners);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", manhattan, t2 - t1);
}

type Beacon = Point3<i32>;

#[derive(Debug)]
#[allow(dead_code)]
struct Report {
    id: i32,
    deltas: Vec<(i32, usize, usize)>,
    beacons: Vec<Beacon>,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Scanner {
    id: i32,
    origin: (i32, i32, i32),
    deltas: Vec<(i32, usize, usize)>,
    beacons: Vec<Beacon>,
}

fn load(input: &str) -> Vec<Report> {
    use nalgebra::point;

    let mut id = -1;
    input.split("\n\n").map(|s| {
        id += 1;
        let beacons = s.lines().skip(1).map(|b| {
                let mut it = b.split(',');
                let x = it.next().unwrap().parse().unwrap();
                let y = it.next().unwrap().parse().unwrap();
                let z = it.next().unwrap().parse().unwrap();
                point![x, y, z]
            }).collect::<Vec<_>>();

        let mut deltas = beacons.iter().enumerate()
            .map(|(i, p1)| beacons[i+1..].iter().enumerate()
                .map(|(j, p2)| (delta(p1, p2), i, j+i+1)).collect::<Vec<_>>()
            )
            .flatten()
            .collect::<Vec<_>>();
        deltas.sort();

        Report { id, deltas, beacons }
    }).collect()
}

fn part_one(reports: &[Report]) -> (i32, Vec<Scanner>) {
    use std::collections::{HashSet, VecDeque};
    use nalgebra::point;

    let mut beacons = HashSet::<Beacon>::from_iter(reports[0].beacons.iter().cloned());

    let s0 = Scanner {
        id: reports[0].id,
        origin: (0, 0, 0),
        deltas: reports[0].deltas.clone(),
        beacons: reports[0].beacons.clone(),
    };

    let mut scanners = vec![s0];
    let mut q = VecDeque::from_iter(reports[1..].iter());
    while let Some(report) = q.pop_front() {
        if let Some((scanner, corr)) = find_matches(&scanners, report) {
            // Find the matching rotation
            for rot in ROTATIONS.iter() {
                let pts: HashSet<(i32, i32, i32)> = HashSet::from_iter(
                    corr.iter().map(|c| {
                        let b1 = &scanner.beacons[c.0.0];
                        let b2 = &report.beacons[c.0.1];
                        let b3 = rot * b2;
                        (b1.x - b3.x, b1.y - b3.y, b1.z - b3.z)
                    })
                );
                if pts.len() == 1 {
                    let (dx, dy, dz) = scanner.origin;
                    let pt = *pts.iter().nth(0).unwrap();
                    let origin = (pt.0 + dx, pt.1 + dy, pt.2 + dz);
                    let v = report.beacons.iter()
                        .map(|b| rot * b)
                        .collect::<Vec<_>>();
                    let s = Scanner {
                        id: report.id,
                        origin: origin,
                        deltas: report.deltas.clone(),
                        beacons: v.clone(),
                    };
                    scanners.push(s);
                    v.iter().for_each(|p| {
                        let pt = point![p.x + origin.0, p.y + origin.1, p.z + origin.2];
                        beacons.insert(pt);
                    });

                    break
                }
            }
        } else {
            q.push_back(report)
        }
    }

    (beacons.len() as i32, scanners)
}

fn part_two(scanners: &[Scanner]) -> i32 {
    scanners.iter().combinations(2)
        .map(|v| {
            let s1 = v[0].origin;
            let s2 = v[1].origin;

            (s1.0 - s2.0).abs() +
            (s1.1 - s2.1).abs() +
            (s1.2 - s2.2).abs()
        })
        .max()
        .unwrap()
}

fn delta(b1: &Beacon, b2: &Beacon) -> i32 {
    (b1.x - b2.x).abs() +
    (b1.y - b2.y).abs() +
    (b1.z - b2.z).abs()
}

fn find_matches<'a>(scanners: &'a [Scanner], report: &Report)
    -> Option<(&'a Scanner, Vec<((usize, usize), i32)>)>
{
    let mut correlations = None;

    let scanner = scanners.iter().find(|s| {
        let matches = get_matches(&s.deltas, &report.deltas);
        let v = correlate(&matches, 5);
        if v.len() > 10 {
            correlations = Some(v);
            true
        } else {
            false
        }
    });

    if let Some(s) = scanner {
        Some((s, correlations.unwrap()))
    } else {
        None
    }
}

fn get_matches(
    d1: &Vec<(i32, usize, usize)>,
    d2: &Vec<(i32, usize, usize)>
) -> Vec<((i32, usize, usize), (i32, usize, usize))>
{
    let mut i = 0;
    let mut matching = d1.iter().fold(vec![], |mut v, n| {
        let mut t = d2[i];
        while t.0 < n.0 && i < d2.len()-1 {
            i += 1;
            t = d2[i];
        }
        if t.0 == n.0 {
            v.push((*n, t))
        }
        v
    });
    matching.sort_by_key(|t| t.0.1);

    matching
}

fn correlate(
    matches: &Vec<((i32, usize, usize), (i32, usize, usize))>,
    threshold: i32
) -> Vec<((usize, usize), i32)>
{
    use std::collections::HashMap;

    let mut map = HashMap::new();
    matches.iter().for_each(|(t1, t2)| {
        *map.entry((t1.1, t2.1)).or_insert(0) += 1;
        *map.entry((t1.1, t2.2)).or_insert(0) += 1;
        *map.entry((t1.2, t2.1)).or_insert(0) += 1;
        *map.entry((t1.2, t2.2)).or_insert(0) += 1;
    });

    let mut counts = map.iter()
        .filter_map(|e| 
            if *e.1 > threshold {
                Some((*e.0, *e.1))
            } else {
                 None
        }).collect::<Vec<_>>();
    counts.sort_by(|a, b| b.1.cmp(&a.1));

    counts
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let reports = load(&input);

        let (beacons, scanners) = part_one(&reports);
        assert_eq!(beacons, 462);

        let manhattan = part_two(&scanners);
        assert_eq!(manhattan, 12158);
    }

    #[test]
    fn sample() {
        let input = fs::read_to_string("./test.txt").unwrap();
        let reports = load(&input);

        let (beacons, _) = part_one(&reports);
        assert_eq!(beacons, 79);
    }
}