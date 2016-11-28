use std::fmt;
use super::Expr;
use super::operator::Operator;
use super::integer::Integer;

pub enum Factor {
    Expr(Box<Expr>),
    Integer(Integer),
}

impl Factor {
    pub fn new() -> Factor {
        Factor::Integer(Integer::new(0))
    }

    pub fn parse(raw_factor: &str) -> Result<(Factor, &str), String> {
        let mut rest_of_expr = raw_factor.trim_left();

        match Operator::parse(rest_of_expr) { // lookahead

            // Is a Bracket Expression (expect a left bracket)
            Ok((Operator::LeftBracket, dirty_factor)) => {
                
                rest_of_expr = dirty_factor;
                let factor = match Expr::parse(rest_of_expr) {
                    Ok((expr, dirty_factor)) => {
                        rest_of_expr = dirty_factor;
                        Factor::Expr(Box::new(expr))
                    },
                    Err(err) => {
                        return Err(err);
                    },
                };

                match Operator::parse(rest_of_expr) {
                    Ok((Operator::RightBracket, dirty_factor)) => {
                        rest_of_expr = dirty_factor;
                    },
                    Ok((operator, _)) => {
                        return Err(format!("Expect a ')' rather than '{}'.", operator));
                    },
                    Err(err) =>
                        { return Err(err); },
                };

                Ok((factor, rest_of_expr))
            },

            // Is an Integer
            Ok(_) | Err(_) => {
                match Integer::parse(rest_of_expr) {
                    Ok((integer, dirty_factor)) => {
                        return Ok((Factor::Integer(integer), dirty_factor));
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
        }
    }

    pub fn eval(&self) -> Result<i32, &str> {
        match self {
            &Factor::Expr(ref expr) => expr.eval(),
            &Factor::Integer(ref integer) => integer.eval(),
        }
    }
}

impl fmt::Display for Factor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Factor::Expr(ref expr) => write!(f, "( {} )", expr),
            &Factor::Integer(ref integer) => write!(f, "{}", integer),
        }
    }
}