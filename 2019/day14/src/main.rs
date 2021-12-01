use anyhow::Result;
use std::collections::{HashMap, VecDeque};

type SpecMap<'a> = HashMap<&'a str, Spec<'a>>;
const PHANTOM: &str = "****";

fn main() {
  let specs = load(include_str!("./formulas.txt")).unwrap();
  let ore = calc_min_ore("FUEL", &specs).unwrap();
  println!("Part one: {}", ore);

  let fuel = calc_max_fuel(&specs).unwrap();
  println!("Part two: {}", fuel)
}

fn load(formulas: &str) -> Result<SpecMap> {
  formulas.lines().map(parse).collect::<Result<SpecMap>>()
}

fn parse(spec: &str) -> Result<(&str, Spec)> {
  let spec = Spec::from(spec)?;
  Ok((spec.id, spec))
}

fn calc_min_ore(start: &str, specs: &SpecMap) -> Result<u64, &'static str> {
  let mut rsrcs: VecDeque<_> = VecDeque::new();
  let fuel = specs.get(start).ok_or("Start not found")?;
  rsrcs.push_back(Resource {
    id: fuel.id,
    count: fuel.count,
  });

  let mut ore = 0u64;
  let mut extras: HashMap<&str, u64> = HashMap::new();

  while let Some(rsrc) = rsrcs.pop_front() {
    if rsrc.id == "ORE" {
      ore += rsrc.count;
    } else {
      let extra = *extras.get(rsrc.id).unwrap_or(&0u64);
      if rsrc.count > extra {
        let want = rsrc.count - extra;
        let spec = specs.get(rsrc.id).ok_or("Resource not found")?;
        let mult = needed(want, spec.count);
        let extra = (spec.count * mult) - want;
        extras.insert(spec.id, extra);
        spec.rsrcs.iter().for_each(|r| {
          rsrcs.push_back(Resource {
            id: r.id,
            count: mult * r.count,
          })
        });
      } else {
        extras.insert(rsrc.id, extra - rsrc.count);
      }
    }
  }

  Ok(ore)
}

fn calc_max_fuel(specs: &SpecMap) -> Result<u64, &'static str> {
  // Better to calc the starting step but it's fast enough now.
  let mut step = 1_000_000;
  let available = 1_000_000_000_000;
  let fuel = available / calc_min_ore("FUEL", specs)?;
  let phantom = Spec {
    id: PHANTOM,
    count: 1,
    rsrcs: vec![Resource {
      id: "FUEL",
      count: fuel,
    }],
  };
  let mut specs = specs.clone();
  specs.insert(PHANTOM, phantom);

  while step != 0 {
    calc_fuel_step(available, step, &mut specs)?;
    step /= 10;
  }

  let phantom = specs.get(PHANTOM).ok_or("Phantom not found")?;
  Ok(phantom.rsrcs[0].count)
}

fn calc_fuel_step(available: u64, step: u64, specs: &mut SpecMap) -> Result<u64, &'static str> {
  let mut needed = calc_min_ore(PHANTOM, &specs)?;
  while needed < available {
    let phantom = specs.get_mut(PHANTOM).ok_or("Phantom not found")?;
    phantom.rsrcs[0].count += step;
    needed = calc_min_ore(PHANTOM, &specs)?;
  }

  let phantom = specs.get_mut(PHANTOM).ok_or("Phantom not found")?;
  phantom.rsrcs[0].count -= step;
  Ok(phantom.rsrcs[0].count)
}

fn needed(want: u64, spec: u64) -> u64 {
  // ceiling division
  (want + spec - 1) / spec
}

#[derive(Clone, Debug, PartialEq)]
struct Resource<'a> {
  id: &'a str,
  count: u64,
}

#[derive(Clone, Debug, PartialEq)]
struct Spec<'a> {
  id: &'a str,
  count: u64,
  rsrcs: Vec<Resource<'a>>,
}

impl<'a> Spec<'a> {
  fn from(spec: &'a str) -> Result<Self> {
    let v: Vec<_> = spec.split("=>").collect();
    let rsrcs = v[0]
      .split(',')
      .map(|v| Spec::parse(v))
      .collect::<Result<_>>()?;
    let rsrc = Spec::parse(v[1])?;

    Ok(Spec {
      id: rsrc.id,
      count: rsrc.count,
      rsrcs,
    })
  }

