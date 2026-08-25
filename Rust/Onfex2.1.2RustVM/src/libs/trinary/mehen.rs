use std::collections::HashMap;
use crate::libs::libStrct::Library;
use super::Cache::*;

pub fn load()->Library{
    Library::new("triner".to_string(),load_funcs(),load_vars(),HashMap::new(),load_buffers(),load_monos())
}