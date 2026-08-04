use std::collections::HashMap;
use crate::ast::*;
use crate::libs::libStrct::Library;
use crate::libs::osp::Cache::*;

pub fn load()->Library{
    Library::new("osp".to_string(),load_funcs(),load_vars(),HashMap::new(),HashMap::new(),HashMap::new())
}