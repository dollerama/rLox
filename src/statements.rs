use std::any::Any;

use crate::tokens::*;
use crate::types::*;

#[derive(Clone, Debug)]
pub struct Print {
    pub expression : Box<dyn Expr>,
}

impl Print {
    pub fn new(expression : Box<dyn Expr>) -> Self {
        Self {
            expression
        }
    }
}

#[derive(Clone, Debug)]
pub struct Return {
    pub name : Token,
    pub value : Option<Box<dyn Expr>>,
}

impl Return {
    pub fn new(name : Token, value : Option<Box<dyn Expr>>) -> Self {
        Self {
            name,
            value
        }
    }
}

#[derive(Clone, Debug)]
pub struct Break {
    pub name : Token,
}

impl Break {
    pub fn new(name : Token) -> Self {
        Self {
            name,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Continue {
    pub name : Token,
}

impl Continue {
    pub fn new(name : Token) -> Self {
        Self {
            name,
        }
    }
}

#[derive(Clone, Debug)]
pub struct If {
    pub condition : Box<dyn Expr>,
    pub then_branch : Box<dyn Stmt>,
    pub else_if_branch : Vec<Option<Box<dyn Stmt>>>,
    pub else_branch : Option<Box<dyn Stmt>>
}

impl If {
    pub fn new(
        condition : Box<dyn Expr>,
        then_branch : Box<dyn Stmt>,
        else_if_branch : Vec<Option<Box<dyn Stmt>>>,
        else_branch : Option<Box<dyn Stmt>>
    ) -> Self {
        Self {
            condition,
            then_branch,
            else_if_branch,
            else_branch
        }
    }
}

#[derive(Clone, Debug)]
pub struct Block {
    pub statements : Vec<Box<dyn Stmt>>
}

impl Block {
    pub fn new(statements : Vec<Box<dyn Stmt>>) -> Self {
        Self { 
            statements
        }
    }
}

#[derive(Clone, Debug)]
pub struct Class {
    pub name : Token,
    pub methods : Vec<Box<dyn Stmt>>,
    pub super_class : Option<Box<dyn Expr>>
}

impl Class {
    pub fn new(name : Token, methods : Vec<Box<dyn Stmt>>, super_class : Option<Box<dyn Expr>>) -> Self {
        Self { 
            name,
            methods,
            super_class
        }
    }
}

#[derive(Clone, Debug)]
pub struct StmtExpr {
    pub expression : Box<dyn Expr>,
}

impl StmtExpr {
    pub fn new(expression : Box<dyn Expr>) -> Self {
        Self {
            expression
        }
    }
}

#[derive(Clone, Debug)]
pub struct Var {
    pub name : Token,
    pub initializer : Option<Box<dyn Expr>>,
}

impl Var {
    pub fn new(name : Token, initializer : Option<Box<dyn Expr>>) -> Self {
        Self {
            name,
            initializer
        }
    }
}

#[derive(Clone, Debug)]
pub struct While {
    pub condition : Box<dyn Expr>,
    pub body : Box<dyn Stmt>,
    pub is_for_loop : bool
}

impl While {
    pub fn new(condition : Box<dyn Expr>, body : Box<dyn Stmt>, is_for_loop : bool) -> Self {
        Self {
            condition,
            body,
            is_for_loop
        }
    }
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name : Box<Token>,
    pub params : Vec<Token>,
    pub body : Vec<Box<dyn Stmt>>
}

impl Function {
    pub fn new(name : Token, params : Vec<Token>, body : Vec<Box<dyn Stmt>>) -> Self {
        Self {
            name : Box::new(name),
            params,
            body
        }
    }
}

impl Stmt for StmtExpr {
    fn accept(&self, visitor : &mut Box<&mut dyn StmtVisitor>)  -> RuntimeError<Option<Literal>> {
        visitor.visit_expression_stmt(self)
    }
    fn clone_dyn(&self) -> Box<dyn Stmt> {
        Box::new(self.clone()) 
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Stmt for Var {
    fn accept(&self, visitor : &mut Box<&mut dyn StmtVisitor>) -> RuntimeError<Option<Literal>> {
        visitor.visit_var_stmt(self)
    }
    fn clone_dyn(&self) -> Box<dyn Stmt> {
        Box::new(self.clone()) 
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Stmt for Print {
    fn accept(&self, visitor : &mut Box<&mut dyn StmtVisitor>) -> RuntimeError<Option<Literal>> {
        visitor.visit_print_stmt(self)
    }
    
    fn clone_dyn(&self) -> Box<dyn Stmt> {
        Box::new(self.clone())
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Stmt for Block {
    fn accept(&self, visitor : &mut Box<&mut dyn StmtVisitor>) -> RuntimeError<Option<Literal>> {
        visitor.visit_block_stmt(self)
    }
    
    fn clone_dyn(&self) -> Box<dyn Stmt> {
        Box::new(self.clone())
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Stmt for Class {
    fn accept(&self, visitor : &mut Box<&mut dyn StmtVisitor>) -> RuntimeError<Option<Literal>> {
        visitor.visit_class_stmt(self)
    }
    
    fn clone_dyn(&self) -> Box<dyn Stmt> {
        Box::new(self.clone())
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Stmt for If {
    fn accept(&self, visitor : &mut Box<&mut dyn StmtVisitor>) -> RuntimeError<Option<Literal>> {
        visitor.visit_if_stmt(self)
    }
    
    fn clone_dyn(&self) -> Box<dyn Stmt> {
        Box::new(self.clone())
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Stmt for While {
    fn accept(&self, visitor : &mut Box<&mut dyn StmtVisitor>) -> RuntimeError<Option<Literal>> {
        visitor.visit_while_stmt(self)
    }
    
    fn clone_dyn(&self) -> Box<dyn Stmt> {
        Box::new(self.clone())
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Stmt for Function {
    fn accept(&self, visitor : &mut Box<&mut dyn StmtVisitor>) -> RuntimeError<Option<Literal>> {
        visitor.visit_function_stmt(self)
    }
    
    fn clone_dyn(&self) -> Box<dyn Stmt> {
        Box::new(self.clone())
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Stmt for Return {
    fn accept(&self, visitor : &mut Box<&mut dyn StmtVisitor>) -> RuntimeError<Option<Literal>> {
        visitor.visit_return_stmt(self)
    }
    
    fn clone_dyn(&self) -> Box<dyn Stmt> {
        Box::new(self.clone())
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Stmt for Break {
    fn accept(&self, visitor : &mut Box<&mut dyn StmtVisitor>) -> RuntimeError<Option<Literal>> {
        visitor.visit_break_stmt(self)
    }
    
    fn clone_dyn(&self) -> Box<dyn Stmt> {
        Box::new(self.clone())
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Stmt for Continue {
    fn accept(&self, visitor : &mut Box<&mut dyn StmtVisitor>) -> RuntimeError<Option<Literal>> {
        visitor.visit_continue_stmt(self)
    }
    
    fn clone_dyn(&self) -> Box<dyn Stmt> {
        Box::new(self.clone())
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}