use std::{fs, env};
use std::io;
use std::io::Write;

use crate::std_lib::*;
use crate::types::*;
use crate::tokens::*;
use crate::scanner::*;
use crate::interpreter::*;
use crate::parser::*;
use crate::environment::*;

pub struct App { 
    pub interpreter : Interpreter,
    pub final_environment : Option<Environment>,
}

impl App {
    pub fn new() -> Self {
        Self { 
            interpreter : Interpreter::new(),
            final_environment : None,
        }        
    }

    pub fn play(&mut self) {
        let args: Vec<String> = env::args().collect();

        if args.len() > 1 {
            if args[1].clone() == "repl" {
                if let Err(e) = self.run_repl() {
                    Self::error(e.0, &e.1);
                }
            }
            else {
                self.run_from_file(args[1].clone());
            }

            if args.len() > 2 {
                if args[2].clone() == "-stdout" {
                    println!("\n---[Output]---");
                    print!("{}\n", self.interpreter.stdout);
                }
            }
        }
        else {
            panic!("Provide a lox file to interpret.");
        }
    }

    pub fn run_repl(&mut self) -> RuntimeError<()> {
        self.add_std_lib()?;
        let stdin = io::stdin(); // We get `Stdin` here.
        
        loop {
            let mut user_input = String::new();
            io::stdout().flush().unwrap();
            if let Err(e) = stdin.read_line(&mut user_input) {
                println!("{}", e);
            }

            if user_input == "exit\n" {
                break;
            }

            let mut scanner = Scanner::new(String::from(user_input));
            let tokens = scanner.scan_tokens()?;

            let mut parser = Parser::new(tokens.to_vec());
            let statements = parser.parse()?;
            
            self.interpreter.interpret(statements)?;
            if self.interpreter.stdout != "" {
                println!("");
            }
            self.interpreter.stdout = "".to_string();
        }

        self.final_environment = Some(self.interpreter.environment.clone());
        Ok(())
    }

    pub fn error(token : Token, message : &str) {
        if token.type_ == TokenType::Eof {
            App::report(token.line, "", message);
        }
        else {
            App::report(token.line, format!(" at '{}'", token.lexeme).as_str(), message);
        }
    }

    pub fn error_at(line : i32, message : &str) {
        App::report(line, "", message);
    }

    fn report(line : i32, where_str : &str, message : &str) {
        println!("[line {} ] error {}: {}", line, where_str, message);
    }

    fn add_std_lib(&mut self) -> RuntimeError<()> {
        self.interpreter.insert_function(function_container!(DebugFunction));
        self.interpreter.insert_function(function_container!(LenFunction));
        self.interpreter.insert_function(function_container!(ClockFunction));
        self.interpreter.insert_function(function_container!(RandomFunction));
        self.interpreter.insert_value("PI", 3.14159265359.into());

        let mut scanner = Scanner::new(String::from(STD_LIB_SCRIPT));
        let tokens = scanner.scan_tokens()?;
        let mut parser = Parser::new(tokens.to_vec());
        let statements = parser.parse()?;
        self.interpreter.interpret(statements)?;

        Ok(())
    }

    pub fn try_run(&mut self, source : &str) -> RuntimeError<()> {
        self.add_std_lib()?;

        let mut scanner = Scanner::new(String::from(source));
        let tokens = scanner.scan_tokens()?;

        let mut parser = Parser::new(tokens.to_vec());
        let statements = parser.parse()?;
        
        self.interpreter.interpret(statements)?;

        self.final_environment = Some(self.interpreter.environment.clone());
        Ok(())
    }

    pub fn run(&mut self, source : &str) {
        if let Err((token, msg)) = self.try_run(source) {
            App::error(token, msg.as_str());
        }
    }

    pub fn run_from_file(&mut self, path : String) {
        let contents = fs::read_to_string(path.clone())
        .expect(format!("Can't read file from path -> {}", path.clone()).as_str());

        self.run(&contents);
    }

    pub fn get_value_raw(&self, name : &str) -> Option<Literal> {
        match &self.final_environment {
            Some(v) => {
                match v.get(&self.interpreter, Token::new(TokenType::Identifier, name, None, 0)) {
                    Ok(v) => {
                        v
                    }
                    Err(_) => {
                        None
                    }
                }
            }
            _ => None
        }
    }

    pub fn get_vec<T : TryFrom<Literal>>(&self, name : &str) -> Result<Vec<T>, &'static str> {
        if let Some(v) =  self.get_value_raw(name) {
            match v {
                Literal::Collection(c) => {
                    let mut vec = Vec::new();
                    for obj in c {
                        if let Some(obj_v) = *obj {
                            match T::try_from(obj_v) {
                                Ok(v) => {
                                    vec.push(v);
                                },
                                Err(_) => return Err("Unable to get value")
                            }
                        }
                    }
                    Ok(vec)
                }
                _ => {
                    Err("Unable to get value")
                }
            }
        }
        else {
            Err("Unable to get value")
        }
    }

    pub fn get_value<T : TryFrom<Literal>>(&self, name : &str) -> Result<T, &'static str> {
        if let Some(v) =  self.get_value_raw(name) {
            match T::try_from(v) {
                Ok(val) => {
                    Ok(val)
                },
                Err(_) => Err("Unable to get value")
            }
        }
        else {
            Err("Nil value")
        }
    }
}