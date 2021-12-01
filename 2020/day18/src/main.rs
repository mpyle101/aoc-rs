
fn main() {
    let exprs = include_str!("./expr.txt");

    let val = part_one(exprs);
    println!("Part 1: {}", val);

    let val = part_two(exprs);
    println!("Part 2: {}", val);
}

fn part_one(input: &str) -> i64 {
    input.lines().map(parser::eval1).map(Result::unwrap).sum()
}

fn part_two(input: &str) -> i64 {
    input.lines().map(parser::eval2).map(Result::unwrap).sum()
}

peg::parser!( grammar parser() for str {
    pub rule eval1() -> i64 = precedence!{
        x:(@) ws() "+" ws() y:@ { x + y }
        x:(@) ws() "*" ws() y:@ { x * y }
        --
        n:number() { n }
        "(" e:eval1() ")" { e }
    }

    pub rule eval2() -> i64 = precedence!{
        x:(@) ws() "*" ws() y:@ { x * y }
        --
        x:(@) ws() "+" ws() y:@ { x + y }
        --
        n:number() { n }
        "(" e:eval2() ")" { e }
    }

    rule number() -> i64
        = n:$(['0'..='9']+) { n.parse().unwrap() }

    rule ws() = [' ']*
});


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let exprs = include_str!("./expr.txt");

    let val = part_one(exprs);
    assert_eq!(val, 36382392389406);

    let val = part_two(exprs);
    assert_eq!(val, 381107029777968);
  }
}