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

        string d = 4%2==0 ? \"4 is even\" : \"4 is not even\";
        ");
        let a = match lox.get_value::<bool>("a") {
            Ok(v) => v,
            Err(_) => false
        };

        let b = match lox.get_value::<f64>("b") {
            Ok(v) => v,
            Err(_) => 0.0
        };    

        let d = match lox.get_value::<String>("d") {
            Ok(v) => v,
            Err(_) => "".to_string()
        };    

        assert_eq!(a, true);
        assert_eq!(b, 1.0);
        assert_eq!(d, "4 is even");
    }

    #[test]
    fn loops() {
        let mut lox = App::new();
        lox.run("
        num a = 0;
        for i < 5 {
            print(a);
            if i%2 == 0 a++;
            else continue;

            a++;
        }

        for var i = 0; i < 5; i++ {
            print(a);
            if i%2 != 0 a++;
            else continue;
            
            a++;
        }

        string b = \"\";
        for i in [1,2,3,4,5] {
            b += i as string;
        }
        
        string c = \"\";
        for i in ![1,2,3,4,5] {
            c += i as string;
        }

        var d = [1,2,3];

        for i in d {
            d[i_iter] += 5;
        }

        var e = false;
        var i = 0;
        while i < 5 {
            i++;
        }
        e = true;
        ");
        let a = match lox.get_value::<f64>("a") {
            Ok(v) => v,
            Err(_) => 0.0
        };
        let b = match lox.get_value::<String>("b") {
            Ok(v) => v,
            Err(_) => "".to_string()
        };
        let c = match lox.get_value::<String>("c") {
            Ok(v) => v,
            Err(_) => "".to_string()
        };
        let d = match lox.get_vec::<f64>("d") {
            Ok(v) => v,
            Err(_) => Vec::new()
        };
        let e = match lox.get_value::<bool>("e") {
            Ok(v) => v,
            Err(_) => false
        };

        assert_eq!(a, 12.0);
        assert_eq!(b, "12345".to_string());
        assert_eq!(c, "54321".to_string());
        assert_eq!(d.iter().sum::<f64>(), 21.0);
        assert_eq!(e, true);
    }
}
