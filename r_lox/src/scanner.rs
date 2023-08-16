use std::collections::HashMap;
use crate::types::*;
use crate::tokens::*;
use crate::application::*;

pub struct Scanner {
    pub source : String,
    pub tokens : Vec<Token>,
    pub start : i32,
    pub current : i32,
    pub line : i32,
    pub keywords : HashMap<String, TokenType>
}

impl Scanner {
    pub fn new(source : String) -> Self {
        let mut keywords_container = HashMap::new();
        keywords_container.insert(String::from("&&"), TokenType::And);
        keywords_container.insert(String::from("class"), TokenType::Class);
        keywords_container.insert(String::from("else"), TokenType::Else);
        keywords_container.insert(String::from("else if"), TokenType::ElseIf);
        keywords_container.insert(String::from("false"), TokenType::False);
        keywords_container.insert(String::from("for"), TokenType::For);
        keywords_container.insert(String::from("fn"), TokenType::Fn);
        keywords_container.insert(String::from("if"), TokenType::If);
        keywords_container.insert(String::from("nil"), TokenType::Nil);
        keywords_container.insert(String::from("||"), TokenType::Or);
        keywords_container.insert(String::from("print"), TokenType::Print);
        keywords_container.insert(String::from("return"), TokenType::Return);
        keywords_container.insert(String::from("super"), TokenType::Super);
        keywords_container.insert(String::from("this"), TokenType::This);
        keywords_container.insert(String::from("true"), TokenType::True);
        keywords_container.insert(String::from("var"), TokenType::Var);
        keywords_container.insert(String::from("bool"), TokenType::Bool);
        keywords_container.insert(String::from("num"), TokenType::Number);
        keywords_container.insert(String::from("string"), TokenType::String);
        keywords_container.insert(String::from("while"), TokenType::While);
        keywords_container.insert(String::from("break"), TokenType::Break);
        keywords_container.insert(String::from("continue"), TokenType::Continue);
        keywords_container.insert(String::from("as"), TokenType::As);
        keywords_container.insert(String::from("is"), TokenType::Is);
    
        Self {
            source,
            tokens : Vec::new(),
            start : 0,
            current : 0,
            line : 0,
            keywords : keywords_container
        }
    }
    
