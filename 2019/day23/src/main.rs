use std::collections::HashMap;
use vm::Vm;

type Network = HashMap<i64, Vm>;
type Packets = HashMap<i64, Vec<(i64, i64)>>;

fn main() {
    let program = include_str!("./nic.txt");

    let y = part_one(program);
    println!("Part 1: {}", y);

    let y = part_two(program);
    println!("Part 2: {}", y);
}

fn part_one(nic: &str) -> i64 {
    let mut packets = HashMap::new();
    let mut network = setup_network(nic);
    loop {
        for vm in network.values_mut() {
            vm.cont().unwrap();
            if let Some(nic) = vm.read() {
                let x = vm.read().unwrap();
                let y = vm.read().unwrap();
                if nic == 255 {
                    return y
                }
                packets.entry(nic).or_insert(Vec::new()).push((x, y));
            }
        };
        process_packets(&mut packets, &mut network);
    }
}

fn part_two(nic: &str) -> i64 {
    let mut nat = (0, 0);
    let mut last_y = -1;
    let mut packets = HashMap::new();
    let mut network = setup_network(nic);
    loop {
        for vm in network.values_mut() {
            vm.cont().unwrap();
            if let Some(nic) = vm.read() {
                let x = vm.read().unwrap();
                let y = vm.read().unwrap();
                if nic == 255 {
                    nat = (x, y);
                } else {
                    packets.entry(nic).or_insert(Vec::new()).push((x, y));
                }
            }
        };
        if packets.len() == 0 {
            if last_y == nat.1 {
                return last_y
            } else {
                last_y = nat.1
            }

            let vm = network.get_mut(&0).unwrap();
            vm.write(nat.0);
            vm.write(nat.1);
        }
        process_packets(&mut packets, &mut network);
    }
}

fn setup_network(nic: &str) -> Network {
    (0..50)
        .map(|n| (n, Vm::new(nic).unwrap()))
        .map(|(n, mut vm)| {
            vm.exec().unwrap();
            vm.write(n);
            (n, vm)
        })
        .collect()
}

fn process_packets(packets: &mut Packets, network: &mut Network) {
    network.iter_mut().for_each(|(nic, vm)| {
        match packets.remove(nic) {
            Some(v) => v.iter().for_each(|&(x, y)| {
                vm.write(x);
                vm.write(y);
            }),
            None => vm.write(-1),
        }
    });
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let program = include_str!("./nic.txt");

    let y = part_one(program);
    assert_eq!(y, 21160);

    let y = part_two(program);
    assert_eq!(y, 14327);
  }
}