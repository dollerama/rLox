use std::collections::HashMap;
use std::any::Any;

use crate::tokens::*;
use crate::interpreter::*;
use crate::environment::*;
use crate::expressions::*;
use crate::statements::*;

pub type RuntimeError<T> = Result<T, (Token, String)>;

#[derive(Clone, Debug)]
pub enum LoopType {
    While,
    For,
    ForEach
}

#[derive(Clone, Debug)]
pub enum FunctionType {
    Normal,
    Method,
    Anon
}

#[derive(Clone, Debug)]
pub struct Parameter {
    pub name : Token,
    pub binding : Option<Token>
}

impl Parameter {
    pub fn new(name : Token, binding : Option<Token>) -> Self {
        Self {
            name,
            binding
        }
    }
}

pub trait LoxCallable : std::fmt::Debug  {
    fn arity(&self) -> usize;
    fn call(&self, interpreter : &mut Interpreter, callee : Token, arguments : Vec<Option<Literal>>, auto_clean : bool) -> RuntimeError<Option<Literal>>;
    fn clone_dyn(&self) -> Box<dyn LoxCallable>;
    fn get_name(&self) -> Token;
    fn as_any(&self) -> &dyn Any;
}

impl Clone for Box<dyn LoxCallable> {
    fn clone(&self) -> Box<dyn LoxCallable> {
        self.clone_dyn()
    }
}
    
pub trait StmtVisitor {
    fn visit_expression_stmt(&mut self, stmt : &StmtExpr) -> RuntimeError<Option<Literal>>;
    fn visit_print_stmt(&mut self, stmt :&Print) -> RuntimeError<Option<Literal>>;
    fn visit_var_stmt(&mut self, stm : &Var) -> RuntimeError<Option<Literal>>;
    fn visit_block_stmt(&mut self, stm : &Block) -> RuntimeError<Option<Literal>>;
    fn visit_class_stmt(&mut self, stm : &Class) -> RuntimeError<Option<Literal>>;
    fn visit_if_stmt(&mut self, stm : &If) -> RuntimeError<Option<Literal>>;
    fn visit_while_stmt(&mut self, stm : &While) -> RuntimeError<Option<Literal>>;
    fn visit_function_stmt(&mut self, stmt : &Function) -> RuntimeError<Option<Literal>>;
    fn visit_return_stmt(&mut self, stmt : &Return) -> RuntimeError<Option<Literal>>;
    fn visit_break_stmt(&mut self, stmt : &Break) -> RuntimeError<Option<Literal>>;
    fn visit_continue_stmt(&mut self, stmt : &Continue) -> RuntimeError<Option<Literal>>;
}
    
pub trait ExprVisitor {
    fn visit_binary_expr(&mut self, expr : &Binary) -> RuntimeError<Option<Literal>>;
    fn visit_ternary_expr(&mut self, expr : &Ternary) -> RuntimeError<Option<Literal>>;
    fn visit_grouping_expr(&mut self, expr : &Grouping) -> RuntimeError<Option<Literal>>;
    fn visit_unary_expr(&mut self, expr : &Unary) -> RuntimeError<Option<Literal>>;
    fn visit_literal_expr(&mut self, expr : &LiteralExp) -> RuntimeError<Option<Literal>>;
    fn visit_var_expr(&mut self, expr : &VarExpr) -> RuntimeError<Option<Literal>>;
    fn visit_assign_expr(&mut self, expr : &Assign) -> RuntimeError<Option<Literal>>;
    fn visit_logical_expr(&mut self, expr : &Logical) -> RuntimeError<Option<Literal>>;
    fn visit_call_expr(&mut self, expr : &Call) -> RuntimeError<Option<Literal>>;
    fn visit_get_expr(&mut self, expr : &Get) -> RuntimeError<Option<Literal>>;
    fn visit_set_expr(&mut self, expr : &Set) -> RuntimeError<Option<Literal>>;
    fn visit_this_expr(&mut self, expr : &This) -> RuntimeError<Option<Literal>>;
    fn visit_super_expr(&mut self, expr : &Super) -> RuntimeError<Option<Literal>>;
    fn visit_index_expr(&mut self, expr : &Index) -> RuntimeError<Option<Literal>>;
    fn visit_index_get_expr(&mut self, expr : &IndexGet) -> RuntimeError<Option<Literal>>;
    fn visit_index_set_expr(&mut self, expr : &IndexSet, coll : Vec<Box<Option<Literal>>>) -> RuntimeError<Option<Literal>>;
}

pub trait Stmt : std::fmt::Debug { 
    fn accept(&self, visitor : &mut Box<&mut dyn StmtVisitor>) -> RuntimeError<Option<Literal>>;
    fn clone_dyn(&self) -> Box<dyn Stmt>;
    fn as_any(&self) -> &dyn Any;
}

