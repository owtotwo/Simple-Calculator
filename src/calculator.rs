#[derive(Debug)]
pub struct Calculator;

impl Calculator {
    pub fn new() -> Calculator { Calculator{} }
    pub fn run(&self) {
        println!("Running...");
    }
}

impl Drop for Calculator {
    fn drop(&mut self) {
        println!("Shutdown!");
    }
}