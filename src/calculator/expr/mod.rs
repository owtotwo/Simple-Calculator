mod operator;
mod term;
mod factor;
mod integer;

use std::fmt;
use self::term::Term;
use self::operator::Operator;

pub struct Expr {
    subexpr: Option<Box<Expr>>,
    term: Box<Term>,
    operator: Option<Operator>,
}

impl Expr {
    pub fn new() -> Expr {
        Expr {
            subexpr: None,
            term: Box::new(Term::new()),
            operator: None,
        }
    }

    pub fn parse(raw_expr: &str) -> Result<(Expr, &str), String> {
        let mut expr = Expr::new();
        let mut rest_of_expr = raw_expr.trim_left();

        match Term::parse(rest_of_expr) {
            Err(err) => { return Err(err) },
            Ok((term, dirty_expr)) => {
                expr.term = Box::new(term);
                rest_of_expr = dirty_expr;
            }
        };

        match Operator::parse(rest_of_expr) {
            Ok((operator @ Operator::Addition, dirty_expr)) => {
                expr.operator = Some(operator);
                rest_of_expr = dirty_expr;
            },
            Ok((operator @ Operator::Subtraction, dirty_expr)) => {
                expr.operator = Some(operator);
                rest_of_expr = dirty_expr;
            },
            Ok(_) | Err(_) => {
                return Ok((expr, rest_of_expr));
            }
        };

        match Expr::parse(rest_of_expr) {
            Err(err) => { return Err(err) },
            Ok((subexpr, dirty_expr)) => {
                expr.subexpr = Some(Box::new(subexpr));
                rest_of_expr = dirty_expr;
            }
        };

        Ok((expr, rest_of_expr))
    }

    pub fn eval(&self) -> Result<i32, &str> {
        let result = match self.term.eval() {
            Ok(result) => result,
            Err(err) => { return Err(err); }
        };
        
        if self.operator.is_none() || self.subexpr.is_none() {
            return Ok(result);
        }

        let diff_result = match self.subexpr {
            Some(ref expr) => expr.eval(),
            None => unreachable!(),
        };

        let diff = match diff_result {
            Ok(result) => result,
            Err(err) => { return Err(err); }
        };

        let (result, is_overflow) = match self.operator {
            Some(Operator::Addition) => result.overflowing_add(diff),
            Some(Operator::Subtraction) => result.overflowing_sub(diff),
            _ => unreachable!(),
        };

        if is_overflow {
            Err("Evaluation Overflow!")
        } else {
            Ok(result)
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Expr {
                subexpr: Some(ref subexpr),
                ref term,
                operator: Some(ref operator),
            } =>
                write!(f, "{} {} {}", *subexpr, operator, *term),
            &Expr { ref term, .. } =>
                write!(f, "{}", *term),
        }
    }
}
