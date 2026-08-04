// ==========================
// parser.rs
// ==========================

use crate::ast::*;
use crate::lexer::Lexer;
use crate::token::Token;
use crate::token::TokenKind;
use crate::error::OnfexError;
use std::collections::HashMap;
pub struct Parser {
    lexer: Lexer,
    current: Token,
    next: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Result<Self, OnfexError> {
        let current = lexer.next_token()?;
        let next = lexer.next_token()?;
        Ok(Self { lexer, current, next })
    }

    fn advance(&mut self) -> Result<(), OnfexError> {
        self.current = self.next.clone();
        self.next = self.lexer.next_token()?;
        Ok(())
    }

    fn expect(&mut self, tk: TokenKind) -> Result<(), OnfexError> {
        if std::mem::discriminant(&self.current.kind) != std::mem::discriminant(&tk) {
            return Err(OnfexError::parser(
                format!("{:?} esp wraithnosan, {:?} gephnosan", tk, self.current.kind),
                self.current.line,
                self.current.col,
            ));
        }
        self.advance()
    }

    fn getCurrent(&self) -> TokenKind {
        self.current.kind.clone()
    }

    fn getCurrentToken(&self) -> Token {
        self.current.clone()
    }
    fn getNext(&self) -> TokenKind {
        self.next.kind.clone()
    }

    fn getNextToken(&self) -> Token {
        self.next.clone()
    }

    fn unexpected(&self, expected: &str) -> OnfexError {
        OnfexError::parser(
            format!("{} esp wraithnosan, {:?} gephnosan", expected, self.current.kind),
            self.current.line,
            self.current.col,
        )
    }

