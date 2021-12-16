
fn main() {
    use std::fs;
    use std::time::Instant;
    
    let input = fs::read_to_string("./input.txt").unwrap();
    let packets = load(&input);

    let t1 = Instant::now();
    let versions = part_one(&packets);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", versions, t2 - t1);
}

fn load(input: &str) -> Vec<u8> {
    let offset = 'A' as u8 - 10;

    input.as_bytes().iter().map(|&b| {
        let v = if b < 'A' as u8 { b - '0' as u8 } else { b - offset };
        (0..4).rev().map(move |n| (v & (1 << n) != 0) as u8)
    }).flatten().collect::<Vec<_>>()
}

#[derive(Debug)]
enum PacketValue {
    Literal(u32),
    Operator(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: u8,
    value: PacketValue,
}

impl Packet {
    fn versions(&self) -> u32 {
        self.version as u32 + match &self.value {
            PacketValue::Literal(_) => 0,
            PacketValue::Operator(packets) => packets.iter().map(|p| p.versions()).sum()
        }
    }
}

fn part_one(packets: &[u8]) -> u32 {
    let packets = parse_packets(packets);
    
    packets.iter().map(|p| p.versions()).sum()
}

fn parse_packets(bits: &[u8]) -> Vec<Packet> {
    // literal with one group
    let smallest = 3 + 3 + 5 - 1;

    let mut i = 0;
    let mut packets = Vec::new();
    while i < bits.len() - smallest {
        let (packet, n) = parse_packet(&bits[i..bits.len()]);
        packets.push(packet);
        i += n;
    }
    
    packets
}

fn parse_packet(bits: &[u8]) -> (Packet, usize) {
    let version = parse_value(&bits[0..3]) as u8;
    let type_id = parse_value(&bits[3..6]);

    let (value, n) = if type_id == 4 {
        parse_literal(&bits[6..bits.len()])
    } else {
        parse_operator(&bits[6..bits.len()])
    };

    (Packet { version, value }, n + 6)
}

fn parse_literal(bits: &[u8]) -> (PacketValue, usize) {
    let mut i = 0;
    let mut more = true;

    let mut value: u32 = 0;
    while more {
        more = bits[i] != 0;
        i += 1;
        value = value.rotate_left(4);
        value |= parse_value(&bits[i..i+4]) as u32 & 0x000F;
        i += 4;
    }

    (PacketValue::Literal(value), i)
}

fn parse_operator(bits: &[u8]) -> (PacketValue, usize) {
    // 0 => number of bits, 1 => number of sub-packets
    let length_type = bits[0];

    let mut r1 = 1 + if length_type == 0 { 15 } else { 11 };
    let n = parse_value(&bits[1..r1]) as usize;

    let packets = if length_type == 0 {
        r1 += n as usize;
        parse_packets(&bits[r1-n..r1])
    } else {
        (0..n).fold(Vec::new(), |mut v, _| {
            let (packet, n) = parse_packet(&bits[r1..bits.len()]);
            r1 += n;
            v.push(packet);
            v
        })
    };

    (PacketValue::Operator(packets), r1)
}

fn parse_value(bits: &[u8]) -> u16 {
    let b = bits.len() - 1;
    bits.iter().enumerate()
        .filter(|(_, &n)| n == 1)
        .fold(0u16, |v, (i, _)| v | (1 << b - i))
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let map = load(&input);

        let versions = part_one(&map);
        assert_eq!(versions, 965);
    }

    #[test]
    fn samples() {
        let map = load("D2FE28");
        let versions = part_one(&map);
        assert_eq!(versions, 6);

        let map = load("38006F45291200");
        let versions = part_one(&map);
        assert_eq!(versions, 9);

        let map = load("EE00D40C823060");
        let versions = part_one(&map);
        assert_eq!(versions, 14);

        let map = load("8A004A801A8002F478");
        let versions = part_one(&map);
        assert_eq!(versions, 16);

        let map = load("620080001611562C8802118E34");
        let versions = part_one(&map);
        assert_eq!(versions, 12);

        let map = load("C0015000016115A2E0802F182340");
        let versions = part_one(&map);
        assert_eq!(versions, 23);

        let map = load("A0016C880162017C3686B18A3D4780");
        let versions = part_one(&map);
        assert_eq!(versions, 31);
    }
}