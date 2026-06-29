// ast.rs
use crate::builtins::*;

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    List(String,Vec<Expr>),
    Dict(String,Vec<(Expr,Expr)>),
    LibList(String,String,Vec<Expr>),
    LibDict(String,String,Vec<(Expr,Expr)>),
    Void,
    ArrayDt(Box<Array>),
    BufferDt(Box<Buffer>),
    Variable(String),
    Call(String,Vec<Expr>),
    LibCall(String,String,Vec<Expr>),
    FuncInht(Frounct),
    Ref(usize),
    Deref(Box<Expr>),
    AddressOf(String),
}

#[derive(Debug, Clone)]
pub enum TypeKind{
    Int,
    Float,
    Str,
    Bool,
    Void,
    Ref,
    ArrayT,
    BufferT,
    FuncInht,
}

impl TypeKind {
    pub fn from_string(name: &str) -> Option<Self> {
        match name {
            "intg" => Some(TypeKind::Int),
            "flotg" => Some(TypeKind::Float),
            "sterg" => Some(TypeKind::Str),
            "nophe" => Some(TypeKind::Void),
            "booltg" => Some(TypeKind::Bool),
            _ => None,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Param{
    pub name:String,
    pub kind:TypeKind,
}
#[derive(Debug, Clone)]
pub struct Frounct{
    pub params:Vec<Param>,
    pub body:Vec<Stmt>,
}
impl Frounct{
    pub fn new(p:Vec<Param>,b:Vec<Stmt>)->Self{
        Self{params:p,body:b,}
    }
}
#[derive(Debug, Clone)]
pub struct Type{
    pub kind: TypeKind,
    pub value:Expr,
    pub ptr: Option<usize>,
}
impl Type{
    pub fn newVoid() -> Self{
        Self{value:Expr::Void,kind:TypeKind::Void,ptr:None}
    }
    pub fn new(b:TypeKind,a:Expr) -> Self{
        Self{kind:b,value:a,ptr:None}
    }
    pub fn __out__(&self) -> String{
        match &self.value{
            Expr::Int(x)=>{
                x.to_string()
            }
            Expr::Float(x)=>{
                x.to_string()
            }
            Expr::Str(x)=>{
                format!("\"{}\"",x)
            }
            Expr::Bool(x)=>{
                if *x{"trunth".to_string()
                }else{"frunth".to_string()}
            }
            Expr::Void=>{
                "noph".to_string()
            }
            Expr::ArrayDt(a)=>{
                (a.base.outFn)(&a.items)
            }
            Expr::BufferDt(a)=>{
                (a.base.outFn)(&a.mapp)
            }
            Expr::Variable(x)=>{
                x.clone()
            }
            Expr::FuncInht(_)=>{
                "<func>".to_string()
            }
            Expr::Ref(a)=>{
                format!("<repher:&{}>",a)
            }
            Expr::AddressOf(a)=>{
                format!("&{}",a)
            }
            Expr::Deref(_)=>{
                "<keonreph>".to_string()
            }
            _=>{"<METHAVEOT>".to_string()}
        }
    }

    pub fn __onfex_eval__(&self) -> Expr{
        self.value.clone()
    }
}

#[derive(Debug, Clone)]
pub enum Stmt{
    Expr(Expr),
    Assign(String,Expr),
    ReAssign(String,Expr),
    Return(Option<Expr>),
    Import(String),ImportAs(String,String),
    Mehen(Vec<Stmt>),
    FuncCre(String,Vec<Param>,Vec<Stmt>,TypeKind),
}