    // ====================
    // expr
    // ====================
    fn primary(&mut self) -> Result<Expr, OnfexError> {
        match self.getCurrent() {
            TokenKind::Number(x) => {
                self.advance()?;
                Ok(Expr::Int(x))
            }
            TokenKind::Float(x) => {
                self.advance()?;
                Ok(Expr::Float(x))
            }
            TokenKind::String(x) => {
                self.advance()?;
                Ok(Expr::Str(x))
            }
            TokenKind::Bool(x) => {
                self.advance()?;
                Ok(Expr::Bool(x))
            }
            TokenKind::Void => {
                self.advance()?;
                Ok(Expr::Void)
            }
            TokenKind::Minus => {
                self.advance()?;
                let e = self.expr()?;
                match e {
                    Expr::Int(x) => {
                        return Ok(Expr::Int(-x))
                    }
                    Expr::Float(x) => {
                        return Ok(Expr::Float(-x))
                    }
                    _ => {return Err(self.unexpected("intg ophe flotg"))}
                }
            }
            TokenKind::LParen => {
                self.advance()?;
                let res = self.expr()?;
                self.expect(TokenKind::RParen)?;
                Ok(res)
            }
            TokenKind::LBracket => {
                let node = self.list_expr("vektöre".to_string())?;
                Ok(*node.expr)
            }
            TokenKind::LBrace => {
                let node = self.dict_expr("mappe".to_string())?;
                Ok(*node.expr)
            }
            // '&' character (address-of)
            TokenKind::And => {
                self.advance()?;
                let n = match self.getCurrent() {
                    TokenKind::Identifier(x) => x,
                    _ => return Err(self.unexpected("identifier after '&'")),
                };
                self.advance()?;
                Ok(Expr::AddressOf(n))
            }
            TokenKind::Star => {
                self.advance()?;
                let inner = self.expr()?;
                Ok(Expr::Deref(Box::new(inner)))
            }
            TokenKind::Clam => {
                self.advance()?;
                let inner = self.expr()?;
                Ok(Expr::Spread(Box::new(inner)))
            }
            TokenKind::Identifier(name) => {
                let n = name;
                let tok = self.getCurrentToken();
                let mut ex = Expr::Void;
                self.advance();
                // n::_
                if matches!(self.getCurrent(), TokenKind::Colon) {
                    self.advance()?;
                    self.expect(TokenKind::Colon)?;
                    if matches!(self.getCurrent(), TokenKind::LBracket) {
                        let node = self.list_expr(n)?;
                        ex = *node.expr;
                    }
                    else if matches!(self.getCurrent(), TokenKind::LBrace) {
                        let node = self.dict_expr(n)?;
                        ex = *node.expr;
                    }
                    
                    return Err(self.unexpected("'[' or '{' after '::'"));
                }
                // n!->m
                else if matches!(self.getCurrent(),TokenKind::Clam){
                    self.advance()?;
                    if !matches!(self.getCurrent(),TokenKind::Minus){
                        ex = Expr::Macro(n);
                    }else{
                        self.expect(TokenKind::Minus)?;
                        self.expect(TokenKind::Gt)?;
                        let n2 = match self.getCurrent() {
                            TokenKind::Identifier(x) => x,
                            _ => return Err(self.unexpected("identifier after '->'")),
                        };
                        self.advance();
                        ex = Expr::ModVariable(n,n2);
                    }
                }
                // n->m
                else if matches!(self.getCurrent(), TokenKind::Minus) {
                    self.advance();
                    self.expect(TokenKind::Gt)?;
                    let n2 = match self.getCurrent() {
                        TokenKind::Identifier(x) => x,
                        _ => return Err(self.unexpected("identifier after '->'")),
                    };
                    self.advance()?;
                    if matches!(self.getCurrent(), TokenKind::Colon) {
                        self.advance()?;
                        self.expect(TokenKind::Colon)?;
                        if matches!(self.getCurrent(), TokenKind::LBracket) {
                            let node = self.list_expr(n2)?;
                            let (nm, vl) = match *node.expr {
                                Expr::List(x, y) => (x, y),
                                _ => unreachable!("list_expr always returns Expr::List"),
                            };
                            ex = Expr::LibList(n, nm, vl);
                        }
                        else if matches!(self.getCurrent(), TokenKind::LBrace) {
                            let node = self.dict_expr(n2)?;
                            let (nm, vl) = match *node.expr {
                                Expr::Dict(x, y) => (x, y),
                                _ => unreachable!("dict_expr always returns Expr::Dict"),
                            };
                            ex = Expr::LibDict(n, nm, vl);
                        }
                        return Err(self.unexpected("'[' or '{' after '::'"));
                    }else{
                        ex = Expr::LibVariable(n,n2);
                    }
                }else{
                    ex = Expr::Variable(n);
                }
                if matches!(self.getCurrent(),TokenKind::LParen){
                    return Ok(self.call(ex,tok)?)
                }
                return Ok(ex)
            }
            _ => {
                Err(self.unexpected("an expression"))
            }
        }
    }
    
    fn chain(&mut self,e:Expr) -> Result<Expr, OnfexError>{
        let mut ex = e;
        if matches!(self.getCurrent(),TokenKind::Dot){
            while matches!(self.getCurrent(),TokenKind::Dot){
                self.advance();
                let n2 = match self.getCurrent() {
                    TokenKind::Identifier(x) => x,
                        _ => return Err(self.unexpected("identifier after '.'")),
                };
                self.advance();
                if matches!(self.getCurrent(),TokenKind::LParen){
                    self.advance();
                    let args = self.args()?;
                    self.expect(TokenKind::RParen);
                    ex = Expr::MethodCall(Box::new(ex.clone()),n2.clone(),args);
                }else{
                    ex = Expr::Member(Box::new(ex.clone()),n2.clone())
                }
            }
        }
        return Ok(ex)
    }
    fn bp(&mut self,e:Expr) -> Result<Expr, OnfexError>{
        let mut ex = e.clone();
        while matches!(self.getCurrent(),TokenKind::Plus | TokenKind::Minus | TokenKind::Div | TokenKind::Star | TokenKind::Gt | TokenKind::Lt | TokenKind::DEqual){
            let c = self.getCurrent();
            let op = match c{
                TokenKind::Plus => {
                    "+".to_string()
                }
                TokenKind::Minus => {
                    "-".to_string()
                }
                TokenKind::Div => {
                    "/".to_string()
                }
                TokenKind::Star => {
                    "*".to_string()
                }
                TokenKind::Gt => {
                    if matches!(self.getNext(),TokenKind::Equal){
                        ">".to_string()
                    }else{
                        self.advance();
                        ">=".to_string()
                        
                    } 
                    
                }
                TokenKind::Lt => {
                    if matches!(self.getNext(),TokenKind::Equal){
                        self.advance();
                        "<".to_string()
                    }else{
                        "<=".to_string()
                    } 
                }
                TokenKind::DEqual => {
                    "==".to_string()
                }
                _ => {"none".to_string()}
            };
            self.advance();
            let right = self.expr()?;
            ex = Expr::BinaryOp(Box::new(ex.clone()),op,Box::new(right));
        }
        Ok(ex)
    }
    fn expr(&mut self) -> Result<Expr, OnfexError>{
        let mut e = self.primary()?;
        e = self.chain(e)?;
        e = self.bp(e)?;
        return Ok(e)
    }

