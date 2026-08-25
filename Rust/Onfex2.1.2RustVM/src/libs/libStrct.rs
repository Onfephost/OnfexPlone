use std::collections::HashMap;
use crate::builtins::*;
use crate::ast::*;
use crate::error::OnfexError;
use std::cell::RefCell;

#[derive(Debug, Clone, PartialEq)]
pub struct Library {
    pub name: String,
    pub funcs: HashMap<String, Fnc>,
    pub vars: RefCell<HashMap<String,Type>>,
    pub array_types: HashMap<String, ArrayType>,
    pub buffer_types: HashMap<String, BufferType>,
    pub mono_types: HashMap<String, MonoType>,
}

impl Library {
    pub fn new(n: String,fs: HashMap<String,Fnc>,
        _vars:HashMap<String,Type>,at: HashMap<String, ArrayType>,bt: HashMap<String, BufferType>,mt:HashMap<String, MonoType>) -> Self {
            Self { name: n, funcs: fs,vars:RefCell::new(_vars), array_types: at, buffer_types: bt ,mono_types:mt}
    }
}
