fn main() {
    use std::fs;
    use std::time::Instant;
    
    let input = fs::read_to_string("./input.txt").unwrap();
    let cmds = load(&input);

    let t1 = Instant::now();
    let model = part_one(&cmds);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", model, t2 - t1);

    let t1 = Instant::now();
    let model = part_two(&cmds);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", model, t2 - t1);
}

fn load(input: &str) -> Vec<Cmd> {
    use Cmd::*;

    input.lines().map(|s| {
        let mut it = s.split(' ');
        let cmd = it.next().unwrap();
        let c = it.next().unwrap().chars().next().unwrap();
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

fn parse(opt: Option<&str>) -> Value {
    let s = opt.unwrap();
    s.parse().map_or_else(
        |_| Value::Variable(s.chars().next().unwrap()),
        Value::Number,
    )
}

fn part_one(cmds: &[Cmd]) -> i64 {
    // Based on the rules we can determine the max values
    // for most inputs so we just iterate down from 99 to
    // 11 for the two unknown values til we find one resulting
    // in zero.
    //
    // W's are inputs not array location (so "off-by-one")
    //  w5 == w4 - 8  => w5  = 1, w4 = 9
    //  w7 == w6 - 4  => w7  = 5, w6 = 9
    //  w8 == w3 + 5  => w8  = 9, w3 = 4
    // w10 == w9      => w10 = 9, w9 = 9
    // w13 == w2 + 1  => w13 = 9, w2 = 8
    // w14 == w1 - 5  => w14 = 4, w1 = 9
    //
    // 9849195999XX94

    let mut monad = Monad::new(cmds);

    let mut n = 99;
    let mut digits = [9,8,4,9,1,9,5,9,9,9,0,0,9,4];

    digits[10] = n / 10;
    digits[11] = n % 10;
    while monad.run(&digits) != 0 && n > 10 {
        n -= 1;
        digits[10] = n / 10;
        digits[11] = n % 10;
    }
    
    digits.iter().fold(0, |v, n| v * 10 + n)
}

fn part_two(cmds: &[Cmd]) -> i64 {
    // Now use the rules to determine the min values.
    //
    // W's are inputs not array locations (ie "off-by-one")
    //  w5 == w4 - 8  => w4  = 9, w5 = 1
    //  w7 == w6 - 4  => w7  = 1, w6 = 5
    //  w8 == w3 + 5  => w3  = 1, w8 = 6
    // w10 == w9      => w10 = 1, w9 = 1
    // w13 == w2 + 1  => w13 = 2, w2 = 1
    // w14 == w1 - 5  => w14 = 1, w1 = 6

    // 6119151611XX21

    let mut monad = Monad::new(cmds);

    let mut n = 11;
    let mut digits = [6,1,1,9,1,5,1,6,1,1,0,0,2,1];

    digits[10] = n / 10;
    digits[11] = n % 10;
    while monad.run(&digits) != 0 && n < 100 {
        n += 1;
        digits[10] = n / 10;
        digits[11] = n % 10;
    }
    
    digits.iter().fold(0, |v, n| v * 10 + n)
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

        let cmds = self.cmds.to_vec();
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