impl Clone for Box<dyn Stmt> {
    fn clone(&self) -> Box<dyn Stmt> {
        self.clone_dyn()
    }
}

pub trait Expr : std::fmt::Debug { 
    fn accept(&self, visitor : &mut Box<&mut dyn ExprVisitor>) -> RuntimeError<Option<Literal>>;
    fn as_any(&self) -> &dyn Any;
    fn clone_dyn(&self) -> Box<dyn Expr>;
}

impl Clone for Box<dyn Expr> {
    fn clone(&self) -> Box<dyn Expr> {
        self.clone_dyn()
    }
}

#[derive(Clone, Debug)]
pub enum Literal {
    Number(f64),
    StrongNumber(f64),
    String(String),
    StrongString(String),
    Boolean(bool),
    StrongBoolean(bool),
    Function(Box<dyn LoxCallable>),
    Class(Box<LoxClass>),
    Instance(Box<LoxInstance>),
    Return(Box<Literal>),
    Keyword(String),
    Collection(Vec<Box<Option<Literal>>>),
    StrongCollection(Vec<Box<Option<Literal>>>)
}

impl TryFrom<Literal> for Vec<Literal> {
    type Error = &'static str;

    fn try_from(v: Literal) -> Result<Self, Self::Error> {
        if let Literal::Collection(x) = v {
            let mut result = Vec::new();
            for obj in x {
                if let Some(value) = *obj {
                    result.push(value);
                }
            }

            Ok(result)
        }
        else {
            Err("Cannot cast Literal to i32")
        }
    }
}

impl TryFrom<Literal> for i32 {
    type Error = &'static str;

    fn try_from(v: Literal) -> Result<Self, Self::Error> {
        if let Literal::Number(x) = v {
            Ok(x as i32)
        }
        else if let Literal::StrongNumber(x) = v {
            Ok(x as i32)
        }
        else {
            Err("Cannot cast Literal to i32")
        }
    }
}

impl TryFrom<&Literal> for i32 {
    type Error = &'static str;

    fn try_from(v: &Literal) -> Result<Self, Self::Error> {
        if let Literal::Number(x) = v {
            Ok(*x as i32)
        }
        else if let Literal::StrongNumber(x) = v {
            Ok(*x as i32)
        }
        else {
            Err("Cannot cast Literal to i32")
        }
    }
}

impl TryFrom<Literal> for i64 {
    type Error = &'static str;

    fn try_from(v: Literal) -> Result<Self, Self::Error> {
        if let Literal::Number(x) = v {
            Ok(x as i64)
        }
        else if let Literal::StrongNumber(x) = v {
            Ok(x as i64)
        }
        else {
            Err("Cannot cast Literal to i64")
        }
    }
}

impl TryFrom<Literal> for f32 {
    type Error = &'static str;

    fn try_from(v: Literal) -> Result<Self, Self::Error> {
        if let Literal::Number(x) = v {
            Ok(x as f32)
        }
        else if let Literal::StrongNumber(x) = v {
            Ok(x as f32)
        }
        else {
            Err("Cannot cast Literal to i64")
        }
    }
}

impl TryFrom<&Literal> for f32 {
    type Error = &'static str;

    fn try_from(v: &Literal) -> Result<Self, Self::Error> {
        if let Literal::Number(x) = v {
            Ok(*x as f32)
        }
        else if let Literal::StrongNumber(x) = v {
            Ok(*x as f32)
        }
        else {
            Err("Cannot cast Literal to i64")
        }
    }
}

impl TryFrom<Literal> for f64 {
    type Error = &'static str;

    fn try_from(v: Literal) -> Result<Self, Self::Error> {
        if let Literal::Number(x) = v {
            Ok(x)
        }
        else if let Literal::StrongNumber(x) = v {
            Ok(x)
        }
        else {
            Err("Cannot cast Literal to i64")
        }
    }
}

impl TryFrom<&Literal> for f64 {
    type Error = &'static str;

    fn try_from(v: &Literal) -> Result<Self, Self::Error> {
        if let Literal::Number(x) = v {
            Ok(*x)
        }
        else if let Literal::StrongNumber(x) = v {
            Ok(*x)
        }
        else {
            Err("Cannot cast Literal to i64")
        }
    }
}

impl TryFrom<Literal> for String {
    type Error = &'static str;

    fn try_from(v: Literal) -> Result<Self, Self::Error> {
        if let Literal::String(x) = v {
            Ok(x)
        }
        else if let Literal::StrongString(x) = v {
            Ok(x)
        }
        else {
            Err("Cannot cast Literal to i64")
        }
    }
}

