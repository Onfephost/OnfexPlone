use std::collections::HashMap;
use crate::libs::libStrct::Library;

use super::Cache::*;

pub fn load() -> Library {
    Library::new("neomOnfex".to_string(), load_funcs(),load_vars(), load_arrays(), HashMap::new(),HashMap::new())
}
