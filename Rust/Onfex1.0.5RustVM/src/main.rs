mod token;
mod lexer;
mod ast;
mod parser;
mod interpreter;
mod bytecode;
mod compiler;
mod vm;
mod environment;
mod libs;
mod runtime;
mod builtinsdata;
mod error;
mod builtins;
mod document;
mod ostools;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;
use error::OnfexError;
use std::fs;
use std::env;
use document::OnfexPloneDoph;
type OPD = OnfexPloneDoph;

fn main() {
    let (path, name) =(
            "/storage/emulated/0/OnfexPlone/Rust/Onfex1.0.5RustVM/src".to_string(),
            "strouct_demo.onfex".to_string(),
        );

    let prog = OPD::new(path, name, false);
    if false {
        prog.run_bytecode();
    } else {
        prog.run();
    }
}