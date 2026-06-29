mod token;mod lexer;mod ast;mod builtins;mod parser;mod interpreter;mod environment;
use lexer::Lexer;use parser::Parser;use interpreter::Interpreter;use std::fs;
mod libs;mod runtime;mod builtinsdata;

fn load() -> String{
    let code = fs::read_to_string("src/test.txt").unwrap();
    return code;
}

fn run(code:String){
    let lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer);
    let program = parser.parse();
    let mut interpreter = Interpreter::new();
    interpreter.run(program);
}

fn main() {
    let code = load();
    run(code);
}