pub mod tokens;
pub mod types;
pub mod interpreter;
pub mod environment;
pub mod statements;
pub mod expressions;
#[macro_use] pub mod std_lib;
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
        lox.run("
        bool a = false;
        if 3 < 5 {
            a = true;
        }

        num b = 0;
        num c = 9;
        if c % 2 == 0 {
            b = -1;
        }
        else if c == 8 {
            b = -1;
        }
        else {
            b = 1;
        }
        ");
        let a = match lox.get_value::<bool>("a") {
            Ok(v) => v,
            Err(_) => false
        };

        let b = match lox.get_value::<f64>("b") {
            Ok(v) => v,
            Err(_) => 0.0
        };    

        assert_eq!(a, true);
        assert_eq!(b, 1.0);
    }

    #[test]
    fn loops() {
        let mut lox = App::new();
        lox.run("
        num a = 0;
        for i < 5 {
            a++;
        }

        for var i = 0; i < 5; i++ {
            a++;
        }

        string b = \"\";
        for i in [1,2,3,4,5] {
            b += i as string;
        }

        var list = [1,2,3];

        for i in list {
            list[i_iter] += 5;
        }
        ");
        let a = match lox.get_value::<f64>("a") {
            Ok(v) => v,
            Err(_) => 0.0
        };
        let b = match lox.get_value::<String>("b") {
            Ok(v) => v,
            Err(_) => "".to_string()
        };
        let c = match lox.get_vec::<f64>("list") {
            Ok(v) => v,
            Err(_) => Vec::new()
        };

        assert_eq!(a, 20.0);
        assert_eq!(b, "12345".to_string());
        assert_eq!(c.iter().sum::<f64>(), 21.0);
    }
}
