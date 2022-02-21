use std::collections::{HashMap, HashSet};

fn main() {
    let passports = load(include_str!("./passports.txt"), false);
    let valid = passports.iter().filter(|p| p.is_valid()).count();
    println!("Part1: {}", valid);

    let passports = load(include_str!("./passports.txt"), true);
    let valid = passports.iter().filter(|p| p.is_valid()).count();
    println!("Part2: {}", valid);
}

type Year = u32;

#[allow(dead_code)]
#[derive(Debug)]
struct Passport<'a> {
    byr: Option<Year>,
    iyr: Option<Year>,
    eyr: Option<Year>,
    hgt: Option<&'a str>,
    hcl: Option<&'a str>,
    ecl: Option<&'a str>,
    pid: Option<&'a str>,
    cid: Option<&'a str>,
}

impl<'a> Passport<'a> {
    fn new(data: &HashMap<&'a str, &'a str>) -> Self {
        Passport {
            byr: data.get("byr").and_then(|v| v.parse::<u32>().ok()),
            iyr: data.get("iyr").and_then(|v| v.parse::<u32>().ok()),
            eyr: data.get("eyr").and_then(|v| v.parse::<u32>().ok()),
            hgt: data.get("hgt").copied(),
            hcl: data.get("hcl").copied(),
            ecl: data.get("ecl").copied(),
            pid: data.get("pid").copied(),
            cid: data.get("cid").copied(),
        }
    }

    fn from(data: &HashMap<&'a str, &'a str>) -> Self {
        let colors: HashSet<&str> = 
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .iter().cloned().collect();

        Passport {
            byr: data.get("byr").and_then(|&v| parse_year(v, 1920, 2002)),
            iyr: data.get("iyr").and_then(|&v| parse_year(v, 2010, 2020)),
            eyr: data.get("eyr").and_then(|&v| parse_year(v, 2020, 2030)),
            hgt: data.get("hgt").and_then(|&v| parse_height(v)),
            hcl: data.get("hcl").and_then(|&v| parse_hair_color(v)),
            ecl: data.get("ecl").and_then(|&v| parse_eye_color(&colors, v)),
            pid: data.get("pid").and_then(|&v| parse_passport_id(v)),
            cid: data.get("cid").copied(),
        }
    }

    fn is_valid(&self) -> bool {
        self.byr.is_some() &&
        self.iyr.is_some() &&
        self.eyr.is_some() &&
        self.hgt.is_some() &&
        self.hcl.is_some() &&
        self.ecl.is_some() &&
        self.pid.is_some()
    }
}

fn load<'a>(passports: &'a str, validate: bool) -> Vec<Passport> {
    let to_tuple = |v: Vec<&'a str>| (v[0], v[1]);
    passports.split("\n\n")
        .map(|p| p.split_ascii_whitespace().collect::<Vec<&str>>())
        .map(|p| p.iter().map(|d| to_tuple(d.split(':').collect())).collect::<HashMap<_,_>>())
        .map(|v| if validate { Passport::from(&v) } else { Passport::new(&v) })
        .collect()
}

fn parse_year(s: &str, min: u32, max: u32) -> Option<u32> {
    let check_year = |v, min, max| (v >= min && v <= max).then(|| v);
    if s.len() == 4 {
        s.parse::<u32>().ok().and_then(|v| check_year(v, min, max))
    } else {
        None
    }
}

fn parse_height(s: &str) -> Option<&str> {
    let check_height = |v, min, max| (v >= min && v <= max).then(|| v);
    if s.len() >= 4 {
        let amt  = &s[..s.len()-2];
        let unit = &s[s.len()-2..];

        match unit {
            "in" => amt.parse::<u32>().ok().and_then(|v| check_height(v, 59, 76)),
            "cm" => amt.parse::<u32>().ok().and_then(|v| check_height(v, 150, 193)),
            _ => None,
        }?;

        Some(s)
    } else {
        None
    }
}

fn parse_hair_color(s: &str) -> Option<&str> {
    if s.len() == 7 && s.as_bytes().get(0) == Some(&b'#') {
        for c in s.bytes().skip(1) {
            match c {
                b'a'..=b'f' => {},
                b'0'..=b'9' => {},
                _ => return None
            }
        }
        Some(s)
    } else {
        None
    }
}

fn parse_passport_id(s: &str) -> Option<&str> {
    if s.len() == 9 {
        for c in s.bytes().skip(1) {
            match c {
                b'0'..=b'9' => {},
                _ => return None
            }
        }
        Some(s)
    } else {
        None
    }
}

fn parse_eye_color<'a>(colors: &HashSet<&str>, s: &'a str) -> Option<&'a str> {
    colors.contains(s).then(|| s) 
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let passports = load(include_str!("./passports.txt"), false);
    let valid = passports.iter().filter(|p| p.is_valid()).count();
    assert_eq!(valid, 196);

    let passports = load(include_str!("./passports.txt"), true);
    let valid = passports.iter().filter(|p| p.is_valid()).count();
    assert_eq!(valid, 114);
  }
}