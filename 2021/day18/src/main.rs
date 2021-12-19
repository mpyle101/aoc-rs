use lazy_static::lazy_static;
use regex::Regex;

mod number;
use number::Number;

lazy_static! {
    static ref RE1: Regex = Regex::new(concat!(
        r"\[(?P<a>[0-9]+),(?P<b>[0-9]+)\]",
    )).unwrap();
    static ref RE2: Regex = Regex::new(r"[0-9]+").unwrap();
}

fn main() {
    use std::fs;
    use std::time::Instant;
    
    let input = fs::read_to_string("./input.txt").unwrap();
    let numbers = load(&input);

    let t1 = Instant::now();
    let magnitude = part_one(&numbers);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", magnitude, t2 - t1);

    let t1 = Instant::now();
    let magnitude = part_two(&numbers);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", magnitude, t2 - t1);
}

fn load(input: &str) -> Vec<&str> {
    input.lines().collect::<Vec<_>>()
}

fn part_one(nums: &[&str]) -> i64 {
    let mut it = nums.iter();
    let first = it.next().unwrap();
    let s = it.fold(first.to_string(), |a, b| add(&a, b));

    magnitude(&s)
}

fn part_two(nums: &[&str]) -> i64 {
    use itertools::Itertools;

    nums.iter().permutations(2).map(|v| {
        magnitude(&add(v[0], v[1]))
    }).max().unwrap()
}

fn add(a: &str, b: &str) -> String {
    reduce(&format!("[{},{}]", a, b))
}

fn magnitude(s: &String) -> i64 {
    let n = Number::new(&mut s[1..].chars());
    n.magnitude()
}

fn reduce(s: &String) -> String {
    let mut sc = s.clone();
    while let Some(n) = explode(&sc).or(split(&sc)) {
        sc = n
    }

    sc
}

fn explode(s: &String) -> Option<String> {
    // We have to play the games below because regex doesn't
    // support overlapping matches so we can't include the
    // regular numbers to the left and right in the re.
    for cap in RE1.captures_iter(&s) {
        let m0 = &cap.get(0).unwrap();
        if get_depth(&s.as_str()[..m0.start()]) == 4 {
            let mut sc = s.clone();

            // Shrapnel to the right
            if let Some(m) = RE2.find(&s.as_str()[m0.end()..]) {
                let b = cap["b"].parse::<u32>().unwrap();
                let n = m.as_str().parse::<u32>().unwrap();
                let i = m.start() + m0.end();
                let j = m.end() + m0.end();
                sc.replace_range(i..j, &(n+b).to_string());
            }
            
            // Replace pair with 0
            sc.replace_range(m0.range(), "0");

            // Shrapnel to the left (search backwards)
            let r = s.as_str()[..m0.start()].chars().rev().collect::<String>();
            if let Some(m) = RE2.find(&r) {
                let v = m.as_str().chars().rev().collect::<String>();
                let b = cap["a"].parse::<u32>().unwrap();
                let n = v.parse::<u32>().unwrap();
                let i = m0.start() - m.end();
                let j = m0.start() - m.start();
                sc.replace_range(i..j, &(n+b).to_string());
            }

            return Some(sc)
        }
    }
    
    None
}

fn split(s: &String) -> Option<String> {
    for m in RE2.find_iter(s) {
        if m.as_str().len() > 1 {
            let n = m.as_str().parse::<u32>().unwrap();
            let a = n / 2;
            let b = (n + 1) / 2;
            let mut s2 = s.clone();
            s2.replace_range(m.start()..m.end(), &format!("[{},{}]", a, b));
            return Some(s2)
        }
    }

    None
}

fn get_depth(s: &str) -> u32 {
    s.chars().fold(0, |d, c| d + match c { '[' => 1, ']' => -1, _ => 0 }) as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let numbers = load(&input);

        let magnitude = part_one(&numbers);
        assert_eq!(magnitude, 2501);

        let magnitude = part_two(&numbers);
        assert_eq!(magnitude, 4935);
    }

    #[test]
    fn exploding() {
        let s = explode(&"[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".to_string()).unwrap();
        assert_eq!(s, "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]");

        let s = explode(&"[[[[0,7],4],[7,[[8,4],9]]],[1,1]]".to_string()).unwrap();
        assert_eq!(s, "[[[[0,7],4],[15,[0,13]]],[1,1]]");
                
        let s = explode(&"[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]".to_string()).unwrap();
        assert_eq!(s, "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
                
        let s = explode(&"[[[[[9,8],1],2],3],4]".to_string()).unwrap();
        assert_eq!(s, "[[[[0,9],2],3],4]");
        
        let s = explode(&"[7,[6,[5,[4,[3,2]]]]]".to_string()).unwrap();
        assert_eq!(s, "[7,[6,[5,[7,0]]]]");

        let s = explode(&"[[6,[5,[4,[3,2]]]],1]".to_string()).unwrap();
        assert_eq!(s, "[[6,[5,[7,0]]],3]");

        let s = explode(&"[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".to_string()).unwrap();
        assert_eq!(s, "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");

        let s = explode(&"[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".to_string()).unwrap();
        assert_eq!(s, "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");

        let s = explode(&"[[[[0,7],4],[[7,8],[0,13]]],[1,1]]".to_string());
        assert_eq!(s, None);
    }

    #[test]
    fn splitting() {
        let s = split(&"[[[[0,7],4],[15,[0,13]]],[1,1]]".to_string()).unwrap();
        assert_eq!(s, "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
        
        let s = split(&"[[[[0,7],4],[[7,8],[0,13]]],[1,1]]".to_string()).unwrap();
        assert_eq!(s, "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");
    }

    #[test]
    fn reducing() {
        let s = reduce(&"[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".to_string());
        assert_eq!(s, "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn adding() {
        let a = "[[[[4,3],4],4],[7,[[8,4],9]]]";
        let b = "[1,1]";
        let n = add(a, b);
        assert_eq!(n, "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");

        let v = ["[2,2]", "[3,3]", "[4,4]"];
        let n = v.iter().fold("[1,1]".to_string(), |a, b| add(&a, b));
        assert_eq!(n, "[[[[1,1],[2,2]],[3,3]],[4,4]]");

        let v = ["[2,2]", "[3,3]", "[4,4]", "[5,5]"];
        let n = v.iter().fold("[1,1]".to_string(), |a, b| add(&a, b));
        assert_eq!(n, "[[[[3,0],[5,3]],[4,4]],[5,5]]");

        let v = ["[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"];
        let n = v.iter().fold("[1,1]".to_string(), |a, b| add(&a, b));
        assert_eq!(n, "[[[[5,0],[7,4]],[5,5]],[6,6]]");

        let a = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]";
        let n = [
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[2,9]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[4,2],2],6],[8,7]]",
        ].iter().fold(a.to_string(), |a, b| add(&a, b));
        assert_eq!(n, "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
    }
}