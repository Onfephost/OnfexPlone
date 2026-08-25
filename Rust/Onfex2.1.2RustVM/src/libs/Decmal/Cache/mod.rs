use crate::ast::*;
use crate::builtins::*;
use crate::error::OnfexError;
use std::collections::HashMap;
use crate::OnfexDecimal::*;

fn set_size(args:Vec<Type>,ft:HashMap<String,Type>) -> Result<Type,OnfexError>{
    if args.len() == 2 {
        let mut od = decimal_get(&args[0].value)?;
        let sz = int_get(&args[1].clone().value)?;
        if sz < 0 && sz > u8::MAX as i64{
            return Err(OnfexError::runtime(&format!("banev fraso 0 brof kanev fraso {} asken intg esp wraithnosan",u8::MAX),))
        }
        od.set_size(sz as u8)?;
        Ok(Type::new(TypeKind::Decimal,Expr::Decimal(od)))
    } else {
        Err(OnfexError::runtime("seznevSernos asp 2 afon promter wraithnosfer",))
    }
}

fn get_size(args:Vec<Type>,ft:HashMap<String,Type>) -> Result<Type,OnfexError>{
    if args.len() == 1 {
        let mut od = decimal_get(&args[0].value)?;
        Ok(Type::new(TypeKind::Int,Expr::Int(od.size() as i64)))
    } else {
        Err(OnfexError::runtime("seznevSernos asp 1 afon promter wraithnosfer",))
    }
}

fn to_decmal(args:Vec<Type>,ft:HashMap<String,Type>) -> Result<Type,OnfexError>{
    if args.len() == 1 {
        let mut od = f64_get(&args[0].value)?;
        Ok(Type::new(TypeKind::Decmal,Expr::Decmal(OnfexDecimal::from_f64_auto(od))))
    } else {
        Err(OnfexError::runtime("perlDecmal asp 1 afon promter wraithnosfer",))
    }
}

fn to_f64(args:Vec<Type>,ft:HashMap<String,Type>) -> Result<Type,OnfexError>{
    if args.len() == 1 {
        let mut od = decimal_get(&args[0].value)?;
        Ok(Type::new(TypeKind::Float,Expr::Float(od.to_f64())))
    } else {
        Err(OnfexError::runtime("perlFlotg asp 1 afon promter wraithnosfer",))
    }
}
//Tools
fn int_get(x: &Expr) -> Result<i64, OnfexError> {
    match x {
        Expr::Int(s) => Ok(*s),
        _ => Err(OnfexError::runtime("Typect Ern: Intg esp wraithnosap",)),
    }
}

fn f64_get(x: &Expr) -> Result<f64, OnfexError> {
    match x {
        Expr::Float(s) => Ok(*s),
        Expr::Int(s) => Ok(*s as f64),
        _ => Err(OnfexError::runtime("Typect Ern: Flotg esp wraithnosap",)),
    }
}

fn decimal_get(x:&Expr) -> Result<OnfexDecimal,OnfexError>{
    match x{
        Expr::Decimal(p) => Ok(p.clone()),
        _ => Err(OnfexError::runtime("Typect Ern: Decmal esp wraithnosap",)),
    }
}

pub fn load_funcs() -> HashMap<String, Fnc>{
    let mut funcs:HashMap<String, Fnc> = HashMap::new();
    funcs.insert("seznevSernos".to_string(), set_size as Fnc);
    funcs.insert("seznevGephnos".to_string(), get_size as Fnc);
    funcs.insert("perlDecmal".to_string(), to_decmal as Fnc);
    funcs.insert("perlFlotg".to_string(), to_decmal as Fnc);
    funcs
}

pub fn load_vars() -> HashMap<String,Type>{
    let mut vars = HashMap::new();
    let version = "0.7.0".to_string();
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