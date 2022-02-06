use std::collections::HashMap;
use bitflags::bitflags;

fn main() {
    use std::time::Instant;

    let immunologers  = immune_system();
    let infectionists = infection();

    let t1 = Instant::now();
    let units = part_one(&immunologers, &infectionists);
    let t2 = Instant::now();
    println!("Part 1: {}  ({:?})", units, t2 - t1);

    // 21127
}

fn part_one(immunologers: &[Group], infectionists: &[Group]) -> i32 {
    let mut immies = immunologers.iter()
        .enumerate().map(|(i, &g)| (i+100, g)).collect::<HashMap<_, _>>();
    let mut bugs = infectionists.iter()
        .enumerate().map(|(i, &g)| (i+200, g)).collect::<HashMap<_, _>>();

    while immies.len() > 0 && bugs.len() > 0 {
        let t1 = select_targets(&immies, &bugs);
        let t2 = select_targets(&bugs, &immies);

        // Gather up target selections, sort by initiative and battle.
        let mut attacks = t1.iter().chain(t2.iter()).collect::<Vec<_>>();
        attacks.sort_by(|a, b| b.initiative.cmp(&a.initiative));
        attacks.iter().for_each(|t| attack(t, &mut immies, &mut bugs));
    }

    let winner = if immies.len() > 0 { immies } else { bugs };
    winner.values().map(|g| g.units).sum()
}

fn attack(target: &Target, immies: &mut Army, bugs: &mut Army) {
    let (allies, enemy) = if let Team::ImmuneSys = target.team {
        (immies, bugs)
    } else {
        (bugs, immies)
    };

    let grp = allies.get(&target.team_key);
    let foe = enemy.get_mut(&target.enemy_key);
    if grp.is_some() && foe.is_some() {
        let grp = grp.unwrap();
        let foe = foe.unwrap();
        if let Some(damage) = calc_damage(grp, foe) {
            let killed = (damage / foe.hp).min(foe.units);
            foe.units -= killed;

            if foe.units <= 0 {
                enemy.remove(&target.enemy_key);
            }
        }
    }
}

fn select_targets(allies: &Army, enemy: &Army) -> Vec<Target> {
    let mut targets = order_units(&enemy);
    order_units(&allies).iter()
        .filter_map(|k| {
            let v = allies.get(k).unwrap();
            pick_target(v, &targets, &enemy).map(|target| {
                let pos = targets.iter().position(|v| *v == target).unwrap();
                targets.remove(pos);

                Target { 
                    team: v.team,
                    team_key: *k, 
                    enemy_key: target,
                    initiative: v.initiative
                }
            })
        })
        .collect()
}

fn pick_target(group: &Group, targets: &[usize], army: &Army) -> Option<usize> {
    let mut damage = targets.iter()
        .filter_map(|k| {
            let enemy = army.get(k).unwrap();
            calc_damage(group, &enemy).map(|damage|
                (damage, enemy.power(), enemy.initiative, *k)
            )
        })
        .collect::<Vec<_>>();

    damage.sort_by(|a, b| b.cmp(&a));
    if damage.len() > 0 {
        Some(damage[0].3)
    } else {
        None
    }
}

fn calc_damage(a: &Group, b: &Group) -> Option<i32> {
    if b.is_immune(a) {
        None
    } else if b.has_weakness(a) {
        Some(a.power() * 2)
    } else {
        Some(a.power())
    }
}

fn order_units(army: &Army) -> Vec<usize> {
    let mut units = army.iter()
        .map(|(k, v)| (k, v.targetting()))
        .collect::<Vec<_>>();
    units.sort_by(|a, b| b.1.cmp(&a.1));
    units.iter().map(|(&k, _)| k).collect()
}

#[derive(Clone, Copy, Debug)]
struct Target {
    team: Team,
    team_key: usize,
    enemy_key: usize,
    initiative: i32,
}

#[derive(Clone, Copy, Debug)]
enum Team {
    Infection,
    ImmuneSys,
}

