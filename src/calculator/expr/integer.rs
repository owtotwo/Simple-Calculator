use std::fmt;
use super::operator::Operator;

pub struct Integer(i32);

impl Integer {
    pub fn new(n: i32) -> Integer {
        Integer(n)
    }

    pub fn parse(raw_int: &str) -> Result<(Integer, &str), &str> {
        let mut number: i64 = 0;
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
            Ok(_) | Err(_) => {
                is_neg = false;
            }
        };

        let mut move_count: usize = 0;

        for c in rest_of_expr.chars() {
            let digit = match c.to_digit(10) {
                Some(digit) => digit as i64,
                None => { break; },
            };
            number = number * 10 + digit;
            if !is_neg && number > i32::max_value() as i64 ||
                    is_neg && number < i32::min_value() as i64 {
                return Err("Parse Integer Overflow!");
            }
            move_count += 1;
        }

        if move_count == 0 {
            return Err("Expect a digit!");
        } else {
            rest_of_expr = &rest_of_expr[move_count..];
        }

        let integer = Integer::new(
            if is_neg { -(number as i64) as i32 } else { number as i32 }
        );

        Ok((integer, rest_of_expr))
    }

    pub fn eval(&self) -> Result<i32, &str> {
        Ok(self.0)
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}