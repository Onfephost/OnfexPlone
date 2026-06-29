// ==========================
// parser.rs
// ==========================

use crate::ast::*;
use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser{
    lexer:Lexer,
    current:Token,
    next:Token,
}

impl Parser{

    pub fn new(mut lexer:Lexer)->Self{
        let current=lexer.next_token();
        let next=lexer.next_token();
        Self{lexer,current,next,}
    }

    fn advance(&mut self){
        self.current=self.next.clone();
        self.next=self.lexer.next_token();
    }
    fn expect(&mut self,tk:Token){
        if std::mem::discriminant(&self.current)!=std::mem::discriminant(&tk){
            panic!("expected {:?}, got {:?}",tk,self.current);
        }
        self.advance();
    }
    // ====================
    // expr
    // ====================
    fn expr(&mut self)->Expr{
        match &self.current{
            Token::Number(x)=>{
                let v=Expr::Int(*x);
                self.advance();
                v
            }
            Token::Float(x)=>{
                let v=Expr::Float(*x);
                self.advance();
                v
            }
            Token::String(x)=>{
                let v=Expr::Str(x.clone());
                self.advance();
                v
            }
            Token::Bool(x)=>{
                let v=Expr::Bool(*x);
                self.advance();
                v
            }
            Token::LBracket =>{
                return self.list_expr("vektöre".to_string());
            }
            Token::LBrace =>{
                return self.dict_expr("mappe".to_string());
            }
            Token::Void=>{self.advance();Expr::Void}
            // & karakteri
            Token::And => {
                self.advance();
                let n=match self.current.clone(){
                        Token::Identifier(x)=>x,
                        _=>panic!("ident expected")
                    };
                self.advance();
                return Expr::AddressOf(n);
            }
            Token::Star => {
                self.advance();
                let res = Expr::Deref(Box::new(self.expr()));
                return res;
            }
            Token::Identifier(name)=>{
                let n=name.clone();
                self.advance();
                // func
                if matches!(self.current,Token::LParen){
                    self.advance();
                    let args=self.args();
                    self.expect(Token::RParen);
                    return Expr::Call(n,args);
                }
                
                else if matches!(self.current,Token::Colon){
                    self.advance();
                    self.expect(Token::Colon);
                    if matches!(self.current,Token::LBracket){
                        return self.list_expr(n);
                    }
                    if matches!(self.current,Token::LBrace){
                        return self.dict_expr(n);
                    }
                
                }
                else if matches!(self.current,Token::Minus){
                    self.advance();
                    self.expect(Token::Gt);
                    let n2=match self.current.clone(){
                        Token::Identifier(x)=>x,
                        _=>panic!("ident expected")
                    };
                    self.advance();
                    if matches!(self.current,Token::LParen){
                        self.advance();
                        let args=self.args();
                        self.expect(Token::RParen);
                        return Expr::LibCall(n,n2,args);
                    }
                    else if matches!(self.current,Token::Colon){
                        self.advance();
                        self.expect(Token::Colon);
                        if matches!(self.current,Token::LBracket){
                            let res =  self.list_expr(n2.clone());
                            let (nm,vl)=match res{
                                Expr::List(x,y)=>(x,y),
                                _=>panic!()
                            };
                            return Expr::LibList(n,nm,vl);
                        }
                        if matches!(self.current,Token::LBrace){
                            let res = self.dict_expr(n2.clone());
                            let (nm,vl)=match res{
                                Expr::Dict(x,y)=>(x,y),_=>panic!()
                            };
                            return Expr::LibDict(n,nm,vl);
                        }
                    }
                }
                Expr::Variable(n)
            }

            _=>panic!("expr error {:?}",&self.current),
        }
    }
    // list
    fn list_expr(&mut self,auf:String)->Expr{
        self.advance();
        let mut vals=vec![];
        loop{
            if matches!(self.current,Token::RBracket){
                break;
            }
            vals.push(self.expr());
            if matches!(self.current,Token::Comma){
                self.advance();
            }else{
                break;
            }
        }
        self.expect(Token::RBracket);
        Expr::List(auf,vals)
    }
    // dict
    fn dict_expr(&mut self,auf:String)->Expr{
        self.advance();
        let mut vals=vec![];
        loop{
            if matches!(self.current,Token::RBrace){
                break;
            }
            let key=self.expr();
            self.expect(Token::Colon);
            let val=self.expr();
            vals.push((key,val));
            if matches!(self.current,Token::Comma){
                self.advance();
            }else{
                break;
            }
        }
        self.expect(Token::RBrace);
        Expr::Dict(auf,vals)
    }
    
