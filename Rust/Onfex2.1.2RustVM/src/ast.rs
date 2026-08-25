// ast.rs
use crate::builtins::*;
use crate::libs::libStrct::Library;
use crate::builtinsdata::{vector_out,map_out};
use std::collections::HashMap;
use std::rc::Rc;
use colored::{Colorize,ColoredString};
use crate::OnfexError;
use crate::OnfexDecimal::OnfexDecimal;
#[derive(Debug, Clone,PartialEq)]
pub struct ExprNode {
    pub expr: Box<Expr>,
    pub line: usize,
    pub col: usize,
}

impl ExprNode {
    pub fn new(e: Expr, l: usize, c: usize) -> Self {
        Self { expr: Box::new(e), line: l, col: c }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(i64),
    TypeKind(Box<TypeKind>),
    Float(f64),
    Map(Vec<String>,Box<Expr>),
    Decimal(OnfexDecimal),
    Str(String),
    ColStr(ColoredString),
    Bool(bool),
    Vect(usize,Vec<Type>),
    Matris(usize,Vec<(Type,Type)>),
    Lib(Box<Library>),
    List(String, Vec<ExprNode>),
    Dict(String, Vec<(ExprNode, ExprNode)>),
    LibDict(String, String, Vec<(ExprNode, ExprNode)>),
    Void,
    Aphe(Box<Expr>),
    Not(Box<Expr>),
    ArrayDt(Box<Array>),
    BufferDt(Box<Buffer>),
    MonoDt(Box<Mono>),
    StructDt(Rc<Struct>),
    StructMethod(Box<Struct>,String,Vec<Expr>),
    MethodCall(Box<Expr>,String,Vec<Expr>),
    Member(Box<Expr>,String),
    Variable(String),
    Call(Box<Expr>, Vec<Expr>,HashMap<String,Expr>),
    Macro(String),
    LibVariable(String,String),
    ModVariable(String,String),
    FuncInht(Frounct),
    StrctInht(StructType),
    Ref(usize),
    Deref(Box<Expr>),
    AddressOf(String),
    Spread(Box<Expr>),
    Sprd(Box<Type>),
    BinaryOp(Box<Expr>,String,Box<Expr>),
    Iter(Rc<Iter>),
}

#[derive(Debug, Clone)]
pub enum TypeKind {
    Dynamic(String),
    TypeKind,
    Int,
    Float,
    Decimal,
    Str,
    ColStr,
    Vect,
    Matris,
    Lib,
    Bool,
    Void,
    Ref,
    ArrayT,
    BufferT,
    MonoT,
    FuncInht,
    StrctT,
    Srel,
    srel,
    Sprd,
    Iter,
    Aphe,
}

impl TypeKind {
    pub fn from_string(name: &str) -> Option<Self> {
        match name {
            "intg" => Some(TypeKind::Int),
            "flotg" => Some(TypeKind::Float),
            "decmal" => Some(TypeKind::Decimal),
            "sterge" => Some(TypeKind::Str),
            "nophe" => Some(TypeKind::Void),
            "booltg" => Some(TypeKind::Bool),
            "arrey" => Some(TypeKind::ArrayT),
            "vek" => Some(TypeKind::Vect),
            "iterfale" => Some(TypeKind::Iter),
            "meatris" => Some(TypeKind::Matris),
            "frounctTc" => Some(TypeKind::FuncInht),
            "strouctTc" => Some(TypeKind::StrctT),
            "Srel" => Some(TypeKind::Srel),
            "srel" => Some(TypeKind::srel),
            "typect" => Some(TypeKind::TypeKind),
            "aphe" => Some(TypeKind::Aphe),
            _ => None,
        }
    }
    pub fn to_string(&self) -> String{
        match self{
            TypeKind::Dynamic(x) => x.clone(),
            TypeKind::Int => "intg".to_string(),
            TypeKind::Float => "flotg".to_string(),
            TypeKind::Decimal => "decmal".to_string(),
            TypeKind::Str => "sterg".to_string(),
            TypeKind::Void => "nophe".to_string(),
            TypeKind::Bool => "booltg".to_string(),
            TypeKind::FuncInht => "frounctTc".to_string(),
            TypeKind::StrctT => "strouctTc".to_string(),
            TypeKind::Ref => "repherfal".to_string(),
            TypeKind::ArrayT => "vektöre".to_string(),
            TypeKind::BufferT => "baffer".to_string(),
            TypeKind::Vect => "vek".to_string(),
            TypeKind::Matris => "meatris".to_string(),
            TypeKind::Lib => "lribrass".to_string(),
            TypeKind::Srel => "Srel".to_string(),
            TypeKind::srel => "srel".to_string(),
            TypeKind::Iter => "iterfale".to_string(),
            TypeKind::Aphe => "aphe".to_string(),
            _ => "<METHAVEOT>".to_string(),
        }
    }
    pub fn equal(&self,t:TypeKind) -> bool{
        return  self.to_string() == t.to_string()
    }
}
impl PartialEq for TypeKind{
    fn eq(&self,t:&TypeKind) -> bool{
        return self.to_string() == t.to_string()
    }
}

#[derive(Debug, Clone,PartialEq)]
pub struct Param {
    pub name: String,
    pub kind: TypeKind,
    pub vararg:bool,
}
#[derive(Debug, Clone,PartialEq)]
pub struct Field {
    pub glb: bool,
    pub name: String,
    pub typ:TypeKind,
}

#[derive(Debug, Clone,PartialEq)]
pub struct Frounct {
    pub name : String,
    pub generics: Vec<String>,
    pub params: Vec<Param>,
    pub body: Vec<StmtNode>,
    pub out : TypeKind,
}

impl Frounct {
    pub fn new(n:String, g: Vec<String>, p: Vec<Param>, b: Vec<StmtNode>, o:TypeKind) -> Self {
        Self {name:n, generics: g, params: p, body: b ,out:o}
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub kind: TypeKind,
    pub value: Expr,
    pub ptr: Option<usize>,
}

impl Type {
    pub fn newVoid() -> Self {
        Self { value: Expr::Void, kind: TypeKind::Void, ptr: None }
    }
    pub fn newAphe(a:Box<Expr>) -> Self {
        Self { value: Expr::Aphe(a), kind: TypeKind::Aphe, ptr: None }
    }
    pub fn new(b: TypeKind, a: Expr) -> Self {
        Self { kind: b, value: a, ptr: None }
    }
    pub fn __out__(&self,st:bool) -> String {
        match &self.value {
            Expr::Int(x) => x.to_string(),
            Expr::Float(x) => x.to_string(),
            Expr::Decimal(x) => format!("<Decmal{}|{}>",x.size(),x.to_string()),
            Expr::TypeKind(x) => x.to_string(),
            Expr::Str(x) => {
                if st{
                    format!("\"{}\"", x)
                }else{format!("{}", x)}
            }
            Expr::ColStr(x) => {
                if st{
                    format!("\"{}\"", x)
                }else{format!("{}", x)}
            }
            Expr::Bool(x) => {
                if *x { "trunth".to_string() } else { "franth".to_string() }
            }
            Expr::Void => "noph".to_string(),
            Expr::ArrayDt(a) => (a.base.outFn)(&a.items),
            Expr::BufferDt(a) => (a.base.outFn)(&a.mapp),
            Expr::MonoDt(a) => (a.base.outFn)(&a.value),
            Expr::Variable(x) => x.clone(),
            Expr::FuncInht(f) =>format!("<frounct|{}>",f.name),
            Expr::StrctInht(s) => format!("<strouct|{}>", s.name),
            Expr::StructDt(inst) => {
                let fld = inst.fld.borrow();
                let mut names: Vec<&String> = fld.keys().collect();
                names.sort();
                let mut vals = String::new();
                for n in names {
                    vals.push_str(&format!("{}: {}, ", n, fld.get(n).unwrap().__out__(true)));
                }
                if vals.len() >= 2 {
                    vals.truncate(vals.len() - 2);
                }
                format!("{} {{ {} }}", inst.base.name, vals)
            }
            Expr::Ref(a) => format!("<repherfal:&{}>", a),
            Expr::AddressOf(a) => format!("&{}", a),
            Expr::Deref(_) => "<keonreph>".to_string(),
            Expr::Vect(_,its) => vector_out(its),
            Expr::Matris(_,its) => map_out(its),
            Expr::Iter(it) => format!("<iterfal: {}/{}>", *it.pos.borrow(), it.items.len()),
            _ => "<METHAVEOT>".to_string(),
        }
    }

