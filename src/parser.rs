use crate::environment::Environment;
use crate::types::*;
use crate::tokens::*;
use crate::expressions::*;
use crate::statements::*;
use crate::app::*;

pub struct Parser {
    pub tokens : Vec<Token>,
    pub current : i32,
    pub in_class : bool,
    pub in_derived : bool
}

impl Parser {
    pub fn new(tokens : Vec<Token>) -> Self {
        Self {
            tokens,
            current : 0,
            in_class : false,
            in_derived : false
        }
    }
    
    pub fn expression(&mut self) -> RuntimeError<Box<dyn Expr>> {
        self.assignment()
    }
    
    pub fn assignment(&mut self) -> RuntimeError<Box<dyn Expr>> {
        let expr = self.or()?;
        
        if self.try_match(vec!(TokenType::Equal)) {
            let equals = self.previous();
            let value = self.assignment()?;

            if let Some(v) = expr.as_any().downcast_ref::<VarExpr>() {
                let name = &v.name;
                return Ok(Box::new(Assign::new(name.clone(), value, None)))
            }
            else if let Some(v) = expr.as_any().downcast_ref::<Get>() {
                return Ok(Box::new(Set::new(v.name.clone(), v.object.clone(), value.clone(), None)));
            }
            else if let Some(v) = expr.as_any().downcast_ref::<IndexGet>() {
                return Ok(Box::new(IndexSet::new(v.keyword.clone(), v.index.clone(), v.object.clone(), value.clone(), None)));
            }
            else {
                App::error(equals, "Invalid assignment target.");
                return Ok(expr) 
            }
        }
        else if self.try_match(vec!(TokenType::PlusEqual)) {
            let equals = self.previous();
            let value = self.assignment()?;
            
            if let Some(v) = expr.as_any().downcast_ref::<VarExpr>() {
                let name = &v.name;
                return Ok(Box::new(Assign::new(name.clone(), value, Some(TokenType::Plus))))
            }
            else if let Some(v) = expr.as_any().downcast_ref::<Get>() {
                return Ok(Box::new(Set::new(v.name.clone(), v.object.clone(), value.clone(), Some(TokenType::Plus))));
            }
            else if let Some(v) = expr.as_any().downcast_ref::<IndexGet>() {
                return Ok(Box::new(IndexSet::new(v.keyword.clone(), v.index.clone(), v.object.clone(), value.clone(), Some(TokenType::Plus))));
            }
            else {
                App::error(equals, "Invalid assignment target.");
                return Ok(expr) 
            }
        }
        else if self.try_match(vec!(TokenType::MinusEqual)) {
            let equals = self.previous();
            let value = self.assignment()?;

            if let Some(v) = expr.as_any().downcast_ref::<VarExpr>() {
                let name = &v.name;
                return Ok(Box::new(Assign::new(name.clone(), value, Some(TokenType::Minus))))
            }
            else if let Some(v) = expr.as_any().downcast_ref::<Get>() {
                return Ok(Box::new(Set::new(v.name.clone(), v.object.clone(), value.clone(), Some(TokenType::Minus))));
            }
            else if let Some(v) = expr.as_any().downcast_ref::<IndexGet>() {
                return Ok(Box::new(IndexSet::new(v.keyword.clone(), v.index.clone(), v.object.clone(), value.clone(), Some(TokenType::Minus))));
            }
            else {
                App::error(equals, "Invalid assignment target.");
                return Ok(expr) 
            }
        }
        else if self.try_match(vec!(TokenType::StarEqual)) {
            let equals = self.previous();
            let value = self.assignment()?;
            
            if let Some(v) = expr.as_any().downcast_ref::<VarExpr>() {
                let name = &v.name;
                return Ok(Box::new(Assign::new(name.clone(), value, Some(TokenType::Star))))
            }
            else if let Some(v) = expr.as_any().downcast_ref::<Get>() {
                return Ok(Box::new(Set::new(v.name.clone(), v.object.clone(), value.clone(), Some(TokenType::Star))));
            }
            else if let Some(v) = expr.as_any().downcast_ref::<IndexGet>() {
                return Ok(Box::new(IndexSet::new(v.keyword.clone(), v.index.clone(), v.object.clone(), value.clone(), Some(TokenType::Star))));
            }
            else {
                App::error(equals, "Invalid assignment target.");
                return Ok(expr) 
            }
        }
        else if self.try_match(vec!(TokenType::SlashEqual)) {
            let equals = self.previous();
            let value = self.assignment()?;
            
            if let Some(v) = expr.as_any().downcast_ref::<VarExpr>() {
                let name = &v.name;
                return Ok(Box::new(Assign::new(name.clone(), value, Some(TokenType::Slash))))
            }
            else if let Some(v) = expr.as_any().downcast_ref::<Get>() {
                return Ok(Box::new(Set::new(v.name.clone(), v.object.clone(), value.clone(), Some(TokenType::Slash))));
            }
            else if let Some(v) = expr.as_any().downcast_ref::<IndexGet>() {
                return Ok(Box::new(IndexSet::new(v.keyword.clone(), v.index.clone(), v.object.clone(), value.clone(), Some(TokenType::Slash))));
            }
            else {
                App::error(equals, "Invalid assignment target.");
                return Ok(expr) 
            }
        }
        else if self.try_match(vec!(TokenType::ModEqual)) {
            let equals = self.previous();
            let value = self.assignment()?;
            
            if let Some(v) = expr.as_any().downcast_ref::<VarExpr>() {
                let name = &v.name;
                return Ok(Box::new(Assign::new(name.clone(), value, Some(TokenType::Mod))))
            }
            else if let Some(v) = expr.as_any().downcast_ref::<Get>() {
                return Ok(Box::new(Set::new(v.name.clone(), v.object.clone(), value.clone(), Some(TokenType::Mod))));
            }
            else if let Some(v) = expr.as_any().downcast_ref::<IndexGet>() {
                return Ok(Box::new(IndexSet::new(v.keyword.clone(), v.index.clone(), v.object.clone(), value.clone(), Some(TokenType::Mod))));
            }
            else {
                App::error(equals, "Invalid assignment target.");
                return Ok(expr) 
            }
        }
        else if self.try_match(vec!(TokenType::Incr)) {
            let incr = self.previous();

            if let Some(v) = expr.as_any().downcast_ref::<VarExpr>() {
                let name = &v.name;
                return Ok(Box::new(Assign::new(name.clone(), Box::new(LiteralExp::new(Some(Literal::Number(1.0)))), Some(TokenType::Plus))))
            }
            else if let Some(v) = expr.as_any().downcast_ref::<Get>() {
                return Ok(Box::new(Set::new(v.name.clone(), v.object.clone(), Box::new(LiteralExp::new(Some(Literal::Number(1.0)))), Some(TokenType::Plus))));
            }
            else if let Some(v) = expr.as_any().downcast_ref::<IndexGet>() {
                return Ok(Box::new(IndexSet::new(v.keyword.clone(), v.index.clone(), v.object.clone(), Box::new(LiteralExp::new(Some(Literal::Number(1.0)))), Some(TokenType::Plus))));
            }
            else {
                App::error(incr, "Invalid assignment target.");
                return Ok(expr) 
            }
        }
        else if self.try_match(vec!(TokenType::Decr)) {
            let decr = self.previous();

            if let Some(v) = expr.as_any().downcast_ref::<VarExpr>() {
                let name = &v.name;
                return Ok(Box::new(Assign::new(name.clone(), Box::new(LiteralExp::new(Some(Literal::Number(1.0)))), Some(TokenType::Minus))))
            }
            else if let Some(v) = expr.as_any().downcast_ref::<Get>() {
                return Ok(Box::new(Set::new(v.name.clone(), v.object.clone(), Box::new(LiteralExp::new(Some(Literal::Number(1.0)))), Some(TokenType::Minus))));
            }
            else if let Some(v) = expr.as_any().downcast_ref::<IndexGet>() {
                return Ok(Box::new(IndexSet::new(v.keyword.clone(), v.index.clone(), v.object.clone(), Box::new(LiteralExp::new(Some(Literal::Number(1.0)))), Some(TokenType::Minus))));
            }
            else {
                App::error(decr, "Invalid assignment target.");
                return Ok(expr) 
            }
        }
        else {
            Ok(expr)
        }
    }
    
