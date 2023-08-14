use crate::types::Literal;

mod tokens;
mod types;
mod interpreter;
mod environment;
mod statements;
mod expressions;
mod std_lib;
mod scanner;
mod parser;
mod app;

fn main() {
    let mut lox = app::App::new();
    lox.play();
    // match lox.get_value::<i32>("d") {
    //     Ok(v) => println!("rust => {}", v),
    //     Err(e) => println!("{}", e) 
    // }
}