    pub fn scan_tokens(&mut self) -> RuntimeError<&Vec<Token>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }
        
        self.tokens.push(Token::new(TokenType::Eof, "", None, self.line));
        
        Ok(&self.tokens)
    }
    
    pub fn scan_token(&mut self) -> RuntimeError<()> {
        match self.advance()? {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            '[' => self.add_token(TokenType::LeftBracket),
            ']' => self.add_token(TokenType::RightBracket),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '#' => self.add_token(TokenType::Hash),
            '?' => self.add_token(TokenType::Question),
            '|' => self.add_token(TokenType::Pipe),
            '-' => {
                if self.try_pair('-') {
                    self.add_token(TokenType::Decr)
                }
                else if self.try_pair('=') {
                    self.add_token(TokenType::MinusEqual)
                }
                else {
                    self.add_token(TokenType::Minus)
                }
            },
            '+' => {
                if self.try_pair('+') {
                    self.add_token(TokenType::Incr)
                }
                else if self.try_pair('=') {
                    self.add_token(TokenType::PlusEqual)
                }
                else {
                    self.add_token(TokenType::Plus)
                }
            },
            ';' => self.add_token(TokenType::Semicolon),
            ':' => self.add_token(TokenType::Colon),
            '*' => {
                match self.try_pair('=') {
                    true => self.add_token(TokenType::StarEqual),
                    false => self.add_token(TokenType::Star)
                }
            },
            '%' => {
                match self.try_pair('=') {
                    true => self.add_token(TokenType::ModEqual),
                    false => self.add_token(TokenType::Mod)
                }
            },
            '!' => {
                match self.try_pair('=') {
                    true => self.add_token(TokenType::BangEqual),
                    false => self.add_token(TokenType::Bang)
                }
            },
            '=' => {
                if self.try_pair('=') {
                    self.add_token(TokenType::EqualEqual)
                }
                else if self.try_pair('>') {
                    self.add_token(TokenType::Lambda)
                }
                else {
                    self.add_token(TokenType::Equal)
                }
            },
            '<' => {
                match self.try_pair('=') {
                    true => self.add_token(TokenType::LessEqual),
                    false => self.add_token(TokenType::Less)
                }
            },
            '>' => {
                match self.try_pair('=') {
                    true => self.add_token(TokenType::GreaterEqual),
                    false => self.add_token(TokenType::Greater)
                }
            },
            '/' => {
                match self.try_pair('/') {
                    true => {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance()?;
                        }
                    },
                    false => {
                        match self.try_pair('=') {
                            true => self.add_token(TokenType::SlashEqual),
                            false => self.add_token(TokenType::Slash)
                        }
                    }
                }
            }
            ' ' | '\r' | '\t' => { },
            '\n' => self.line += 1,
            '"' => self.string()?,
            v if Scanner::is_digit(v) => self.number()?,
            v if Scanner::is_alpha(v) => self.identifier()?,
            _ => {
                App::error_at(self.line,  "Unexpected character.")
            }
        };
        
        Ok(())
    }
    
    pub fn is_digit(c : char) -> bool {
        c >= '0' && c <= '9'
    }
    
    pub fn is_alpha(c : char) -> bool {
        (c >= 'a' && c <= 'z') ||
        (c >= 'A' && c <= 'Z') ||
        (c == '_')
    }
    
    pub fn is_alpha_numeric(c : char) -> bool {
        Scanner::is_digit(c) || Scanner::is_alpha(c)
    }
    
    pub fn identifier(&mut self) -> RuntimeError<()> {
        while Scanner::is_alpha_numeric(self.peek()) {
            self.advance()?;
        }
        
        let value = String::from(&self.source.as_str()[self.start as usize..self.current as usize]);
        let type_ = match self.keywords.get(&value) {
            Some(v) => v,
            None => &TokenType::Identifier
        };
        
        self.add_token(*type_);
        
        Ok(())
    }
    
    pub fn number(&mut self) -> RuntimeError<()>  {
        while Scanner::is_digit(self.peek()) {
            self.advance()?;
        }
        
        if self.peek() == '.' && Scanner::is_digit(self.peek_next()) {
            self.advance()?;
            
            while Scanner::is_digit(self.peek()) {
                self.advance()?;
            }
        }
        
        let num = &self.source.as_str()[self.start as usize..self.current as usize].parse::<f64>();
        
        match num {
            Ok(val) => {
                self.add_token_with_literal(TokenType::Number, Some(Literal::Number(*val)));
                Ok(())
            },
            Err(_) => Err((self.tokens[self.current as usize].clone(), "Could not parse num.".to_string()))
        }
    }
     
    pub fn string(&mut self) -> RuntimeError<()> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            
            self.advance()?;
        }
        
        if self.is_at_end() {
            Err((self.tokens[self.current as usize].clone(), "Unterminated String.".to_string()))
        }
        else {
            self.advance()?;
            
            let value = String::from(&self.source.as_str()[(self.start+1) as usize..(self.current-1) as usize]);
            self.add_token_with_literal(TokenType::String, Some(Literal::String(value)));
            
            Ok(())
        }
    }
    
    pub fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        }
        else {
            self.source.chars().nth(self.current as usize).unwrap()
        }
    }
    
    pub fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() as i32 {
            '\0'
        }
        else {
            self.source.chars().nth((self.current+1) as usize).unwrap()
        }
    }
    
    pub fn try_pair(&mut self, expected : char) -> bool {
        if self.is_at_end() || 
        self.source.chars().nth(self.current as usize).unwrap() != expected {
            false
        }
        else {
            self.current += 1;
            
            true
        }
    }
     
    pub fn advance(&mut self) -> RuntimeError<char> {
        let current_char = self.source.chars().nth(self.current as usize);
        self.current += 1;
    
        match current_char {
            Some(v) => Ok(v),
            None => Err((self.tokens[self.current as usize].clone(), "Character not found.".to_string()))
        }
    }
    
    fn add_token(&mut self, type_ : TokenType) {
        self.add_token_with_literal(type_, None);
    }
    
    fn add_token_with_literal(&mut self, type_ : TokenType, literal : Option<Literal>) {
        let text = &self.source.as_str()[self.start as usize..self.current as usize];
        self.tokens.push(Token::new(type_, text, literal, self.line));
    }
    
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as i32
    }
}