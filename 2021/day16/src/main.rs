
fn main() {
    use std::fs;
    use std::time::Instant;
    
    let input = fs::read_to_string("./input.txt").unwrap();
    let packets = load(&input);

    let t1 = Instant::now();
    let versions = part_one(&packets);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", versions, t2 - t1);

    let t1 = Instant::now();
    let value = part_two(&packets);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", value, t2 - t1);
}

fn load(input: &str) -> Vec<u8> {
    let offset = b'A' - 10;

    input.as_bytes().iter().map(|&b| {
        let v = if b < b'A' { b - b'0' } else { b - offset };
        (0..4).rev().map(move |n| (v & (1 << n) != 0) as u8)
    }).flatten().collect::<Vec<_>>()
}

#[derive(Debug)]
enum PacketType {
    Literal(u64),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    Greater(Vec<Packet>),
    LessThan(Vec<Packet>),
    EqualTo(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: u8,
    value: PacketType,
}

impl Packet {
    fn versions(&self) -> u32 {
        use PacketType::*;

        self.version as u32 + 
        match &self.value {
            Literal(_) => 0,
            Sum(packets) => packets.iter().map(|p| p.versions()).sum(),
            Product(packets) => packets.iter().map(|p| p.versions()).sum(),
            Minimum(packets) => packets.iter().map(|p| p.versions()).sum(),
            Maximum(packets) => packets.iter().map(|p| p.versions()).sum(),
            EqualTo(packets) => packets.iter().map(|p| p.versions()).sum(),
            Greater(packets) => packets.iter().map(|p| p.versions()).sum(),
            LessThan(packets) => packets.iter().map(|p| p.versions()).sum(),
        }
    }

    fn eval(&self) -> u64 {
        use PacketType::*;

        match &self.value {
            Literal(n)        => *n,
            Sum(packets)      => packets.iter().map(|p| p.eval()).sum(),
            Product(packets)  => packets.iter().map(|p| p.eval()).product(),
            Minimum(packets)  => packets.iter().map(|p| p.eval()).min().unwrap(),
            Maximum(packets)  => packets.iter().map(|p| p.eval()).max().unwrap(),
            EqualTo(packets)  => (packets[0].eval() == packets[1].eval()) as u64,
            Greater(packets)  => (packets[0].eval() > packets[1].eval()) as u64,
            LessThan(packets) => (packets[0].eval() < packets[1].eval()) as u64,
        }
    }
}

fn part_one(packets: &[u8]) -> u32 {
    parse_packets(packets).iter().map(|p| p.versions()).sum()
}

fn part_two(packets: &[u8]) -> u64 {
    parse_packets(packets).first().unwrap().eval()
}

fn parse_packets(bits: &[u8]) -> Vec<Packet> {
    // literal with one group
    let smallest = 3 + 3 + 1;

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
    use PacketType::*;

    let version = parse_value(&bits[0..3]) as u8;
    let type_id = parse_value(&bits[3..6]);

    let (value, n) = if type_id == 4 {
        let (value, n) = parse_literal(&bits[6..bits.len()]);
        (Literal(value as u64), n)
    } else {
        let (packets, n) = parse_operator(&bits[6..bits.len()]);
        (match type_id {
            0 => Sum(packets),
            1 => Product(packets),
            2 => Minimum(packets),
            3 => Maximum(packets),
            5 => Greater(packets),
            6 => LessThan(packets),
            7 => EqualTo(packets),
            _ => panic!("Unknown operator: {}", type_id),
        }, n)

    };

    (Packet { version, value }, n + 6)
}

fn parse_literal(bits: &[u8]) -> (u64, usize) {
    let mut i = 0;
    let mut more = true;

    let mut value: u64 = 0;
    while more {
        more = bits[i] != 0;
        i += 1;
        value = value.rotate_left(4);
        value |= parse_value(&bits[i..i+4]) as u64 & 0xF;
        i += 4;
    }

    (value, i)
}

fn parse_operator(bits: &[u8]) -> (Vec<Packet>, usize) {
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

    (packets, r1)
}

fn parse_value(bits: &[u8]) -> u16 {
    let b = bits.len() - 1;
    bits.iter().enumerate()
        .filter(|(_, &n)| n == 1)
        .fold(0u16, |v, (i, _)| v | 1 << (b - i))
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let packets = load(&input);

        let versions = part_one(&packets);
        assert_eq!(versions, 965);

        let value = part_two(&packets);
        assert_eq!(value, 116672213160);
    }

    #[test]
    fn versions() {
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

    #[test]
    fn evals() {
        let map = load("C200B40A82");
        let value = part_two(&map);
        assert_eq!(value, 3);

        let map = load("04005AC33890");
        let value = part_two(&map);
        assert_eq!(value, 54);

        let map = load("880086C3E88112");
        let value = part_two(&map);
        assert_eq!(value, 7);

        let map = load("CE00C43D881120");
        let value = part_two(&map);
        assert_eq!(value, 9);

        let map = load("D8005AC2A8F0");
        let value = part_two(&map);
        assert_eq!(value, 1);

        let map = load("F600BC2D8F");
        let value = part_two(&map);
        assert_eq!(value, 0);

        let map = load("9C005AC2F8F0");
        let value = part_two(&map);
        assert_eq!(value, 0);

        let map = load("9C0141080250320F1802104A08");
        let value = part_two(&map);
        assert_eq!(value, 1);
    }
}