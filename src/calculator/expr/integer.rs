use std::fmt;
use std::error;
use std::error::Error;
use super::operator::Operator;
use super::Result;

pub struct Integer(pub i32);

impl Integer {
    pub fn parse(raw_int: &str) -> Result<(Integer, &str)> {
        let mut unsigned: u64 = 0;
        let is_neg: bool;
        let mut rest_of_expr = raw_int.trim_left();
        
        match Operator::parse(rest_of_expr) {
            Ok((Operator::Addition, dirty_expr)) => {
                is_neg = false;
                rest_of_expr = dirty_expr;
            },
            Ok((Operator::Subtraction, dirty_expr)) => {
                is_neg = true;
                rest_of_expr = dirty_expr;
            },
            _ => is_neg = false,
        };

        let mut move_count: usize = 0;

        for c in rest_of_expr.chars() {
            let digit = match c.to_digit(10) {
                Some(digit) => digit as u64,
                None => if move_count == 0 { // first char is not a digit
                    return Err(Box::new(ParseIntError { kind: IntErrorKind::InvalidDigit }));
                } else { // have read some digit and end it for non-digit chars
                    break;
                },
            };
            unsigned = unsigned * 10 + digit;
            if !is_neg && unsigned > i32::max_value() as u64 {
                return Err(Box::new(ParseIntError { kind: IntErrorKind::Overflow }));
            } else if is_neg && unsigned > -(i32::min_value() as i64) as u64 {
                return Err(Box::new(ParseIntError { kind: IntErrorKind::Underflow }));
            }
            move_count += 1;
        }

        if move_count == 0 { // rest_of_expr is empty
            return Err(Box::new(ParseIntError { kind: IntErrorKind::Empty }));
        } else { // success
            rest_of_expr = &rest_of_expr[move_count..];
        }

        let integer = Integer(
            if is_neg { -(unsigned as i64) as i32 } else { unsigned as i32 }
        );

        Ok((integer, rest_of_expr))
    }

    pub fn eval(&self) -> Result<i32> { Ok(self.0) }
}


impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseIntError {
    kind: IntErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum IntErrorKind {
    Empty,
    InvalidDigit,
    Overflow,
    Underflow,
}

impl error::Error for ParseIntError {
    fn description(&self) -> &str {
        match self.kind {
            IntErrorKind::Empty => "cannot parse integer from empty string",
            IntErrorKind::InvalidDigit => "invalid digit found in string",
            IntErrorKind::Overflow => "number too large to fit in 32bit Integer",
            IntErrorKind::Underflow => "number too small to fit in 32bit Integer",
        }
    }
}

impl fmt::Display for ParseIntError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}
