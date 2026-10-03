use crate::ast::*;
use crate::builtins::*;
use crate::error::OnfexError;
use std::collections::HashMap;

fn meas(args:Vec<Type>,ft:HashMap<String,Type>) -> Result<Type,OnfexError>{
    if args.len() == 3{
        let s = str_get(&args[0].clone().value)?;
        let mut mn = 0 as usize;
        let mut mx =  0 as usize;
        if let x = int_get(&args[1].clone().value)? {
            if x < 0{
                return Err(OnfexError::runtime("Typect Ern: intf 0+ ol valtue asken intg esp wraithnosanes",));
            }
            mn = x as usize;
        } 
        if let y = int_get(&args[2].clone().value)? {
            if y < 0{
                return Err(OnfexError::runtime("Typect Ern: intf 0+ ol valtue asken intg esp wraithnosanes",));
            }
            mx = y as usize;
        }
        let chs = s.chars().collect::<Vec<char>>();
        if (mx.clone() > chs.len()-1) || (mx < mn){
            return Err(OnfexError::runtime(&format!("Iyndexe Ern: sprachnen {}:{} mut bephnosan ..{} asp lenev",mn,mx,chs.len()),));
        } 
        let mut res = "".to_string();
        for c in &chs[(mn)..(mx)]{
            res.push(*c);
        }
        return Ok(Type::new(TypeKind::Str,Expr::Str(res)))
    }else{
        Err(OnfexError::runtime("mersnos asp 3 afon promter wraithnosfer",))
    }
}

//Tools
fn int_get(x: &Expr) -> Result<i64, OnfexError> {
    match x {
        Expr::Int(s) => Ok(*s),
        _ => Err(OnfexError::runtime("Typect Ern: Intg esp wraithnosap",)),
    }
}
fn str_get(x:&Expr) -> Result<String, OnfexError>{
    match x{
        Expr::Str(s)=>{Ok(s.clone())},
        _ => Err(OnfexError::runtime("Typect Ern:sterge esp wraithnosan")),
    }
}

pub fn load_funcs() -> HashMap<String, Fnc>{
    let mut funcs:HashMap<String, Fnc> = HashMap::new();
    funcs.insert("mersnos".to_string(), meas as Fnc);
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