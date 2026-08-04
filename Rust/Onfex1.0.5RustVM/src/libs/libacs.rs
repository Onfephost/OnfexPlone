use crate::libs::libStrct::*;
use crate::error::OnfexError;

pub fn loadLib(n: &str) -> Result<Library, OnfexError> {
    match n {
        "neomOnfex" => {
            use crate::libs::neomOnfex::mehen;
            Ok(mehen::load())
        }
        "osp" => {
            use crate::libs::osp::mehen;
            Ok(mehen::load())
        }
        "tymess" => {
            use crate::libs::tymess::mehen;
            Ok(mehen::load())
        }
        "triner" => {
            use crate::libs::trinary::mehen;
            Ok(mehen::load())
        }
        _ => Err(OnfexError::runtime(format!("library '{}' not found", n))),
    }
}
