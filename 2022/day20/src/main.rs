use std::collections::VecDeque;

fn main() {
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    println!("Part 1: {} ({:?})", part_one(input), t.elapsed());

    let t = Instant::now();
    println!("Part 2: {} ({:?})", part_two(input), t.elapsed());

    let t = Instant::now();
    println!("Part 3: {} ({:?})", part_three(input), t.elapsed());
}

fn part_one(input: &str) -> i64 {
    let numbers = input.lines()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut mixed = VecDeque::from((0..numbers.len()).collect::<Vec<_>>());

    mix(&numbers, &mut mixed);
    coords(&numbers, &mixed)
}

fn part_two(input: &str) -> i64 {
    let numbers = input.lines()
        .map(|s| s.parse::<i64>().unwrap())
        .map(|n| n * 811589153)
        .collect::<Vec<_>>();
    let mut mixed = VecDeque::from((0..numbers.len()).collect::<Vec<_>>());

    (0..10).for_each(|_| mix(&numbers, &mut mixed));
    coords(&numbers, &mixed)
}

fn part_three(input: &str) -> i64 {
    let numbers = input.lines()
        .map(|s| s.parse::<i64>().unwrap())
        .map(|n| n * 811589153)
        .collect::<Vec<_>>();
    let mut mixed = VecDeque::from((0..numbers.len()).collect::<Vec<_>>());

    let len = numbers.len() - 1;
    (0..10).for_each(|_| {
        (0..numbers.len()).for_each(|i| {
            let p = mixed.iter().position(|n| *n == i).unwrap();
            mixed.remove(p);
            let n = (p as i64 + numbers[i]).rem_euclid(len as i64);
            mixed.insert(n as usize, i);    
        })
    });
    coords(&numbers, &mixed)
}

fn mix(numbers: &[i64], mixed: &mut VecDeque<usize>) {
    let len = numbers.len() - 1;
    (0..numbers.len())
        .for_each(|i| {
            // Move the index to the front of the queue; remove it and then
            // rotate the queue left or right based on the sign of the move
            // (modding the rotate amount by the new length: original - 1).
            // Then push the index onto the end of the queue.
            let p = mixed.iter().position(|n| *n == i).unwrap();
            mixed.rotate_left(p);
            mixed.pop_front();

            if numbers[i] < 0 {
                mixed.rotate_right(-numbers[i] as usize % len);
            } else {
                mixed.rotate_left(numbers[i] as usize % len);
            }
            mixed.push_back(i);
        });
}

fn coords(numbers: &[i64], mixed: &VecDeque<usize>) -> i64 {
    let zero_ix = mixed.iter().position(|i| numbers[*i] == 0).unwrap();
    [1000, 2000, 3000].iter()
        .map(|i| numbers[mixed[(zero_ix + i) % numbers.len()]])
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one() {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 13883);
    }

    #[test]
    fn input_part_two() {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 19185967576920);
    }

    #[test]
    fn example_part_one() {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 3);
    }

    #[test]
    fn example_part_two() {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 1623178306);
    }
}
