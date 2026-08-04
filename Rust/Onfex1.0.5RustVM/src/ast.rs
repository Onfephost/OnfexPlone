// ast.rs
use crate::builtins::*;
use crate::libs::libStrct::Library;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Vect(usize,Vec<Type>),
    Lib(Box<Library>),
    List(String, Vec<ExprNode>),
    Dict(String, Vec<(ExprNode, ExprNode)>),
    LibList(String, String, Vec<ExprNode>),
    LibDict(String, String, Vec<(ExprNode, ExprNode)>),
    Void,
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
}

#[derive(Debug, Clone)]
pub enum TypeKind {
    Int,
    Float,
    Str,
    Vect,
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
}

impl TypeKind {
    pub fn from_string(name: &str) -> Option<Self> {
        match name {
            "intg" => Some(TypeKind::Int),
            "flotg" => Some(TypeKind::Float),
            "sterg" => Some(TypeKind::Str),
            "nophe" => Some(TypeKind::Void),
            "booltg" => Some(TypeKind::Bool),
            "frounctTc" => Some(TypeKind::FuncInht),
            "strouctTc" => Some(TypeKind::StrctT),
            "Srel" => Some(TypeKind::Srel),
            "srel" => Some(TypeKind::srel),
            _ => None,
        }
    }
    pub fn to_string(&self) -> String{
        let res = match self{
            TypeKind::Int => {"intg"}
            TypeKind::Float => {"flotg"}
            TypeKind::Str => {"sterg"}
            TypeKind::Void => {"nophe"}
            TypeKind::Bool => {"booltg"}
            TypeKind::FuncInht => {"frounctTc"}
            TypeKind::StrctT => {"strouctTc"}
            TypeKind::Ref => {"repherfal"}
            TypeKind::ArrayT => {"vektöre"}
            TypeKind::BufferT => {"meatris"}
            TypeKind::Lib => {"lribrass"}
            TypeKind::Srel => {"Srel"}
            TypeKind::srel => {"srel"}
            _ => {"<METHAVEOT>"}
        };
        res.to_string()
    }
    pub fn equal(&self,t:TypeKind) -> bool{
        return  self.to_string() == t.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub kind: TypeKind,
    pub vararg:bool,
}
#[derive(Debug, Clone)]
pub struct Field {
    pub glb: bool,
    pub name: String,
    pub typ:TypeKind,
}

#[derive(Debug, Clone)]
pub struct Frounct {
    pub params: Vec<Param>,
    pub body: Vec<StmtNode>,
    pub out : TypeKind,
}

impl Frounct {
    pub fn new(p: Vec<Param>, b: Vec<StmtNode>,o:TypeKind) -> Self {
        Self { params: p, body: b ,out:o}
    }
}

#[derive(Debug, Clone)]
pub struct Type {
    pub kind: TypeKind,
    pub value: Expr,
    pub ptr: Option<usize>,
}

impl Type {
    pub fn newVoid() -> Self {
        Self { value: Expr::Void, kind: TypeKind::Void, ptr: None }
    }
    pub fn new(b: TypeKind, a: Expr) -> Self {
        Self { kind: b, value: a, ptr: None }
    }
    pub fn __out__(&self,st:bool) -> String {
        match &self.value {
            Expr::Int(x) => x.to_string(),
            Expr::Float(x) => x.to_string(),
            Expr::Str(x) => {
                if st{
                    format!("\"{}\"", x)
                }else{format!("\"{}\"", x)}
            }
            Expr::Bool(x) => {
                if *x { "trunth1".to_string() } else { "franth0".to_string() }
            }
            Expr::Void => "noph".to_string(),
            Expr::ArrayDt(a) => (a.base.outFn)(&a.items),
            Expr::BufferDt(a) => (a.base.outFn)(&a.mapp),
            Expr::MonoDt(a) => (a.base.outFn)(&a.value),
            Expr::Variable(x) => x.clone(),
            Expr::FuncInht(_) => "<frounct>".to_string(),
            Expr::StrctInht(s) => format!("<strouct {}>", s.name),
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
            _ => "<METHAVEOT>".to_string(),
        }
    }

    pub fn __onfex_eval__(&self) -> Expr {
        self.value.clone()
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Stmt {
    ExprNode(ExprNode),
    Assign(String, ExprNode),
    ReAssign(String, ExprNode),
    MemberAssign(ExprNode, String, ExprNode),
    Return(Option<ExprNode>),
    Import(String),
    Mod(String),
    Mehen(Vec<StmtNode>),
    FuncCre(String, Vec<Param>, Vec<StmtNode>, TypeKind),
    StrctCre(String, HashMap<String,Field>, HashMap<String,Stmt>),
    TypeLib(String,String),
    TypeMod(String,String),
    IfElse(Box<StmtNode>,Vec<StmtNode>,Option<Vec<StmtNode>>),
}