bitflags! {
    struct Damage: u8 {
        const NONE        = 0b00000000;
        const COLD        = 0b00000001;
        const FIRE        = 0b00000010;
        const SLASHING    = 0b00000100;
        const BLUDGEONING = 0b00001000;
        const RADIATION   = 0b00010000;
    }
}

#[derive(Clone, Copy, Debug)]
struct Group {
    team: Team,
    units: i32,
    hp: i32,
    damage: i32,
    inflicts: Damage,
    initiative: i32,
    immunities: Damage,
    weaknesses: Damage,
}

impl Group {
    fn power(&self) -> i32 {
        self.units * self.damage
    }

    fn targetting(&self) -> (i32, i32) {
        (self.power(), self.initiative)
    }

    fn is_immune(&self, other: &Self) -> bool {
        self.immunities & other.inflicts == other.inflicts
    }

    fn has_weakness(&self, other: &Self) -> bool {
        self.weaknesses & other.inflicts == other.inflicts
    }
}

type Army = HashMap<usize, Group>;
type Groups = Vec<Group>;

#[allow(dead_code)]
fn demo_immune_system() -> Groups {
    vec![
        Group {
            team: Team::ImmuneSys,
            units: 17,
            hp: 5390,
            damage: 4507,
            inflicts: Damage::FIRE,
            initiative: 2,
            immunities: Damage::NONE,
            weaknesses: Damage::RADIATION | Damage::BLUDGEONING,
        },
        Group {
            team: Team::ImmuneSys,
            units: 989,
            hp: 1274,
            damage: 25,
            inflicts: Damage::SLASHING,
            initiative: 3,
            immunities: Damage::FIRE,
            weaknesses: Damage::BLUDGEONING | Damage::SLASHING,
        }
    ]
}

#[allow(dead_code)]
fn demo_infection() -> Groups {
    vec![
        Group {
            team: Team::Infection,
            units: 801,
            hp: 4706,
            damage: 116,
            inflicts: Damage::BLUDGEONING,
            initiative: 1,
            immunities: Damage::NONE,
            weaknesses: Damage::RADIATION,
        },
        Group {
            team: Team::Infection,
            units: 4485,
            hp: 2961,
            damage: 12,
            inflicts: Damage::SLASHING,
            initiative: 4,
            immunities: Damage::RADIATION,
            weaknesses: Damage::FIRE | Damage::COLD,
        }
    ]
}

#[allow(dead_code)]
fn immune_system() -> Groups {
    vec![
        Group {
            team: Team::ImmuneSys,
            units: 8808,
            hp: 5616,
            damage: 5,
            inflicts: Damage::BLUDGEONING,
            initiative: 10,
            immunities: Damage::COLD,
            weaknesses: Damage::RADIATION,
        },
        Group {
            team: Team::ImmuneSys,
            units: 900,
            hp: 13511,
            damage: 107,
            inflicts: Damage::RADIATION,
            initiative: 20,
            immunities: Damage::NONE,
            weaknesses: Damage::RADIATION,
        },
        Group {
            team: Team::ImmuneSys,
            units: 581,
            hp: 10346,
            damage: 140,
            inflicts: Damage::FIRE,
            initiative: 14,
            immunities: Damage::SLASHING,
            weaknesses: Damage::RADIATION,
        },
        Group {
            team: Team::ImmuneSys,
            units: 57,
            hp: 9991,
            damage: 1690,
            inflicts: Damage::FIRE,
            initiative: 4,
            immunities: Damage::SLASHING | Damage::RADIATION | Damage::FIRE,
            weaknesses: Damage::BLUDGEONING,
        },
        Group {
            team: Team::ImmuneSys,
            units: 4074,
            hp: 6549,
            damage: 15,
            inflicts: Damage::RADIATION,
            initiative: 2,
            immunities: Damage::NONE,
            weaknesses: Damage::FIRE,
        },
        Group {
            team: Team::ImmuneSys,
            units: 929,
            hp: 5404,
            damage: 45,
            inflicts: Damage::FIRE,
            initiative: 16,
            immunities: Damage::BLUDGEONING | Damage::RADIATION,
            weaknesses: Damage::FIRE,
        },
        Group {
            team: Team::ImmuneSys,
            units: 2196,
            hp: 3186,
            damage: 10,
            inflicts: Damage::FIRE,
            initiative: 11,
            immunities: Damage::RADIATION,
            weaknesses: Damage::FIRE,
        },
        Group {
            team: Team::ImmuneSys,
            units: 4420,
            hp: 9691,
            damage: 21,
            inflicts: Damage::FIRE,
            initiative: 7,
            immunities: Damage::FIRE,
            weaknesses: Damage::RADIATION,
        },
        Group {
            team: Team::ImmuneSys,
            units: 3978,
            hp: 2306,
            damage: 4,
            inflicts: Damage::FIRE,
            initiative: 12,
            immunities: Damage::NONE,
            weaknesses: Damage::COLD | Damage::RADIATION,
        },
        Group {
            team: Team::ImmuneSys,
            units: 1284,
            hp: 4487,
            damage: 32,
            inflicts: Damage::SLASHING,
            initiative: 19,
            immunities: Damage::NONE,
            weaknesses: Damage::RADIATION | Damage::BLUDGEONING,
        },
    ]
}

