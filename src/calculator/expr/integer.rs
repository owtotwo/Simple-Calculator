use std::fmt;
use super::operator::Operator;

pub struct Integer(pub i32);

impl Integer {
    pub fn parse(raw_int: &str) -> Option<(Integer, &str)> {
        let mut unsigned: u64 = 0;
        let is_neg: bool;
        let mut rest_of_expr = raw_int.trim_left();
        
        match Operator::parse(rest_of_expr) {
            Some((Operator::Addition, dirty_expr)) => {
                is_neg = false;
                rest_of_expr = dirty_expr;
            },
            Some((Operator::Subtraction, dirty_expr)) => {
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
                    return None;
                } else { // have read some digit and end it for non-digit chars
                    break;
                },
            };
            unsigned = unsigned * 10 + digit;
            if !is_neg && unsigned > i32::max_value() as u64 || 
                is_neg && unsigned > -(i32::min_value() as i64) as u64 {
                return None;
            }
            move_count += 1;
        }

        if move_count == 0 { // rest_of_expr is empty
            return None;
        } else { // success
            rest_of_expr = &rest_of_expr[move_count..];
        }

        let integer = Integer(
            if is_neg { -(unsigned as i64) as i32 } else { unsigned as i32 }
        );

        Some((integer, rest_of_expr))
    }

    pub fn eval(&self) -> Option<i32> { Some(self.0) }
}


impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