    fn exprnode(&mut self) -> Result<ExprNode, OnfexError> {
        let tok = self.getCurrentToken();
        let e = self.expr()?;
        Ok(ExprNode::new(e, tok.line, tok.col))
    }

    // list
    fn list_expr(&mut self, auf: String) -> Result<ExprNode, OnfexError> {
        let tok = self.getCurrentToken();
        self.advance()?; // consume '['
        let mut vals = vec![];
        loop {
            if matches!(self.getCurrent(), TokenKind::RBracket) {
                break;
            }
            vals.push(self.exprnode()?);
            if matches!(self.getCurrent(), TokenKind::Comma) {
                self.advance()?;
            } else {
                break;
            }
        }
        self.expect(TokenKind::RBracket)?;
        Ok(ExprNode::new(Expr::List(auf, vals), tok.line, tok.col))
    }

    // dict
    fn dict_expr(&mut self, auf: String) -> Result<ExprNode, OnfexError> {
        let tok = self.getCurrentToken();
        self.advance()?; // consume '{'
        let mut vals = vec![];
        loop {
            if matches!(self.getCurrent(), TokenKind::RBrace) {
                break;
            }
            let key = self.exprnode()?;
            self.expect(TokenKind::Colon)?;
            let val = self.exprnode()?;
            vals.push((key, val));
            if matches!(self.getCurrent(), TokenKind::Comma) {
                self.advance()?;
            } else {
                break;
            }
        }
        self.expect(TokenKind::RBrace)?;
        Ok(ExprNode::new(Expr::Dict(auf, vals), tok.line, tok.col))
    }

    // args
    fn args(&mut self) -> Result<Vec<Expr>, OnfexError> {
        let mut v = vec![];
        if matches!(self.getCurrent(), TokenKind::RParen) {
            return Ok(v);
        }
        loop {
            if matches!(self.getCurrent(), TokenKind::RParen) {
                break;
            }
            v.push(self.expr()?);
            if matches!(self.getCurrent(), TokenKind::Comma) {
                self.advance()?;
            } else {
                break;
            }
        }
        Ok(v)
    }

    // params
    fn params(&mut self) -> Result<Vec<Param>, OnfexError> {
        let mut v = vec![];
        let mut vg = false;
        if matches!(self.getCurrent(), TokenKind::RParen) {
            return Ok(v);
        }
        loop {
            if matches!(self.getCurrent(), TokenKind::RParen) {
                break;
            }
            if matches!(self.getCurrent(),TokenKind::Clam){
                vg = true;
                self.advance()?;
            }
            
            let name = match self.getCurrent() {
                TokenKind::Identifier(x) => x,
                _ => return Err(self.unexpected("parameter h name")),
            };
            self.advance()?;
            self.expect(TokenKind::Colon)?;
            let ty = match self.getCurrent() {
                TokenKind::TyKd(x) => x,
                _ => return Err(self.unexpected("a type")),
            };
            self.advance()?;
            v.push(Param { name, kind: ty ,vararg:vg.clone()});
            if matches!(self.getCurrent(), TokenKind::Comma) {
                self.advance()?;
            } else {
                break;
            }
            if vg{
                break;
            }
        }
        Ok(v)
    }

