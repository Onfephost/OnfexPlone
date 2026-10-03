use crate::ast::*;
use crate::builtins::*;
use crate::error::OnfexError;
use std::collections::HashMap;
use std::process::Command;
use crate::ostools::*;
use std::io::{self, Write};

fn system(args:Vec<Type>,ft:HashMap<String,Type>) -> Result<Type, OnfexError>{
    let cmd = str_get(&args[0].value)?;
    let mut _args:Vec<String> = Vec::with_capacity(args[1..].len());
    for i in &args[1..]{
        _args.push(str_get(&i.value)?);
    }
    let res = command(cmd.clone(),_args.clone());
    let expr = Expr::Str(res.clone());
    Ok(Type::new(TypeKind::Str,expr,))
}

fn ask(args:Vec<Type>,ft:HashMap<String,Type>) -> Result<Type,OnfexError>{
    if !args.len() ==1{
        return Err(OnfexError::runtime("Osp Ern:morfenlnos asp  korn afon sterg promter wraithnosfer"));
    }
    let mut res = String::new();
    let text = match args.get(0).map(|a| &a.value) {
        Some(Expr::Str(x)) => x.clone(),
        _ => {
            return Err(OnfexError::runtime("Typect Ern:sterge esp wraithnosan"));
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

fn systemP(args:Vec<Type>,ft:HashMap<String,Type>) -> Result<Type, OnfexError>{
    let cmd = str_get(&args[0].value)?;
    let mut _args:Vec<String> = Vec::with_capacity(args[1..].len());
    for i in &args[1..]{
        match i.clone().value{
            Expr::Sprd(x) => {match x.value{
                    Expr::Vect(s,v) => {
                        _args.extend(v.iter().map(|t| str_get(&t.value).unwrap()));
                        break;
                    }
                    _ => {return Err(OnfexError::runtime("Ops:speadnen esp wraithnosan"))}
                }
            }
            Expr::Str(x) => {_args.push(x);}
            _=> {return Err(OnfexError::runtime("Typect Ern:sterge esp wraithnosan"))}
        }
    }
    let res = command(cmd.clone(),_args.clone());
    println!("{}",res.clone());
    let expr = Expr::Str(res.clone());
    Ok(Type::new(TypeKind::Str,expr,))
}

fn c_pwd(args:Vec<Type>,ft:HashMap<String,Type>) -> Result<Type, OnfexError>{
    let res:String = command("pwd".to_string(),vec![]);
    Ok(Type::new(TypeKind::Str,Expr::Str(res)))
}

fn aus(args:Vec<Type>,ft:HashMap<String,Type>) -> Result<Type,OnfexError>{
    if args.len() == 1{
        let txt:String = str_get(&args[0].value)?;
        let res:String = autocommand(txt);
        Ok(Type::new(TypeKind::Str,Expr::Str(res)))
    }else{
        return Err(OnfexError::runtime("Promter Ern: termifal frounct asp 1 promter beg gephnosfer"));
    }
}

fn chdir(args:Vec<Type>,ft:HashMap<String,Type>) -> Result<Type,OnfexError>{
    if args.len() == 1{
        let txt:String = str_get(&args[0].value)?;
        let res:String = autocommand(format!("cd {}",txt));
        Ok(Type::newVoid())
    }else{
        return Err(OnfexError::runtime("Promter Ern: cg frounct asp 1 promter beg gephnosfer"));
    }
}
fn mkdir(args:Vec<Type>,ft:HashMap<String,Type>) -> Result<Type,OnfexError>{
    if args.len() == 1{
        let txt:String = str_get(&args[0].value)?;
        let res:String = autocommand(format!("mkdir {}",txt));
        Ok(Type::newVoid())
    }else{
        return Err(OnfexError::runtime("Promter Ern: grosserCernos frounct asp 1 promter beg gephnosfer"));
    }
}
//Tools
fn str_get(x:&Expr) -> Result<String, OnfexError>{
    match x{
        Expr::Str(s)=>{Ok(s.clone())},
        _ => Err(OnfexError::runtime("Typect Ern:sterge esp wraithnosan")),
    }
}

pub fn load_funcs() -> HashMap<String, Fnc>{
    let mut funcs = HashMap::new();
    funcs.insert("systess".to_string(), system as Fnc);
    funcs.insert("systessPyrintlbrof".to_string(), systemP as Fnc);
    funcs.insert("termifal".to_string(), aus as Fnc);
    funcs.insert("kernevGrosser".to_string(), c_pwd as Fnc);
    funcs.insert("grosserCernos".to_string(), mkdir as Fnc);
    funcs.insert("cg".to_string(), chdir as Fnc);
    funcs.insert("intfpossGephnos".to_string(), chdir as Fnc);
    
    funcs
}

pub fn load_vars() -> HashMap<String,Type>{
    let mut vars = HashMap::new();
    let version = "0.5.1".to_string();
    vars.insert("verzen".to_string(), Type::new(TypeKind::Str,Expr::Str(version.clone())));
    vars
}
