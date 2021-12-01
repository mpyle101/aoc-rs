use indextree::{Arena, NodeId};

fn main() {
    use std::time::Instant;

    let (arena, root) = load(include_str!("./input.txt"));

    let t1 = Instant::now();
    let meta = part_one(&arena);
    let t2 = Instant::now();
    println!("Part 1: {}  ({:?})", meta, t2 - t1);

    let t1 = Instant::now();
    let value = part_two(&root, &arena);
    let t2 = Instant::now();
    println!("Part 2: {}  ({:?})", value, t2 - t1);
}

fn part_one(arena: &Arena<Meta>) -> u32 {
    arena.iter().map(|n| n.get().sum()).sum()
}

fn part_two(root: &NodeId, arena: &Arena<Meta>) -> u32 {
    calc_node(root, arena)
}

fn load(input: &str) -> (Arena<Meta>, NodeId) {
    let mut arena = Arena::new();

    let mut iter = input.split(' ').map(|s| s.parse::<u32>().unwrap());
    let root = add_node(&mut arena, &mut iter);

    (arena, root)
}

fn add_node(arena: &mut Arena<Meta>, iter: &mut impl Iterator<Item = u32>) -> NodeId {
    let nid = arena.new_node(Meta::new());
    let children = iter.next().unwrap();
    let metadata = iter.next().unwrap();

    (0..children).for_each(|_| nid.append(add_node(arena, iter), arena));
    let node = arena.get_mut(nid).unwrap();
    let meta = node.get_mut();
    (0..metadata).for_each(|_| meta.push(iter.next().unwrap()));

    nid
}

fn calc_node(nid: &NodeId, arena: &Arena<Meta>) -> u32 {
    let children: Vec<_> = nid.children(arena).collect();
    if children.len() == 0 {
        arena.get(*nid).unwrap().get().sum()
    } else {
        let node = arena.get(*nid).unwrap();
        let meta = node.get();
        meta.iter().map(|&n| {
            if n == 0 || n > children.len() as u32 {
                0
            } else {
                calc_node(&children[n as usize - 1], arena)
            }
        })
        .sum()
    }
}

struct Meta {
    data: Vec<u32>,
}

impl Meta {
    fn new() -> Meta {
        Meta { data: Vec::new() }
    }

    fn push(&mut self, val: u32) {
        self.data.push(val)
    }

    fn iter(&self) -> impl Iterator<Item = &u32> {
        self.data.iter()
    }

    fn sum(&self) -> u32 {
        self.data.iter().sum()
    }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let (arena, root) = load(include_str!("./input.txt"));

    let meta = part_one(&arena);
    assert_eq!(meta, 48155);

    let value = part_two(&root, &arena);
    assert_eq!(value, 40292);
  }
}
