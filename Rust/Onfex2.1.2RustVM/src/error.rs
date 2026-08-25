use std::cell::RefCell;
use std::fmt;
use colored::Colorize;

// =========================================================================
// KAYNAK BAĞLAMI (source context)
//
// Hata mesajlarının rustc'deki gibi ilgili kaynak satırını gösterebilmesi
// için, o an işlenmekte olan dosyanın adı + kaynak kodu burada, thread-local
// bir YIĞIN (stack) olarak tutulur. Yığın kullanılmasının sebebi: bir dosya
// başka bir dosyayı `mot` ile içeri alırken (iç içe modül yükleme), her
// dosyanın kendi bağlamı ayrı kalmalı ve içeri alınan dosyanın işlenmesi
// bitince dış dosyanın bağlamı otomatik olarak geri gelmelidir.
// =========================================================================

#[derive(Debug, Clone)]
struct SourceContext {
    filename: String,
    lines: Vec<String>,
}

thread_local! {
    static SOURCE_STACK: RefCell<Vec<SourceContext>> = RefCell::new(Vec::new());
}

fn current_source() -> Option<SourceContext> {
    SOURCE_STACK.with(|s| s.borrow().last().cloned())
}

fn push_source_context(filename: String, code: &str) {
    SOURCE_STACK.with(|s| {
        s.borrow_mut().push(SourceContext {
            filename,
            lines: code.lines().map(|l| l.to_string()).collect(),
        });
    });
}

fn pop_source_context() {
    SOURCE_STACK.with(|s| {
        s.borrow_mut().pop();
    });
}

/// `SourceContextGuard` scope'ta yaşadığı sürece hata mesajları verilen
/// dosyanın kaynak satırlarını gösterebilir; guard drop olduğunda (scope
/// bittiğinde -- erken `return`/`?` dahil) bağlam otomatik olarak
/// kaldırılır ve varsa bir önceki (dış) dosyanın bağlamı tekrar aktif olur.
///
/// Kullanım: `let _src = set_source_context(format!("{}/{}", path, name), &code);`
pub struct SourceContextGuard {
    _private: (),
}

impl Drop for SourceContextGuard {
    fn drop(&mut self) {
        pop_source_context();
    }
}

pub fn set_source_context(filename: impl Into<String>, code: &str) -> SourceContextGuard {
    push_source_context(filename.into(), code);
    SourceContextGuard { _private: () }
}

// =========================================================================
// ONFEXERROR
// =========================================================================

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
    /// `line`/`col` başlangıçta bilinmeyebilir (`OnfexError::runtime(...)`
    /// bunları `None` bırakır); en yakın `Stmt`/opcode sınırı hatayı
    /// yukarı doğru yayarken `with_location_default` ile doldurur (bkz.
    /// `interpreter.rs::exec` ve `vm.rs::run`). Hata en DERİNDEN geldiği
    /// noktadaki konumu korur -- dıştaki katmanlar sadece eksikse doldurur.
    Runtime {
        message: String,
        line: Option<usize>,
        col: Option<usize>,
    },
    Qtriner {
        message: String,
        line: Option<usize>,
        col: Option<usize>,
    },
}

impl OnfexError {
    /// Human readable name of the error category.
    pub fn kind(&self) -> &'static str {
        match self {
            OnfexError::Lexer { .. } => "Lexer Ern",
            OnfexError::Parser { .. } => "Parser Ern",
            OnfexError::Runtime { .. } => "Runtime Ern",
            OnfexError::Qtriner { .. } => "Qtriner Ern",
        }
    }

    /// The raw message carried by this error, without any decoration.
    pub fn message(&self) -> &str {
        match self {
            OnfexError::Lexer { message, .. }
            | OnfexError::Parser { message, .. }
            | OnfexError::Runtime { message, .. }
            | OnfexError::Qtriner { message, .. } => message,
        }
    }

    /// Source location of this error, if one is tracked.
    pub fn location(&self) -> Option<(usize, usize)> {
        match self {
            OnfexError::Lexer { line, col, .. } => Some((*line, *col)),
            OnfexError::Parser { line, col, .. } => Some((*line, *col)),
            OnfexError::Runtime { line, col, .. } => line.zip(*col),
            OnfexError::Qtriner { line, col, .. } => line.zip(*col),
        }
    }

    /// Bu hatanın henüz bir kaynak konumu YOKSA (`Runtime`/`Qtriner` ve
    /// hâlâ `None`), verilen konumu ekler. Zaten bir konumu varsa (ör. daha
    /// içteki bir ifadeden/opcode'dan geldiyse) DOKUNMAZ -- böylece hata
    /// dıştan içe değil İÇTEN DIŞA yayılırken en isabetli (en derin) konumu
    /// korur; dıştaki katmanlar sadece hâlâ eksikse doldurur.
    pub fn with_location_default(self, line: usize, col: usize) -> Self {
        match self {
            OnfexError::Runtime { message, line: None, .. } => OnfexError::Runtime {
                message,
                line: Some(line),
                col: Some(col),
            },
            OnfexError::Qtriner { message, line: None, .. } => OnfexError::Qtriner {
                message,
                line: Some(line),
                col: Some(col),
            },
            other => other,
        }
    }

    pub fn lexer(message: impl Into<String>, line: usize, col: usize) -> Self {
        OnfexError::Lexer { message: message.into(), line, col }
    }

    pub fn parser(message: impl Into<String>, line: usize, col: usize) -> Self {
        OnfexError::Parser { message: message.into(), line, col }
    }

    pub fn runtime(message: impl Into<String>) -> Self {
        OnfexError::Runtime { message: message.into(), line: None, col: None }
    }

    pub fn qtriner(message: impl Into<String>) -> Self {
        OnfexError::Qtriner { message: message.into(), line: None, col: None }
    }
}

impl fmt::Display for OnfexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "{}{} {}",
            "Error".bright_red().bold(),
            ":".bold(),
            self.message().bold()
        )?;

        match self.location() {
            Some((line, col)) => {
                let src = current_source();
                let filename = src
                    .as_ref()
                    .map(|c| c.filename.clone())
                    .unwrap_or_else(|| "<onfex>".to_string());
                let gutter = line.to_string();
                let pad = " ".repeat(gutter.len());

                writeln!(
                    f,
                    "{}{} {}:{}:{}",
                    pad,
                    "-->".bright_blue().bold(),
                    filename,
                    line,
                    col
                )?;
                writeln!(f, "{} {}", pad, "|".bright_blue().bold())?;

                let src_line = src
                    .as_ref()
                    .and_then(|c| c.lines.get(line.saturating_sub(1)))
                    .cloned();

                match src_line {
                    Some(text) => {
                        writeln!(
                            f,
                            "{} {} {}",
                            gutter.bright_blue().bold(),
                            "|".bright_blue().bold(),
                            text
                        )?;
                        let caret_pad = " ".repeat(col.saturating_sub(1));
                        writeln!(
                            f,
                            "{} {} {}{}",
                            pad,
                            "|".bright_blue().bold(),
                            caret_pad,
                            "^".bright_red().bold()
                        )?;
                    }
                    None => {
                        writeln!(f, "{} {} (kaynak satırı gephnosan)", pad, "|".bright_blue().bold())?;
                    }
                }
                write!(f, "{} {}: {}", pad, "=".bright_blue().bold(), self.kind().dimmed())
            }
            None => {
                write!(
                    f,
                    "  {} {}: {}",
                    "=".bright_blue().bold(),
                    self.kind().dimmed(),
                    "(konum bilgisi mevcut değil)".dimmed()
                )
            }
        }
    }
}