    pub fn __onfex_eval__(&self) -> Expr {
        self.value.clone()
    }
    pub fn to_string(&self,st:bool) -> String {
        self.__out__(st)
    }
    
    pub fn boolout(&self) -> Result<bool,OnfexError>{
        match &self.value{
            Expr::Bool(c) => Ok(*c),
            _ => return Err(OnfexError::runtime("Typect Ern"))
        }
    }
    pub fn as_bool(&self) -> Result<bool,OnfexError>{
        match &self.value{
            Expr::Bool(c) => Ok(*c),
            Expr::Void => Ok(false),
            _ => return Err(OnfexError::runtime("Typect Ern"))
        }
    }
    pub fn to_f64(&self) -> Result<f64,OnfexError>{
        match &self.value{
            Expr::Int(c) => Ok(c.clone() as f64),
            Expr::Float(c) => Ok(c.clone()),
            Expr::Decimal(c) => c.to_f64(),
            _ => return Err(OnfexError::runtime("Typect Ern"))
        }
    }
    pub fn to_i64(&self) -> Result<i64,OnfexError>{
        match &self.value{
            Expr::Int(c) => Ok(*c),
            Expr::Float(c) => Ok(*c as i64),
            Expr::Decimal(c) => Ok(c.clone().value as i64),
            _ => return Err(OnfexError::runtime("Typect Ern"))
        }
    }
}

#[derive(Debug, Clone,PartialEq)]
pub struct StmtNode {
    pub stmt: Stmt,
    pub line: usize,
    pub col: usize,
}

impl StmtNode {
    pub fn new(e: Stmt, l: usize, c: usize) -> Self {
        Self { stmt: e, line: l, col: c }
    }
}

#[derive(Debug, Clone,PartialEq)]
pub enum Stmt {
    ExprNode(ExprNode),
    Assign(String, ExprNode),
    ReAssign(String, ExprNode),
    ValueAssign(ExprNode, ExprNode),
    MemberAssign(ExprNode, String, ExprNode),
    Return(Option<ExprNode>),
    Import(String),
    Mod(String),
    Mehen(Vec<StmtNode>),
    FuncCre(String, Vec<String>, Vec<Param>, Vec<StmtNode>, TypeKind),
    StrctCre(String, Vec<String>, HashMap<String,Field>, HashMap<String,Stmt>),
    TypeLib(String,String),
    TypeMod(String,String),
    IfElse(Box<StmtNode>,Vec<StmtNode>,Option<Vec<StmtNode>>),
    Raise(ExprNode),
    Forp(Vec<String>, Box<ExprNode>, Vec<StmtNode>),
}