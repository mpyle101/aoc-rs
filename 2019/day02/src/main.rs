use itertools::Itertools;

fn main() {
    let program = include_str!("./program.txt")
      .split(',')
      .map(|s| s.parse::<usize>())
      .map(Result::unwrap)
      .collect::<Vec<_>>();

    let (_, n, v) = (0..=99).permutations(2)
      .map(|v| {
        let noun = v[0];
        let verb = v[1];
        let result = exec(&program, noun, verb);
        (result[0], noun, verb)
      }
    ).find(|&r| r.0 == 19690720).unwrap();

    let result = exec(&program, n, v);
    println!("{}", result[0]);

    println!("Result: {}", n * 100 + v);
}

fn exec(program: &[usize], noun: usize, verb: usize) -> Vec<usize> {
    let mut p = program.to_owned();
    p[1] = noun;
    p[2] = verb;

    let mut ip = 0;
    while ip < p.len() {
        match p[ip] {
            1 => {
                let pos = p[ip + 1];
                let a   = p[pos];
                let pos = p[ip + 2];
                let b   = p[pos];
                let pos = p[ip + 3];
                p[pos] = a + b;
            },
            2  => {
                let pos = p[ip + 1];
                let a   = p[pos];
                let pos = p[ip + 2];
                let b   = p[pos];
                let pos = p[ip + 3];
                p[pos] = a * b;
            },
            99 => break,
            _  => panic!("Unknown opcode encountered: {}", p[ip]), 
        }

        ip += 4;
    }

    p
}


/** Unit Tests */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works1() {
        let program = vec![1,9,10,3,2,3,11,0,99,30,40,50];
        let result  = exec(&program, 9, 10);
        assert_eq!(result, vec![3500,9,10,70,2,3,11,0,99,30,40,50]);
    }

    #[test]
    fn it_works2() {
        let program = vec![1,0,0,0,99];
        let result  = exec(&program, 0, 0);
        assert_eq!(result, vec![2,0,0,0,99]);
    }

    #[test]
    fn it_works3() {
        let program = vec![2,3,0,3,99];
        let result  = exec(&program, 3, 0);
        assert_eq!(result, vec![2,3,0,6,99]);
    }

    #[test]
    fn it_works4() {
        let program = vec![2,4,4,5,99,0];
        let result  = exec(&program, 4, 4);
        assert_eq!(result, vec![2,4,4,5,99,9801]);
    }

    #[test]
    fn it_works5() {
        let program = vec![1,1,1,4,99,5,6,0,99];
        let result  = exec(&program, 1, 1);
        assert_eq!(result, vec![30,1,1,4,2,5,6,0,99]);
    }
}
