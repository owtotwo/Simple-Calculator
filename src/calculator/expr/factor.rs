use std::fmt;
use super::{Expr, Result};
use super::operator::Operator;
use super::integer::Integer;


pub enum Factor {
    Expr(Box<Expr>),
    Integer(Integer),
}


impl Factor {
    pub fn new() -> Factor {
        Factor::Integer(Integer(0))
    }

    pub fn parse(raw_factor: &str) -> Result<(Factor, &str)> {
        let mut rest_of_expr = raw_factor.trim_left();
        let is_integer;

        match Operator::parse(rest_of_expr) { // lookahead
            Ok((Operator::LeftBracket, dirty_factor)) => {
                rest_of_expr = dirty_factor;
                is_integer = false;
            },
            _ => is_integer = true,
        }

        if is_integer {

            Integer::parse(rest_of_expr).map(|(integer, dirty_factor)|
                (Factor::Integer(integer), dirty_factor)
            )

        } else { // Is a Bracket Expression (expect a left bracket)

            let factor = match Expr::parse(rest_of_expr) {
                Ok((expr, dirty_factor)) => {
                    rest_of_expr = dirty_factor;
                    Factor::Expr(Box::new(expr))
                },
                _ => return Err(Box::new(ParseFactorError)),
            };

            match Operator::parse(rest_of_expr) {
                Ok((Operator::RightBracket, dirty_factor)) => {
                    rest_of_expr = dirty_factor;
                },
                _ => return Err(Box::new(ParseFactorError)),
            };

            Ok((factor, rest_of_expr))
        }
    }

    pub fn eval(&self) -> Result<i32> {
        match *self {
            Factor::Expr(ref expr) => expr.eval(),
            Factor::Integer(ref integer) => integer.eval(),
        }
    }
}


impl fmt::Display for Factor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Factor::Expr(ref expr) => write!(f, "({})", expr),
            Factor::Integer(ref integer) => write!(f, "{}", integer),
        }
    }
}

use std::error::Error;
new_error_type!(ParseFactorError);