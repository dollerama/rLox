use std::any::Any;
use crate::types::*;
use crate::tokens::*;

#[derive(Clone, Debug)]
pub struct Binary {
    pub left : Box<dyn Expr>,
    pub right : Box<dyn Expr>,
    pub operator : Token
}

impl Binary {
    pub fn new(left : Box<dyn Expr>, operator : Token, right : Box<dyn Expr>) -> Self {
        Self {
            left,
            right, 
            operator
        }
    }
}

#[derive(Clone, Debug)]
pub struct Grouping {
    pub expression : Box<dyn Expr>,
}

impl Grouping {
    pub fn new(expression : Box<dyn Expr>) -> Self {
        Self {
            expression
        }
    }
}

#[derive(Clone, Debug)]
pub struct Get {
    pub name : Token,
    pub object : Box<dyn Expr>,
}

impl Get {
    pub fn new(name : Token, object : Box<dyn Expr>) -> Self {
        Self {
            name,
            object
        }
    }
}

#[derive(Clone, Debug)]
pub struct Set {
    pub name : Token,
    pub object : Box<dyn Expr>,
    pub value : Box<dyn Expr>,
    pub assign_type : Option<TokenType>
}

impl Set {
    pub fn new(name : Token, object : Box<dyn Expr>, value : Box<dyn Expr>, assign_type : Option<TokenType>) -> Self {
        Self {
            name,
            object,
            value,
            assign_type
        }
    }
}

#[derive(Clone, Debug)]
pub struct Super {
    pub keyword : Token,
    pub method : Token
}

impl Super {
    pub fn new(keyword : Token, method : Token) -> Self {
        Self {
            keyword,
            method
        }
    }
}

#[derive(Clone, Debug)]
pub struct LiteralExp {
    pub value : Option<Literal>
}

impl LiteralExp {
    pub fn new(value : Option<Literal>) -> Self {
        Self {
            value
        }
    }
}

#[derive(Clone, Debug)]
pub struct Unary {
    pub operator : Token,
    pub right : Box<dyn Expr>
}

impl Unary {
    pub fn new(operator : Token, right : Box<dyn Expr>) -> Self {
        Self {
            operator, 
            right
        }
    }
}

#[derive(Clone, Debug)]
pub struct VarExpr {
    pub name : Token,
}

impl VarExpr {
    pub fn new(name : Token) -> Self {
        Self {
            name, 
        }
    }
}

#[derive(Clone, Debug)]
pub struct Assign {
    pub name : Token,
    pub assign_type : Option<TokenType>,
    pub value : Box<dyn Expr>
}

impl Assign {
    pub fn new(name : Token, value : Box<dyn Expr>, assign_type : Option<TokenType>) -> Self {
        Self {
            name,
            value,
            assign_type
        }
    }
}

#[derive(Clone, Debug)]
pub struct Logical {
    pub left : Box<dyn Expr>,
    pub operator : Token,
    pub right : Box<dyn Expr>
}

impl Logical {
    pub fn new(left : Box<dyn Expr>, operator : Token, right : Box<dyn Expr>) -> Self {
        Self {
            left,
            right, 
            operator
        }
    }
}

#[derive(Clone, Debug)]
pub struct Call {
    pub callee : Box<dyn Expr>,
    pub paren : Token,
    pub arguments : Vec<Box<dyn Expr>>
}

impl Call {
    pub fn new(callee : Box<dyn Expr>, paren : Token, arguments : Vec<Box<dyn Expr>>) -> Self {
        Self {
            callee,
            paren, 
            arguments
        }
    }
}

#[derive(Clone, Debug)]
pub struct This {
    pub keyword : Token
}

impl This {
    pub fn new(keyword : Token) -> Self {
        Self {
            keyword
        }
    }
}
    
#[derive(Clone, Debug)]
pub struct Index {
    pub collection : Vec<Box<dyn Expr>>
}

impl Index {
    pub fn new(collection : Vec<Box<dyn Expr>>) -> Self {
        Self {
            collection
        }
    }
}

#[derive(Clone, Debug)]
pub struct IndexGet {
    pub keyword : Token,
    pub index : Box<dyn Expr>,
    pub object : Box<dyn Expr>,
}

impl IndexGet {
    pub fn new(keyword : Token, index : Box<dyn Expr>, object : Box<dyn Expr>) -> Self {
        Self {
            keyword,
            index,
            object
        }
    }
}

#[derive(Clone, Debug)]
pub struct IndexSet {
    pub name : Token,
    pub index : Box<dyn Expr>,
    pub object : Box<dyn Expr>,
    pub value : Box<dyn Expr>,
    pub assign_type : Option<TokenType>
}

impl IndexSet {
    pub fn new(name : Token, index : Box<dyn Expr>, object : Box<dyn Expr>, value : Box<dyn Expr>, assign_type : Option<TokenType>) -> Self {
        Self {
            name,
            index,
            object,
            value,
            assign_type
        }
    }
}

#[derive(Clone, Debug)]
pub struct Ternary {
    pub condition : Box<dyn Expr>,
    pub operator_a : Token,
    pub left : Box<dyn Expr>,
    pub operator_b : Token,
    pub right : Box<dyn Expr>
}

