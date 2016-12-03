use std::fmt;
use super::factor::Factor;
use super::operator::Operator;

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

    pub fn parse(raw_term: &str) -> Option<(Term, &str)> {
        let mut term = Term::new();
        let mut rest_of_expr = raw_term.trim_left();

        if let None = Factor::parse(rest_of_expr).map(|(factor, dirty_term)| {
            term.factor = Box::new(factor);
            rest_of_expr = dirty_term;
        }) { return None; };

        match Operator::parse(rest_of_expr) {
            Some((operator @ Operator::Multiplication, dirty_term)) => {
                term.operator = Some(operator);
                rest_of_expr = dirty_term;
            },
            Some((operator @ Operator::Division, dirty_term)) => {
                term.operator = Some(operator);
                rest_of_expr = dirty_term;
            },
            _ => return Some((term, rest_of_expr)),
        };

        if let None = Term::parse(rest_of_expr).map(|(subterm, dirty_term)| {
            term.subterm = Some(Box::new(subterm));
            rest_of_expr = dirty_term;
        }) { return None; };

        Some((term, rest_of_expr))
    }

    pub fn eval(&self) -> Option<i32> {
        let result = match self.factor.eval() {
            Some(val) => val,
            None => return None,
        };
        
        if self.operator.is_none() || self.subterm.is_none() {
            return Some(result);
        }

        let times = match self.subterm {
            Some(ref term) => match term.eval() {
                Some(val) => val,
                None => return None,
            },
            None => unreachable!(),
        };

        let (result, is_overflow) = match self.operator {
            Some(Operator::Multiplication) => result.overflowing_mul(times),
            Some(Operator::Division) => {
                if times == 0 { return None; }
                result.overflowing_div(times)
            }
            _ => unreachable!(),
        };

        if is_overflow { None } else { Some(result) }
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