fn main() {
    use std::{fs, time::Instant};

    let input = fs::read_to_string("./input.txt").unwrap();
    let actions = load(&input);

    let t1 = Instant::now();
    let lcd = part_one(&actions);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", lcd.iter().sum::<usize>(), t2 - t1);

    let t1 = Instant::now();
    part_two(&lcd);
    let t2 = Instant::now();
    println!("Part 2: ({:?})", t2 - t1);

    // ZJHRKCPLYJ
}

#[derive(Debug)]
enum Action {
    Row(usize, usize),
    Col(usize, usize),
    Rect(usize, usize),
}

fn load(input: &str) -> Vec<Action> {
    input.lines().map(|s| {
        let v = s.split(' ').collect::<Vec<_>>();
        match v[1] {
            "row" => {
                let r = v[2].split('=').collect::<Vec<_>>();
                Action::Row(
                    r[1].parse::<usize>().unwrap(),
                    v[4].parse::<usize>().unwrap(),
                )
            },
            "column" => {
                let c = v[2].split('=').collect::<Vec<_>>();
                Action::Col(
                    c[1].parse::<usize>().unwrap(),
                    v[4].parse::<usize>().unwrap(),
                )
            },
            _ => { 
                let r = v[1].split('x').collect::<Vec<_>>();
                Action::Rect(
                    r[0].parse::<usize>().unwrap(),
                    r[1].parse::<usize>().unwrap(),
                )
            },
        }
    })
    .collect()
}

fn part_one(actions: &[Action]) -> [usize;300] {
    actions.iter().fold([0usize;300], |g, action| {
        let mut lcd = g.clone();
        match action {
            Action::Row(y, n) => {
                (0..50).for_each(|x| lcd[y*50+x] = 0);
                (0..50).for_each(|x| lcd[y*50+(x+n) % 50] = g[y*50+x]);
            },
            Action::Col(x, n) => {
                (0..6).for_each(|y| lcd[y*50+x] = 0);
                (0..6).for_each(|y| lcd[((y+n)%6)*50+x] = g[y*50+x]);
            },
            Action::Rect(x, y) => {
                (0..*y).for_each(|i| (0..*x).for_each(|j| lcd[i*50+j] = 1));
            },
        }
        lcd
    })
}

fn part_two(grid: &[usize]) {
    (0..6).for_each(|y| {
        (0..50).for_each(|x| print!("{}", if grid[y*50+x] == 1 { '#' } else { '.' }));
        println!();
    });
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let actions = load(&input);
    
        let lcd = part_one(&actions);
        assert_eq!(lcd.iter().sum::<usize>(), 110);
    }
}