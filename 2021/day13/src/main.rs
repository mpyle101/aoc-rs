use std::collections::HashSet;

fn main() {
    use std::fs;
    use std::time::Instant;
    
    let input = fs::read_to_string("./input.txt").unwrap();
    let (dots, folds) = load(&input);

    let t1 = Instant::now();
    let visible = part_one(&dots, &folds);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", visible, t2 - t1);

    let t1 = Instant::now();
    let folded = part_two(&dots, &folds);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", folded.len(), t2 - t1);

//    print(&folded);
}

#[derive(Debug)]
enum Fold {
    Vert(u32),
    Horz(u32),
}

fn load(input: &str) -> (HashSet<(u32, u32)>, Vec<Fold>) {
    let mut it = input.split("\n\n");
    let dots = it.next().unwrap().lines()
        .map(|l| {
            let mut iter = l.split(',');
            (
                iter.next().unwrap().parse::<u32>().unwrap(),
                iter.next().unwrap().parse::<u32>().unwrap()
            )
        })
        .collect::<HashSet<_>>();

    let folds = it.next().unwrap().lines()
        .map(|l| {
            // fold along x=655
            let mut iter = l.split(' ');
            iter.next(); iter.next();
            let mut iter = iter.next().unwrap().split('=');
            let c = iter.next().unwrap().chars().next().unwrap();
            let n = iter.next().unwrap().parse::<u32>().unwrap();
            if c == 'x' { Fold::Vert(n) } else { Fold::Horz(n) }
        })
        .collect::<Vec<_>>();

    (dots, folds)
}

fn part_one(dots: &HashSet<(u32, u32)>, folds: &[Fold]) -> usize {
    let axis = folds.first().unwrap();
    dots.iter().map(|d| fold(&axis, d)).count()
}

fn part_two(dots: &HashSet<(u32, u32)>, folds: &[Fold]) -> HashSet<(u32, u32)> {
    folds.iter().fold(dots.clone(), |paper, axis|
        paper.iter().map(|d| fold(axis, d)).collect::<HashSet<_>>()
    )
}

fn fold(axis: &Fold, dot: &(u32, u32)) -> (u32, u32) {
    let (x, y) = dot;
    match axis {
        Fold::Vert(n) if x > n => (n - (x - n), *y),
        Fold::Horz(n) if y > n => (*x, n - (y - n)),
        _ => (*x, *y)
    }
}

#[allow(dead_code)]
fn print(dots: &HashSet<(u32, u32)>) {
    let mut v = dots.iter().collect::<Vec<_>>();
    v.sort();
    
    (0..6).for_each(|y| {
        (0..40).for_each(|x| {
            print!(".");
            let c = if dots.contains(&(x, y)) { '#' } else { '.' };
            print!("{}", c);
        });
        println!("");
    })
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let (dots, folds) = load(&input);

        let visible = part_one(&dots, &folds);
        assert_eq!(visible, 621);

        let folded = part_two(&dots, &folds);
        assert_eq!(folded.len(), 95);

        // HKUJGAJZ
    }
}