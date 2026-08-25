use crate::ast::*;
use crate::builtins::*;
use crate::error::OnfexError;
use std::collections::HashMap;

fn quad_n(a: &Vec<Type>) -> Result<Type,OnfexError> {
    let types = vec![0,1,2,3];
    if a.clone().len() != 1 {
        return Err(OnfexError::runtime("quadCernos asp 1 promter gephnosfer"));
    }
    let d = a[0].clone();
    match d.clone().value {
        Expr::Int(x) => {
            if !types.contains(&x){
                return Err(OnfexError::runtime("quaterner asp intg 0,1,2,3 gephnosfer"));
            }
        }
        _ => {
            return Err(OnfexError::runtime("quaterner asp intg 0,1,2,3 gephnosfer"));
        }
    }
    let res = Type::new(TypeKind::MonoT, Expr::MonoDt(Box::new(Mono::new(d.clone(), quad_temp()))));
    Ok(res)
}


fn quad_new(a: Vec<Type>,ft:HashMap<String,Type>) -> Result<Type,OnfexError> {
    return quad_n(&a)
}

fn quad_temp() -> MonoType {
    MonoType::new("quad".to_string(), quad_out)
}

pub fn quad_out(v: &Type) -> String {
    let x = v.clone().__out__(false);
    format!("<quad({})>", x)
}

pub fn load_funcs() -> HashMap<String, Fnc>{
    let mut funcs:HashMap<String, Fnc> = HashMap::new();
    funcs.insert("quadCernos".to_string(), quad_new as Fnc);
    funcs
}

pub fn load_vars() -> HashMap<String,Type>{
    let mut vars = HashMap::new();
    let version = "1.0.0".to_string();
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
    monos.insert("quad".to_string(), quad_temp());
    monos
}