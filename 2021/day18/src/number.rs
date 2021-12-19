use std::fmt;
use std::str::Chars;

#[derive(Clone, Debug)]
enum NumberType {
    Null,
    Regular(u32),
    Complex(Box<Number>),
}

impl fmt::Display for NumberType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use NumberType::*;
        match self {
            Null       => write!(f, ""),
            Regular(n) => write!(f, "{}", n),
            Complex(n) => write!(f, "{}", n)
        }
    }
}

#[derive(Clone, Debug)]
pub struct Number {
    left:  NumberType,
    right: NumberType,
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
        use NumberType::*;

        let ml = 3 * match &self.left {
            Regular(n) => *n as i64,
            Complex(n) => (*n).magnitude(),
            Null => panic!("Null number found on left"),
        };
        let mr = 2 * match &self.right {
            Regular(n) => *n as i64,
            Complex(n) => (*n).magnitude(),
            Null => panic!("Null number found on right"),
        };

        ml + mr
    }
}

fn read_value(chars: &mut Chars) -> NumberType {
    use NumberType::*;

    if let Some(c) = chars.next() {
        if c == '[' {
            Complex(Box::from(Number::new(chars)))
        } else if c == ',' {
            read_value(chars)
        } else if c == ']' {
            read_value(chars)
        } else {
            Regular(c.to_digit(10).unwrap())
        }
    } else {
        Null
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