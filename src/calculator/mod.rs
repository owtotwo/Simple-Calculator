mod expr;

use self::expr::Expr;
use std::io;
use std::io::Write; // for stdout().write()

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
            self.disp(&self.prompt, false);

            let user_input = self.get_input();

            if user_input.len() == 0 { break; }
            if user_input.trim().len() == 0 { continue; }

            let expr = match self.parse_expr(&user_input) {
                Ok(expr) => expr,
                Err(err) => {
                    self.disp(err, true);
                    continue;
                },
            };

            let result = match self.eval(&expr) {
                Ok(result) => result,
                Err(err) => {
                    self.disp(err, true);
                    continue;
                },
            };

            self.disp(&result.to_string(), true);
        }
    }
}


impl Calculator {
    fn get_input(&self) -> String {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect(
            "can not get chars from stdin");
        let line = line.to_string();
        line
    }

    fn disp(&self, msg: &str, newline: bool) {
        let mut stdout = io::stdout();
        stdout.write(msg.as_bytes()).unwrap();
        if newline { stdout.write(msg.as_bytes()).unwrap(); }
        stdout.flush().unwrap();
    }

    fn parse_expr<'a>(&'a self, expr: &'a str) -> Result<Expr, &str> {
        match Expr::parse(expr) {
            Ok((expr, dirty_expr)) => {
                if dirty_expr.trim_right().len() == 0 {
                    Ok(expr)
                } else {
                    Err("Too More Chars")
                }
            },
            Err(err) => Err(err),
        }
    }

    fn eval<'a>(&'a self, expr: &'a Expr) -> Result<i32, &str> {
        expr.eval()
    }
}
