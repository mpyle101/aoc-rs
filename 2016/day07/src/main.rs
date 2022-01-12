fn main() {
    use std::fs;
    use std::time::Instant;

    let input = fs::read_to_string("./input.txt").unwrap();

    let t1 = Instant::now();
    let tls = part_one(&input);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", tls, t2 - t1);

    let t1 = Instant::now();
    let ssl = part_two(&input);
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", ssl, t2 - t1);
}

fn part_one(input: &str) -> i32 {
    input.lines().fold(0, |n, s| {
        let bytes = s.as_bytes();
        let mut in_hyper = false;
        let mut tls = false;
        for i in 0..bytes.len() - 4 {
            if bytes[i] == '[' as u8 {
                in_hyper = true
            } else if bytes[i] == ']' as u8 {
                in_hyper = false
            }

            if is_abba(&bytes[i..i+4]) {
                if in_hyper { 
                    tls = false;
                    break;
                 } else { 
                     tls = true
                }
            }
        }

        n + tls as i32
    })
}

fn part_two(input: &str) -> i32 {
    use std::collections::HashSet;

    input.lines().fold(0, |n, s| {
        let mut supernet = HashSet::new();
        let mut hypernet = HashSet::new();

        let bytes = s.as_bytes();
        let mut in_hyper = false;
        for i in 0..bytes.len() - 2 {
            if bytes[i] == '[' as u8 {
                in_hyper = true
            } else if bytes[i] == ']' as u8 {
                in_hyper = false
            }

            if is_aba(&bytes[i..i+3]) {
                if in_hyper { 
                    hypernet.insert(&bytes[i..i+3]);
                 } else { 
                    supernet.insert(&bytes[i..i+3]);
                }
            }
        }

        let ssl = supernet.iter().any(|aba| {
            let bab = to_bab(aba);
            hypernet.contains(&bab[..])
        });

        n + ssl as i32
    })
}

fn is_abba(bytes: &[u8]) -> bool {
    bytes[0] == bytes[3] && bytes[1] == bytes[2]
}

fn is_aba(bytes: &[u8]) -> bool {
    bytes[0] == bytes[2] && bytes[0] != bytes[1]
}

fn to_bab(bytes: &[u8]) -> [u8;3] {
    [bytes[1], bytes[0], bytes[1]]
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();

        let tls = part_one(&input);
        assert_eq!(tls, 105);

        let ssl = part_two(&input);
        assert_eq!(ssl, 258);
    }

    #[test]
    fn examples() {
        let ssl = part_two("aba[bab]xyz");
        assert_eq!(ssl, 1);

        let ssl = part_two("xyx[xyx]xyx");
        assert_eq!(ssl, 0);

        let ssl = part_two("aaa[kek]eke");
        assert_eq!(ssl, 1);

        let ssl = part_two("zazbz[bzb]cdb");
        assert_eq!(ssl, 1);
    }
}