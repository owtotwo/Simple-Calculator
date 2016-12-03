mod expr;

use std::error;
use std::fmt;
use std::io::{self, Write};
use std::result;

use self::expr::Expr;


const PROMPT_DEFAULT: &'static str = ">> ";


pub struct Calculator {
    prompt: String,
}

type Result<T> = result::Result<T, Box<error::Error>>;

#[derive(Debug)]
enum CalculatorError {
    IOError(io::Error),
    TailRedundantChars,
}

impl From<io::Error> for CalculatorError {
    fn from(err: io::Error) -> CalculatorError {
        CalculatorError::IOError(err)
    }
}

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CalculatorError::IOError(ref e) =>
                e.fmt(f),
            CalculatorError::TailRedundantChars =>
                write!(f, "too more chars in tail"),
        }
    }
}

impl error::Error for CalculatorError {
    fn description(&self) -> &str {
        match *self {
            CalculatorError::IOError(ref e) => e.description(),
            CalculatorError::TailRedundantChars => "too more chars in tail",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CalculatorError::IOError(ref e) => Some(e),
            CalculatorError::TailRedundantChars => None,
        }
    }
}

impl Calculator {
    pub fn new() -> Calculator {
        Calculator {
            prompt: String::from(PROMPT_DEFAULT),
        }
    }
    
    pub fn cli(&self) {
        loop {
            // display the prompt
            self.disp(&self.prompt, false);

            // get user input from console
            let user_input = match self.get_input() {
                Ok(input) => input,
                Err(why) => {
                    self.disp(why.description(), true);
                    break;
                }
            };

            // EOF to exit
            if user_input.len() == 0 { break; }

            // Empty content to next loop
            if user_input.trim().len() == 0 { continue; }

            // parse expression
            let expr = match self.parse_expr(&user_input) {
                Ok(expr) => expr,
                Err(why) => {
                    self.disp(why.description(), true);
                    continue;
                },
            };

            // evaluate expression
            let result = match self.eval(&expr) {
                Ok(result) => result,
                Err(why) => {
                    self.disp(why.description(), true);
                    continue;
                },
            };

            // show the result
            self.disp(&result.to_string(), true);
        }
    }
}


impl Calculator {
    fn get_input(&self) -> Result<String> {
        let mut line = String::new();
        io::stdin().read_line(&mut line) ?;
        Ok(line)
    }

    fn disp(&self, msg: &str, newline: bool) {
        let mut stdout = io::stdout();
        // if it is not able to display anything, I can't do any more except panic.
        stdout.write(msg.as_bytes()).unwrap();
        if newline { stdout.write("\n".as_bytes()).unwrap(); }
        stdout.flush().unwrap();
    }

    fn parse_expr<'a>(&'a self, expr: &'a str) -> Result<Expr> {
        Expr::parse(expr).and_then(|(expr, dirty_expr)| {
            if dirty_expr.trim_right().len() == 0 {
                Ok(expr)
            } else {
                Err(Box::new(CalculatorError::TailRedundantChars))
            }
        })
    }

    fn eval<'a>(&'a self, expr: &'a Expr) -> Result<i32> {
        expr.eval()
    }
}
