
fn main() {
    use std::time::Instant;

    let t1 = Instant::now();
    let checksum = calc_checksum("01110110101001000", 272);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", checksum, t2 - t1);

    let t1 = Instant::now();
    let checksum = calc_checksum("01110110101001000", 35651584);
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", checksum, t2 - t1);
}

fn calc_checksum(state: &str, len: usize) -> String {
    let mut data = state.chars().map(|c| c == '1').collect::<Vec<_>>();

    while data.len() <= len {
        let b = data.iter().rev().map(|v| !v).collect::<Vec<_>>();
        data.push(false);
        data.extend(b);
    }

    let mut cs = data[0..len].to_vec();
    while cs.len() % 2 == 0 {
        cs = checksum(&cs);
    }

    cs.iter().map(|v| if *v { '1' } else { '0' }).collect()
}

fn checksum(v: &[bool]) -> Vec<bool> {
    (0..v.len()).step_by(2).map(|i| v[i] == v[i+1]).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let checksum = calc_checksum("01110110101001000", 272);
        assert_eq!(checksum, "11100111011101111");

        let checksum = calc_checksum("01110110101001000", 35651584);
        assert_eq!(checksum, "10001110010000110");
    }
}