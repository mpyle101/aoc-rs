use z3::{ast::Int, Config, Context, Optimize};

fn main() {
    use std::time::Instant;

    let bots = load(include_str!("./input.txt"));

    let t1 = Instant::now();
    let in_range = part_one(&bots);
    let t2 = Instant::now();
    println!("Part 1: {}  ({:?})", in_range, t2 - t1);

    let t1 = Instant::now();
    let pts = part_two(&bots);
    let t2 = Instant::now();
    println!("Part 2: {}  ({:?})", pts, t2 - t1);

    // 47141479
}

#[derive(Clone, Copy, Debug)]
struct Nanobot {
    x: i64,
    y: i64,
    z: i64,
    r: i64,
}

impl Nanobot {
    fn md(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() +
        (self.y - other.y).abs() +
        (self.z - other.z).abs()
    }
}

fn load(input: &str) -> Vec<Nanobot> {
    input.lines().map(|l| {
        let mut it = l.split(">, r=");
        let s = &it.next().unwrap()[5..];
        let r = read(&mut it);

        let mut it = s.split(",");
        let x = read(&mut it);
        let y = read(&mut it);
        let z = read(&mut it);

        Nanobot { x, y, z, r }
    })
    .collect()
}

fn read(it: &mut std::str::Split<&str>) -> i64 {
    it.next().map_or(0, |v| v.parse::<i64>().map_or(0, |n| n))
}

fn part_one(bots: &[Nanobot]) -> usize {
    let mut bots = bots.to_vec();
    bots.sort_by(|a, b| b.r.cmp(&a.r));
    let bot = bots[0];

    bots.iter().filter(|b| bot.md(b) <= bot.r).count()
}

fn part_two(bots: &[Nanobot]) -> i64 {
    use std::ops::{Add, Mul, Sub};

    let ctx = Context::new(&Config::new());
    let x = Int::fresh_const(&ctx, "x");
    let y = Int::fresh_const(&ctx, "y");
    let z = Int::fresh_const(&ctx, "z");

    let mut count = Int::from_i64(&ctx, 0);
    for b in bots {
        let bx = Int::from_i64(&ctx, b.x);
        let by = Int::from_i64(&ctx, b.y);
        let bz = Int::from_i64(&ctx, b.z);
    
        let dx = bx.sub(&x);
        let dx = dx.le(&Int::from_i64(&ctx, 0)).ite(&Int::from_i64(&ctx, -1).mul(&dx), &dx);
        let dy = by.sub(&y);
        let dy = dy.le(&Int::from_i64(&ctx, 0)).ite(&Int::from_i64(&ctx, -1).mul(&dy), &dy);
        let dz = bz.sub(&z);
        let dz = dz.le(&Int::from_i64(&ctx, 0)).ite(&Int::from_i64(&ctx, -1).mul(&dz), &dz);

        let br = Int::from_i64(&ctx, b.r);
        let md = dx.add(&dy).add(&dz);
        let in_range = md.le(&br);
        count = count.add(
            in_range.ite(&Int::from_i64(&ctx, 1), &Int::from_i64(&ctx, 0))
        );
    }

    let optimizer = Optimize::new(&ctx);
    optimizer.maximize(&count);

    let dx = abs(&ctx, &x);
    let dy = abs(&ctx, &y);
    let dz = abs(&ctx, &z);
    let md = dx.add(&dy).add(&dz);
    optimizer.minimize(&md);

    optimizer.check(&[]);
    let model = optimizer.get_model().unwrap();
    let res = model.eval(&md, true).unwrap();

    res.as_i64().unwrap()
}

fn abs<'a>(ctx: &'a Context, n: &'a Int) -> Int<'a> {
    use std::ops::Mul;

    let zero = Int::from_i64(ctx, 0);
    let neg1 = Int::from_i64(ctx, -1);
    n.le(&zero).ite(&neg1.mul(n), n)
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let bots = load(include_str!("./input.txt"));

    let in_range = part_one(&bots);
    assert_eq!(in_range, 481);
  }
}