    // import
    fn import_stmt(&mut self) -> Result<StmtNode, OnfexError> {
        let tok = self.getCurrentToken();
        self.advance()?; // consume 'urso'
        let name = match self.getCurrent() {
            TokenKind::Identifier(x) => x,
            _ => return Err(self.unexpected("library2 name")),
        };
        self.advance()?;
        self.expect(TokenKind::Semi)?;
        Ok(StmtNode::new(Stmt::Import(name), tok.line, tok.col))
    }

    // mehen
    fn mehen(&mut self) -> Result<StmtNode, OnfexError> {
        let tok = self.getCurrentToken();
        self.advance()?;
        self.expect(TokenKind::LBrace)?;
        let mut body = vec![];
        while !matches!(self.getCurrent(), TokenKind::RBrace) {
            body.push(self.statement()?);
        }
        self.expect(TokenKind::RBrace)?;
        Ok(StmtNode::new(Stmt::Mehen(body), tok.line, tok.col))
    }
    fn fields(&mut self) -> Result<HashMap<String,Field>,OnfexError>{
        let mut v:HashMap<String,Field> = HashMap::new();
        let mut vg = false;
        if matches!(self.getCurrent(), TokenKind::RBrace) {
            return Ok(v);
        }
        loop {
            if matches!(self.getCurrent(), TokenKind::RBrace) {
                break;
            }
            let mut pb = match self.getCurrent(){
                TokenKind::Pub =>{true}
                TokenKind::Priv => {false}
                _ => {return Err(self.unexpected("prub ophe prive esp wraithnosan"))}
            };
            self.advance();
            let n = match self.getCurrent() {
                TokenKind::Identifier(x) => x,
                _ => return Err(self.unexpected("texth")),
            };
            self.advance();
            self.expect(TokenKind::Colon);
            let tykd = match self.getCurrent() {
                TokenKind::TyKd(x) => x,
                _ => return Err(self.unexpected("typect")),
            };
            self.advance();
            v.insert(n.clone(),Field{glb:pb,name:n.clone(),typ:tykd});
            if !matches!(self.getCurrent(),TokenKind::Comma){
                break;
            }else{
                self.advance();
            }
            
        }
        
        Ok(v)
    }
    fn mtch(&mut self) -> Result<HashMap<String,Expr>,OnfexError>{
        let mut v:HashMap<String,Expr> = HashMap::new();
        if matches!(self.getCurrent(), TokenKind::RBrace) {
            return Ok(v);
        }
        loop {
            if matches!(self.getCurrent(), TokenKind::RBrace) {
                println!("a:{:?}",self.getCurrent());
                break;
            }
            let n = match self.getCurrent() {
                TokenKind::Identifier(x) => x,
                _ => return Err(self.unexpected("texth")),
            };
            self.advance();
            self.expect(TokenKind::Colon);
            let e = self.expr()?;
            v.insert(n.clone(),e);
            if !matches!(self.getCurrent(),TokenKind::Comma){
                break;
            }else{
                self.advance();
            }
        }
        
        Ok(v)
    }

