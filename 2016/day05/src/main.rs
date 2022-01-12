fn main() {
    use std::time::Instant;

    let input = "abbhdwsy";

    let t1 = Instant::now();
    let password = part_one(input);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", password, t2 - t1);

    let t1 = Instant::now();
    let password = part_two(input);
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", password, t2 - t1);
}

fn part_one(door_id: &str) -> String {
    let mut password = ['-';8];

    let marker  = ['0';5];
    let mut buf = ['0';6];
    let mut index = 0;

    let mut i = 0;
    while i < 8 {
        let s = format!("{}{}", door_id, index);
        let digest = md5::compute(s);
        let hash = hex::encode(digest.iter());
        hash.chars().enumerate().take(6).for_each(|(n, c)| buf[n] = c);
        if buf[0..5] == marker {
            password[i] = buf[5];
            i += 1;
        }

        index += 1;
    }

    password.iter().collect()
}

fn part_two(door_id: &str) -> String {
    let mut password = ['-';8];

    let marker  = ['0';5];
    let mut buf = ['0';7];
    let mut index = 0;

    let mut i = 0;
    while i < 8 {
        let s = format!("{}{}", door_id, index);
        let digest = md5::compute(s);
        let hash = hex::encode(digest.iter());
        hash.chars().enumerate().take(7).for_each(|(n, c)| buf[n] = c);
        if buf[0..5] == marker && (buf[5] as u8) > 47 && (buf[5] as u8) < 56 {
            let ix = buf[5].to_digit(10).unwrap() as usize;
            if password[ix] == '-' { 
                password[ix] = buf[6];
                i += 1 ;
                println!("{}", password.iter().collect::<String>());
            }
        }

        index += 1;
    }

    password.iter().collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "abbhdwsy";
    
        let password = part_one(input);
        assert_eq!(password, "801b56a7");
    
        let password = part_two(input);
        assert_eq!(password, "424a0197");
    }
}