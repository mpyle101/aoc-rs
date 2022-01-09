use lazy_static::lazy_static;

struct Item {
    cost: i32,
    armor: i32,
    damage: i32,
}

impl Item {
    fn new(cost: i32, damage: i32, armor: i32) -> Item {
        Item { cost, armor, damage }
    }
}

lazy_static! {
    static ref ARMOR: [Item;6] = [
        Item::new(  0, 0, 0),   // None
        Item::new( 13, 0, 1),   // Leather
        Item::new( 31, 0, 2),   // Chainmail
        Item::new( 53, 0, 3),   // Splintmail
        Item::new( 75, 0, 4),   // Bandedmail
        Item::new(102, 0, 5),   // Platemail
    ];

    static ref WEAPONS: [Item;5] = [
        Item::new( 8, 4, 0),    // Dagger
        Item::new(10, 5, 0),    // Shortsword
        Item::new(25, 6, 0),    // Warhammer
        Item::new(40, 7, 0),    // Longsword
        Item::new(74, 8, 0),    // Greataxe
    ];

    static ref RINGS: [Item;7] = [
        Item::new(  0, 0, 0),   // None
        Item::new( 25, 1, 0),   // Damage +1
        Item::new( 50, 2, 0),   // Damage +2
        Item::new(100, 3, 0),   // Damage +3
        Item::new( 20, 0, 1),   // Defense +1
        Item::new( 40, 0, 2),   // Defense +2
        Item::new( 80, 0, 3),   // Defense +3
    ];
}

struct Player {
    hp: i32,
    armor: i32,
    damage: i32,
}

fn main() {
    use std::time::Instant;

    let boss = load(include_str!("./input.txt"));

    let t1 = Instant::now();
    let cost = part_one(&boss);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", cost, t2 - t1);

    let t1 = Instant::now();
    let cost = part_two(&boss);
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", cost, t2 - t1);
}

fn load(input: &str) -> Player {
    let mut it = input.lines()
        .map(|l| {
            let v = l.split(": ").collect::<Vec<_>>();
            v[1].parse::<i32>().unwrap()
        });
    
    Player {
        hp:     it.next().unwrap(),
        damage: it.next().unwrap(),
        armor:  it.next().unwrap(),
    }
}

fn part_one(boss: &Player) -> i32 {
    use std::collections::HashMap;
    use itertools::iproduct;

    let mut cache = HashMap::new();

    iproduct!(
        0..RINGS.len(),
        0..RINGS.len(),
        0..ARMOR.len(),
        0..WEAPONS.len()
    ).filter_map(|(r1, r2, a, w)| {
        if r1 == r2 { 
            None
        } else {
            let cost   = ARMOR[a].cost + WEAPONS[w].cost + RINGS[r1].cost + RINGS[r2].cost;
            let armor  = ARMOR[a].armor + RINGS[r1].armor + RINGS[r2].armor;
            let damage = WEAPONS[w].damage + RINGS[r1].damage + RINGS[r2].damage;
            let player = Player { hp: 100, armor, damage };
            let won = if let Some(result) = cache.get(&(armor, damage)) {
                *result
            } else {
                let result = fight(&player, &boss);
                cache.insert((armor, damage), result);
                result
            };
            if won { Some(cost) } else { None }
        }
    })
    .min()
    .unwrap()
}

fn part_two(boss: &Player) -> i32 {
    use std::collections::HashMap;
    use itertools::iproduct;

    let mut cache = HashMap::new();

    iproduct!(
        0..RINGS.len(),
        0..RINGS.len(),
        0..ARMOR.len(),
        0..WEAPONS.len()
    ).filter_map(|(r1, r2, a, w)| {
        if r1 == r2 { 
            None
        } else {
            let cost   = ARMOR[a].cost + WEAPONS[w].cost + RINGS[r1].cost + RINGS[r2].cost;
            let armor  = ARMOR[a].armor + RINGS[r1].armor + RINGS[r2].armor;
            let damage = WEAPONS[w].damage + RINGS[r1].damage + RINGS[r2].damage;
            let player = Player { hp: 100, armor, damage };
            let won = if let Some(result) = cache.get(&(armor, damage)) {
                *result
            } else {
                let result = fight(&player, &boss);
                cache.insert((armor, damage), result);
                result
            };
            if won { None } else { Some(cost) }
        }
    })
    .max()
    .unwrap()
}

fn fight(player: &Player, boss: &Player) -> bool {
    let players = [player, boss];
    let mut hp  = [player.hp, boss.hp];

    let mut p = 1;  // player goes first
    loop {
        let damage = players[1-p].damage - players[p].armor;
        hp[p] -= if damage < 1 { 1 } else { damage };
            
        // Return true if the player wins
        if hp[p] < 1 { break p == 1 }

        p = 1 - p;  // switch players
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let boss = load(include_str!("./input.txt"));

    let cost = part_one(&boss);
    assert_eq!(cost, 111);

    let cost = part_two(&boss);
    assert_eq!(cost, 188);
  }
}