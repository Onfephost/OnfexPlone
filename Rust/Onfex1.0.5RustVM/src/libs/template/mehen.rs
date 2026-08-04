use std::collections::HashMap;
use crate::ast::*;
use crate::libs::libStrct::Library;
use super::Cache::*;

pub fn load()->Library{
    Library::new("tmp".to_string(),HashMap::new(),HashMap::new(),HashMap::new(),HashMap::new(),HashMap::new())
}