impl TryFrom<Literal> for bool {
    type Error = &'static str;

    fn try_from(v: Literal) -> Result<Self, Self::Error> {
        if let Literal::Boolean(x) = v {
            Ok(x)
        }
        else if let Literal::StrongBoolean(x) = v {
            Ok(x)
        }
        else {
            Err("Cannot cast Literal to bool")
        }
    }
}

impl TryFrom<&Literal> for bool {
    type Error = &'static str;

    fn try_from(v: &Literal) -> Result<Self, Self::Error> {
        if let Literal::Boolean(x) = v {
            Ok(*x)
        }
        else if let Literal::StrongBoolean(x) = v {
            Ok(*x)
        }
        else {
            Err("Cannot cast Literal to bool")
        }
    }
}

impl From<i32> for Literal {
    fn from(v: i32) -> Literal {
        Literal::Number(v as f64)
    }
}

impl From<f32> for Literal {
    fn from(v: f32) -> Literal {
        Literal::Number(v as f64)
    }
}

impl From<f64> for Literal {
    fn from(v: f64) -> Literal {
        Literal::Number(v)
    }
}

impl From<bool> for Literal {
    fn from(v: bool) -> Literal {
        Literal::Boolean(v)
    }
}

impl From<&str> for Literal {
    fn from(v: &str) -> Literal {
        Literal::String(v.to_string())
    }
}

#[derive(Clone, Debug)]
pub struct LoxFunction {
    pub declaration : Function,
    pub closure : Environment,
    pub f_type : FunctionType,
    pub is_init : bool,
}

impl LoxFunction {
    pub fn new(declaration : Function, closure : Environment, f_type : FunctionType, is_init : bool) -> Self {
        Self {
            declaration,
            closure,
            f_type,
            is_init
        }
    }
    
    pub fn bind(&mut self, interpreter : &mut Interpreter, instance : &LoxInstance) -> LoxFunction {
        let mut environment = Environment::new_with_enclosing(self.closure.clone());
        environment.define(
            interpreter,
            "this".to_string(), 
            Some(Literal::Instance(Box::new(instance.clone())))
        );
        
        LoxFunction::new(self.declaration.clone(), environment, FunctionType::Method, self.is_init)
    }
}

impl LoxCallable for LoxFunction {
    fn get_name(&self) -> Token {
        *self.declaration.name.clone()
    }

    fn arity(&self) -> usize {
        self.declaration.params.len()
    }
    
    fn call(&self, interpreter : &mut Interpreter, callee : Token, arguments : Vec<Option<Literal>>, auto_clean : bool) -> RuntimeError<Option<Literal>> {
        let mut environment = self.closure.clone();
        environment.enclosing = Some(Box::new(interpreter.environment.clone()));

        interpreter.environment = environment.clone();
        
        for i in 0..self.declaration.params.len() {
            if let Some(binding) = self.declaration.params[i].binding.clone() {
                match (binding.type_, arguments[i].clone()) {
                    (TokenType::Number, Some(Literal::StrongNumber(_))) => {  }
                    (TokenType::Number, Some(Literal::Number(_))) => { }
                    (TokenType::String, Some(Literal::StrongString(_))) => { }
                    (TokenType::String, Some(Literal::String(_))) => { }
                    (TokenType::Bool, Some(Literal::StrongBoolean(_))) => { }
                    (TokenType::Bool, Some(Literal::Boolean(_))) => { }
                    _ => {
                        return Err((
                            callee.clone(), 
                            format!(
                                "Invalid arg for parameter '{}'. Expected type '{}'", 
                                self.declaration.params[i].name.lexeme.clone(),
                                binding.lexeme
                            )
                        ));
                    }
                }
            }
            
            let mut e = interpreter.environment.clone();
            e.define(interpreter,
                self.declaration.params[i].name.lexeme.clone(), 
                arguments[i].clone()
            );
            interpreter.environment = e;
        }
        
        let res = interpreter.execute_block(&self.declaration.body)?;
        
        if self.is_init {
            match interpreter.environment
            .get(interpreter, Token::new(TokenType::Identifier, "this", None, self.declaration.name.line)) {
                Ok(Some(v)) => {
                    if auto_clean {
                        if let Some(prev) = interpreter.environment.clone().enclosing {
                            interpreter.environment = *prev;
                        }
                    }
                    Ok(Some(Literal::Return(Box::new(v))))
                },
                Err(e) => {
                    Err(e)
                },
                _ => {
                    Err((Token::new(TokenType::Identifier, "this", None, self.declaration.name.line), "Couldnt find this.".to_string()))
                }
            }
        }
        else {
            if auto_clean {
                if let Some(prev) = interpreter.environment.clone().enclosing {
                    interpreter.environment = *prev;
                }
            }
            
            if let Some(Literal::Return(v)) = res {
                Ok(Some(*v))
            }
            else {
                Ok(None)
            }
        }
    }
    
