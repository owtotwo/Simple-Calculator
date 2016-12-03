use std::fmt;
use self::term::Term;
use self::operator::Operator;

mod operator;
mod term;
mod factor;
mod integer;


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

    pub fn parse(raw_expr: &str) -> Option<(Expr, &str)> {
        let mut expr = Expr::new();
        let mut rest_of_expr = raw_expr.trim_left();

        if let None = Term::parse(rest_of_expr).map(|(term, dirty_expr)| {
            expr.term = Box::new(term);
            rest_of_expr = dirty_expr;
        }) { return None; };

        match Operator::parse(rest_of_expr) {
            Some((operator @ Operator::Addition, dirty_expr)) => {
                expr.operator = Some(operator);
                rest_of_expr = dirty_expr;
            },
            Some((operator @ Operator::Subtraction, dirty_expr)) => {
                expr.operator = Some(operator);
                rest_of_expr = dirty_expr;
            },
            _ => return Some((expr, rest_of_expr)),
        };

        if let None = Expr::parse(rest_of_expr).map(|(subexpr, dirty_expr)| {
            expr.subexpr = Some(Box::new(subexpr));
            rest_of_expr = dirty_expr;
        }) { return None; };

        Some((expr, rest_of_expr))
    }

    pub fn eval(&self) -> Option<i32> {
        let result = match self.term.eval() {
            Some(val) => val,
            None => return None,
        };
        
        if self.operator.is_none() || self.subexpr.is_none() {
            return Some(result);
        }

        let diff = match self.subexpr {
            Some(ref expr) => match expr.eval() {
                Some(val) => val,
                None => return None,
            },
            None => unreachable!(),
        };

        let (result, is_overflow) = match self.operator {
            Some(Operator::Addition) => result.overflowing_add(diff),
            Some(Operator::Subtraction) => result.overflowing_sub(diff),
            _ => unreachable!(),
        };

        if is_overflow { None } else { Some(result) }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expr {
                subexpr: Some(ref subexpr),
                ref term,
                operator: Some(ref operator),
            } =>
                write!(f, "{} {} {}", subexpr, operator, term),
            Expr { ref term, .. } =>
                write!(f, "{}", term),
        }
    }
}
