// builtinsdata.rs

use crate::ast::*;
use crate::builtins::*;
use crate::error::OnfexError;
use std::io::{self, Write};
use std::collections::HashMap;
use std::rc::Rc;
pub fn pr(args: Vec<Type>,ft:HashMap<String,Type>) -> Result<Type, OnfexError> {
    if args.is_empty() {
        println!();
        return Ok(Type::newVoid());
    }
    let format = match args[0].clone().value{
        Expr::Str(x) => {x}
        _ => return Err(OnfexError::runtime(format!("Typect Ern:sterge esp wraithnosan {}",args[0].clone().kind.to_string()))),
    };

    let mut result = String::new();
    let mut index = 1;

    let chars: Vec<char> = format.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '{' && i + 1 < chars.len() && chars[i + 1] == '}'{
            if index < args.len() {
                result.push_str(&args[index].__out__(false));
                index += 1;
            } else {
                result.push_str("");
            }
            i += 2;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result = result.replace("[?ml]","\n");

    print!("{}",result);
    io::stdout()
        .flush()
        .map_err(|e| OnfexError::runtime(format!("failed to flush stdout: {}", e)))?;
    Ok(Type::newVoid())
}

pub fn format(args: Vec<Type>,ft:HashMap<String,Type>) -> Result<Type, OnfexError> {
    if args.is_empty() {
        return Ok(Type::newVoid());
    }
    
    let format = match args[0].clone().value{
        Expr::Str(x) => {x}
        _ => return Err(OnfexError::runtime(format!("Typect Ern:sterge esp wraithnosan {}",
            args[0].clone().kind.to_string()))),
    };

    let mut result = String::new();
    let mut index = 1;

    let chars: Vec<char> = format.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '{'
            && i + 1 < chars.len()
            && chars[i + 1] == '}'
        {
            if index < args.len() {
                result.push_str(&args[index].__out__(false));
                index += 1;
            } else {
                result.push_str("{}");
            }
            i += 2;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    Ok(Type::new(TypeKind::Str,Expr::Str(result)))
}
pub fn prln(args: Vec<Type>,ft:HashMap<String,Type>) -> Result<Type, OnfexError> {
    if args.is_empty() {
        println!();
        return Ok(Type::newVoid());
    }
    
    let format = match args[0].clone().value{
        Expr::Str(x) => {x}
        _ => return Err(OnfexError::runtime(format!("Typect Ern:sterge esp wraithnosan {}",args[0].clone().kind.to_string()))),
    };

    let mut result = String::new();
    let mut index = 1;

    let chars: Vec<char> = format.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '{'
            && i + 1 < chars.len()
            && chars[i + 1] == '}'
        {
            if index < args.len() {
                result.push_str(&args[index].__out__(false));
                index += 1;
            } else {
                result.push_str("{}");
            }
            i += 2;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }

    print!("{}\n",result);
    io::stdout()
        .flush()
        .map_err(|e| OnfexError::runtime(format!("failed to flush stdout: {}", e)))?;
    Ok(Type::newVoid())
}

pub fn ask(args: Vec<Type>,ft:HashMap<String,Type>) -> Result<Type, OnfexError> {
    let mut res = String::new();
    let text = match args.get(0).map(|a| &a.value) {
        Some(Expr::Str(x)) => x.clone(),
        _ => {
            return Err(OnfexError::runtime("morfenlnos expects a single string argument"));
        }
    };
    print!("{text}");
    io::stdout()
        .flush()
        .map_err(|e| OnfexError::runtime(format!("failed to flush stdout: {}", e)))?;
    io::stdin()
        .read_line(&mut res)
        .map_err(|e| OnfexError::runtime(format!("failed to read input: {}", e)))?;
    Ok(Type::new(TypeKind::Str, Expr::Str(res.trim().to_string())))
}

fn typeOf(args:Vec<Type>,ft:HashMap<String,Type>) -> Result<Type,OnfexError>{
    if args.clone().len() == 1{
        let obj = args[0].clone().kind;
        return Ok(Type::new(TypeKind::TypeKind,Expr::TypeKind(Box::new(obj))))
    }else{
        return Err(OnfexError::runtime("typect expects a single string argument"));
    }
}

fn len(args:Vec<Type>,ft:HashMap<String,Type>) -> Result<Type,OnfexError>{
    if args.clone().len() == 1{
        let obj = args[0].clone();
        let ln = match &obj.value{
            Expr::Str(s) => {s.chars().collect::<Vec<char>>().len()}
            Expr::Vect(s,_) => {s.clone()}
            Expr::Matris(s,_) => {s.clone()}
            _ => {return Err(OnfexError::runtime("lenev expects a single string argument"));}
        };
        return Ok(Type::new(TypeKind::Int,Expr::Int(ln as i64)))
    }else{
        return Err(OnfexError::runtime("lenev expects a single string argument"));
    }
}
fn size(args:Vec<Type>,ft:HashMap<String,Type>) -> Result<Type,OnfexError>{
    if args.clone().len() == 1{
        let obj = args[0].clone();
        let ln = obj.value.clone();
        let res = std::mem::size_of_val(&ln) as i64;
        return Ok(Type::new(TypeKind::Int,Expr::Int(res)))
    }else{
        return Err(OnfexError::runtime("lenev expects a single string argument"));
    }
}


fn to_it(args:Vec<Type>,ft:HashMap<String,Type>) -> Result<Type,OnfexError>{
    if args.len() == 1{
        let p = args[0].clone();
        let res = to_iter(&p).unwrap();
        Ok(Type::new(TypeKind::Iter,Expr::Iter(Rc::new(res))))
    }else{
        return Err(OnfexError::runtime("iterfal expects a single string argument"));
    }
}
fn next(args:Vec<Type>,ft:HashMap<String,Type>) -> Result<Type,OnfexError>{
    if args.len() == 1{
        let p = match args[0].clone().value{
            Expr::Iter(x) => {Ok(x)}
            _ => {return Err(OnfexError::runtime("nexist asp iterfal wraithnosan"));}
        }?;
        if p.has_next(){
            let res = p.clone().next().unwrap();
            return Ok(res)
        }else{
            return Ok(Type::newVoid())
        }
    }else{
        return Err(OnfexError::runtime("iterfal expects a single string argument"));
    }
}

fn collect(args:Vec<Type>,ft:HashMap<String,Type>) -> Result<Type,OnfexError>{
    if args.len() == 1{
        let p = match args[0].clone().value{
            Expr::Iter(x) => {Ok(x)}
            _ => {return Err(OnfexError::runtime("nexist asp iterfal wraithnosan"));}
        }?;
        let res = p.collect().clone();
        let ln = res.len();
        return Ok(Type::new(TypeKind::Vect,Expr::Vect(ln,res)))
    }else{
        return Err(OnfexError::runtime("iterfal expects a single string argument"));
    }
}

pub fn isinstance(args:Vec<Type>,ft:HashMap<String,Type>) -> Result<Type,OnfexError>{
    if args.len() == 2{
        let a1 = match args[0].clone().value{
            Expr::StructDt(x) => {Ok(x.base.clone().name)}
            Expr::StrctInht(x) => {Ok(x.clone().name)}
            Expr::Bool(_) => {Ok("<booltg>".to_string())}
            Expr::Int(_) => {Ok("<intg>".to_string())}
            Expr::Float(_) => {Ok("<flotg>".to_string())}
            _ => {return Err(OnfexError::runtime("yenh asp iterfal wraithnosan"));}
        }?;
        let a2 = match args[1].clone().value{
            Expr::StructDt(x) => {Ok(x.base.clone().name)}
            Expr::StrctInht(x) => {Ok(x.clone().name)}
            Expr::Bool(_) => {Ok("<booltg>".to_string())}
            Expr::Int(_) => {Ok("<intg>".to_string())}
            Expr::Float(_) => {Ok("<flotg>".to_string())}
            _ => {return Err(OnfexError::runtime("yenh asp iterfal wraithnosan"));}
        }?;
        if a1 == a2{
            return Ok(Type::new(TypeKind::Bool,Expr::Bool(true)))
        }else{
            return Ok(Type::new(TypeKind::Bool,Expr::Bool(false)))
        }
    }else{
        return Err(OnfexError::runtime("yenh expects a single string argument"));
    }
}

pub fn create_builtins_funcs() -> HashMap<String, FUNC> {
    use crate::builtinsdata::*;
    let mut builtins: HashMap<String, FUNC> = HashMap::new();
    builtins.insert("pyrintnos".to_string(), FUNC { func: pr });
    builtins.insert("pyrintnosFowLt".to_string(), FUNC { func: prln });
    builtins.insert("morfenlnos".to_string(), FUNC { func: ask });
    builtins.insert("phormatte".to_string(), FUNC { func: format });
    builtins.insert("typectOft".to_string(), FUNC { func: typeOf });
    builtins.insert("iterfal".to_string(), FUNC { func: to_it });
    builtins.insert("lenev".to_string(), FUNC { func: len });
    builtins.insert("seznevOft".to_string(), FUNC { func: size });
    builtins.insert("nexist".to_string(), FUNC { func: next });
    builtins.insert("yenh".to_string(), FUNC { func: isinstance });
    builtins
}

// VECTOR OUT
pub fn vector_out(v: &Vec<Type>) -> String {
    let mut vals = String::new();
    for i in v {
        vals.push_str(&format!("{} | ", i.__out__(true)));
    }
    if vals.len() >= 3 {
        vals.truncate(vals.len() - 3);
    }
    format!("<Vektöre [ {} ]>", vals)
}

// MAP OUT
pub fn map_out(v: &Vec<(Type, Type)>) -> String {
    let mut vals = String::new();
    for (k, val) in v {
        vals.push_str(&format!("{} => {}, ", k.__out__(true), val.__out__(true)));
    }
    if vals.len() >= 2 {
        vals.truncate(vals.len() - 2);
    }
    format!("<Map {{{}}}>", vals)
}

// CREATE ARRAY TYPES
pub fn create_array_types() -> HashMap<String, ArrayType> {
    let mut arrs = HashMap::new();
    let mut v1 = ArrayType::new("vektöre".to_string(), vector_out);
    // vektöre
    arrs.insert(
        "vektöre".to_string(),
        v1.clone()
    );
    arrs
}

// CREATE BUFFER TYPES
pub fn create_buffer_types() -> HashMap<String, BufferType> {
    let mut buffs = HashMap::new();
    buffs.insert(
        "mappe".to_string(),
        BufferType::new("mappe".to_string(), map_out),
    );
    buffs
}
