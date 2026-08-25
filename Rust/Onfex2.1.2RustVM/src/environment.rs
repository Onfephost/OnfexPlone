use crate::ast::*;
use std::collections::HashMap;


#[derive(Debug,Clone)]
pub struct Environment {
    pub parent: Option<Box<Environment>>,
    // isim -> adres
    pub names: HashMap<String, usize>,
    // adres -> değer
    pub heap: HashMap<usize, Type>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            parent: None,
            names: HashMap::new(),
            heap: HashMap::new(),
        }
    }

    pub fn child(parent: Environment) -> Self {
        Self {
            parent: Some(Box::new(parent)),
            names: HashMap::new(),
            heap: HashMap::new(),
        }
    }
}
