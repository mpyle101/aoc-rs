use z3::{ast::Int, Config, Context, Optimize};

fn main() {
    use std::time::Instant;

    let bots = load(include_str!("./input.txt"));

    let t1 = Instant::now();
    let in_range = part_one(&bots);
    let t2 = Instant::now();
    println!("Part 1: {in_range}  ({:?})", t2 - t1);

    let t1 = Instant::now();
    let pts = part_two(&bots);
    let t2 = Instant::now();
    println!("Part 2: {pts}  ({:?})", t2 - t1);
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

        let mut it = s.split(',');
        let x = read(&mut it);
        let y = read(&mut it);
        let z = read(&mut it);

        Nanobot { x, y, z, r }
    })
    .collect()
}

fn read<'a>(mut it: impl Iterator<Item=&'a str>) -> i64 {
    it.next().map_or(0, |v| v.parse::<i64>().map_or(0, |n| n))
}

fn part_one(bots: &[Nanobot]) -> usize {
    let mut bots = bots.to_vec();
    bots.sort_by(|a, b| b.r.cmp(&a.r));
    let bot = bots[0];

    bots.iter().filter(|b| bot.md(b) <= bot.r).count()
}

fn part_two(bots: &[Nanobot]) -> i64 {
    let ctx = Context::new(&Config::new());
    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");
    let z = Int::new_const(&ctx, "z");

    let one  = Int::from_i64(&ctx, 1);
    let zero = Int::from_i64(&ctx, 0);

    let mut count = Int::from_i64(&ctx, 0);
    for b in bots {
        let bx = Int::from_i64(&ctx, b.x);
        let by = Int::from_i64(&ctx, b.y);
        let bz = Int::from_i64(&ctx, b.z);
        let br = Int::from_i64(&ctx, b.r);
    
        let dx = bx - &x;
        let dx = dx.le(&zero).ite(&dx.unary_minus(), &dx);
        let dy = by - &y;
        let dy = dy.le(&zero).ite(&dy.unary_minus(), &dy);
        let dz = bz - &z;
        let dz = dz.le(&zero).ite(&dz.unary_minus(), &dz);
        let md = &dx + &dy + &dz;
        count += md.le(&br).ite(&one, &zero);
    }

    let optimizer = Optimize::new(&ctx);
    optimizer.maximize(&count);

    let dx = x.le(&zero).ite(&x.unary_minus(), &x);
    let dy = y.le(&zero).ite(&y.unary_minus(), &y);
    let dz = z.le(&zero).ite(&z.unary_minus(), &z);
    let md = &dx + &dy + &dz;
    optimizer.minimize(&md);

    optimizer.check(&[]);
    let model = optimizer.get_model().unwrap();
    let res = model.eval(&md, true).unwrap();

    res.as_i64().unwrap()
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let bots = load(include_str!("./input.txt"));

    let in_range = part_one(&bots);
    assert_eq!(in_range, 481);

    let pts = part_two(&bots);
    assert_eq!(pts, 47141479);
  }
}
