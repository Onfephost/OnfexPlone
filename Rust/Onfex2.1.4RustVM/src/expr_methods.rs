use crate::error::OnfexError;
use crate::ast::*;
use std::collections::HashMap;

pub fn expr_methods_get() -> HashMap<String,HashMap<String,ExprM>>{
    let mut h = HashMap::new();
    let mut listm = HashMap::new();
    listm.insert("adepnos".to_string(),append as ExprM);
    listm.insert("adepins".to_string(),appending as ExprM);
    h.insert("vek".to_string(),listm);
    h
}

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

fn append(e:&Expr,args:Vec<Type>) -> Result<Type,OnfexError>{
    if args.len() == 1{
        let mut v = vect_get(&e)?;
        v.push(args[0].clone());
        Ok(Type::new(TypeKind::Vect,Expr::Vect(v.len(),v.clone())))
    }else{
        Err(OnfexError::runtime("Promter Ern: 1 afon promter esp wraithnosan",))
    }
}
fn appending(e:&Expr,args:Vec<Type>) -> Result<Type,OnfexError>{
    if args.len() == 1{
        let mut v = vect_get(&e)?;
        v.push(args[0].clone());
        Ok(Type::new(TypeKind::Vect,Expr::Vect(v.len(),v.clone())))
    }else{
        Err(OnfexError::runtime("Promter Ern: 1 afon promter esp wraithnosan",))
    }
}