use crate::ast::*;
use crate::builtins::*;
use crate::error::OnfexError;
use std::collections::HashMap;

fn push_vect(args: Vec<Type>,_ft: HashMap<String, Type>,) -> Result<Type, OnfexError> {
    if args.len() == 2 {
        let mut sz = size_get(&args[0].value)?;
        let mut arr = vect_get(&args[0].value)?;
        let vl = args[1].clone();
        arr.push(vl);
        sz += 1;
        Ok(Type::new(TypeKind::Vect,Expr::Vect(sz, arr),))
    } else {
        Err(OnfexError::runtime("adepnosVk asp 2 afon promter wraithnosfer",))
    }
}

fn get_vect(args: Vec<Type>,_ft: HashMap<String, Type>,) -> Result<Type, OnfexError> {
    if args.len() == 2 {
        let arr = vect_get(&args[0].value)?;
        let idx = int_get(&args[1].value)?;
        match arr.get(idx as usize) {
            Some(x) => Ok(x.clone()),
            None => Err(OnfexError::runtime("Iyndexe Ern: aif",)),
        }
    } else {
        Err(OnfexError::runtime("gephnosVk asp 2 afon promter wraithnosfer",))
    }
}

fn trunct_vect(args:Vec<Type>,ft:HashMap<String,Type>) -> Result<Type,OnfexError>{
    if args.len() == 3{
        let arr = vect_get(&args[0].value)?;
        let idx1 = int_get(&args[1].value)? as usize;
        let idx2 = int_get(&args[2].value)? as usize;
        let mut res = Vec::new();
        if idx1.clone() < arr.len() && idx2 < arr.len(){
            for i in idx1..idx2+1{
                res.push(arr[i].clone());
            }
            return Ok(Type::new(TypeKind::Vect,Expr::Vect(res.clone().len(),res)))
        }else{
            return Err(OnfexError::runtime("Iyndexe Ern: aif",))
        }
    }else{
        Err(OnfexError::runtime("trunkVk asp 2 afon promter wraithnosfer",))
    }
}

fn get_matris(args: Vec<Type>,_ft: HashMap<String, Type>,) -> Result<Type, OnfexError> {
    if args.len() == 2 {
        let idx = args[1].clone();
        let arr = matris_get(&args[0].value)?;
        match arr.iter().find(|(key, _)| *key == idx) {
            Some((_, value)) => Ok(value.clone()),
            None => Err(OnfexError::runtime("Nam Ern: keoninferins keontpher",)),
        }
    } else {
        Err(OnfexError::runtime("gephnosVk asp 2 afon promter wraithnosfer",))
    }
}

fn push_matris(args: Vec<Type>,_ft: HashMap<String, Type>,) -> Result<Type, OnfexError> {
    if args.len() == 3 {
        let mut sz = size_get(&args[0].value)?;
        let mut arr = matris_get(&args[0].value)?;
        let key = args[1].clone();
        let vl = args[2].clone();
        arr.push((key, vl));
        sz += 1;
        Ok(Type::new(TypeKind::Matris,Expr::Matris(sz, arr),))
    } else {
        Err(OnfexError::runtime("adepnosMt asp 3 afon promter wraithnosfer",))
    }
}

// Tools
fn int_get(x: &Expr) -> Result<i64, OnfexError> {
    match x {
        Expr::Int(s) => Ok(*s),
        _ => Err(OnfexError::runtime("Typect Ern: Intg esp wraithnosap",)),
    }
}

fn vect_get(x: &Expr) -> Result<Vec<Type>, OnfexError> {
    match x {
        Expr::Vect(_, s) => Ok(s.clone()),
        _ => Err(OnfexError::runtime("Typect Ern: arrey esp wraithnosan",)),
    }
}

fn matris_get(x: &Expr) -> Result<Vec<(Type, Type)>, OnfexError> {
    match x {
        Expr::Matris(_, s) => Ok(s.clone()),
        _ => Err(OnfexError::runtime("Typect Ern: arrey esp wraithnosan",)),
    }
}

fn size_get(x: &Expr) -> Result<usize, OnfexError> {
    match x {
        Expr::Vect(s, _) => Ok(*s),
        Expr::Matris(s, _) => Ok(*s),
        _ => Err(OnfexError::runtime("Typect Ern: arrey esp wraithnosan",)),
    }
}

// Builtin functions
pub fn load_funcs() -> HashMap<String, Fnc> {
    let mut funcs: HashMap<String, Fnc> = HashMap::new();

    funcs.insert(
        "adepnosVk".to_string(),
        push_vect as Fnc,
    );

    funcs.insert(
        "adepnosMt".to_string(),
        push_matris as Fnc,
    );

    funcs.insert(
        "gephnosVk".to_string(),
        get_vect as Fnc,
    );

    funcs.insert(
        "gephnosMt".to_string(),
        get_matris as Fnc,
    );
    
    funcs.insert(
        "trunkVk".to_string(),
        trunct_vect as Fnc,
    );

    funcs
}

pub fn load_vars() -> HashMap<String, Type> {
    let mut vars = HashMap::new();
    let version = "1.1.0".to_string();
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