impl Ternary {
    pub fn new(condition : Box<dyn Expr>, operator_a : Token, left : Box<dyn Expr>,
    operator_b : Token, right : Box<dyn Expr>) -> Self {
        Self {
            condition,
            operator_a,
            left,
            operator_b,
            right
        }
    }
}

impl Expr for VarExpr {
    fn accept(&self, visitor : &mut Box<&mut dyn ExprVisitor>) -> RuntimeError<Option<Literal>> {
        visitor.visit_var_expr(self)
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_dyn(&self) -> Box<dyn Expr> {
        Box::new(self.clone()) 
    }
}

impl Expr for Unary { 
    fn accept(&self, visitor : &mut Box<&mut dyn ExprVisitor>) -> RuntimeError<Option<Literal>> {
        visitor.visit_unary_expr(self)
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_dyn(&self) -> Box<dyn Expr> {
        Box::new(self.clone()) 
    }
}

impl Expr for LiteralExp { 
    fn accept(&self, visitor : &mut Box<&mut dyn ExprVisitor>) -> RuntimeError<Option<Literal>> {
        visitor.visit_literal_expr(self)
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_dyn(&self) -> Box<dyn Expr> {
        Box::new(self.clone()) 
    }
}

impl Expr for Binary {
    fn accept(&self, visitor : &mut Box<&mut dyn ExprVisitor>) -> RuntimeError<Option<Literal>> {
        visitor.visit_binary_expr(self)
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_dyn(&self) -> Box<dyn Expr> {
        Box::new(self.clone()) 
    }
}

impl Expr for Logical {
    fn accept(&self, visitor : &mut Box<&mut dyn ExprVisitor>) -> RuntimeError<Option<Literal>> {
        visitor.visit_logical_expr(self)
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_dyn(&self) -> Box<dyn Expr> {
        Box::new(self.clone()) 
    }
}

impl Expr for Grouping { 
    fn accept(&self, visitor : &mut Box<&mut dyn ExprVisitor>) -> RuntimeError<Option<Literal>> {
        visitor.visit_grouping_expr(self)
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_dyn(&self) -> Box<dyn Expr> {
        Box::new(self.clone()) 
    }
}

impl Expr for Assign { 
    fn accept(&self, visitor : &mut Box<&mut dyn ExprVisitor>) -> RuntimeError<Option<Literal>> {
        visitor.visit_assign_expr(self)
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_dyn(&self) -> Box<dyn Expr> {
        Box::new(self.clone()) 
    }
}

impl Expr for Call {
    fn accept(&self, visitor : &mut Box<&mut dyn ExprVisitor>) -> RuntimeError<Option<Literal>> {
        visitor.visit_call_expr(self)
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_dyn(&self) -> Box<dyn Expr> {
        Box::new(self.clone()) 
    }
}

impl Expr for Get {
    fn accept(&self, visitor : &mut Box<&mut dyn ExprVisitor>) -> RuntimeError<Option<Literal>> {
        visitor.visit_get_expr(self)
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_dyn(&self) -> Box<dyn Expr> {
        Box::new(self.clone()) 
    }
}

impl Expr for Set {
    fn accept(&self, visitor : &mut Box<&mut dyn ExprVisitor>) -> RuntimeError<Option<Literal>> {
        visitor.visit_set_expr(self)
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_dyn(&self) -> Box<dyn Expr> {
        Box::new(self.clone()) 
    }
}

impl Expr for Super {
    fn accept(&self, visitor : &mut Box<&mut dyn ExprVisitor>) -> RuntimeError<Option<Literal>> {
        visitor.visit_super_expr(self)
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_dyn(&self) -> Box<dyn Expr> {
        Box::new(self.clone()) 
    }
}

impl Expr for This {
    fn accept(&self, visitor : &mut Box<&mut dyn ExprVisitor>) -> RuntimeError<Option<Literal>> {
        visitor.visit_this_expr(self)
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_dyn(&self) -> Box<dyn Expr> {
        Box::new(self.clone()) 
    }
}
    
impl Expr for Index {
    fn accept(&self, visitor : &mut Box<&mut dyn ExprVisitor>) -> RuntimeError<Option<Literal>> {
        visitor.visit_index_expr(self)
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_dyn(&self) -> Box<dyn Expr> {
        Box::new(self.clone()) 
    }
}

impl Expr for IndexGet {
    fn accept(&self, visitor : &mut Box<&mut dyn ExprVisitor>) -> RuntimeError<Option<Literal>> {
        visitor.visit_index_get_expr(self)
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_dyn(&self) -> Box<dyn Expr> {
        Box::new(self.clone()) 
    }
}

impl Expr for IndexSet {
    fn accept(&self, visitor : &mut Box<&mut dyn ExprVisitor>) -> RuntimeError<Option<Literal>> {
        visitor.visit_index_set_expr(self, vec!())
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_dyn(&self) -> Box<dyn Expr> {
        Box::new(self.clone()) 
    }
}

impl Expr for Ternary {
    fn accept(&self, visitor : &mut Box<&mut dyn ExprVisitor>) -> RuntimeError<Option<Literal>> {
        visitor.visit_ternary_expr(self)
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_dyn(&self) -> Box<dyn Expr> {
        Box::new(self.clone()) 
    }
}