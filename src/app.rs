use std::{fs, env};
use crate::std_lib::STD_LIB_SCRIPT;
use crate::types::*;
use crate::tokens::*;
use crate::scanner::*;
use crate::interpreter::*;
use crate::parser::*;

pub struct App { 
    pub interpreter : Interpreter,
}

impl App {
    pub fn new() -> Self {
        Self { 
            interpreter : Interpreter::new()
        }        
    }

    pub fn play(&mut self) {
        let args: Vec<String> = env::args().collect();

        if args.len() > 1 {
            self.run_from_file(args[1].clone());
        }
        else {
            panic!("Provide a lox file to interpret.");
        }
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

    fn try_run(&mut self, source : &str)  -> RuntimeError<()> {
        let mut scanner = Scanner::new(String::from(STD_LIB_SCRIPT));
        let mut interpreter = Interpreter::new();
        let mut tokens = scanner.scan_tokens()?;
        let mut parser = Parser::new(tokens.to_vec());
        let mut statements = parser.parse()?;
        interpreter.interpret(statements)?;
        
        scanner = Scanner::new(String::from(source));
        tokens = scanner.scan_tokens()?;
        parser = Parser::new(tokens.to_vec());
        statements = parser.parse()?;
        
        interpreter.interpret(statements)?;

        Ok(())
    }

    fn run(&mut self, source : &str) {
        if let Err((token, msg)) = self.try_run(source) {
            App::error(token, msg.as_str());
        }
    }

    fn run_from_file(&mut self, path : String) {
        let contents = fs::read_to_string(path.clone())
        .expect(format!("Can't read file from path -> {}", path.clone()).as_str());

        self.run(&contents);
    }

    pub fn get_value_raw(&self, name : &str) -> Option<Literal> {
        match self.interpreter.environment.get(Token::new(TokenType::Identifier, name, None, 0)) {
            Ok(v) => {
                v
            }
            Err(_) => {
                None
            }
        }
    }

    pub fn get_value<T : TryFrom<Literal>>(&self, name : &str) -> Result<T, &str> {
        if let Some(v) =  self.get_value_raw(name) {
            match T::try_from(v) {
                Ok(val) => Ok(val),
                Err(_) => Err("Unable to get value")
            }
        }
        else {
            Err("Nil value")
        }
    }
}