#[allow(dead_code)]
fn infection() -> Groups {
    vec![
        Group {
            team: Team::Infection,
            units: 4262,
            hp: 23427,
            damage: 9,
            inflicts: Damage::SLASHING,
            initiative: 8,
            immunities: Damage::FIRE,
            weaknesses: Damage::SLASHING,
        },
        Group {
            team: Team::Infection,
            units: 217,
            hp: 9837,
            damage: 73,
            inflicts: Damage::BLUDGEONING,
            initiative: 1,
            immunities: Damage::NONE,
            weaknesses: Damage::BLUDGEONING,
        },
        Group {
            team: Team::Infection,
            units: 5497,
            hp: 33578,
            damage: 11,
            inflicts: Damage::SLASHING,
            initiative: 17,
            immunities: Damage::NONE,
            weaknesses: Damage::RADIATION | Damage::COLD,
        },
        Group {
            team: Team::Infection,
            units: 866,
            hp: 41604,
            damage: 76,
            inflicts: Damage::RADIATION,
            initiative: 15,
            immunities: Damage::NONE,
            weaknesses: Damage::COLD,
        },
        Group {
            team: Team::Infection,
            units: 1823,
            hp: 19652,
            damage: 20,
            inflicts: Damage::SLASHING,
            initiative: 13,
            immunities: Damage::NONE,
            weaknesses: Damage::FIRE | Damage::COLD,
        },
        Group {
            team: Team::Infection,
            units: 2044,
            hp: 23512,
            damage: 22,
            inflicts: Damage::SLASHING,
            initiative: 9,
            immunities: Damage::NONE,
            weaknesses: Damage::COLD,
        },
        Group {
            team: Team::Infection,
            units: 373,
            hp: 40861,
            damage: 215,
            inflicts: Damage::SLASHING,
            initiative: 18,
            immunities: Damage::COLD,
            weaknesses: Damage::NONE,
        },
        Group {
            team: Team::Infection,
            units: 5427,
            hp: 43538,
            damage: 15,
            inflicts: Damage::SLASHING,
            initiative: 5,
            immunities: Damage::RADIATION,
            weaknesses: Damage::BLUDGEONING,
        },
        Group {
            team: Team::Infection,
            units: 3098,
            hp: 19840,
            damage: 12,
            inflicts: Damage::RADIATION,
            initiative: 3,
            immunities: Damage::NONE,
            weaknesses: Damage::BLUDGEONING | Damage::COLD,
        },
        Group {
            team: Team::Infection,
            units: 785,
            hp: 14669,
            damage: 30,
            inflicts: Damage::FIRE,
            initiative: 6,
            immunities: Damage::NONE,
            weaknesses: Damage::NONE,
        },
    ]
}



#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let immunologers  = immune_system();
    let infectionists = infection();

    let units = part_one(&immunologers, &infectionists);
    assert_eq!(units, 21127);
  }
}
