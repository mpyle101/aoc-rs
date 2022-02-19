
fn main() {
    use std::time::Instant;

    let t1 = Instant::now();
    let password = generate("hepxcrrq");
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", password, t2 - t1);

    let t1 = Instant::now();
    let password = generate(&password);
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", password, t2 - t1);
}

fn generate(pword: &str) -> String {
    // Turn our input into an array of zero based bytes.
    // This will make incrementing trivial using mod 26.
    // hepxcrrq
    let mut password = [0u8;8];
    pword.as_bytes().iter()
        .enumerate()
        .for_each(|(i, b)| password[i] = *b - b'a');

    increment(&mut password);
    while !is_valid(&password) {
        increment(&mut password);
    }

    // Rehydrate back into 'a' based characters.
    password.iter().map(|b| (b + b'a') as char).collect::<String>()
}

fn increment(s: &mut [u8;8]) {
    let mut i = 7;
    s[i] = (s[i] + 1) % 26;
    while s[i] == 0 && i > 0 {
        i -= 1;
        s[i] = (s[i] + 1) % 26;
    }
}

fn is_valid(s: &[u8;8]) -> bool {
    let mut pairs = 0;

    let mut straight = false;
    for i in 0..6 {
        straight |= s[i+1] == s[i] + 1 && s[i+2] == s[i] + 2
    }
    if straight {
        let mut i = 0;
        while i < 7 && pairs < 2 {
            i = if s[i] == s[i+1] {
                pairs += 1;
                i + 2
            } else {
                i + 1
            }
        }
    }

    pairs > 1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let password = generate("hepxcrrq");
        assert_eq!(password, "hepxxyzz");

        let password = generate("hepxxyzz");
        assert_eq!(password, "heqaabcc");
    }
}