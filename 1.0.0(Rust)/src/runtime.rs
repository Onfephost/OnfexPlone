use crate::ast::Type;
#[derive(Debug)]
pub enum RuntimeSignal{
    Return(Type),
    Error(String),
}
pub type OnfexResult=Result<Type,RuntimeSignal>;