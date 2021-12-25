fn main() {
    use std::fs;
    use std::time::Instant;
    
    let input = fs::read_to_string("./input.txt").unwrap();
    let cmds = load(&input);

    let t1 = Instant::now();
    let model = part_one(&cmds);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", model, t2 - t1);
}

fn load(input: &str) -> Vec<Cmd> {
    use Cmd::*;

    input.lines().map(|s| {
        let mut it = s.split(' ');
        let cmd = it.next().unwrap();
        let c = it.next().unwrap().chars().nth(0).unwrap();
        match cmd {
            "inp" => Inp(c),
            "add" => Add(c, parse(it.next())),
            "mul" => Mul(c, parse(it.next())),
            "div" => Div(c, parse(it.next())),
            "mod" => Mod(c, parse(it.next())),
            "eql" => Eql(c, parse(it.next())),
                _ => panic!("Unknown command: {}", cmd),
        }
    })
    .collect()
}

fn parse<'a >(opt: Option<&'a str>) -> Value {
    let s = opt.unwrap();
    s.parse().map_or_else(
        |_| Value::Variable(s.chars().nth(0).unwrap()),
        |v| Value::Number(v),
    )
}

fn part_one(cmds: &[Cmd]) -> i64 {
    let mut monad = Monad::new(cmds);

    let mut n = 999;
    let mut digits = [9,9,9,9,1,9,5,9,9,9,7,9,9,4];
    while monad.run(&digits) != 0 && n > 111 {
        n -= 1;
        digits[0] = n / 100;
        digits[1] = (n / 10) % 10;
        digits[2] = n % 10;
    }
    
    digits.iter().fold(0, |v, n| v * 10 + n)
}

#[allow(dead_code)]
fn is_valid(n: u64) -> bool {
    let mut i = 0;
    let mut v = n;
    let mut digits = [0u8;14];
    while v > 0 {
        let n = (v % 10) as u8;
        v /= 10;
        if n == 0 { return false }
        digits[13 - i] = n;
        i += 1;
    }

    digits[4]  == digits[3] - 8  &&
    digits[6]  == digits[5] - 4  &&
    digits[7]  == digits[6]      &&
    digits[9]  == digits[8]      &&
    digits[11] == digits[10] + 2 &&
    digits[13] == digits[12] - 5
}

#[derive(Clone, Debug)]
enum Value {
    Number(i64),
    Variable(char),
}

#[derive(Clone, Debug)]
enum Cmd {
    Inp(char),
    Add(char, Value),
    Mul(char, Value),
    Div(char, Value),
    Mod(char, Value),
    Eql(char, Value),
}

struct Monad {
    cmds: Vec<Cmd>,
    vars: [i64;4],
}

impl Monad {
    fn new(cmds: &[Cmd]) -> Monad {
        Monad {
            cmds: cmds.to_vec(),
            vars: [0;4],
        }
    }

    #[allow(dead_code)]
    fn run(&mut self, model: &[i64;14]) -> i64 {
        self.execute(model, false)
    }

    #[allow(dead_code)]
    fn debug(&mut self, model: &[i64;14]) -> i64 {
        self.execute(model, true)
    }

    fn execute(&mut self, model: &[i64;14], debug: bool) -> i64 {
        use Cmd::*;

        let mut i = 0;

        self.vars = [0;4];

        let cmds = self.cmds.iter().cloned().collect::<Vec<_>>();
        cmds.iter().for_each(|cmd| {
            match cmd {
                Inp(a) => {
                    self.setv(a, model[i]);
                    i += 1
                },
                Add(a, b) => {
                    let n = self.getv(a);
                    self.setv(a, n + self.val(b))
                },
                Mul(a, b) => {
                    let n = self.getv(a);
                    self.setv(a, n * self.val(b))
                },
                Div(a, b) => {
                    let n = self.getv(a);
                    self.setv(a, n / self.val(b))
                },
                Mod(a, b) => {
                    let n = self.getv(a);
                    self.setv(a, n % self.val(b))
                },
                Eql(a, b) => {
                    let n = self.getv(a);
                    self.setv(a, (n == self.val(b)) as i64)
                },
            };

            if debug { println!("{:?} => {:?}", cmd, self.vars); }
        });

        self.getv(&'z')
    }

    #[allow(dead_code)]
    fn result(&self) -> i64 {
        self.getv(&'z')
    }

    fn val(&self, val: &Value) -> i64 {
        match val {
            Value::Number(n)   => *n,
            Value::Variable(c) => self.getv(c),
        }
    }

    fn getv(&self, var: &char) -> i64 {
        let i = *var as usize - 'w' as usize;
        self.vars[i]
    }

    fn setv(&mut self, var: &char, val: i64) {
        let i = *var as usize - 'w' as usize;
        self.vars[i] = val;
    }
}
