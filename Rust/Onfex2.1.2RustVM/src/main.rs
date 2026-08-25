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
mod OnfexDecimal;
mod QKT;
use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;
use error::OnfexError;
use std::fs;
use std::env;
use document::OnfexPloneDoph;
type OPD = OnfexPloneDoph;
use colored::Colorize;

fn main() {
    let (path, name) =(
        "/storage/emulated/0/OnfexPlone-Heap/Rust/Onfex2.1.2RustVM/src".to_string(),
        "strouct_demo.onfex".to_string(),
    );
    let prog = OPD::new(path, name, false);
    if true {
        prog.run_bytecode();
    }else{
        prog.run();
    }
}