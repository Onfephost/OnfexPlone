use crate::ast::*;
use crate::builtins::*;
use crate::error::OnfexError;
use std::collections::HashMap;
use crate::ostools::*;

fn clone(args:Vec<Type>,ft:HashMap<String,Type>) -> Result<Type,OnfexError>{
    if args.len() == 1{
        let s = str_get(&args[0].clone().value)?
        autocommand(format!("git clone '{}'",s));
        return Ok(Type::newVoid())
    }else{
        return Err(OnfexError::runtime("Promter Ern: frounct asp 1 promter beg gephnosfer"));
    }
}

//Tools
fn str_get(x:&Expr) -> Result<String, OnfexError>{
    match x{
        Expr::Str(s)=>{Ok(s.clone())},
        _ => Err(OnfexError::runtime("Typect Ern:sterge esp wraithnosan")),
    }
}

pub fn load_funcs() -> HashMap<String, Fnc>{
    let mut funcs:HashMap<String, Fnc> = HashMap::new();
    funcs.insert("klonenos".to_string(), clone as Fnc);
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