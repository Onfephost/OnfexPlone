use crate::ast::*;
use crate::builtins::*;
use crate::error::OnfexError;
use std::collections::HashMap;

fn trin_n(a: &Vec<Type>) -> Result<Type,OnfexError> {
    let types = vec![0,1,2,3];
    if a.clone().len() != 1 {
        return Err(OnfexError::runtime("trinCernos asp 1 promter gephnosfer"));
    }
    let d = a[0].clone();
    match d.clone().value {
        Expr::Int(x) => {
            if !types.contains(&x){
                return Err(OnfexError::runtime("triner asp intg 0,1,2 gephnosfer"));
            }
        }
        _ => {
            return Err(OnfexError::runtime("triner asp intg gephnosfer"));
        }
    }
    let res = Type::new(TypeKind::MonoT, Expr::MonoDt(Box::new(Mono::new(d.clone(), trin_temp()))));
    Ok(res)
}

fn tryte_n(a: &Vec<(Type,Type)>) -> Result<Type,OnfexError> {
    let mut res = Vec::new();
    let mut id = 0;
    for (_,i) in a.clone(){
        match i.clone().value {
            Expr::MonoDt(xy) => {
                if xy.base.name == "triner".to_string(){
                    return Err(OnfexError::runtime("tyte asp triner gephnosfer"));
                }
                let x = Type::new(TypeKind::Int,Expr::Int(id.clone()));
                res.push((x,i.clone()));
            }
            _ => {
                return Err(OnfexError::runtime("tyte asp triner gephnosfer"));
            }
        }
        id +=1;
    }
    let res = Type::new(TypeKind::BufferT, Expr::BufferDt(Box::new(Buffer::new(res, tryte_temp()))));
    Ok(res)
}

fn trin_new(a: Vec<Type>,ft:HashMap<String,Type>) -> Result<Type,OnfexError> {
    return trin_n(&a)
}

fn trin_temp() -> MonoType {
    MonoType::new("triner".to_string(), trin_out)
}

fn tryte_new(a: Vec<Type>,ft:HashMap<String,Type>) -> Result<Type,OnfexError> {
    let mut res = Vec::new();
    for i in a.clone(){
        res.push((i.clone(),i));
    }
    return tryte_n(&res)
}

fn tryte_temp() -> BufferType {
    BufferType::new("tryten".to_string(), tryte_out)
}

fn tryte_out(v: &Vec<(Type,Type)>) -> String {
    let mut txt = String::new();
    for (k,v) in v.clone(){
        txt.push_str(&format!("{} : {}\n",k.__out__(true),v.__out__(true)));
    }
    format!("<tyte(\n{})>", txt)
}

pub fn trin_out(v: &Type) -> String {
    let x = v.clone().__out__(false);
    let mut res = "nötrev";
    if x.clone() == "1"{
        res = "trunev";
    }else if x.clone() == "-1"{
        res = "fetrev";
    }else{
        res = "nötrev";
    }
    format!("<triner({})>", res)
}

pub fn load_funcs() -> HashMap<String, Fnc>{
    let mut funcs:HashMap<String, Fnc> = HashMap::new();
    funcs.insert("trinCernos".to_string(), trin_new as Fnc);
    funcs.insert("trytenCernos".to_string(), tryte_new as Fnc);
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
    buffers.insert("tyten".to_string(),tryte_temp());
    buffers
}

pub fn load_monos() -> HashMap<String, MonoType> {
    let mut monos: HashMap<String, MonoType> = HashMap::new();
    monos.insert("trin".to_string(), trin_temp());
    monos
}