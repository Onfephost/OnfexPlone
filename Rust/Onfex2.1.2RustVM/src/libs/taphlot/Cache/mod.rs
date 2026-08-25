use crate::ast::*;
use crate::builtins::*;
use crate::error::OnfexError;
use std::collections::HashMap;

fn show(args: Vec<Type>, ft: HashMap<String, Type>) -> Result<Type, OnfexError> {
    if args.len() != 2 {
        return Err(OnfexError::runtime(
            "Taphlot Ern:morfenlnos asp 1 afon sterg promter wraithnosfer",
        ));
    }

    let name = str_get(&args[0].value)?;
    let tp = matris_get(&args[1].value)?;

    let mut kl = 2;
    let mut vl = 2;

    for (k, v) in tp.clone() {
        let k_len = k.to_string(true).chars().count();
        let v_len = v.to_string(true).chars().count();

        kl = kl.max(k_len);
        vl = vl.max(v_len);
    }

    println!("/-{}", name);

    for (k, v) in tp.clone() {
        let ky = k.to_string(true);
        let val = v.to_string(true);

        let k_len = ky.chars().count();
        let v_len = val.chars().count();

        let key = format!(
            "|{}{}|",
            ky,
            " ".repeat(kl - k_len)
        );

        let value = format!(
            "|{}{}|",
            val,
            " ".repeat(vl - v_len)
        );

        println!("{} => {}", key, value);
    }

    Ok(Type::newVoid())
}

// tools

fn str_get(x: &Expr) -> Result<String, OnfexError> {
    match x {
        Expr::Str(s) => Ok(s.clone()),
        _ => Err(OnfexError::runtime("Typect Ern:sterge esp wraithnosan",)),
    }
}

fn matris_get(x: &Expr) -> Result<Vec<(Type, Type)>, OnfexError> {
    match x {
        Expr::Matris(_, s) => Ok(s.clone()),
        _ => Err(OnfexError::runtime("Typect Ern: arrey esp wraithnosan",)),
    }
}

pub fn load_funcs() -> HashMap<String, Fnc> {
    let mut funcs: HashMap<String, Fnc> = HashMap::new();
    funcs.insert("wernos".to_string(), show as Fnc);
    funcs
}

pub fn load_vars() -> HashMap<String, Type> {
    let mut vars = HashMap::new();
    let version = "1.0.1".to_string();
    vars.insert("verzen".to_string(), Type::new(TypeKind::Str,Expr::Str(version.clone())));
    vars
}

pub fn load_arrays() -> HashMap<String, ArrayType> {
    let arrays: HashMap<String, ArrayType> = HashMap::new();

    arrays
}

pub fn load_buffers() -> HashMap<String, BufferType> {
    let buffers: HashMap<String, BufferType> = HashMap::new();

    buffers
}

pub fn load_monos() -> HashMap<String, MonoType> {
    let monos: HashMap<String, MonoType> = HashMap::new();

    monos
}