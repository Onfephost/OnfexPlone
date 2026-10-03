use crate::ast::*;
use std::collections::HashMap;

#[derive(Debug,Clone)]
pub struct Environment {
    pub parent: Option<Box<Environment>>,
    pub names: HashMap<String, Vec<usize>>,
    pub types : HashMap<Vec<usize>,TypeKind>,
    pub mutable: HashMap<Vec<usize>,bool>,
    pub heap: HashMap<usize, Type>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            parent: None,
            names: HashMap::new(),
            types: HashMap::new(),
            mutable: HashMap::new(),
            heap: HashMap::new(),
        }
    }

    pub fn child(parent: Environment) -> Self {
        Self {
            parent: Some(Box::new(parent)),
            names: HashMap::new(),
            types: HashMap::new(),
            mutable: HashMap::new(),
            heap: HashMap::new(),
        }
    }
}
