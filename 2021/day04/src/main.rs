fn main() {
    use std::fs;
    use std::time::Instant;

    let (cards, mut boards) = load(&fs::read_to_string("./input.txt").unwrap());

    let t1 = Instant::now();
    let (winner, squid) = do_it(&cards, &mut boards);
    let t2 = Instant::now();
    println!("Part 1: {}, Part 2: {} ({:?})", winner, squid, t2 - t1);
}

fn load(input: &str) -> (Vec<i32>, Vec<Vec<(i32, bool)>>) {
    let mut iter = input.split("\n\n");
    let cards = iter
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let boards = iter.fold(Vec::new(), |mut v, s| {
        let m = s
            .split_whitespace()
            .map(|s| (s.parse::<i32>().unwrap(), false))
            .collect::<Vec<_>>();
        v.push(m);
        v
    });

    (cards, boards)
}

fn do_it(cards: &[i32], boards: &mut Vec<Vec<(i32, bool)>>) -> (i32, i32) {
    // Draw the first 5 cards
    let mut iter = cards.iter();
    (0..4).for_each(|_| mark(iter.next().unwrap(), boards));
    let mut card = iter.next().unwrap();
    mark(card, boards);

    let mut found = boards.iter().position(bingo);
    while found.is_none() {
        card = iter.next().unwrap();
        mark(card, boards);
        found = boards.iter().position(bingo);
    }
    let idx = found.unwrap();
    let board = boards.get(idx).unwrap();
    let unmarked: i32 = board.iter().filter(|t| !t.1).map(|t| t.0).sum();
    let winner = unmarked * card;
    boards.remove(idx);

    let mut squid = 0;
    for card in iter {
        mark(card, boards);
        while let Some(idx) = boards.iter().position(bingo) {
            let board = boards.get(idx).unwrap();
            let unmarked: i32 = board.iter().filter(|t| !t.1).map(|t| t.0).sum();
            squid = unmarked * card;
            boards.remove(idx);
        }
    }

    (winner, squid)
}

fn mark(n: &i32, boards: &mut [Vec<(i32, bool)>]) {
    boards.iter_mut().for_each(|b| {
        b.iter_mut().for_each(|t| (*t).1 = t.1 | (t.0 == *n));
    })
}

fn bingo(tab: &Vec<(i32, bool)>) -> bool {
    tab.chunks(5).any(|row| row.iter().all(|v| v.1))
        || (0..5).any(|i| get_col(i, tab).all(|v| v.1))
}

fn get_col(col: usize, tab: &[(i32, bool)]) -> impl Iterator<Item = &(i32, bool)> {
    tab.iter().skip(col).step_by(5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (cards, mut boards) = load(include_str!("../input.txt"));

        let (winner, squid) = do_it(&cards, &mut boards);
        assert_eq!(winner, 58838);
        assert_eq!(squid, 6256);
    }
}
