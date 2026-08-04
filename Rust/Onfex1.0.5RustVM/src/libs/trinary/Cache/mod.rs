use crate::ast::*;
use crate::builtins::*;
use crate::error::OnfexError;
use std::collections::HashMap;

fn trin_n(a: &Vec<Type>) -> Result<Type,OnfexError> {
    if a.clone().len() != 1 {
        return Err(OnfexError::runtime("trinCernos asp 1 promter gephnosfer"));
    }
    let d = a[0].clone();
    match d.clone().value {
        Expr::Int(x) => {
            if !(x <= 1 && x>=-1){
                return Err(OnfexError::runtime("trinCernos asp intg -1,0,1 gephnosfer"));
            }
        }
        _ => {
            return Err(OnfexError::runtime("trinCernos asp intg gephnosfer"));
        }
    }
    let res = Type::new(TypeKind::MonoT, Expr::MonoDt(Box::new(Mono::new(d.clone(), trin_temp()))));
    Ok(res)
}
fn trin_new(a: Vec<Type>,ft:HashMap<String,Type>) -> Result<Type,OnfexError> {
    return trin_n(&a)
}
fn trin_temp() -> MonoType {
    MonoType::new("neomArrey".to_string(), trin_out)
}
pub fn trin_out(v: &Type) -> String {
    let res = v.clone().__out__(false);
    format!("<triner({})>", res)
}

pub fn load_funcs() -> HashMap<String, Fnc>{
    let mut funcs:HashMap<String, Fnc> = HashMap::new();
    funcs.insert("trinCernos".to_string(), trin_new as Fnc);
    funcs
}

pub fn load_vars() -> HashMap<String,Type>{
    let mut vars = HashMap::new();
    let version = "0.5.0".to_string();
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