    fn clone_dyn(&self) -> Box<dyn LoxCallable> {
        Box::new(self.clone())
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct LoxClass {
    pub name : String,
    pub methods : HashMap<String, LoxFunction>,
    pub super_class : Option<Box<LoxClass>>
}

impl LoxClass {
    pub fn new(name : String, methods : HashMap<String, LoxFunction>, super_class : Option<Box<LoxClass>>) -> Self {
        Self {
            name,
            methods,
            super_class
        }
    }
    
    pub fn find_method(&self, name : String) -> Option<LoxFunction> {
        if self.methods.contains_key(&name.clone()) {
            Some(self.methods[&name.clone()].clone())
        }
        else {
            if let Some(sc) = self.super_class.clone() {
                sc.find_method(name.clone())
            }
            else {
                None
            }
        }
    }
}

impl LoxCallable for LoxClass {
    fn get_name(&self) -> Token {
        Token::new(TokenType::Class, self.name.as_str(), None, 0)
    }

    fn arity(&self) -> usize {
        let initializer = self.find_method(self.get_name().lexeme.clone());
        if let Some(v) = initializer {
            v.arity()
        }
        else {
            0
        }
    }
    
    fn call(&self, interpreter : &mut Interpreter, callee : Token, arguments : Vec<Option<Literal>>, _auto_clean : bool) -> RuntimeError<Option<Literal>> {
        let mut instance = LoxInstance::new(Box::new(self.clone()));
        
        let initializer = self.find_method(self.get_name().lexeme.clone());
        
        if let Some(mut v) = initializer {
            if let Some(Literal::Return(r)) = 
            v.bind(interpreter, &instance).call(interpreter, callee.clone(), arguments, true)? {
                if let Literal::Instance(i) = *r {
                    instance = *i;
                }
            }
        }
        
        Ok(Some(Literal::Instance(Box::new(instance))))
    }
    
    fn clone_dyn(&self) -> Box<dyn LoxCallable> {
        Box::new(self.clone())
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct LoxInstance {
    pub class : Box<LoxClass>,
    pub fields : HashMap<String, Option<Literal>>,
    pub address : Option<usize>
}

impl LoxInstance {
    pub fn new(class : Box<LoxClass>) -> Self {
        Self {
            class,
            fields : HashMap::new(),
            address : None
        }
    }

    pub fn get_internal(&self, name : Token, interpreter : &Interpreter) -> RuntimeError<Option<Literal>> {
        if self.fields.contains_key(&name.lexeme) {
            if let Some(Literal::Instance(inst)) = self.fields[&name.lexeme].clone() {
                if let Some(address) = inst.address {
                    if let Some(a) = interpreter.references.get(address) {
                        Ok(a.clone())
                    }
                    else {
                        Err((name.clone(), format!("Undefined property '{}'.", name.lexeme.clone())))
                    }
                }
                else {
                    Err((name.clone(), format!("Undefined property '{}'.", name.lexeme.clone())))
                }
            }
            else {
                Ok(self.fields[&name.lexeme].clone())
            }
        }
        else {
            Err((name.clone(), format!("Undefined property '{}'.", name.lexeme.clone())))
        }
    }

    pub fn get(&self, name : Token, interpreter : &mut Interpreter) -> RuntimeError<Option<Literal>> {
        if self.fields.contains_key(&name.lexeme) {
            if let Some(Literal::Instance(inst)) = self.fields[&name.lexeme].clone() {
                if let Some(address) = inst.address {
                    if let Some(a) = interpreter.references.get(address) {
                        Ok(a.clone())
                    }
                    else {
                        Err((name.clone(), format!("Undefined property '{}'.", name.lexeme.clone())))
                    }
                }
                else {
                    Err((name.clone(), format!("Undefined property '{}'.", name.lexeme.clone())))
                }
            }
            else {
                Ok(self.fields[&name.lexeme].clone())
            }
        }
        else {
            if let Some(mut v) = self.class.find_method(name.lexeme.clone()) {
                Ok(Some(Literal::Function(Box::new(v.bind(interpreter, self)))))
            }
            else {
                Err((name.clone(), format!("Undefined property '{}'.", name.lexeme.clone())))
            }
        }
    }

    pub fn set(&mut self, name : Token, value : Option<Literal>, interpreter : &mut Interpreter) {
        if let Some(Literal::Instance(inst)) = value.clone() {
            if let Some(address) = inst.address {
                interpreter.references[address] = value.clone();
            }
        }
        
        self.fields.insert(name.lexeme.clone(), value.clone());
    }
}