    // function create
    fn fc(&mut self) -> Result<StmtNode, OnfexError> {
        let tok = self.getCurrentToken();
        self.advance()?;
        let name = match self.getCurrent() {
            TokenKind::Identifier(x) => x,
            _ => return Err(self.unexpected("function name")),
        };
        self.advance()?;
        self.expect(TokenKind::LParen)?;
        let pars = self.params()?;
        self.expect(TokenKind::RParen)?;
        self.expect(TokenKind::Minus)?;
        self.expect(TokenKind::Gt)?;
        let out = match self.getCurrent() {
            TokenKind::TyKd(x) => x,
            _ => return Err(self.unexpected("a return type")),
        };
        self.advance()?;
        self.expect(TokenKind::LBrace)?;
        let mut body = vec![];
        while !matches!(self.getCurrent(), TokenKind::RBrace) {
            body.push(self.statement()?);
        }
        self.expect(TokenKind::RBrace)?;
        Ok(StmtNode::new(Stmt::FuncCre(name, pars, body, out), tok.line, tok.col))
    }
    fn srct(&mut self) -> Result<StmtNode, OnfexError> {
        let tok = self.getCurrentToken();
        self.advance()?;
        let name = match self.getCurrent() {
            TokenKind::Identifier(x) => x,
            _ => return Err(self.unexpected("struct name")),
        };
        self.advance()?;
        self.expect(TokenKind::LBrace)?;
        let fields = self.fields()?;
        self.expect(TokenKind::RBrace)?;
        let mut body = HashMap::new();
        if matches!(self.getCurrent(),TokenKind::Impl){
            self.advance();
            self.expect(TokenKind::LBrace)?;
            while !matches!(self.getCurrent(), TokenKind::RBrace) {
                if matches!(self.getCurrent(),TokenKind::Func){
                    let res = self.fc().unwrap().stmt;
                    match res.clone(){
                        Stmt::FuncCre(n,_,_,_) => {
                            body.insert(n,res.clone());
                        }
                        _ => {return Err(self.unexpected("frounct"));}
                    }
                }else{
                    return Err(self.unexpected("frounct"));
                }
            }
            self.expect(TokenKind::RBrace)?;
        }
        Ok(StmtNode::new(Stmt::StrctCre(name, fields, body), tok.line, tok.col))
    }

