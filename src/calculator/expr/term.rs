use std::fmt;
use super::factor::Factor;
use super::operator::Operator;
use super::Result;
use super::error::{EvalError, EvalErrorKind};

pub struct Term {
    subterm: Option<Box<Term>>,
    factor: Box<Factor>,
    operator: Option<Operator>,
}

impl Term {
    pub fn new() -> Term {
        Term {
            subterm: None,
            factor: Box::new(Factor::new()),
            operator: None,
        }
    }

    pub fn parse(raw_term: &str) -> Result<(Term, &str)> {
        let mut term = Term::new();
        let mut rest_of_expr = raw_term.trim_left();

        Factor::parse(rest_of_expr).map(|(factor, dirty_term)| {
            term.factor = Box::new(factor);
            rest_of_expr = dirty_term;
        }) ?;

        match Operator::parse(rest_of_expr) {
            Ok((operator @ Operator::Multiplication, dirty_term)) => {
                term.operator = Some(operator);
                rest_of_expr = dirty_term;
            },
            Ok((operator @ Operator::Division, dirty_term)) => {
                term.operator = Some(operator);
                rest_of_expr = dirty_term;
            },
            Ok(_) | Err(_) => return Ok((term, rest_of_expr)),
        };

        Term::parse(rest_of_expr).map(|(subterm, dirty_term)| {
            term.subterm = Some(Box::new(subterm));
            rest_of_expr = dirty_term;
        }) ?;

        Ok((term, rest_of_expr))
    }

    pub fn eval(&self) -> Result<i32> {
        let result = self.factor.eval()?;
        
        if self.operator.is_none() || self.subterm.is_none() {
            return Ok(result);
        }

        let times = match self.subterm {
            Some(ref term) => term.eval()?,
            None => unreachable!(),
        };

        let (result, is_overflow) = match self.operator {
            Some(Operator::Multiplication) => result.overflowing_mul(times),
            Some(Operator::Division) => {
                if times == 0 {
                    return Err(Box::new(EvalError { kind: EvalErrorKind::DivideByZero }));
                }
                result.overflowing_div(times)
            }
            _ => unreachable!(),
        };

        if is_overflow {
            Err(Box::new(EvalError { kind: EvalErrorKind::Overflow }))
        } else {
            Ok(result)
        }
    }
}


impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Term {
                subterm: Some(ref subterm),
                ref factor,
                operator: Some(ref operator)
            } =>
                write!(f, "{} {} {}", subterm, operator, factor),
            Term { ref factor, .. } =>
                write!(f, "{}", factor),
        }
    }
}