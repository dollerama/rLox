use crate::types::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    //Single-character tokens
    LeftParen, RightParen, 
    LeftBrace, RightBrace,
    LeftBracket, RightBracket,
    Comma, Dot, Minus, Plus, Semicolon, Colon, Slash, Star,
    Mod, 

    //One Or two character tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual, Incr, 
    Decr, PlusEqual, MinusEqual,
    StarEqual, SlashEqual, 
    ModEqual, Hash, Lambda, 
    Pipe, Question,
    
    //Literals
    Identifier, String, Number, Bool,
    
    //keywords
    And, Class, Else, ElseIf, False, Fn, For, If, Nil, Or, As, Is, In,
    Print, Return, Super, This, True, Var, While, Break, Continue,
    
    Eof
}

#[derive(Clone, Debug)]
pub struct Token {
    pub type_ : TokenType,
    pub lexeme : String,
    pub literal : Option<Literal>,
    pub line : i32,
}

impl Token {
    pub fn new(type_ : TokenType, lexeme_ : &str, literal : Option<Literal>, line : i32) -> Self {
        Self {
            type_,
            lexeme : String::from(lexeme_),
            literal,
            line
        }
    }
}