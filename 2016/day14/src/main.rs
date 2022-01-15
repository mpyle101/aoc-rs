//use std::collections::HashMap;

fn main() {
    use std::time::Instant;

    let t1 = Instant::now();
    let index = part_one("ihaygndm");
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", index, t2 - t1);

    let t1 = Instant::now();
    let index = part_two("ihaygndm");
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", index, t2 - t1);
}

fn part_one(salt: &str) -> usize {
    get_keys(salt, 1)
}

fn part_two(salt: &str) -> usize {
    get_keys(salt, 2017)
}

fn get_keys(salt: &str, n: i32) -> usize {
    let mut keys = Vec::new();
    let mut candidates: Vec<(char, usize)> = vec![];

    let mut index = 0;
    while keys.len() < 64 {
        let key  = format!("{}{}", salt, index);
        let hash = mash(key.clone(), n);

        if let Some(c1) = check5(&hash) {
            candidates.iter().for_each(|(c2, ix)|
                if (index - ix) <= 1000 && c1 == *c2 {
                    keys.push(*ix);
                }
            );
        }

        if let Some(c) = check3(&hash) {
            candidates.push((c, index));
        }

        index += 1;
    }
    keys.sort();

    keys[63]
}

fn mash(key: String, n: i32) -> Vec<char> {
    let result = (0..n).fold(key, |k, _| {
        format!("{:x}", md5::compute(k))
    });
    
    result.chars().collect::<Vec<char>>()
}

fn check3(hash: &[char]) -> Option<char> {
    for i in 0..hash.len() - 2 {
        if hash[i] == hash[i+1] &&
           hash[i] == hash[i+2] {
            return Some(hash[i])
        }
    }

    None
}

fn check5(hash: &[char]) -> Option<char> {
    for i in 0..hash.len() - 4 {
        if hash[i] == hash[i+1] &&
           hash[i] == hash[i+2] && 
           hash[i] == hash[i+3] && 
           hash[i] == hash[i+4] {
            return Some(hash[i])
        }
    }

    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let index = part_one("ihaygndm");
        assert_eq!(index, 15035);

        let index = part_two("ihaygndm");
        assert_eq!(index, 19968);
    }

    #[test]
    fn samples() {
        let index = part_one("abc");
        assert_eq!(index, 22728);

        let index = part_two("abc");
        assert_eq!(index, 22551);
    }
}