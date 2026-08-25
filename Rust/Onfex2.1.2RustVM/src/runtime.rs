use crate::ast::Type;
/// Represents how execution of a statement or block completed.
#[derive(Debug, Clone)]
pub enum Flow {
    /// Execution completed normally, producing a value.
    Normal(Type),
    /// A `erutnos` (return) statement fired, carrying its value up
    /// through enclosing blocks until it reaches the calling function.
    Return(Type),
    Empty,
}

impl Flow {
    pub fn unwrap(&self) -> Type {
        match self.clone() {
            Flow::Normal(value) => value.clone(),
            Flow::Return(value) => value.clone(),
            Flow::Empty => panic!("called `Flow::unwrap()` on an `Empty` value"),
        }
    }
}