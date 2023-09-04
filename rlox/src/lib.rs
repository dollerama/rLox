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

        let a = lox.get_value::<bool>("a").expect("Nil");

        let b = lox.get_value::<f64>("b").expect("Nil");   

        let d = lox.get_value::<String>("d").expect("Nil"); 

        assert_eq!(a, true);
        assert_eq!(b, 1.0);
        assert_eq!(d, "4 is even");
    }

    #[test]
    fn classes() {
        let mut lox = App::new();
        lox.run("
        class a {
            a(i) {
                this.i = i;
            }

            set(i) {
                this.i = i;
            }

            incr() {
                this.i++;
            }
        }

        class b : a {
            b(i) {
                this.i = i;
            }

            incr() {
                super.incr();
                this.i++;
            }
        }

        var aa = a(0);
        var bb = b(0);

        aa.incr();
        bb.incr();

        println(aa);
        println(bb);
        ");
        let ai = lox.get_field::<f64>("aa", "i").expect("Nil");
        let bi = lox.get_field::<f64>("bb", "i").expect("Nil"); 
        assert_eq!(ai, 1.0);
        assert_eq!(bi, 2.0);
    }

    #[test]
    fn functions() {
        let mut lox = App::new();
        lox.run("
        fn hello_fun(msg : string) {
            return \"Hello \"+msg;
        }

        var c = hello_fun(\"World\");

        var a => |a, b| a+b;
        var cmp => |a, b| {
            if a < b {
                return false;
            }
            else {
                return true;
            }
        };

        var b = a(2,5);
        var bb = cmp(b, 3);
        ");

        let b = lox.get_value::<f64>("b").expect("Nil");

        let bb = lox.get_value::<bool>("bb").expect("Nil");

        let c = lox.get_value::<String>("c").expect("Nil");

        assert_eq!(b, 7.0);
        assert_eq!(bb, true);
        assert_eq!(c, "Hello World");
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
        let a = lox.get_value::<f64>("a").expect("Nil");

        let b = lox.get_value::<String>("b").expect("Nil");

        let c = lox.get_value::<String>("c").expect("Nil");

        let d = lox.get_vec::<f64>("d").expect("Nil");

        let e = lox.get_value::<bool>("e").expect("Nil");

        assert_eq!(a, 12.0);
        assert_eq!(b, "12345".to_string());
        assert_eq!(c, "54321".to_string());
        assert_eq!(d.iter().sum::<f64>(), 21.0);
        assert_eq!(e, true);
    }
}
