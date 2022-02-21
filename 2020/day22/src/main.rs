use std::collections::VecDeque;

#[derive(Debug, Eq, PartialEq)]
enum Player {
    One,
    Two,
}

type Deck = VecDeque<usize>;

fn main() {
    let decks = load(include_str!("./input.txt"));

    let score = part_one(&decks);
    println!("Part 1: {score}");

    let score = part_two(&decks);
    println!("Part 2: {score}");
}

fn load(input: &str) -> Vec<Vec<usize>> {
    input.split("\n\n").map(|s| {
        let mut it = s.lines();
        it.next();  // skip player
        it.map(|v| v.parse::<usize>().unwrap()).collect()
    })
    .collect()
}

fn part_one(decks: &[Vec<usize>]) -> usize {
    let mut deck1: Deck = decks[0].iter().cloned().collect();
    let mut deck2: Deck = decks[1].iter().cloned().collect();

    while !deck1.is_empty() && !deck2.is_empty() {
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();

        if card1 > card2 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else if card2 > card1 {
            deck2.push_back(card2);
            deck2.push_back(card1);
        } else {
            panic!("Matching cards!")
        }
    }

    let winner = if !deck1.is_empty() { deck1 } else { deck2 };
    winner.iter()
        .rev()
        .enumerate()
        .map(|(i, &v)| v * (i + 1))
        .sum()
}

fn part_two(decks: &[Vec<usize>]) -> usize {
    let mut deck1: Deck = decks[0].iter().cloned().collect();
    let mut deck2: Deck = decks[1].iter().cloned().collect();

    let winner = play(&mut deck1, &mut deck2);
    let cards = if winner == Player::One { deck1 } else { deck2 };
    cards.iter()
        .rev()
        .enumerate()
        .map(|(i, &v)| v * (i + 1))
        .sum()
}

fn play(deck1: &mut Deck, deck2: &mut Deck) -> Player {
    let mut rounds = Vec::new();

    let mut winner = None;
    while winner.is_none() {
        let round = Round (deck1.clone(), deck2.clone());
        if rounds.iter().any(|r| *r == round) {
            winner = Some(Player::One)
        } else {
            let card1 = deck1.pop_front().unwrap();
            let card2 = deck2.pop_front().unwrap();

            let round_winner = if deck1.len() >= card1 && deck2.len() >= card2 {
                let mut d1: Deck = deck1.iter().take(card1).cloned().collect();
                let mut d2: Deck = deck2.iter().take(card2).cloned().collect();
                play(&mut d1, &mut d2)
            } else if card1 > card2 { 
                Player::One 
            } else { 
                Player::Two
            };

            if round_winner == Player::One {
                deck1.push_back(card1);
                deck1.push_back(card2);
            } else {
                deck2.push_back(card2);
                deck2.push_back(card1);
            }
            rounds.push(round);

            if deck1.is_empty() {
                winner = Some(Player::Two)
            } else if deck2.is_empty() {
                winner = Some(Player::One)
            }
        }
    }

    winner.unwrap()
}

#[derive(Eq, PartialEq)]
struct Round (Deck, Deck);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let decks = load(include_str!("./input.txt"));

        let score = part_one(&decks);
        assert_eq!(score, 35818);

        let score = part_two(&decks);
        assert_eq!(score, 34771);
    }

    #[test]
    fn small() {
        let decks = load(include_str!("./test.txt"));

        let score = part_one(&decks);
        assert_eq!(score, 306);

        let score = part_two(&decks);
        assert_eq!(score, 291);
    }
}