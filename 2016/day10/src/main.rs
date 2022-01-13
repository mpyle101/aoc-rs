use std::collections::HashMap;

fn main() {
    use std::{fs, time::Instant};

    let input = fs::read_to_string("./input.txt").unwrap();
    let bots = load(&input);

    let t1 = Instant::now();
    let bot = part_one(&bots);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", bot, t2 - t1);

    let t1 = Instant::now();
    let val = part_two(&bots);
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", val, t2 - t1);
}

#[derive(Clone, Copy, Debug)]
struct Bot {
    id: i32,
    low: i32,
    high: i32,
    chips: [i32;2],
}

impl Bot {
    fn new(id: i32) -> Bot {
        Bot { id, low: 0, high: 0, chips: [0;2] }
    }

    fn add_chip(&mut self, n: i32) {
        if self.chips[0] == 0 { 
            self.chips[0] = n;
        } else {
            self.chips[1] = n;
            self.chips.sort();
        };
    }
}

fn load(input: &str) -> HashMap<i32, Bot> {
    input.lines().fold(HashMap::new(), |mut bots, s| {
        let v = s.split(' ').collect::<Vec<_>>();
        if v[0] == "value" {
            let n = v[1].parse::<i32>().unwrap();
            let b = v[5].parse::<i32>().unwrap();
            let bot = bots.entry(b).or_insert(Bot::new(b));
            bot.add_chip(n);
        } else {
            let b = v[1].parse::<i32>().unwrap();
            let l = v[6].parse::<i32>().unwrap();
            let h = v[11].parse::<i32>().unwrap();
            let bot = bots.entry(b).or_insert(Bot::new(b));
            bot.low  = if  v[5] == "output" { -l } else { l };
            bot.high = if v[10] == "output" { -h } else { h };
        }

        bots
    })
}

fn part_one(input: &HashMap<i32, Bot>) -> i32 {
    use std::collections::VecDeque;

    let mut bots = input.clone();
    let (&id, _) = bots.iter().find(|(_, bot)| bot.chips[1] != 0).unwrap();
    let mut q = VecDeque::from([id]);

    while let Some(id) = q.pop_front() {
        let bot = *bots.get(&id).unwrap();

        if bot.chips == [17, 61] {
            return bot.id
        } else {
            if bot.low > 0 {
                let b1 = bots.get_mut(&bot.low).unwrap();
                b1.add_chip(bot.chips[0]);
                if b1.chips[1] != 0 {
                    q.push_back(b1.id);
                }
            }
            if bot.high > 0 {
                let b2 = bots.get_mut(&bot.high).unwrap();
                b2.add_chip(bot.chips[1]);
                if b2.chips[1] != 0 {
                    q.push_back(b2.id);
                }
            }

            bots.get_mut(&id).unwrap().chips = [0;2];
        }
    }

    0
}

fn part_two(input: &HashMap<i32, Bot>) -> i32 {
    use std::collections::VecDeque;

    let mut bots = input.clone();
    let mut outputs = [0;3];
    let (&id, _) = bots.iter().find(|(_, bot)| bot.chips[1] != 0).unwrap();
    let mut q = VecDeque::from([id]);

    while let Some(id) = q.pop_front() {
        let bot = *bots.get(&id).unwrap();

        if bot.low > 0 {
            let b1 = bots.get_mut(&bot.low).unwrap();
            b1.add_chip(bot.chips[0]);
            if b1.chips[1] != 0 {
                q.push_back(b1.id);
            }
        } else if bot.low > -3 {
            outputs[-bot.low as usize] = bot.chips[0];
        }
        if bot.high > 0 {
            let b2 = bots.get_mut(&bot.high).unwrap();
            b2.add_chip(bot.chips[1]);
            if b2.chips[1] != 0 {
                q.push_back(b2.id);
            }
        } else if bot.high > -3 {
            outputs[-bot.high as usize] = bot.chips[1];
        }

        bots.get_mut(&id).unwrap().chips = [0;2];

        let n = outputs.iter().product();
        if n > 0 { return n }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let bots = load(&input);
    
        let bot = part_one(&bots);
        assert_eq!(bot, 157);
    
        let val = part_two(&bots);
        assert_eq!(val, 1085);
    }
}