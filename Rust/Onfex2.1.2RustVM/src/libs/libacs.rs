use crate::libs::libStrct::*;
use crate::error::OnfexError;
use std::collections::HashMap;

pub fn loadLib(n: &str) -> Result<Library, OnfexError> {
    match n {
        "rust::osp" => {
            use crate::libs::osp::mehen;
            Ok(mehen::load())
        }
        "rust::tymess" => {
            use crate::libs::tymess::mehen;
            Ok(mehen::load())
        }
        "rust::trinery" => {
            use crate::libs::trinary::mehen;
            Ok(mehen::load())
        }
        "rust::quaternery" => {
            use crate::libs::quaternery::mehen;
            Ok(mehen::load())
        }
        "rust::counth" => {
            use crate::libs::color::mehen;
            Ok(mehen::load())
        }
        "rust::vekris" => {
            use crate::libs::vect_matris::mehen;
            Ok(mehen::load())
        }
        "rust::taphlot" => {
            use crate::libs::taphlot::mehen;
            Ok(mehen::load())
        }
        "rust::Sterge" => {
            use crate::libs::Sterge::mehen;
            Ok(mehen::load())
        }
        "rust::Decmal" => {
            use crate::libs::Decmal::mehen;
            Ok(mehen::load())
        }
        _ => Err(OnfexError::runtime(format!("Lrib '{}' asp neat inferins lrib", n))),
    }
}
