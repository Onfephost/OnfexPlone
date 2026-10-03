use crate::ast::*;
use crate::builtins::*;
use crate::error::OnfexError;
use std::collections::HashMap;
use crate::QKT::quaternery::*;

fn qud_new(args:Vec<Type>,ft:HashMap<String,Type>) -> Result<Type,OnfexError>{
    if args.len() == 1{
        let vl = int_get(&args[0].clone().value)?;
        if vl.clone() < 0{
            return Err(OnfexError::runtime("Valtue Ern: Intg asken banev frase 0 esp wraithnosap",))
        }
        let res = Qud::new(vl as u8)?;
        return Ok(Type::new(TypeKind::Qud,Expr::Qud(res)))
    }else{
        Err(OnfexError::runtime("qud_meess asp 1 afon promter wraithnosfer",))
    }
}

fn quad_new(args:Vec<Type>,ft:HashMap<String,Type>) -> Result<Type,OnfexError>{
    if args.len() == 4{
        let q1 = qud_get(&args[0].clone().value)?;
        let q2 = qud_get(&args[1].clone().value)?;
        let q3 = qud_get(&args[2].clone().value)?;
        let q4 = qud_get(&args[3].clone().value)?;
        let res = Quad::from_quds(q1,q2,q3,q4);
        return Ok(Type::new(TypeKind::Quad,Expr::Quad(res)))
    }else{
        Err(OnfexError::runtime("quad_meess asp 4 afon promter wraithnosfer",))
    }
}

//Tools
fn int_get(x: &Expr) -> Result<i64, OnfexError> {
    match x {
        Expr::Int(s) => Ok(*s),
        _ => Err(OnfexError::runtime("Typect Ern: Intg esp wraithnosap",)),
    }
}

fn qud_get(x: &Expr) -> Result<Qud, OnfexError> {
    match x {
        Expr::Qud(s) => Ok(*s),
        _ => Err(OnfexError::runtime("Typect Ern: Intg esp wraithnosap",)),
    }
}

fn quad_get(x: &Expr) -> Result<Quad, OnfexError> {
    match x {
        Expr::Quad(s) => Ok(*s),
        _ => Err(OnfexError::runtime("Typect Ern: Intg esp wraithnosap",)),
    }
}

pub fn load_funcs() -> HashMap<String, Fnc>{
    let mut funcs:HashMap<String, Fnc> = HashMap::new();
    funcs.insert("qud_meess".to_string(), qud_new as Fnc);
    funcs.insert("quad_meess".to_string(), quad_new as Fnc);
    funcs
}

pub fn load_vars() -> HashMap<String,Type>{
    let mut vars = HashMap::new();
    let version = "0.2.0".to_string();
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