fn main() {
    use std::{fs, time::Instant};

    let input = fs::read_to_string("./input.txt").unwrap();
    let rooms = load(&input);

    let t1 = Instant::now();
    let sectors = part_one(&rooms);
    let t2 = Instant::now();
    println!("Part 1: {} ({:?})", sectors, t2 - t1);

    let t1 = Instant::now();
    let room = part_two(&rooms);
    let t2 = Instant::now();
    println!("Part 2: {} ({:?})", room, t2 - t1);
}

#[derive(Debug)]
struct Room<'a> {
    id: i32,
    name: &'a str,
    checksum: [char;5],
}

fn load(input: &str) -> Vec<Room> {
    input.lines().map(|s| {
        let i = s.rfind('-').unwrap();
        let name = &s[0..i];
        let id = s[i+1..i+4].parse::<i32>().unwrap();
        let mut checksum = ['a';5];
        s[i+5..i+10].chars().enumerate().for_each(|(i, c)| checksum[i] = c);

        Room { id, name, checksum }
    })
    .collect()
}

fn part_one(rooms: &[Room]) -> i32 {
    rooms.iter().fold(0, |n, r| n + if verify(r) { r.id } else { 0 })
}

fn part_two(rooms: &[Room]) -> i32 {
    // Gotten by printing out all the decoded names.
    let north_pole_objects = "northpole object storage";
    let room = rooms.iter()
        .find(|r| verify(r) && decode(r) == north_pole_objects)
        .unwrap();

    room.id
}

fn verify(room: &Room) -> bool {
    use std::cmp::Reverse;
    use std::collections::HashMap;

    let counts = room.name.chars()
        .fold(HashMap::new(), |mut map, c| {
            if c != '-' { *map.entry(c).or_insert(0) += 1 }
            map
        });
    let mut order = counts.iter().map(|(c, n)| (n, c)).collect::<Vec<_>>();
    order.sort_by_key(|a| (Reverse(a.0), a.1));
    order.iter().enumerate().take(5)
        .fold(true, |valid, (i, c)| valid && room.checksum[i] == *c.1)
}

fn decode(room: &Room) -> String {
    room.name.chars().map(|c| 
        if c == '-' {
            ' '
        } else {
            // Reduce to zero based character value to rotate via mod 26.
            // Then "rehydrate" by adding back 'a' and cast to char.
            let mut c1 = c as i32 - 'a' as i32 + room.id;
            c1 %= 26;
            c1 += 'a' as i32;
            c1 as u8 as char
        }
    ).collect()
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let rooms = load(&input);
    
        let sectors = part_one(&rooms);
        assert_eq!(sectors, 409147);
    
        let sector_id = part_two(&rooms);
        assert_eq!(sector_id, 991);
    }

    #[test]
    fn decoding() {
        let room = Room {
            id: 343,
            name: "qzmt-zixmtkozy-ivhz",
            checksum: ['a';5]
        };


        let decoded = decode(&room);
        assert_eq!(decoded, "very encrypted name");
    }
}