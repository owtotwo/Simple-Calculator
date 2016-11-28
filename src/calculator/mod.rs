mod expr;

use self::expr::Expr;
use std::io;
use std::io::Write;

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
            self.disp(self.prompt.clone());

            let user_input = self.get_input();

            let expr = match self.parse_expr(&user_input) {
                Ok(expr) => expr,
                Err(err) => {
                    self.disp(format!("{}\n", err));
                    continue;
                },
            };

            let result = match self.eval(&expr) {
                Ok(result) => result,
                Err(err) => {
                    self.disp(format!("{}\n", err));
                    continue;
                },
            };

            self.disp(format!("{}\n", result));
        }
    }
}

///////////////////////////////////////////////
//  Detials
///////////////////////////////////////////////

impl Calculator {

    fn get_input(&self) -> String {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect(
            "can not get chars from stdin");
        let line = line.trim().to_string();
        line
    }

    fn disp(&self, msg: String) {
        io::stdout().write(msg.as_bytes()).unwrap();
        io::stdout().flush().unwrap();
    }

    fn parse_expr(&self, expr: &str) -> Result<Expr, String> {
        match Expr::parse(expr) {
            Ok((expr, dirty_expr)) => {
                if dirty_expr.trim_right().len() == 0 {
                    Ok(expr)
                } else {
                    Err("Too More Chars".to_string())
                }
            },
            Err(err) => Err(err),
        }
    }

    fn eval<'a>(&'a self, expr: &'a Expr) -> Result<i32, &str> {
        expr.eval()
    }
}
