// builtinsdata.rs

use crate::ast::*;
use crate::builtins::*;
use std::collections::HashMap;
use std::io::{self, Write};
use std::cell::RefCell;

pub fn pr(args:Vec<Type>) -> Type{
    if args.len() == 1{
        let a = &args[0];
        if matches!(a.value,Expr::Str(_)){
            let res = a.__out__();let ln = res.len();
            print!("{}\n", &res[1..ln-1]);
        }else{print!("{}\n", a.__out__());}
    }
    else{
        let mut res="".to_string();
        for a in args {
            if matches!(a.value,Expr::Str(_)){
                let rest = a.__out__();
                let ln = rest.len();
                res.push_str(&rest[1..ln-1]);
                
            }else{res.push_str(&a.__out__());}
            res.push_str(" ");
        }
        res.push_str("\n");
        print!("{}", res);
    }
    io::stdout().flush().unwrap();
    Type::newVoid()
}

pub fn ask(args:Vec<Type>) -> Type{
    let mut res = "".to_string();
    let text = match &args[0].value{Expr::Str(x)=>x,_=>panic!("str gir piç")};
    print!("{text}");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut res).expect("yok");
    Type::new(
        TypeKind::Str,Expr::Str(res.trim().to_string()),
    )
}

// VECTOR OUT
pub fn vector_out(v:&Vec<Type>) -> String{
    let mut vals = String::new();
    for i in v{
        vals.push_str(&format!("{} | ",i.__out__()));
    }
    if vals.len() >= 3{
        vals.truncate(vals.len()-3);
    }
    format!("<Vektöre [ {} ]>",vals)
}

// MAP OUT
pub fn map_out(v:&Vec<(Type,Type)>) -> String{
    let mut vals = String::new();
    for (k,val) in v{
        vals.push_str(&format!("{} => {}, ",k.__out__(),val.__out__()));
    }
    if vals.len() >= 2{
        vals.truncate(vals.len()-2);
    }
    format!("<Map {{{}}}>",vals)
}

// CREATE ARRAY TYPES
pub fn create_array_types()-> std::collections::HashMap<String,ArrayType>{
    let mut arrs =std::collections::HashMap::new();
    // vektöre
    arrs.insert(
        "vektöre".to_string(),
        ArrayType::new("vektöre".to_string(),vector_out),
    );
    arrs
}

// CREATE BUFFER TYPES
pub fn create_buffer_types()-> std::collections::HashMap<String,BufferType>{
    let mut buffs =std::collections::HashMap::new();
    buffs.insert(
        "mappe".to_string(),
        BufferType::new("mappe".to_string(),map_out),
    );
    buffs
}