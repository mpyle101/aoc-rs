fn main() {
    use std::fs;
    use std::time::Instant;
    
    let input = fs::read_to_string("./input.txt").unwrap();
    let scanners = load(&input);

    let t1 = Instant::now();
    let beacons = part_one(&scanners);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", beacons, t2 - t1);
}

#[allow(dead_code)]
#[derive(Debug)]
struct Beacon {
    x: i32,
    y: i32,
    z: i32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Scanner {
    facing: i32,
    rotation: i32,
    beacons: Vec<Beacon>,
}

fn load(input: &str) -> Vec<Scanner> {
    input.split("\n\n").map(|s| {
        Scanner {
            facing: 0,
            rotation: 0,
            beacons: 
                s.lines().skip(1).map(|b| {
                    let mut v = b.split(',').map(|n| n.parse().unwrap());
                    Beacon {
                        x: v.next().unwrap(),
                        y: v.next().unwrap(),
                        z: v.next().unwrap(),
                    }
                }).collect(),
        }
    }).collect()
}

fn part_one(_scanners: &[Scanner]) -> i64 {
    use nalgebra::{Reflection, Rotation3, Vector3};

    let vec = Vector3::new(8.0, 0.0, 7.0);

    let angle = Vector3::x() * std::f32::consts::FRAC_PI_2;
    let rot_x = Rotation3::new(angle);

    let pt = rot_x * vec;
    println!("x: 90 => {:?}", pt);
    let pt = rot_x * pt;
    println!("x:180 => {:?}", pt);
    let pt = rot_x * pt;
    println!("x:270 => {:?}", pt);

    let angle = Vector3::y() * std::f32::consts::FRAC_PI_2;
    let rot_y = Rotation3::new(angle);

    let pt = rot_y * vec;
    println!("y: 90 => {:?}", pt);
    let pt = rot_y * pt;
    println!("y:180 => {:?}", pt);
    let pt = rot_y * pt;
    println!("y:270 => {:?}", pt);

    let angle = Vector3::z() * std::f32::consts::FRAC_PI_2;
    let rot_z = Rotation3::new(angle);
    
    let pt = rot_z * vec;
    println!("z: 90 => {:?}", pt);
    let pt = rot_z * pt;
    println!("z:180 => {:?}", pt);
    let pt = rot_z * pt;
    println!("z:270 => {:?}", pt);

    let unit = Vector3::new(1.0, 0.0, 0.0);
    let refl = Reflection::new(unit, 0);

    0
}