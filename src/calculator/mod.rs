use std::io::{self, Write};

use self::expr::Expr;

mod expr;


const PROMPT_DEFAULT: &'static str = ">> ";


pub struct Calculator {
    prompt: String,
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
                Some(input) => input,
                None => {
                    self.disp("Error: Failed to Get User Input", true);
                    break;
                }
            };

            // EOF to exit
            if user_input.len() == 0 { break; }

            // Empty content to next loop
            if user_input.trim().len() == 0 { continue; }

            // parse expression
            let expr = match self.parse_expr(&user_input) {
                Some(expr) => expr,
                None => {
                    self.disp("Error: Failed to Parse the Expression", true);
                    continue;
                },
            };

            // evaluate expression
            let result = match self.eval(&expr) {
                Some(result) => result,
                None => {
                    self.disp("Error: Failed to Evaluate the Expression", true);
                    continue;
                },
            };

            // show the result
            self.disp(&result.to_string(), true);
        }
    }
}


impl Calculator {
    fn get_input(&self) -> Option<String> {
        let mut line = String::new();
        io::stdin().read_line(&mut line).map(|_| line).ok()
    }

    fn disp(&self, msg: &str, newline: bool) {
        let mut stdout = io::stdout();
        // if it is not able to display anything, I can't do any more except panic.
        stdout.write(msg.as_bytes()).unwrap();
        if newline { stdout.write("\n".as_bytes()).unwrap(); }
        stdout.flush().unwrap();
    }

    fn parse_expr<'a>(&'a self, expr: &'a str) -> Option<Expr> {
        Expr::parse(expr).and_then(|(expr, dirty_expr)| {
            if dirty_expr.trim_right().len() == 0 { Some(expr) } else { None }
        })
    }

    fn eval<'a>(&'a self, expr: &'a Expr) -> Option<i32> {
        expr.eval()
    }
}
