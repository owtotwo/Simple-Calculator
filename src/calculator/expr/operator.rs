use std::fmt;

pub enum Operator {
    Addition,       // "+"
    Subtraction,    // "-"
    Multiplication, // "*"
    Division,       // "/"
    LeftBracket,    // "("
    RightBracket,   // ")"
}

impl Operator {
    pub fn parse(raw_expr: &str) -> Result<(Operator, &str), &str> {
        match raw_expr.trim_left() {
            s if s.starts_with("+") => Ok((Operator::Addition,       &s[1..])),
            s if s.starts_with("-") => Ok((Operator::Subtraction,    &s[1..])),
            s if s.starts_with("*") => Ok((Operator::Multiplication, &s[1..])),
            s if s.starts_with("/") => Ok((Operator::Division,       &s[1..])),
            s if s.starts_with("(") => Ok((Operator::LeftBracket,    &s[1..])),
            s if s.starts_with(")") => Ok((Operator::RightBracket,   &s[1..])),
            _ => Err("Expect '+', '-', '*', '/', '(' or ')'."),
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            &Operator::Addition => '+',
            &Operator::Subtraction => '-',
            &Operator::Multiplication => '*',
            &Operator::Division => '/',
            &Operator::LeftBracket => '(',
            &Operator::RightBracket => ')',
        })
    }
}
