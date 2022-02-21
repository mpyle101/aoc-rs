use std::fmt;
use std::str::Chars;

#[derive(Clone, Debug)]
enum Value {
    Null,
    Regular(u32),
    Complex(Box<Number>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Null       => write!(f, ""),
            Value::Regular(n) => write!(f, "{}", n),
            Value::Complex(n) => write!(f, "{}", n)
        }
    }
}

#[derive(Clone, Debug)]
pub struct Number {
    left:  Value,
    right: Value,
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},{}]", self.left, self.right)
    }
}

impl Number {
    pub fn new(chars: &mut Chars) -> Number {
        Number {
            left:  read_value(chars),
            right: read_value(chars),
        }    
    }

    pub fn magnitude(&self) -> i64 {
        let ml = 3 * match &self.left {
            Value::Regular(n) => *n as i64,
            Value::Complex(n) => (*n).magnitude(),
            Value::Null => panic!("Null found on left"),
        };
        let mr = 2 * match &self.right {
            Value::Regular(n) => *n as i64,
            Value::Complex(n) => (*n).magnitude(),
            Value::Null => panic!("Null found on right"),
        };

        ml + mr
    }
}

fn read_value(chars: &mut Chars) -> Value {
    if let Some(c) = chars.next() {
        if c == '[' {
            Value::Complex(Box::from(Number::new(chars)))
        } else if c == ',' || c == ']' {
            read_value(chars)
        } else {
            Value::Regular(c.to_digit(10).unwrap())
        }
    } else {
        Value::Null
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let s = "[1,1]";
        let n = Number::new(&mut s[1..].chars());
        assert_eq!(format!("{}", n), s);

        let s = "[[8,8],7]";
        let n = Number::new(&mut s[1..].chars());
        assert_eq!(format!("{}", n), s);

        let s = "[7,[[4,3],[8,5]]]";
        let n = Number::new(&mut s[1..].chars());
        assert_eq!(format!("{}", n), s);

        let s = "[[[[0,0],2],9],[[[2,1],1],[5,[4,7]]]]";
        let n = Number::new(&mut s[1..].chars());
        assert_eq!(format!("{}", n), s);

        let s = "[[[[8,8],[6,7]],[[1,0],6]],[[5,[2,8]],[[8,0],[3,7]]]]";
        let n = Number::new(&mut s[1..].chars());
        assert_eq!(format!("{}", n), s);

        let s = "[[[[9,8],[4,6]],[7,[9,1]]],[[[8,7],[4,7]],[[6,6],[8,1]]]]";
        let n = Number::new(&mut s[1..].chars());
        assert_eq!(format!("{}", n), s);
    }
}