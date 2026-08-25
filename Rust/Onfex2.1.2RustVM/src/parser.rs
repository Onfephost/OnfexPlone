// ==========================
// parser.rs
// ==========================

use crate::ast::*;
use crate::lexer::Lexer;
use crate::token::Token;
use crate::token::TokenKind;
use crate::error::OnfexError;
use std::collections::HashMap;
use crate::OnfexDecimal::*;

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
                Ok(Expr::Decimal(OnfexDecimal::from_f64_auto(x)?))
            }
            TokenKind::String(x) => {
                self.advance()?;
                Ok(Expr::Str(x))
            }
            TokenKind::Bool(x) => {
                self.advance()?;
                Ok(Expr::Bool(x))
            }
            TokenKind::TyKd(x) => {
                self.advance()?;
                Ok(Expr::TypeKind(Box::new(x)))
            }
            TokenKind::Void => {
                self.advance()?;
                Ok(Expr::Void)
            }
            TokenKind::Not => {
                self.advance()?;
                Ok(Expr::Not(Box::new(self.expr()?)))
            }
            TokenKind::Minus => {
                self.advance()?;
                let e = self.expr()?;
                match e {
                    Expr::Int(x) => {
                        return Ok(Expr::Int(-x))
                    }
                    Expr::Float(x) => {
                        return Ok(Expr::Decimal(OnfexDecimal::from_f64_auto(-x)?))
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
                let node = self.dict_expr("meatris".to_string())?;
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
                // n!
                if matches!(self.getCurrent(),TokenKind::Clam){
                    self.advance()?;
                    ex = Expr::Macro(n);
                }
                else if matches!(self.getCurrent(),TokenKind::Minus){
                    self.expect(TokenKind::Minus)?;
                    self.expect(TokenKind::Gt)?;
                    let n2 = match self.getCurrent() {
                        TokenKind::Identifier(x) => x,
                            _ => return Err(self.unexpected("identifier after '->'")),
                    };
                    self.advance();
                    ex = Expr::ModVariable(n,n2);
                }
                // n::m
                else if matches!(self.getCurrent(), TokenKind::Colon) {
                    self.advance();
                    self.expect(TokenKind::Colon)?;
                    let n2 = match self.getCurrent() {
                        TokenKind::Identifier(x) => x,
                        _ => return Err(self.unexpected("identifier after '->'")),
                    };
                    self.advance()?;
                    ex = Expr::LibVariable(n,n2);
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
    // `dn`: içinde bulunulan strouct'un jenerik tip parametreleri (varsa).
    // Bir metod parametresi `v: T` gibi jenerik bir tip kullanıyorsa, `T`
    // burada `dn` içinde aranarak `TypeKind::Dynamic("T")` olarak kabul edilir.
    fn params(&mut self,dn:Option<Vec<String>>) -> Result<Vec<Param>, OnfexError> {
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
                TokenKind::Identifier(x) => {
                    if dn.clone().unwrap_or_default().contains(&x){
                        TypeKind::Dynamic(x)
                    }else{return Err(self.unexpected("a type (tanımsız jenerik tip)"))}
                }
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
        let mut sl:Vec<String> = vec![];
        self.advance()?; // consume 'urso'
        match self.getCurrent() {
            TokenKind::Identifier(x) => sl.push(x),
            _ => return Err(self.unexpected("library2 name")),
        };
        self.advance();
        while matches!(self.getCurrent(),TokenKind::Colon){
            self.advance();
            self.expect(TokenKind::Colon);
            match self.getCurrent() {
                TokenKind::Identifier(x) => sl.push(x),
                _ => return Err(self.unexpected("library2 name")),
            };
            self.advance()?;
        }
        let mut txt = String::new();
        for i in sl{
            txt.push_str(&format!("::{}",i));
        }
        txt = txt[2..txt.clone().len()].to_string();
        self.expect(TokenKind::Semi)?;
        Ok(StmtNode::new(Stmt::Import(txt), tok.line, tok.col))
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
    fn fields(&mut self,dn:Option<Vec<String>>) -> Result<HashMap<String,Field>,OnfexError>{
        let mut v:HashMap<String,Field> = HashMap::new();
        if matches!(self.getCurrent(), TokenKind::RBrace) {
            return Ok(v);
        }
        loop {
            if matches!(self.getCurrent(), TokenKind::RBrace) {
                break;
            }
            let pb = match self.getCurrent(){
                TokenKind::Pub =>{true}
                TokenKind::Priv => {false}
                _ => {return Err(self.unexpected("prub ophe prive esp wraithnosan"))}
            };
            self.advance()?;
            let n = match self.getCurrent() {
                TokenKind::Identifier(x) => x,
                _ => return Err(self.unexpected("texth")),
            };
            self.advance()?;
            self.expect(TokenKind::Colon)?;
            // alan tipi ya bilinen bir TyKd'dir, ya da bu strouct'un
            // `<T, U, ...>` listesinde bildirilmiş bir jenerik isimdir.
            let tykd = match self.getCurrent() {
                TokenKind::TyKd(x) => x,
                TokenKind::Identifier(x) => {
                    if dn.clone().unwrap_or_default().contains(&x){
                        TypeKind::Dynamic(x)
                    }else{return Err(self.unexpected("typect (tanımsız jenerik tip)"))}
                }
                _ => return Err(self.unexpected("typect")),
            };
            self.advance()?;
            v.insert(n.clone(),Field{glb:pb,name:n.clone(),typ:tykd});
            if !matches!(self.getCurrent(),TokenKind::Comma){
                break;
            }else{
                self.advance()?;
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
    // `dn`: içinde bulunulan strouct'un (varsa) jenerik tip parametreleri.
    // Fonksiyonun kendi `frounct name<T, U>(...)` listesi bunlarla
    // birleştirilir; ama Stmt::FuncCre'ye sadece bu fonksiyonun KENDİ
    // bildirdiği isimler kaydedilir (Rust'taki `impl<T> Foo<T> { frounct
    // bar<U>(...) }` senaryosunda T strouct'tan, U metoddan gelir).
    fn fc(&mut self,dn:Option<Vec<String>>) -> Result<StmtNode, OnfexError> {
        let tok = self.getCurrentToken();
        self.advance()?;
        let name = match self.getCurrent() {
            TokenKind::Identifier(x) => x,
            _ => return Err(self.unexpected("function name")),
        };
        self.advance()?;

        let own_generics = self.generic_params_list()?;
        let mut visible = dn.clone().unwrap_or_default();
        visible.extend(own_generics.iter().cloned());
        let vdn = Some(visible);

        self.expect(TokenKind::LParen)?;
        let pars = self.params(vdn.clone())?;
        self.expect(TokenKind::RParen)?;
        self.expect(TokenKind::Minus)?;
        self.expect(TokenKind::Gt)?;
        let out = match self.getCurrent() {
            TokenKind::TyKd(x) => x,
            TokenKind::Identifier(x) => {
                    if vdn.clone().unwrap_or_default().contains(&x){
                        TypeKind::Dynamic(x)
                    }else{return Err(self.unexpected("typect (tanımsız jenerik tip)"))}
                }
            _ => return Err(self.unexpected("a return type")),
        };
        self.advance()?;
        self.expect(TokenKind::LBrace)?;
        let mut body = vec![];
        while !matches!(self.getCurrent(), TokenKind::RBrace) {
            body.push(self.statement()?);
        }
        self.expect(TokenKind::RBrace)?;
        Ok(StmtNode::new(Stmt::FuncCre(name, own_generics, pars, body, out), tok.line, tok.col))
    }

    /// Mevcut token `<` ise `<T, U, ...>` jenerik tip parametre listesini
    /// ayrıştırır (kapanış `>` dahil) ve isimleri döndürür; `<` yoksa boş
    /// liste döner. `strouct Name<T,...>` ve `frounct name<T,...>(...)`
    /// tarafından ortak kullanılır.
    fn generic_params_list(&mut self) -> Result<Vec<String>, OnfexError> {
        let mut generics: Vec<String> = Vec::new();
        if !matches!(self.getCurrent(), TokenKind::Lt) {
            return Ok(generics);
        }
        self.advance()?;
        loop {
            let g = match self.getCurrent() {
                TokenKind::Identifier(x) => x,
                _ => return Err(self.unexpected("jenerik tip adı (örn. T)")),
            };
            self.advance()?;
            generics.push(g);
            if matches!(self.getCurrent(), TokenKind::Comma) {
                self.advance()?;
                continue;
            }
            break;
        }
        self.expect(TokenKind::Gt)?;
        Ok(generics)
    }
    // strouct Name<T, U, ...> { alanlar } impelnos { metodlar } (Rust'taki
    // generic struct sözdizimine benzer). <...> kısmı tamamen opsiyoneldir.
    fn srct(&mut self) -> Result<StmtNode, OnfexError> {
        let tok = self.getCurrentToken();
        self.advance()?; // 'strouct' anahtar kelimesini tüket
        let name = match self.getCurrent() {
            TokenKind::Identifier(x) => x,
            _ => return Err(self.unexpected("struct name")),
        };
        self.advance()?;

        // jenerik tip parametre listesi: <T, U, ...>
        let generics = self.generic_params_list()?;
        let dn = Some(generics.clone());

        self.expect(TokenKind::LBrace)?;
        let fields = self.fields(dn.clone())?;
        self.expect(TokenKind::RBrace)?;
        let mut body = HashMap::new();
        if matches!(self.getCurrent(),TokenKind::Impl){
            self.advance()?;
            self.expect(TokenKind::LBrace)?;
            while !matches!(self.getCurrent(), TokenKind::RBrace) {
                if matches!(self.getCurrent(),TokenKind::Func){
                    let res = self.fc(dn.clone())?.stmt;
                    match res.clone(){
                        Stmt::FuncCre(n,_,_,_,_) => {
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
        Ok(StmtNode::new(Stmt::StrctCre(name, generics, fields, body), tok.line, tok.col))
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
            TokenKind::Func => self.fc(None),
            
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
            TokenKind::Raise => {
                let tok = self.getCurrentToken();
                self.advance()?;
                let val = self.exprnode()?;
                self.expect(TokenKind::Semi)?;
                Ok(StmtNode::new(Stmt::Raise(val), tok.line, tok.col))
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
            TokenKind::Ifnt => {
                let tok = self.getCurrentToken();
                self.advance();
                self.expect(TokenKind::LParen)?;
                let cond = self.statement()?;
                self.expect(TokenKind::RParen)?;
                self.expect(TokenKind::LBrace)?;
                let mut body = vec![];
                while !matches!(self.getCurrent(), TokenKind::RBrace) {
                    body.push(self.statement()?);
                }
                self.expect(TokenKind::RBrace)?;
                if matches!(self.getCurrent(),TokenKind::Elsnt){
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
            TokenKind::Forp => {
                // forp x, y, z intf <ifade> { <gövde> }
                let tok = self.getCurrentToken();
                self.advance()?;
                let mut vars = vec![];
                self.expect(TokenKind::LParen);
                loop {
                    let n = match self.getCurrent() {
                        TokenKind::Identifier(x) => x,
                        _ => return Err(self.unexpected("döngü değişkeni adı")),
                    };
                    self.advance()?;
                    vars.push(n);
                    if matches!(self.getCurrent(), TokenKind::Comma) {
                        self.advance()?;
                        continue;
                    }
                    break;
                }
                self.expect(TokenKind::RParen);
                self.expect(TokenKind::Intf)?;
                self.expect(TokenKind::LParen);
                let iter_expr = self.exprnode()?;
                self.expect(TokenKind::RParen);
                self.expect(TokenKind::LBrace)?;
                let mut body = vec![];
                while !matches!(self.getCurrent(), TokenKind::RBrace) {
                    body.push(self.statement()?);
                }
                self.expect(TokenKind::RBrace)?;
                Ok(StmtNode::new(Stmt::Forp(vars, Box::new(iter_expr), body), tok.line, tok.col))
            }
            _ => {
                let e = self.exprnode()?;
                let (line, col) = (e.line, e.col);
                if matches!(self.getCurrent(), TokenKind::Equal){
                    self.advance();
                    let e2 = self.exprnode()?;
                    self.expect(TokenKind::Semi)?;
                    return Ok(StmtNode::new(Stmt::ValueAssign(e,e2), line, col))
                }
                self.expect(TokenKind::Semi)?;
                Ok(StmtNode::new(Stmt::ExprNode(e), line, col))
            }
        }
    }
    
    // parse
    pub fn parse(&mut self) -> Result<Vec<StmtNode>, OnfexError> {
        let mut v = vec![];
        
        while !matches!(self.getCurrent(), TokenKind::EOF) {
            v.push(self.statement()?);
        }
        Ok(v)
    }
}
