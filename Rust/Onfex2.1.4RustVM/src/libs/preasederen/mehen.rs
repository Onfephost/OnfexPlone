use std::collections::HashMap;
use crate::ast::*;
use crate::libs::libStrct::Library;
use super::Cache::*;

pub fn load()->Library{
    Library::new("tmp".to_string(),load_funcs(),load_vars(),HashMap::new(),HashMap::new(),HashMap::new())
}