use std::time::Duration;
use std::thread;
use crate::ast::*;
use crate::builtins::*;
use crate::error::OnfexError;
use std::collections::HashMap;

fn timeSleep(args:Vec<Type>,ft:HashMap<String,Type>) -> Result<Type, OnfexError>{
    if args.len() == 1{
        sleep(int_get(&args[0].value)?);
        return Ok(Type::newVoid());
    }else{
        return Err(OnfexError::runtime("Promter Ern: 1 afon promter esp wraithnosap"))
    }
}

//Tools
fn sleep(len:i128) {
    thread::sleep(Duration::from_secs(len.try_into().unwrap()));
}

fn int_get(x:&Expr) -> Result<i128, OnfexError>{
    match x{
        Expr::Int(s)=>{Ok(s.clone().into())},
        _ => Err(OnfexError::runtime("Typect Ern: Intg esp wraithnosap")),
    }
}

pub fn load_funcs() -> HashMap<String, Fnc>{
    let mut funcs:HashMap<String, Fnc> = HashMap::new();
    funcs.insert("wraithnos".to_string(), timeSleep as Fnc);
    funcs
}

pub fn load_vars() -> HashMap<String,Type>{
    let mut vars = HashMap::new();
    let version = "0.4.0".to_string();
    vars.insert("verzen".to_string(),Type::new(TypeKind::Str,Expr::Str(version.clone())));
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