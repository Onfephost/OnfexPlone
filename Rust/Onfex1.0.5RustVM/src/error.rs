use std::fmt;

#[derive(Debug, Clone)]
pub enum OnfexError {
    Lexer {
        message: String,
        line: usize,
        col: usize,
    },

    Parser {
        message: String,
        line: usize,
        col: usize,
    },

    Runtime {
        message: String,
    },
}

impl OnfexError {
    /// Human readable name of the error category.
    pub fn kind(&self) -> &'static str {
        match self {
            OnfexError::Lexer { .. } => "Lexer Error",
            OnfexError::Parser { .. } => "Parser Error",
            OnfexError::Runtime { .. } => "Runtime Error",
        }
    }

    /// The raw message carried by this error, without any decoration.
    pub fn message(&self) -> &str {
        match self {
            OnfexError::Lexer { message, .. } => message,
            OnfexError::Parser { message, .. } => message,
            OnfexError::Runtime { message } => message,
        }
    }

    /// Source location of this error, if one was tracked.
    pub fn location(&self) -> Option<(usize, usize)> {
        match self {
            OnfexError::Lexer { line, col, .. } => Some((*line, *col)),
            OnfexError::Parser { line, col, .. } => Some((*line, *col)),
            OnfexError::Runtime { .. } => None,
        }
    }

    /// Prints a detailed rendering of this error immediately, at the
    /// point it is created. This is called automatically by the
    /// `lexer`/`parser`/`runtime` constructors below, so an error is
    /// visible right where it originates -- even if it is later
    /// propagated (via `?`) through several more layers before someone
    /// finally handles it.
    fn report(&self) {
        println!("{}", self);
    }

    pub fn lexer(message: impl Into<String>, line: usize, col: usize) -> Self {
        let e = OnfexError::Lexer { message: message.into(), line, col };
        e.report();
        e
    }

    pub fn parser(message: impl Into<String>, line: usize, col: usize) -> Self {
        let e = OnfexError::Parser { message: message.into(), line, col };
        e.report();
        e
    }

    pub fn runtime(message: impl Into<String>) -> Self {
        let e = OnfexError::Runtime { message: message.into() };
        e.report();
        e
    }
}

impl fmt::Display for OnfexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const BAR: &str = "──────────────────────────────────────────";
        writeln!(f, "——{} {}", self.kind(), BAR)?;
        writeln!(f, " Error  : {}", self.message())?;
        match self.location() {
            Some((line, col)) => {
                writeln!(f, "   Location : line {}, column {}", line, col)?;
            }
            None => {
                writeln!(f, "   Location : (not tracked for runtime errors)")?;
            }
        }
        write!(f, " -{}", BAR)
    }
}
