use std::collections::HashMap;
use crate::{types::*, tokens::*};

#[derive(Debug, Clone)]
pub struct Environment {
    pub values : HashMap<String, Option<Literal>>,
    pub enclosing : Option<Box<Environment>>
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values : HashMap::new(),
            enclosing : None
        }
    }
    
    pub fn new_with_enclosing(enclosing_ : Environment) -> Self {
        Self { 
            values : HashMap::new(),
            enclosing : Some(Box::new(enclosing_))
        }
    }
    
    pub fn define(&mut self, name : String, value : Option<Literal>) {
        self.values.insert(name, value);
    }
    
    pub fn get(&self, name : Token) -> RuntimeError<Option<Literal>> {
        if self.values.contains_key(&name.lexeme) {
            Ok(self.values.get(&name.lexeme).unwrap().clone())
        }
        else {
            match self.enclosing.as_ref() {
                Some(v) => {
                    v.get(name.clone())
                },
                None => {
                    
                    Err((name.clone(), format!("Could not Find Var")))
                }
            }
        }
    }

    pub fn assign(&mut self, name : Token, value : Option<Literal>) -> RuntimeError<Option<Box<Environment>>> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value);
            Ok(None)
        }
        else {
            if let Some(_) = self.enclosing {
                self.enclosing.as_mut().unwrap().assign(name, value)
            }
            else {
                Err((
                    name.clone(), 
                    format!("Undefined Variable '{}'.", name.lexeme.clone())
                ))
            }
        }
    }
}