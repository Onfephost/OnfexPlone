// builtinsdata.rs

use crate::ast::*;
use crate::builtins::*;
use crate::error::OnfexError;
use std::io::{self, Write};
use std::collections::HashMap;

pub fn pr(args: Vec<Type>,ft:HashMap<String,Type>) -> Result<Type, OnfexError> {
    if args.is_empty() {
        println!();
        return Ok(Type::newVoid());
    }
    
    
    let format = match args[0].clone().value{
        Expr::Str(x) => {x}
        _ => return Err(OnfexError::Runtime{message:format!("Typect Ern:sterge esp wraithnosan {}",args[0].clone().kind.to_string())}),
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

    print!("{}",result);
    io::stdout()
        .flush()
        .map_err(|e| OnfexError::runtime(format!("failed to flush stdout: {}", e)))?;
    Ok(Type::newVoid())
}
pub fn prln(args: Vec<Type>,ft:HashMap<String,Type>) -> Result<Type, OnfexError> {
    if args.is_empty() {
        println!();
        return Ok(Type::newVoid());
    }
    
    let format = match args[0].clone().value{
        Expr::Str(x) => {x}
        _ => return Err(OnfexError::Runtime{message:format!("Typect Ern:sterge esp wraithnosan {}",args[0].clone().kind.to_string())}),
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
        vals.push_str(&format!("{} : {}, ", k.__out__(true), val.__out__(true)));
    }
    if vals.len() >= 2 {
        vals.truncate(vals.len() - 2);
    }
    format!("<Map {{{}}}>", vals)
}

// CREATE ARRAY TYPES
pub fn create_array_types() -> HashMap<String, ArrayType> {
    let mut arrs = HashMap::new();
    // vektöre
    arrs.insert(
        "vektöre".to_string(),
        ArrayType::new("vektöre".to_string(), vector_out),
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
