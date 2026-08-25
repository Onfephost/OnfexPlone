use crate::ast::*;
use crate::builtins::*;
use crate::error::OnfexError;
use std::collections::HashMap;

pub fn neom_out(v: &Vec<Type>) -> String {
    let mut vals = String::new();
    for i in v {
        vals.push_str(&format!("{} | ", i.__out__(true)));
    }
    if vals.len() >= 3 {
        vals.truncate(vals.len() - 3);
    }
    format!("<neomOnfex|neomArrey [ {} ]>", vals)
}

fn neom_temp() -> ArrayType {
    ArrayType::new("neomArrey".to_string(), neom_out)
}

fn neom_new(a: Vec<Type>) -> Type {
    Type::new(TypeKind::ArrayT, Expr::ArrayDt(Box::new(Array::new(a, neom_temp()))))
}

fn get_array(x: &Expr) -> Result<Array, OnfexError> {
    match x {
        Expr::ArrayDt(a) => Ok((**a).clone()),
        _ => Err(OnfexError::runtime("neomArrey esp wraithnosan")),
    }
}
//tool
fn f64out(x: &Expr) -> Result<f64, OnfexError> {
    match x {
        Expr::Float(p) => Ok(*p),
        Expr::Int(p) => Ok(*p as f64),
        Expr::Decimal(p) => Ok(p.to_f64()?),
        _ => Err(OnfexError::runtime("intg esp wraithnosan")),
    }
}

fn test(_args: Vec<Type>,ft:HashMap<String,Type>) -> Result<Type, OnfexError> {
    println!("tested");
    Ok(Type::newVoid())
}

fn plus(args: Vec<Type>,ft:HashMap<String,Type>) -> Result<Type, OnfexError> {
    if args.len() != 2 {
        return Err(OnfexError::runtime("adnos asp 2 promter gephnosfer"));
    }
    let at = get_array(&args[0].value)?;
    let bt = get_array(&args[1].value)?;
    let ty = neom_temp();
    if !(at.base.isinstance(&ty) && bt.base.isinstance(&ty)) {
        return Err(OnfexError::runtime(format!(
            "adnos asp 2 promter wraithnosan,{}_{} gephnosan",
            at.base.isinstance(&ty),
            bt.base.isinstance(&ty)
        )));
    }
    let mut res: Vec<Type> = vec![];
    for (x, y) in at.items.iter().zip(bt.items.iter()) {
        let xn = f64out(&x.value)?;
        let yn = f64out(&y.value)?;
        res.push(Type::new(TypeKind::Float, Expr::Float(xn + yn)));
    }
    Ok(neom_new(res))
}

fn mult(args: Vec<Type>,ft:HashMap<String,Type>) -> Result<Type, OnfexError> {
    if args.len() != 2 {
        return Err(OnfexError::runtime("multnos asp 2 promter gephnosfer"));
    }
    let at = get_array(&args[0].value)?;
    let bt = get_array(&args[1].value)?;
    let ty = neom_temp();
    if !(at.base.isinstance(&ty) && bt.base.isinstance(&ty)) {
        return Err(OnfexError::runtime(format!(
            "multnos asp 2 promter wraithnosan,{}_{} gephnosan",
            at.base.isinstance(&ty),
            bt.base.isinstance(&ty)
        )));
    }
    let mut res: Vec<Type> = vec![];
    for (x, y) in at.items.iter().zip(bt.items.iter()) {
        let xn = f64out(&x.value)?;
        let yn = f64out(&y.value)?;
        res.push(Type::new(TypeKind::Float, Expr::Float(xn * yn)));
    }
    Ok(neom_new(res))
}
fn div(args: Vec<Type>,ft:HashMap<String,Type>) -> Result<Type, OnfexError> {
    if args.len() != 2 {
        return Err(OnfexError::runtime("dernos asp 2 promter gephnosfer"));
    }
    let at = get_array(&args[0].value)?;
    let bt = get_array(&args[1].value)?;
    let ty = neom_temp();
    if !(at.base.isinstance(&ty) && bt.base.isinstance(&ty)) {
        return Err(OnfexError::runtime(format!(
            "dernos asp 2 promter wraithnosan,{}_{} gephnosan",
            at.base.isinstance(&ty),
            bt.base.isinstance(&ty)
        )));
    }
    let mut res: Vec<Type> = vec![];
    for (x, y) in at.items.iter().zip(bt.items.iter()) {
        let xn = f64out(&x.value)?;
        let yn = f64out(&y.value)?;
        res.push(Type::new(TypeKind::Float, Expr::Float(xn / yn)));
    }
    Ok(neom_new(res))
}
fn ceil(args: Vec<Type>,ft:HashMap<String,Type>) -> Result<Type, OnfexError> {
    if args.len() != 2 {
        return Err(OnfexError::runtime("ednos asp 2 promter gephnosfer"));
    }
    let at = get_array(&args[0].value)?;
    let bt = get_array(&args[1].value)?;
    let ty = neom_temp();
    if !(at.base.isinstance(&ty) && bt.base.isinstance(&ty)) {
        return Err(OnfexError::runtime(format!(
            "ednos asp 2 promter wraithnosan,{}_{} gephnosan",
            at.base.isinstance(&ty),
            bt.base.isinstance(&ty)
        )));
    }
    let mut res: Vec<Type> = vec![];
    for (x, y) in at.items.iter().zip(bt.items.iter()) {
        let xn = f64out(&x.value)?;
        let yn = f64out(&y.value)?;
        res.push(Type::new(TypeKind::Float, Expr::Float(xn - yn)));
    }
    Ok(neom_new(res))
}


pub fn load_funcs() -> HashMap<String, Fnc> {
    let mut funcs = HashMap::new();
    funcs.insert("test".to_string(), test as Fnc);
    funcs.insert("adnos".to_string(), plus as Fnc);
    funcs.insert("multnos".to_string(), mult as Fnc);
    funcs.insert("dernos".to_string(), div as Fnc);
    funcs.insert("ednos".to_string(), ceil as Fnc);
    funcs
}
pub fn load_vars() -> HashMap<String,Type>{
    let mut vars = HashMap::new();
    let version = "0.8.0".to_string();
    vars.insert("verzen".to_string(), Type::new(TypeKind::Str,Expr::Str(version.clone())));
    vars
}
pub fn load_arrays() -> HashMap<String, ArrayType> {
    let mut arrays: HashMap<String, ArrayType> = HashMap::new();
    arrays.insert("neomArrey".to_string(), neom_temp());
    arrays
}