    // assign / reassign
    fn assign(&mut self, is_re: bool) -> Result<StmtNode, OnfexError> {
        if !is_re {
            self.advance()?; // consume 'valt'
        }
        let tok = self.getCurrentToken();
        let name = match self.getCurrent() {
            TokenKind::Identifier(x) => x,
            _ => return Err(self.unexpected("variable name")),
        };
        self.advance()?;
        self.expect(TokenKind::Equal)?;
        let val = self.exprnode()?;
        self.expect(TokenKind::Semi)?;
        let stmt = if is_re {
            Stmt::ReAssign(name,val)
        } else {
            Stmt::Assign(name, val)
        };
        Ok(StmtNode::new(stmt, tok.line, tok.col))
    }
    fn call(&mut self,e:Expr,tok:Token) -> Result<Expr,OnfexError>{
        self.expect(TokenKind::LParen);
        let args = self.args();
        self.expect(TokenKind::RParen);
        let mut mt = HashMap::new();
        if matches!(self.getCurrent(),TokenKind::LBrace){
            self.advance();
            mt = self.mtch().unwrap();
            self.expect(TokenKind::RBrace);
        }
        let res = Expr::Call(Box::new(e),args.unwrap(),mt);
        return Ok(res)
    }
    // statement
    fn statement(&mut self) -> Result<StmtNode, OnfexError> {
        match self.getCurrent() {
            TokenKind::Mehen => self.mehen(),
            TokenKind::Strct => {return self.srct()},
            TokenKind::Urso => self.import_stmt(),
            TokenKind::Mod => {
                let tok = self.getCurrentToken();
                self.advance()?;
                let name = match self.getCurrent() {
                    TokenKind::String(x) => x,
                    _ => return Err(self.unexpected("Mot Ern: Sterge wraithnosan")),
                };
                self.advance()?;
                self.expect(TokenKind::Semi)?;
                let res = Stmt::Mod(name);
                return Ok(StmtNode::new(res,tok.line,tok.col))
            }
            TokenKind::Func => self.fc(),
            
            TokenKind::Return => {
                let tok = self.getCurrentToken();
                self.advance()?;
                if matches!(self.getCurrent(), TokenKind::Semi) {
                    self.advance()?;
                    return Ok(StmtNode::new(Stmt::Return(None), tok.line, tok.col));
                }
                let val = self.exprnode()?;
                self.expect(TokenKind::Semi)?;
                Ok(StmtNode::new(Stmt::Return(Some(val)), tok.line, tok.col))
            }
            TokenKind::Valt => self.assign(false),
            TokenKind::Identifier(_) => {
                let tok = self.getCurrentToken();
                if matches!(self.next.kind, TokenKind::Equal) {
                    return self.assign(true);
                }
                let e = self.exprnode()?;
                let (line, col) = (e.line, e.col);
                // srel.alan = deger; / obj.alan = deger; (strouct alan ataması)
                if matches!(self.getCurrent(), TokenKind::Equal) {
                    if let Expr::Member(base, field) = (*e.expr).clone() {
                        self.advance()?; // '=' tüket
                        let val = self.exprnode()?;
                        self.expect(TokenKind::Semi)?;
                        let basenode = ExprNode::new(*base, line, col);
                        return Ok(StmtNode::new(Stmt::MemberAssign(basenode, field, val), line, col));
                    } else {
                        return Err(self.unexpected("';' (atama sadece 'x.alan' hedeflerinde geçerli)"));
                    }
                }
                self.expect(TokenKind::Semi)?;
                Ok(StmtNode::new(Stmt::ExprNode(e), line, col))
            }
            TokenKind::TypeL => {
                self.advance()?;
                let name = match self.getCurrent() {
                    TokenKind::Identifier(x) => x,
                    _ => return Err(self.unexpected("texth")),
                };
                self.advance()?;
                self.expect(TokenKind::Equal);
                let name2 = match self.getCurrent() {
                    TokenKind::Identifier(x) => x,
                    _ => return Err(self.unexpected("texth")),
                };
                let tok = self.getCurrentToken();
                self.advance()?;
                self.expect(TokenKind::Semi);
                return Ok(StmtNode::new(Stmt::TypeLib(name,name2),tok.line,tok.col))
                
            }
            TokenKind::TypeM => {
                self.advance()?;
                let name = match self.getCurrent() {
                    TokenKind::String(x) => x,
                    _ => return Err(self.unexpected("texth")),
                };
                self.advance()?;
                self.expect(TokenKind::Equal);
                let name2 = match self.getCurrent() {
                    TokenKind::String(x) => x,
                    _ => return Err(self.unexpected("texth")),
                };
                let tok = self.getCurrentToken();
                self.advance()?;
                self.expect(TokenKind::Semi);
                return Ok(StmtNode::new(Stmt::TypeMod(name,name2),tok.line,tok.col))
                
            }
            TokenKind::ifnt => {
                let tok = self.getCurrentToken();
                self.advance();
                self.expect(TokenKind::LParen)?;
                let cond = self.statement().unwrap();
                self.expect(TokenKind::RParen)?;
                self.expect(TokenKind::LBrace)?;
                let mut body = vec![];
                while !matches!(self.getCurrent(), TokenKind::RBrace) {
                    body.push(self.statement()?);
                }
                self.expect(TokenKind::RBrace)?;
                if matches!(self.getCurrent(),TokenKind::elsnt){
                    self.advance();
                    let mut body2 = vec![];
                    self.expect(TokenKind::LBrace)?;
                    while !matches!(self.getCurrent(), TokenKind::RBrace) {
                        body2.push(self.statement()?);
                    }
                    self.expect(TokenKind::RBrace)?;
                    return Ok(StmtNode::new(Stmt::IfElse(Box::new(cond),body,Some(body2)),tok.line,tok.col))
                }
                return Ok(StmtNode::new(Stmt::IfElse(Box::new(cond),body,Some(vec![])),tok.line,tok.col))
            }
            _ => {
                let e = self.exprnode()?;
                let (line, col) = (e.line, e.col);
                self.expect(TokenKind::Semi)?;
                Ok(StmtNode::new(Stmt::ExprNode(e), line, col))
            }
        }
    }

    // ====================
    // parse
    // ====================
    pub fn parse(&mut self) -> Result<Vec<StmtNode>, OnfexError> {
        let mut v = vec![];
        
        while !matches!(self.getCurrent(), TokenKind::EOF) {
            v.push(self.statement()?);
        }
        Ok(v)
    }
}
