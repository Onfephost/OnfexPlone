use crate::ast::{Type,Expr,TypeKind};
use crate::builtins::*;
use crate::error::OnfexError;
use std::collections::HashMap;
use colored::Colorize;

fn custom(mut a: Vec<Type>, _ft: HashMap<String, Type>) -> Result<Type, OnfexError> {
    if a.len() < 2 {
        return Err(OnfexError::runtime("cesnos asp 2+ promter gephnosfer"));
    }
    let mut txt = str_get(&a[0].value)?.white();
    a.remove(0);
    for x in &a {
        match str_get(&x.value)?.as_str() {
            "refet" => txt = txt.red(),
            "meove" => txt = txt.blue(),
            "eüngen" => txt = txt.green(),
            "wlour" => txt = txt.white(),
            "magent" => txt = txt.magenta(),
            "phosmor" => txt = txt.purple(),
            "sorphor" => txt = txt.black(),
            "yewtev" => txt = txt.yellow(),
            "cerean" => txt = txt.cyan(),
            "eonch" => txt = txt.truecolor(240,185,0),
            
            "dolb" => txt = txt.bold(),
            "ithalik" => txt = txt.italic(),

            "fite_refet" => txt = txt.bright_red(),
            "fite_meove" => txt = txt.bright_blue(),
            "fite_eüngen" => txt = txt.bright_green(),
            "fite_magent" => txt = txt.bright_magenta(),
            "fite_sorphor" => txt = txt.bright_black(),
            "fite_phosmor" => txt = txt.bright_purple(),
            "fite_yewtev" => txt = txt.bright_yellow(),
            "fite_cerean" => txt = txt.bright_cyan(),
            "fite_eonch" => txt = txt.truecolor(240,165,120),

            "onerl_refet" => txt = txt.on_red(),
            "onerl_meove" => txt = txt.on_blue(),
            "onerl_eüngen" => txt = txt.on_green(),
            "onerl_wlour" => txt = txt.on_white(),
            "onerl_magent" => txt = txt.on_magenta(),
            "onerl_phosmor" => txt = txt.on_purple(),
            "onerl_sorphor" => txt = txt.on_black(),
            "onerl_yewtev" => txt = txt.on_yellow(),
            "onerl_cerean" => txt = txt.on_cyan(),

            "onerl_fite_refet" => txt = txt.on_bright_red(),
            "onerl_fite_meove" => txt = txt.on_bright_blue(),
            "onerl_fite_eüngen" => txt = txt.on_bright_green(),
            "onerl_fite_magent" => txt = txt.on_bright_magenta(),
            "onerl_fite_sorphor" => txt = txt.on_bright_black(),
            "onerl_fite_phosmor" => txt = txt.on_bright_purple(),
            "onerl_fite_yewtev" => txt = txt.on_bright_yellow(),
            "onerl_fite_cerean" => txt = txt.on_bright_cyan(),

            _ => {
                return Err(OnfexError::runtime(
                    "TexthWernen Ern:cesnos asp korn afon counth wraithnosfer",
                ));
            }
        }
    }

    Ok(Type::new(TypeKind::ColStr, Expr::ColStr(txt)))
}

fn color(a: Vec<Type>, ft: HashMap<String, Type>) -> Result<Type, OnfexError> {
    colored::control::set_override(true);
    if a.len() != 1 {
        return Err(OnfexError::runtime("cesnos asp 4 promter gephnosfer"));
    }

    let r = ft.get("refet").ok_or_else(|| OnfexError::runtime("refet esp neat froundnosap"))?;

    let b = ft.get("meove").ok_or_else(|| OnfexError::runtime("meove esp neat froundnosap"))?;

    let g = ft.get("eüngen").ok_or_else(|| OnfexError::runtime("eüngen esp neat froundnosap"))?;

    let txt = str_get(&a[0].value)?.truecolor(
            u8_get(&r.value)?,
            u8_get(&g.value)?,
            u8_get(&b.value)?,
        );

    Ok(Type::new(TypeKind::ColStr, Expr::ColStr(txt)))
}

//tools
fn str_get(x:&Expr) -> Result<String, OnfexError>{
    match x{
        Expr::Str(s)=>{Ok(s.clone())},
        _ => Err(OnfexError::runtime("Typect Ern:sterge esp wraithnosan")),
    }
}

fn u8_get(x:&Expr) -> Result<u8,OnfexError>{
    match x{
        Expr::Int(s)=>{Ok(*s as u8)},
        Expr::Float(s)=>{Ok(*s as u8)},
        _ => Err(OnfexError::runtime("Typect Ern:intg ophe flotg esp wraithnosan")),
    }
}

pub fn load_funcs() -> HashMap<String, Fnc>{
    let mut funcs:HashMap<String, Fnc> = HashMap::new();
    funcs.insert("cesnos".to_string(), custom as Fnc);
    funcs.insert("counthnos".to_string(), color as Fnc);
    funcs
}

pub fn load_vars() -> HashMap<String,Type>{
    let mut vars = HashMap::new();
    let version = "1.0.0".to_string();
    vars.insert("verzen".to_string(),Type::new(TypeKind::Str,Expr::Str(version.clone())));
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