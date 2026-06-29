// builtins.rs

use crate::ast::*;
use std::collections::HashMap;
use std::cell::RefCell;

// ================= FUNC =================

#[derive(Debug, Clone)]
pub struct FUNC{
    pub func: fn(Vec<Type>) -> Type,
}

impl FUNC{
    pub fn run(&self,args:Vec<Type>) -> Type{
        (self.func)(args)
    }
}
// ================= ARRAY TYPE =================

#[derive(Debug, Clone)]
pub struct ArrayType{
    pub name:String,
    pub outFn:fn(&Vec<Type>) -> String,
    pub methods:RefCell<HashMap<String,FUNC>>,
}

impl ArrayType{
    pub fn new(name:String,outFn:fn(&Vec<Type>) -> String,)->Self{
        Self{name,outFn,methods:RefCell::new(HashMap::new()),}
    }
    pub fn isinstance(&self,x:&ArrayType)->bool{
        return self.name == x.name;
    }
}

// ================= BUFFER TYPE =================
#[derive(Debug, Clone)]
pub struct BufferType{
    pub name:String,
    pub outFn:fn(&Vec<(Type,Type)>) -> String,
    pub methods:RefCell<HashMap<String,FUNC>>,
}

impl BufferType{
    pub fn new(name:String,outFn:fn(&Vec<(Type,Type)>) -> String,)->Self{
        Self{name,outFn,methods:RefCell::new(HashMap::new()),}
    }
    pub fn isinstance(&self,x:String)->bool{
        return self.name == x;
    }
}

// ================= ARRAY =================
#[derive(Debug, Clone)]
pub struct Array{
    pub items:Vec<Type>,
    pub base:ArrayType,
}

impl Array{
    pub fn new(items:Vec<Type>,base:ArrayType)->Self{
        Self{items,base,}
    }
    pub fn runM(&self,func:String,items:Vec<Type>,base:ArrayType)->Type{
        if let Some(i) = base.methods.borrow().get(&func.clone()){
            Type::newVoid()
        }
        else{
            panic!("Undefined method {}",func);
        }
    }
}

// ================= BUFFER =================

#[derive(Debug, Clone)]
pub struct Buffer{
    pub mapp:Vec<(Type,Type)>,
    pub base:BufferType,
}
impl Buffer{
    pub fn new(mapp:Vec<(Type,Type)>,base:BufferType)->Self{
        Self{mapp,base,}
    }
}

// ================= DEFAULT OUT =================
pub fn default_array_out(v:&Vec<Type>) -> String{
    let mut vals=String::new();
    for i in v{
        vals.push_str(&format!("{}, ",i.__out__()));
    }
    if vals.len() >= 2{vals.truncate(vals.len()-2);}
    format!("[{}]",vals)
}

pub fn default_buffer_out(v:&Vec<(Type,Type)>) -> String{
    let mut vals=String::new();
    for (k,val) in v{
        vals.push_str(&format!("{}:{}, ",k.__out__(),val.__out__()));
    }
    if vals.len() >= 2{vals.truncate(vals.len()-2);}
    format!("{{{}}}",vals)
}

pub fn create_builtins_funcs() -> HashMap<String, FUNC> {
    use crate::builtinsdata::*;
    let mut builtins: HashMap<String, FUNC> = HashMap::new();
    builtins.insert(
        "pyrintnos".to_string(),FUNC{func:pr}
    );
    builtins.insert(
        "morfenlnos".to_string(),FUNC{func:ask}
    );
    builtins
}
