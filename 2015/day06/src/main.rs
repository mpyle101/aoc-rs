use ndarray::{Array2, SliceInfo, SliceInfoElem, Dim};

fn main() {
    let cmds = load(include_str!("./input.txt"));

    let lights = part_one(&cmds);
    println!("Part 1: {}", lights);

    let brightness = part_two(&cmds);
    println!("Part 2: {}", brightness);
}

fn part_one(cmds: &[Cmd]) -> i32 {
    cmds.iter()
        .map(|cmd| (cmd, cmd.slice()))
        .fold(
            Array2::<u8>::zeros((1000, 1000)),
            |mut m, (cmd, sl)| {
                let mut n = m.slice_mut(sl);
                match cmd {
                    Cmd::On(_)  => { n.iter_mut().for_each(|v| *v = 1); m },
                    Cmd::Off(_) => { n.iter_mut().for_each(|v| *v = 0); m },
                    Cmd::Tog(_) => { n.iter_mut().for_each(|v| *v = (*v + 1) % 2); m },
                }
            }
        )
        .fold(0, |acc, &v| acc + v as i32)
}

fn part_two(cmds: &[Cmd]) -> i32 {
    cmds.iter()
        .map(|cmd| (cmd, cmd.slice()))
        .fold(
            Array2::<u8>::zeros((1000, 1000)),
            |mut m, (cmd, sl)| {
                let mut n = m.slice_mut(sl);
                match cmd {
                    Cmd::On(_)  => { n.iter_mut().for_each(|v| *v += 1); m }
                    Cmd::Tog(_) => { n.iter_mut().for_each(|v| *v += 2); m },
                    Cmd::Off(_) => { n.iter_mut().for_each(|v| if *v > 0 { *v -= 1 }); m },
                }
            }
        )
        .fold(0, |acc, &v| acc + v as i32)
}

type Rect = ((i32, i32), (i32, i32));
type Slice = SliceInfo<[SliceInfoElem; 2], Dim<[usize; 2]>, Dim<[usize; 2]>>;

enum Cmd {
    On(Rect),
    Off(Rect),
    Tog(Rect),
}

impl Cmd {
    fn slice(&self) -> Slice {
        use ndarray::s;

        match self {
            Cmd::On((p1, p2))  => s![p1.0..=p2.0, p1.1..=p2.1],
            Cmd::Off((p1, p2)) => s![p1.0..=p2.0, p1.1..=p2.1],
            Cmd::Tog((p1, p2)) => s![p1.0..=p2.0, p1.1..=p2.1],
        }
    }
}

fn load(input: &str) -> Vec<Cmd> {
    input.lines()
        .map(|line| line.split(' ').collect::<Vec<_>>())
        .map(|v| match v[1] {
            "on"  => Cmd::On(make_rect(v[2], v[4])),
            "off" => Cmd::Off(make_rect(v[2], v[4])),
            _     => Cmd::Tog(make_rect(v[1], v[3])),
        })
        .collect()
}

fn make_rect(pt1: &str, pt2: &str) -> Rect {
    let v1: Vec<_> = pt1.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
    let v2: Vec<_> = pt2.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
    ((v1[0], v1[1]), (v2[0], v2[1]))
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let cmds = load(include_str!("./input.txt"));

    let lights = part_one(&cmds);
    assert_eq!(lights, 543903);

    let brightness = part_two(&cmds);
    assert_eq!(brightness, 14687245);
  }
}
