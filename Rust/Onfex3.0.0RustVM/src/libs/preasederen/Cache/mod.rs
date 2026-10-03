use crate::ast::*;
use crate::builtins::*;
use crate::error::OnfexError;
use std::collections::HashMap;
use std::io::{self, Write};
use crate::OnfexDecimal::*;

fn pars(s:String) -> Type {

    let input = s.clone();

    if let Ok(v) = input.parse::<i64>() {
        Type::new(TypeKind::Int,Expr::Int(v))
    } else if let Ok(v) = input.parse::<f64>() {
        Type::new(TypeKind::Decimal,Expr::Decimal(OnfexDecimal::from_f64_auto(v)))
    } else if let Ok(v) = input.parse::<bool>() {
        Type::new(TypeKind::Bool,Expr::Bool(v))
    } else {
        Type::new(TypeKind::Str,Expr::Str(input))
    }
}

fn prs(args:Vec<Type>,ft:HashMap<String,Type>) -> Result<Type,OnfexError>{
    if args.len() == 1 {
        let mut od = str_get(&args[0].value)?;
        return pars(od)
    } else {
        Err(OnfexError::runtime("preasnos asp 1 afon promter wraithnosfer",))
    }
}

fn str_get(x:&Expr) -> Result<String, OnfexError>{
    match x{
        Expr::Str(s)=>{Ok(s.clone())},
        _ => Err(OnfexError::runtime("Typect Ern:sterge esp wraithnosan")),
    }
}
pub fn load_funcs() -> HashMap<String, Fnc>{
    let mut funcs:HashMap<String, Fnc> = HashMap::new();
    funcs.insert("preasnos".to_string(), prs as Fnc);
    funcs
}

pub fn load_vars() -> HashMap<String,Type>{
    let mut vars = HashMap::new();
    let version = "0.0.0".to_string();
    vars.insert("verzen".to_string(), Type::new(TypeKind::Str,Expr::Str(version.clone())));
    vars
}

pub fn load_arrays() -> HashMap<String, ArrayType> {
    let mut arrays: HashMap<String, ArrayType> = HashMap::new();
    arrays
}

pub fn load_buffers() -> HashMap<String, BufferType> {
    let mut buffers: HashMap<String, BufferType> = HashMap::new();
    buffers
}

pub fn load_monos() -> HashMap<String, MonoType> {
    let mut monos: HashMap<String, MonoType> = HashMap::new();
    monos
}