    pub fn synchronize(&mut self) {
        self.advance();
        
        while !self.is_at_end() {
            if self.previous().type_ == TokenType::Semicolon {
                break;
            }
            
            match self.peek().type_ {
                TokenType::Class => {},
                TokenType::Fn => {},
                TokenType::Var => {},
                TokenType::For => {},
                TokenType::If => {},
                TokenType::While => {},
                TokenType::Print => {},
                TokenType::Break => {},
                TokenType::Return => break,
                _ => break
            }
            
            self.advance();
        }
    }
    
    pub fn equality(&mut self) -> RuntimeError<Box<dyn Expr>> {
        let mut expr = self.comparison()?;
    
        while self.try_match(vec!(TokenType::BangEqual, TokenType::EqualEqual)) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Box::new(Binary::new(expr, operator, right));
        }
        
        Ok(expr)
    }
    
    pub fn or(&mut self) -> RuntimeError<Box<dyn Expr>> {
        let mut expr = self.and()?;
        
        if self.try_match(vec!(TokenType::Question)) {
            let operator_a = self.previous();
            let left = self.and()?;
            let operator_b = self.consume(TokenType::Colon, "Expect ':'.")?;
            let right = self.and()?;
            return Ok(Box::new(Ternary::new(expr, operator_a, left, operator_b, right)));
        }
        
        while self.try_match(vec!(TokenType::Or)) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Box::new(Logical::new(expr, operator, right));
        }
        
        Ok(expr)
    }
    
    pub fn and(&mut self) -> RuntimeError<Box<dyn Expr>> {
        let mut expr = self.equality()?;
        
        while self.try_match(vec!(TokenType::And)) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Box::new(Logical::new(expr, operator, right));
        }
        
        Ok(expr)
    }
    
    pub fn comparison(&mut self) -> RuntimeError<Box<dyn Expr>> {
        let mut expr = self.term()?;
        
        while self.try_match(vec!(TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual)) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Box::new(Binary::new(expr, operator, right));
        }
        
        Ok(expr)
    }
    
    pub fn term(&mut self) -> RuntimeError<Box<dyn Expr>> {
        let mut expr = self.factor()?;
        
        while self.try_match(vec!(TokenType::Minus, TokenType::Plus)) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Box::new(Binary::new(expr, operator, right));
        }
        
        Ok(expr)
    }
    
    pub fn factor(&mut self) -> RuntimeError<Box<dyn Expr>> {
        let mut expr = self.unary()?;
        
        while self.try_match(vec!(TokenType::Slash, TokenType::Star, TokenType::Mod)) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Box::new(Binary::new(expr, operator, right));
        }
        
        Ok(expr)
    }
    
    pub fn unary(&mut self) -> RuntimeError<Box<dyn Expr>> {
        if self.try_match(vec!(TokenType::Bang, TokenType::Minus)) {
            let operator = self.previous();
            let right = self.unary()?;
            Ok(Box::new(Unary::new(operator, right)))
        }
        else if self.try_match(vec!(TokenType::Hash)) {
            let operator = self.previous();
            let right = self.unary()?;
            Ok(Box::new(Unary::new(operator, right)))
        }
        else if self.try_match(vec!(TokenType::Incr, TokenType::Decr)) {
            let operator = self.previous();
            let right = self.unary()?;
            Ok(Box::new(Unary::new(operator, right)))
        }
        else if self.try_match(vec!(TokenType::LeftBracket)) {
            let mut inds = Vec::new();
            
            if !self.check(TokenType::RightBracket) {
                loop {
                    inds.push(self.expression()?);
                    
                    if !self.try_match(vec!(TokenType::Comma)) {
                        break;
                    }
                }
            }
            
            self.consume(TokenType::RightBracket, "Expect ']' after indexing.")?;
            Ok(Box::new(Index::new(inds)))
        } 
        else {
            self.call()
        }
    }
    
    pub fn call(&mut self) -> RuntimeError<Box<dyn Expr>> {
        let mut expr = self.primary()?;
        
        loop {
            if self.try_match(vec!(TokenType::LeftParen)) {
                expr = self.finish_call(expr)?;
            } 
            else if self.try_match(vec!(TokenType::Dot)) {
                let name = self.consume(TokenType::Identifier, "Expect property name after '.'.")?;
                expr = Box::new(Get::new(name, expr));
            }
            else if self.try_match(vec!(TokenType::LeftBracket)) {
                let index = self.expression()?;
                let keyword = self.consume(TokenType::RightBracket, "Expect ']' after index.")?;
                expr = Box::new(IndexGet::new(keyword, index, expr));
            }
            else {
                break;  
            }
        }
        
        Ok(expr)
    }
    
    pub fn finish_call(&mut self, callee : Box<dyn Expr>) -> RuntimeError<Box<dyn Expr>> {
        let mut arguments = Vec::new();
        
        if !self.check(TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    self.error(self.peek(), "Can't have mOre than 255 arguments.");
                }
                arguments.push(self.expression()?);
                
                if !self.try_match(vec!(TokenType::Comma)) {
                    break;
                }
            }
        }
        
        let paren = self.consume(TokenType::RightParen, "Expect ')' after args.")?;

        Ok(Box::new(Call::new(callee, paren, arguments)))
    }
    
    pub fn primary(&mut self) -> RuntimeError<Box<dyn Expr>> {
        if self.try_match(vec!(TokenType::False)) {
            Ok(Box::new(LiteralExp::new(Some(Literal::Boolean(false)))))
        }
        else if self.try_match(vec!(TokenType::True)) {
            Ok(Box::new(LiteralExp::new(Some(Literal::Boolean(true)))))
        }
        else if self.try_match(vec!(TokenType::Nil)) {
            Ok(Box::new(LiteralExp::new(None)))
        }
        else if self.try_match(vec!(TokenType::Number, TokenType::String)) {
            Ok(Box::new(LiteralExp::new(self.previous().literal)))
        }
        else if self.try_match(vec!(TokenType::LeftParen)) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            Ok(Box::new(Grouping::new(expr)))
        }
        else if self.try_match(vec!(TokenType::Super)) {
            let keyword = self.previous();
            
            match self.in_derived {
                true => {
                    self.consume(TokenType::Dot, "Expect '.' after 'super'.")?;
                    let method = self.consume(TokenType::Identifier, "Expect Super-class method name.")?;
            
                    Ok(Box::new(Super::new(keyword, method)))
                },
                false => Err((self.previous(), "Found 'super' outside of derived class.".to_string()))
            }
        }
        else if self.try_match(vec!(TokenType::This)) {
            match self.in_class {
                true => Ok(Box::new(This::new(self.previous()))),
                false => Err((self.previous(), "Found 'this' outside of class.".to_string()))
            }
        }
        else if self.try_match(vec!(TokenType::Identifier)) {
            Ok(Box::new(VarExpr::new(self.previous())))
        }
        else if self.try_match(vec!(TokenType::Pipe)) {
            match self.anon() {
                Ok(v) => Ok(v),
                Err((e, v)) => return Err((e, v))
            }
        }
        else {
            self.error(self.peek(), "Expect expression.");
            Ok(Box::new(LiteralExp::new(None)))
        }
    }
    
    pub fn parse(&mut self) -> RuntimeError<Vec<Box<dyn Stmt>>> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            if let Some(v) = self.declaration()? {
                statements.push(v);
            }
        }
        
        Ok(statements)
    }
    
    pub fn declaration(&mut self) -> RuntimeError<Option<Box<dyn Stmt>>> {
        if self.try_match(vec!(TokenType::Var)) {
            if let Ok(v) = self.var_declaration() {
                Ok(Some(v))
            }
            else {
                self.synchronize();
                Ok(None)
            }
        }
        else if self.try_match(vec!(TokenType::Class)) {
            match self.class_declaration() {
                Ok(v) => Ok(Some(v)),
                Err((e, v)) => Err((e, v))
            }
        }
        else if self.try_match(vec!(TokenType::Fn)) {
            match self.function("function") {
                Ok(v) => Ok(Some(v)),
                Err((e, v)) => Err((e, v))
            }
        }
        else {
            match self.statement() {
                Ok(v) => Ok(Some(v)),
                Err((e, v)) => Err((e, v))
            }
        }
    }
    
    pub fn class_declaration(&mut self) -> RuntimeError<Box<dyn Stmt>> {
        self.in_class = true;
        
        let name = self.consume(TokenType::Identifier, "Expect class name.")?;
        
        let mut super_class = None;
        if self.try_match(vec!(TokenType::Less)) {
            self.consume(TokenType::Identifier, "Expect super-class identifier.")?;
            super_class = Some(Box::new(VarExpr::new(self.previous())) as Box<dyn Expr>); 
            self.in_derived = true;
        }
        
        self.consume(TokenType::LeftBrace, "Expect '{' before class body.")?;
        
        let mut methods = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            methods.push(self.function("method")?);
        }
        
        self.consume(TokenType::RightBrace, "Expect '}' after class body")?;
        self.in_class = false;
        self.in_derived = false;
    
        Ok(Box::new(Class::new(name, methods, super_class)))
    }
    
    pub fn function(&mut self, kind : &str) -> RuntimeError<Box<dyn Stmt>> {
        let name = self.consume(TokenType::Identifier, format!("Expect {} name.", kind).as_str())?;
    
        self.consume(TokenType::LeftParen, format!("Expect {} name.", kind).as_str())?;
        let mut parameters = Vec::new();
        
        if !self.check(TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    self.error(self.peek(), "Can't have mOre than 255 parameters.");
                }
                
                parameters.push(self.consume(TokenType::Identifier, "Expect parameter name.")?);
                
                if !self.try_match(vec!(TokenType::Comma)) {
                    break;
                }
            } 
        }
        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;
        
        self.consume(TokenType::LeftBrace, format!("Expect '{{' before {} body.", kind).as_str())?;
    
        let body = self.block()?;
        
        Ok(Box::new(Function::new(name, parameters, body)))
    }

    pub fn anon(&mut self) -> RuntimeError<Box<dyn Expr>> {
        let mut parameters = Vec::new();
        
        if !self.check(TokenType::Pipe) {
            loop {
                if parameters.len() >= 255 {
                    self.error(self.peek(), "Can't have mOre than 255 parameters.");
                }
                
                parameters.push(self.consume(TokenType::Identifier, "Expect parameter name.")?);
                
                if !self.try_match(vec!(TokenType::Comma)) {
                    break;
                }
            } 
        }
        self.consume(TokenType::Pipe, "Expect '|' after parameters.")?;
        
        self.consume(TokenType::LeftBrace, "Expect '{{' before function body.")?;
    
        let body = self.block()?;
        
        Ok(Box::new(LiteralExp::new(Some(Literal::Function(Box::new(LoxFunction::new(Function::new(Token::new(TokenType::Identifier, "anon", None, 0), parameters, body), Environment::new(), FunctionType::Anon, false)))))))
    }
    
    pub fn anon_lambda(&mut self) -> RuntimeError<Box<dyn Expr>> {
        let mut parameters = Vec::new();
        
        if !self.check(TokenType::Pipe) {
            loop {
                if parameters.len() >= 255 {
                    self.error(self.peek(), "Can't have mOre than 255 parameters.");
                }
                
                parameters.push(self.consume(TokenType::Identifier, "Expect parameter name.")?);
                
                if !self.try_match(vec!(TokenType::Comma)) {
                    break;
                }
            } 
        }
        self.consume(TokenType::Pipe, "Expect '|' after parameters.")?;
    
        let mut body = vec!();
        
        if let Some(d) = self.declaration()? {
            if let Some(exp) = d.as_any().downcast_ref::<StmtExpr>() {
                body.push(Box::new(
                    Return::new(
                        Token::new(
                            TokenType::Return,
                            "return",
                            None,
                            0
                        ),
                        Some(exp.expression.clone())
                    )
                ) as Box<dyn Stmt>);
            }
            else {
                body.push(d);
            }
        }
        
        Ok(Box::new(LiteralExp::new(Some(Literal::Function(Box::new(LoxFunction::new(Function::new(Token::new(TokenType::Identifier, "anon", None, 0), parameters, body), Environment::new(), FunctionType::Anon, false)))))))
    }
    
    pub fn var_declaration(&mut self) -> RuntimeError<Box<dyn Stmt>> {
        let name = self.consume(TokenType::Identifier, "Expect Variable name.")?;
        
        let mut initializer : Option<Box<dyn Expr>> = None;
        if self.try_match(vec!(TokenType::Equal)) {
            initializer = Some(self.expression()?);
            self.consume(TokenType::Semicolon, "Expect ';' after Variable declaration.")?;
        }
        else if self.try_match(vec!(TokenType::Lambda)) {
            self.consume(TokenType::Pipe, "Expect '|' to declare lambda.")?;
            match self.anon_lambda() {
                Ok(v) => {
                    initializer = Some(v);
                },
                Err((e, v)) => return Err((e, v))
            }
        }
        else {
            self.consume(TokenType::Semicolon, "Expect ';' after Variable declaration.")?;
        }

        Ok(Box::new(Var::new(name, initializer)))
    }
    
    pub fn type_declaration(&mut self, type_of : TokenType) -> RuntimeError<Box<dyn Stmt>> {
        let name = self.consume(TokenType::Identifier, "Expect Variable name.")?;
        
        let mut initializer : Option<Box<dyn Expr>> = None;
        if self.try_match(vec!(TokenType::Equal)) {
            if let Some(exp) = self.expression()?.as_any().downcast_ref::<LiteralExp>() {
                match type_of {
                    TokenType::Number => {
                        if let Some(Literal::Number(a)) = exp.value.clone() {
                            initializer = 
                            Some(Box::new(
                                LiteralExp::new(
                                    Some(Literal::StrongNumber(a))
                                )
                            ));
                        }
                        else {
                            return Err((name.clone(), "Expect number.".to_string()))
                        }
                    }
                    TokenType::Bool => {
                        if let Some(Literal::Boolean(a)) = exp.value.clone() {
                            initializer = 
                            Some(Box::new(
                                LiteralExp::new(
                                    Some(Literal::StrongBoolean(a))
                                )
                            ));
                        }
                        else {
                            return Err((name.clone(), "Expect boolean.".to_string()))
                        }
                    }
                    TokenType::String => {
                        if let Some(Literal::String(a)) = exp.value.clone() {
                            initializer = 
                            Some(Box::new(
                                LiteralExp::new(
                                    Some(Literal::StrongString(a))
                                )
                            ));
                        }
                        else {
                            return Err((name.clone(), "Expect string.".to_string()))
                        }
                    }
                    _ => return Err((name.clone(), "Invalid type".to_string()))
                }
            }
            self.consume(TokenType::Semicolon, "Expect ';' after Variable declaration.")?;
        }

        Ok(Box::new(Var::new(name, initializer)))
    }

    pub fn statement(&mut self) -> RuntimeError<Box<dyn Stmt>> {
        if self.try_match(vec!(TokenType::If)) {
            self.if_statement()
        }
        else if self.try_match(vec!(TokenType::Print)) {
            self.print_statement()
        }
        else if self.try_match(vec!(TokenType::Return)) {
            self.return_statement()
        }
        else if self.try_match(vec!(TokenType::Break)) {
            self.break_statement()
        }
        else if self.try_match(vec!(TokenType::Continue)) {
            self.continue_statement()
        }
        else if self.try_match(vec!(TokenType::While)) {
            self.while_statement()
        }
        else if self.try_match(vec!(TokenType::For)) {
            self.for_statement()
        }
        else if self.try_match(vec!(TokenType::LeftBrace)) {
            Ok(Box::new(Block::new(self.block()?)))
        }
        else {
            self.expression_statement()
        }
    }
    
    pub fn block(&mut self) -> RuntimeError<Vec<Box<dyn Stmt>>> {
        let mut statements = Vec::new();
        
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            if let Some(v) = self.declaration()? {
                statements.push(v);
            }
        }
        
        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }
    
    pub fn if_statement(&mut self) -> RuntimeError<Box<dyn Stmt>> {
        let condition = self.expression()?;
        
        let then_branch = self.statement()?;
        let mut else_branch : Option<Box<dyn Stmt>> = None;
        let mut else_if_branch : Vec<Option<Box<dyn Stmt>>> = vec!();
        
        while self.try_match(vec!(TokenType::ElseIf)) {
            else_if_branch.push(Some(self.statement()?));
        }
        
        if self.try_match(vec!(TokenType::Else)) {
            else_branch = Some(self.statement()?);
        }
        
        Ok(Box::new(If::new(condition, then_branch, else_if_branch, else_branch)))
    }
    
    pub fn for_statement(&mut self) -> RuntimeError<Box<dyn Stmt>> {
        let mut loop_type = LoopType::For;
        let mut initializer : Option<Box<dyn Stmt>> = None;
        let mut initializer2 : Option<Box<dyn Stmt>> = None;
        
        let mut initializer_alt = None;
        
        if self.try_match(vec!(TokenType::Var)) {
            initializer = Some(self.var_declaration()?);
        }
        else if self.try_match(vec!(TokenType::Semicolon)) {
            initializer = None;
        }
        else {
            initializer_alt = Some(self.consume(TokenType::Identifier, "Expect identifier after for statement.")?);
        }
        
        let mut condition : Option<Box<dyn Expr>> = None;
        let mut increment : Option<Box<dyn Expr>> = None;
        let mut increment2 : Option<Box<dyn Expr>> = None;
        
        if !self.check(TokenType::Colon) {
            if !self.check(TokenType::Semicolon) {
                condition = Some(self.expression()?);
            } 
            
            self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;
    
            if !self.check(TokenType::RightParen) {
                increment = Some(self.expression()?);
            }
        }
        else {
            loop_type = LoopType::ForEach;
        
            self.consume(TokenType::Colon, "Expect ':' after for intializer.")?;
            let condition_val_alt = self.expression()?;
            
            let condition_val_alt_var = 
            if let Some(v) = condition_val_alt.as_any().downcast_ref::<Grouping>() {
                if let Some(v2) = v.expression.as_any().downcast_ref::<Unary>() {
                    v2.right.clone()
                }
                else {
                    condition_val_alt.clone()
                }
            }
            else if let Some(v) = condition_val_alt.as_any().downcast_ref::<Unary>() {
                v.right.clone()
            }
            else {
                condition_val_alt.clone()
            };

            if condition_val_alt_var.as_any().downcast_ref::<LiteralExp>().is_none() {
                initializer = 
                Some(
                    Box::new(Var::new(
                        initializer_alt.clone().unwrap(),
                        Some(
                            Box::new(IndexGet::new(
                                Token::new(
                                    TokenType::RightBracket,
                                    "]",
                                    None,
                                    initializer_alt.clone().unwrap().line
                                ),
                                Box::new(LiteralExp::new(
                                    Some(Literal::Number(0.0))
                                )),
                                condition_val_alt.clone()
                            ))
                        )
                    ))
                );
                
                initializer2 = 
                Some(
                    Box::new(Var::new(
                        Token::new(
                            TokenType::Identifier,
                            "_internal_iterator_lox_exlcusive_",
                            Some(Literal::Number(0.0)),
                            initializer_alt.clone().unwrap().line
                        ),
                        Some(
                            Box::new(LiteralExp::new(
                                Some(Literal::Number(0.0))
                            ))
                        )
                    ))
                );
            
                condition = 
                Some(
                    Box::new(Binary::new(
                        Box::new(VarExpr::new(
                            Token::new(
                                TokenType::Identifier,
                                "_internal_iterator_lox_exlcusive_",
                                Some(Literal::Number(0.0)),
                                initializer_alt.clone().unwrap().line
                            )
                        )),
                        Token::new(
                            TokenType::Less,
                            "<",
                            None,
                            initializer_alt.clone().unwrap().line
                        ),
                        Box::new(
                            Unary::new(
                                Token::new(
                                    TokenType::Hash,
                                    "#",
                                    None,
                                    initializer_alt.clone().unwrap().line
                                ),
                                condition_val_alt.clone()
                            )
                        )
                    ))
                );
                
                increment2 =
                Some(
                    Box::new(Assign::new(
                        initializer_alt.clone().unwrap(),
                        Box::new(IndexGet::new(
                            Token::new(
                                TokenType::RightBracket,
                                "]",
                                None,
                                initializer_alt.clone().unwrap().line
                            ),
                            Box::new(VarExpr::new(
                                Token::new(
                                    TokenType::Identifier,
                                    "_internal_iterator_lox_exlcusive_",
                                    None,
                                    initializer_alt.clone().unwrap().line
                                )
                            )),
                            condition_val_alt.clone()
                        )),
                        None
                    ))
                );
                
                increment =
                Some(
                    Box::new(Assign::new(
                        Token::new(
                            TokenType::Identifier,
                            "_internal_iterator_lox_exlcusive_",
                            None,
                            initializer_alt.clone().unwrap().line
                        ),
                        Box::new(LiteralExp::new(
                            Some(Literal::Number(1.0))
                        )),
                        Some(TokenType::Plus)
                    ))
                );
            }
            else {
                initializer = 
                Some(
                    Box::new(Var::new(
                        initializer_alt.clone().unwrap(),
                        Some(
                            Box::new(LiteralExp::new(
                                Some(Literal::Number(0.0))
                            ))
                        )
                    ))
                );
                
                condition = 
                Some(
                    Box::new(Binary::new(
                        Box::new(VarExpr::new(
                            initializer_alt.clone().unwrap()
                        )),
                        Token::new(
                            TokenType::Less,
                            "<",
                            None,
                            initializer_alt.clone().unwrap().line
                        ),
                        condition_val_alt_var.clone()
                    ))
                );
                
                increment =
                Some(
                    Box::new(Assign::new(
                        initializer_alt.clone().unwrap(),
                        Box::new(LiteralExp::new(
                            Some(Literal::Number(1.0))
                        )),
                        Some(TokenType::Plus)
                    ))
                );
            }
        }
        
        let mut body = self.statement()?;
        
        if let Some(v) = increment {
            if let Some(v2) = increment2 {
                body = Box::new(Block::new(vec!(body, Box::new(StmtExpr::new(v)), Box::new(StmtExpr::new(v2)))));
            }
            else {
                body = Box::new(Block::new(vec!(body, Box::new(StmtExpr::new(v)))));
            }
        }
        
        if let None = condition {
            condition = Some(Box::new(LiteralExp::new(Some(Literal::Boolean(true)))));
        }
        
        body = Box::new(While::new(condition.unwrap(), body, loop_type));
        
        if let Some(v) = initializer {
            if let Some(v2) = initializer2 {
                body = Box::new(Block::new(vec!(v, v2, body)));
            }
            else {
                body = Box::new(Block::new(vec!(v, body)));
            }    
        }

        Ok(body)
    }
    
    pub fn while_statement(&mut self) -> RuntimeError<Box<dyn Stmt>> {
        let condition = self.expression()?;
        let body = self.statement()?;
        
        Ok(Box::new(While::new(condition, body, LoopType::While)))
    }
    
    fn print_statement(&mut self) -> RuntimeError<Box<dyn Stmt>> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'print'.")?;
        let value = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after 'print'.")?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Box::new(Print::new(value)))
    }
    
    pub fn return_statement(&mut self) -> RuntimeError<Box<dyn Stmt>> {
        let keyword = self.previous();
        let mut value : Option<Box<dyn Expr>> = None;
        
        if !self.check(TokenType::Semicolon) {
            value = Some(self.expression()?);
        }
        
        self.consume(TokenType::Semicolon, "Expect ';' after Return value.")?;
        Ok(Box::new(Return::new(keyword, value)))
    }
    
    pub fn break_statement(&mut self) -> RuntimeError<Box<dyn Stmt>> {
        let keyword = self.previous();
        self.consume(TokenType::Semicolon, "Expect ';' after Break.")?;
        Ok(Box::new(Break::new(keyword)))
    }
    
    pub fn continue_statement(&mut self) -> RuntimeError<Box<dyn Stmt>> {
        let keyword = self.previous();
        self.consume(TokenType::Semicolon, "Expect ';' after Break.")?;
        Ok(Box::new(Continue::new(keyword)))
    }
    
    pub fn expression_statement(&mut self) -> RuntimeError<Box<dyn Stmt>> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Box::new(StmtExpr::new(expr)))
    }
    
    pub fn consume(&mut self, type_ : TokenType, message : &str) -> RuntimeError<Token> {
        if self.check(type_) {
            Ok(self.advance())
        }
        else {
            Err((self.tokens[self.current as usize].clone(), message.to_string()))
        }
    }
    
    pub fn error(&mut self, token : Token, message : &str) {
        App::error(token, message);
    }
    
    pub fn try_match(&mut self, t : Vec<TokenType>) -> bool {
        for t_ in t {
            if self.check(t_) {
                self.advance();
                return true;
            }
        }
        
        false
    }
    
    pub fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        
        self.previous()
    }
    
    pub fn is_at_end(&self) -> bool {
        self.peek().type_ == TokenType::Eof
    }
    
    pub fn peek(&self) -> Token {
        self.tokens.get(self.current as usize).unwrap().clone()
    }
    
    pub fn previous(&self) -> Token {
        self.tokens.get((self.current-1) as usize).unwrap().clone()
    }
    
    pub fn check(&self, type_to_check : TokenType) -> bool {
        if self.is_at_end() {
            false
        }
        else {
            self.peek().type_ == type_to_check
        }
    }
}