use crate::ast::*;
use crate::builtins::*;
use crate::error::OnfexError;
use std::collections::HashMap;



pub fn load_funcs() -> HashMap<String, Fnc>{
    let mut funcs:HashMap<String, Fnc> = HashMap::new();
    //funcs.insert("test".to_string(), test as Fnc);
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