  fn parse(rsrc: &str) -> Result<Resource> {
    let v: Vec<_> = rsrc.trim().split(' ').collect();
    Ok(Resource {
      id: v[1],
      count: v[0].parse::<u64>()?,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part_one_works() {
    let specs = load(include_str!("./formulas.txt")).unwrap();
    let ore = calc_min_ore("FUEL", &specs).unwrap();

    assert_eq!(ore, 1967319)
  }

  #[test]
  fn part_two_works() {
    let specs = load(include_str!("./formulas.txt")).unwrap();
    let ore = calc_max_fuel(&specs).unwrap();

    assert_eq!(ore, 1122036)
  }

  #[test]
  fn part_one_basic() {
    let specs = load(
      "9 ORE => 2 A\n\
      8 ORE => 3 B\n\
      7 ORE => 5 C\n\
      3 A, 4 B => 1 AB\n\
      5 B, 7 C => 1 BC\n\
      4 C, 1 A => 1 CA\n\
      2 AB, 3 BC, 4 CA => 1 FUEL",
    )
    .unwrap();
    let ore = calc_min_ore("FUEL", &specs).unwrap();

    assert_eq!(ore, 165)
  }

  #[test]
  fn part_one_small() {
    let specs = load(
      "157 ORE => 5 NZVS\n\
      165 ORE => 6 DCFZ\n\
      44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n\
      12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n\
      179 ORE => 7 PSHF\n\
      177 ORE => 5 HKGWZ\n\
      7 DCFZ, 7 PSHF => 2 XJWVT\n\
      165 ORE => 2 GPVTF\n\
      3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
    )
    .unwrap();
    let ore = calc_min_ore("FUEL", &specs).unwrap();

    assert_eq!(ore, 13312)
  }

  #[test]
  fn part_one_medium() {
    let specs = load(
      "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\n\
      17 NVRVD, 3 JNWZP => 8 VPVL\n\
      53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\n\
      22 VJHF, 37 MNCFX => 5 FWMGM\n\
      139 ORE => 4 NVRVD\n\
      144 ORE => 7 JNWZP\n\
      5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\n\
      5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\n\
      145 ORE => 6 MNCFX\n\
      1 NVRVD => 8 CXFTF\n\
      1 VJHF, 6 MNCFX => 4 RFSQX\n\
      176 ORE => 6 VJHF",
    )
    .unwrap();
    let ore = calc_min_ore("FUEL", &specs).unwrap();

    assert_eq!(ore, 180697)
  }

  #[test]
  fn part_one_large() {
    let specs = load(
      "171 ORE => 8 CNZTR\n\
      7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL\n\
      114 ORE => 4 BHXH\n\
      14 VRPVC => 6 BMBT\n\
      6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL\n\
      6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT\n\
      15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW\n\
      13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW\n\
      5 BMBT => 4 WPTQ\n\
      189 ORE => 9 KTJDG\n\
      1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP\n\
      12 VRPVC, 27 CNZTR => 2 XDBXC\n\
      15 KTJDG, 12 BHXH => 5 XCVML\n\
      3 BHXH, 2 VRPVC => 7 MZWV\n\
      121 ORE => 7 VRPVC\n\
      7 XCVML => 6 RJRHP\n\
      5 BHXH, 4 VRPVC => 5 LTCX",
    )
    .unwrap();
    let ore = calc_min_ore("FUEL", &specs).unwrap();

    assert_eq!(ore, 2210736)
  }

  #[test]
  fn part_two_small() {
    let specs = load(
      "157 ORE => 5 NZVS\n\
      165 ORE => 6 DCFZ\n\
      44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n\
      12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n\
      179 ORE => 7 PSHF\n\
      177 ORE => 5 HKGWZ\n\
      7 DCFZ, 7 PSHF => 2 XJWVT\n\
      165 ORE => 2 GPVTF\n\
      3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
    )
    .unwrap();
    let ore = calc_max_fuel(&specs).unwrap();

    assert_eq!(ore, 82892753)
  }

  #[test]
  fn part_two_medium() {
    let specs = load(
      "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\n\
      17 NVRVD, 3 JNWZP => 8 VPVL\n\
      53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\n\
      22 VJHF, 37 MNCFX => 5 FWMGM\n\
      139 ORE => 4 NVRVD\n\
      144 ORE => 7 JNWZP\n\
      5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\n\
      5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\n\
      145 ORE => 6 MNCFX\n\
      1 NVRVD => 8 CXFTF\n\
      1 VJHF, 6 MNCFX => 4 RFSQX\n\
      176 ORE => 6 VJHF",
    )
    .unwrap();
    let ore = calc_max_fuel(&specs).unwrap();

    assert_eq!(ore, 5586022)
  }

  #[test]
  fn part_two_large() {
    let specs = load(
      "171 ORE => 8 CNZTR\n\
      7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL\n\
      114 ORE => 4 BHXH\n\
      14 VRPVC => 6 BMBT\n\
      6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL\n\
      6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT\n\
      15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW\n\
      13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW\n\
      5 BMBT => 4 WPTQ\n\
      189 ORE => 9 KTJDG\n\
      1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP\n\
      12 VRPVC, 27 CNZTR => 2 XDBXC\n\
      15 KTJDG, 12 BHXH => 5 XCVML\n\
      3 BHXH, 2 VRPVC => 7 MZWV\n\
      121 ORE => 7 VRPVC\n\
      7 XCVML => 6 RJRHP\n\
      5 BHXH, 4 VRPVC => 5 LTCX",
    )
    .unwrap();
    let ore = calc_max_fuel(&specs).unwrap();

    assert_eq!(ore, 460664)
  }
}
