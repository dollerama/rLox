use std::any::Any;

use crate::types::*;
use crate::tokens::*;
use crate::interpreter::*;

macro_rules! native_function {
    ($($name:ident $s:expr, $arity:expr => {$($fun:tt)*})*) => {
        $(
            #[derive(Debug, Clone)]
            pub struct $name;

            impl $name {
                pub fn new() -> Self {
                    Self {}
                }
            }

            impl LoxCallable for $name {
                fn get_name(&self) -> Token {
                    Token::new(TokenType::Identifier, $s, None, 0)
                }

                fn arity(&self) -> usize {
                    $arity as usize
                }
                
                $($fun)*
                
                fn clone_dyn(&self) -> Box<dyn LoxCallable> {
                    Box::new(self.clone())
                }
                
                fn as_any(&self) -> &dyn Any {
                    self
                }
            }
        )*
    }
}

pub const STD_LIB_SCRIPT: &str = "
    class Stack {
        Stack() {
            this.items = [];
        }

        push(item) {
            this.items += item;
        }

        pop() {
            var item = this.items[-1];
            this.items -= -1;
            return item;
        }

        count() {
            return #this.items;
        }
    }
";

native_function! {
    DebugFunction "debug", 1 => {
        fn call(&self, _interpreter : &mut Interpreter, arguments : Vec<Option<Literal>>, _auto_clean : bool) -> RuntimeError<Option<Literal>> {
            println!("{:#?}", arguments[0].clone());
            Ok(None)
        }
    }

    LenFunction "len", 1 => {
        fn call(&self, _interpreter : &mut Interpreter, arguments : Vec<Option<Literal>>, _auto_clean : bool) -> RuntimeError<Option<Literal>> {
            if let Some(Literal::Collection(c)) = arguments[0].clone() {
                Ok(Some(Literal::Number(c.len() as f64)))
            }
            else {
                Ok(Some(Literal::Number(1.0)))
            }
        }
    }

    ClockFunction "clock", 0 => {
        fn call(&self, interpreter : &mut Interpreter, _arguments : Vec<Option<Literal>>, _auto_clean : bool) -> RuntimeError<Option<Literal>> {
            Ok(Some(Literal::Number(interpreter.time.elapsed().as_millis() as f64)))
        }
    }
}