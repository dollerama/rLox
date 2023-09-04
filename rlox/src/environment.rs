use std::collections::HashMap;
use crate::{types::*, tokens::*};
use crate::interpreter::Interpreter;

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
    
    pub fn define_reference(&mut self, i : &mut Interpreter, value : Option<Literal>) -> usize {
        if let Some(Literal::Instance(x)) = value.clone() {
            i.references.push(Some(Literal::Instance(x.clone())));
        }
        i.references.len()-1
    }

    pub fn define(&mut self, i : &mut Interpreter, name : String, value : Option<Literal>) {
        if let Some(Literal::Instance(x)) = value.clone() {
            if let Some(a) = x.address {
                if i.references.get(a).is_some() {
                    self.values.insert(name.clone(), Some(Literal::Instance(x.clone())));
                }
            }
            else {
                let mut new_i = x.clone();
                new_i.address = Some(i.references.len());
                i.references.push(Some(Literal::Instance(new_i.clone())));
                self.values.insert(name.clone(), Some(Literal::Instance(new_i.clone())));
            }
        }
        else {
            self.values.insert(name.clone(), value.clone());
        }
    }

    pub fn get(&self, i : &Interpreter, name : Token) -> RuntimeError<Option<Literal>> {
        if self.values.contains_key(&name.lexeme.clone()) {
            if let Some(Literal::Instance(inst)) = self.values.get(&name.lexeme).unwrap().clone() {
                match inst.address {
                    Some(a) => {
                        match i.references.get(a) {
                            Some(v) => Ok(v.clone()),
                            None => Err((name.clone(), format!("Could not Find Var")))
                        }
                    }
                    None => {
                        Err((name.clone(), format!("Could not Find Var")))
                    }
                }
            }
            else {
                Ok(self.values.get(&name.lexeme).unwrap().clone())
            }
        }
        else {
            match self.enclosing.as_ref() {
                Some(v) => {
                    v.get(i, name.clone())
                },
                None => {
                    Err((name.clone(), format!("Could not Find Var")))
                }
            }
        }
    }

    pub fn assign(&mut self, i : &mut Interpreter, name : Token, value : Option<Literal>) -> RuntimeError<Option<Box<Environment>>> {
        if self.values.contains_key(&name.lexeme) {
            if let Some(Literal::Instance(inst2)) = value.clone() {
                i.references[inst2.address.unwrap()] = Some(Literal::Instance(inst2.clone()));
                self.values.insert(name.lexeme, Some(Literal::Instance(inst2.clone())));
            }
            else {
                self.values.insert(name.lexeme, value);
            }
            
            Ok(None)
        }
        else {
            if let Some(_) = self.enclosing {
                self.enclosing.as_mut().unwrap().assign(i, name, value)
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