    // args
    fn args(&mut self)->Vec<Expr>{
        let mut v=vec![];
        if matches!(self.current,Token::RParen){
            return v;
        }
        loop{
            v.push(self.expr());
            if matches!(self.current,Token::Comma){
                self.advance();
            }else{
                break;
            }
        }
        v
    }
    
    // params
    fn params(&mut self)->Vec<Param>{
        let mut v=vec![];
        if matches!(self.current,Token::RParen){
            return v;
        }
        loop{
            let name=match &self.current{
                Token::Identifier(x)=>{x.clone()}
                _=>panic!("param expected")
            };
            self.advance();
            self.expect(Token::Colon);
            let ty=match &self.current{Token::TyKd(x)=>{x.clone()}_=>panic!("type expected")};
            self.advance();
            v.push(Param{name,kind:ty,});
            if matches!(self.current,Token::Comma){
                self.advance();
            }else{
                break;
            }
        }
        v
    }
    // import
    fn import_stmt(&mut self)->Stmt{
        self.advance();
        let name=match self.current.clone(){
            Token::Identifier(x)=>x,
            _=>panic!("import name expected")
        };
        self.advance();
        if matches!(self.current,Token::As){
            self.advance();
            let name2=match self.current.clone(){
                Token::Identifier(x)=>x,
                _=>panic!("import name expected")
            };
            self.advance();
            self.expect(Token::Semi);
            return Stmt::ImportAs(name,name2);
        }
        self.expect(Token::Semi);
        Stmt::Import(name)
    }
    
    // mehen
    fn mehen(&mut self)->Stmt{
        self.advance();
        self.expect(Token::LParen);
        self.expect(Token::RParen);
        self.expect(Token::LBrace);
        let mut body=vec![];
        while !matches!(self.current,Token::RBrace){
            body.push(self.statement());
        }
        self.expect(Token::RBrace);
        Stmt::Mehen(body)
    }
    
    // function create
    fn fc(&mut self)->Stmt{
        self.advance();
        let name=match &self.current{
            Token::Identifier(x)=>{x.clone()}
            _=>panic!("func name expected")
        };
        self.advance();
        self.expect(Token::LParen);
        let pars=self.params();
        self.expect(Token::RParen);
        self.expect(Token::Minus);
        self.expect(Token::Gt);
        let out=match &self.current{
            Token::TyKd(x)=>{x.clone()}
            _=>panic!("type expected")
            };
        self.advance();
        self.expect(Token::LBrace);
        let mut body=vec![];
        while !matches!(self.current,Token::RBrace){
            body.push(self.statement());
        }
        self.expect(Token::RBrace);
        Stmt::FuncCre(name,pars,body,out)
    }
    // assign
    fn assign(&mut self,is_re:bool)->Stmt{
        if !is_re{self.advance();}

        let name=match &self.current{
            Token::Identifier(x)=>{x.clone()},
            _=>panic!("ident expected")
        };
        self.advance();
        self.expect(Token::Equal);

        let val=self.expr();
        self.expect(Token::Semi);

        if is_re{
            Stmt::ReAssign(name,val)
        }else{
            Stmt::Assign(name,val)
        }
    }
    // statement
    fn statement(&mut self)->Stmt{

        match &self.current{
            Token::Mehen=>{self.mehen()},
            Token::Urso=>{self.import_stmt()},
            Token::Func=>{self.fc()},
            Token::Return=>{
                self.advance();
                let val=self.expr();
                self.expect(Token::Semi);
                Stmt::Return(Some(val))
            },

            Token::Valt=>{self.assign(false)},
            Token::Identifier(_)=>{
                if matches!(self.next,Token::Equal){
                    return self.assign(true);
                }
                let e=self.expr();

                self.expect(Token::Semi);

                Stmt::Expr(e)
            },
            _=>{
                let e=self.expr();
                self.expect(Token::Semi);
                Stmt::Expr(e)
            }
        }
    }
    // ====================
    // parse
    // ====================
    pub fn parse(&mut self)->Vec<Stmt>{
        let mut v=vec![];
        while !matches!(self.current,Token::EOF){
            v.push(self.statement());
        }
        v
    }
}