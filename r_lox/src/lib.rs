pub mod tokens;
pub mod types;
pub mod interpreter;
pub mod environment;
pub mod statements;
pub mod expressions;
pub mod std_lib;
pub mod scanner;
pub mod parser;
pub mod application;

#[cfg(test)]
mod tests {
    use crate::application::App;
    use crate::types::Literal;

    #[test]
    fn conditionals() {
        let mut lox = App::new();
        lox.try_run("
        var a = false;
        if 4 == 4 {
            a = true;
        }
        ");
        let a = match lox.get_value::<bool>("a") {
            Ok(v) => v,
            Err(e) => false
        };
        
        assert_eq!(a, true);
    }
}
