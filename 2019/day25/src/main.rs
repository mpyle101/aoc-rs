// Password: 8462464

fn main() {
  use std::io;
  use vm::Vm;

  let program = include_str!("./program.txt");
  let mut vm = Vm::new(program).unwrap();
  vm.exec().unwrap();

  loop {
    let prompt: String = vm.drain().iter().map(|v| (*v as u8) as char).collect();
    print!("{}", prompt);

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let cmd = match buf.as_str() {
      "n\n" => "north\n",
      "s\n" => "south\n",
      "e\n" => "east\n",
      "w\n" => "west\n",
      "q\n" => break,
      "quit\n" => break,
      _ => &buf
    };
    cmd.as_bytes().iter().for_each(|&b| vm.write(b));
    vm.cont().unwrap();
  };
}
