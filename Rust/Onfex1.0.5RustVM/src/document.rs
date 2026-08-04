use crate::ast;
use crate::environment::*;
use crate::runtime;
use crate::builtinsdata;
use crate::builtins;
use crate::token::*;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::Interpreter;
use crate::bytecode::*;
use crate::compiler::*;
use crate::vm::*;
use crate::error::OnfexError;
use std::fs;
use std::cell::RefCell;

pub struct OnfexPloneDoph{
    pub version:String,
    pub path:String,
    pub name:String,
    pub isImport:bool,
    pub code:String,
}

impl OnfexPloneDoph{
    pub fn new(path:String,name:String,isImport:bool)->Self{
        let code = match fs::read_to_string(format!("{}/{}", path, name)) {
            Ok(c) => c,
            Err(e) => {
                // reported immediately by the runtime() constructor; fall back
                // to an empty document instead of panicking.
                OnfexError::runtime(format!("Dophcumt Opernal Ern: ern oft rendnen dophcumt: {}",path.clone() ));
                String::new()
            }
        };
        Self{version:"0.7.1".to_string(),path,name,isImport,code,}
    }
    
    pub fn verifyVersion(&self,v:String) -> bool{
        return self.version == v;
    }
    
    pub fn run(&self) -> Option<RefCell<Environment>>{
        let lexer = Lexer::new(self.code.clone());
        let program = match Parser::new(lexer).and_then(|mut parser| parser.parse()) {
            Ok(p) => p,
            Err(e) => {
                println!("{}", e);
                return None;
            }
        };
        let interpreter = Interpreter::new(self.path.clone());
        *interpreter.Import.borrow_mut() = self.isImport.clone();
        if self.isImport.clone(){
            match interpreter.run(program) {
                Ok(_) => Some(interpreter.env),
                Err(e) => {
                    println!("{}", e);
                    return None;
                }
            }
        }else{
            match interpreter.run(program) {
                Ok(_) => return None,
                Err(e) => {
                println!("{}", e);
                    return None;
                }
            }
        }
    }
    // Aynı `run()` gibi çalışır, ancak ağaç-yürüten Interpreter yerine
    // bytecode derleyicisi + VM kullanır. Şu an için urso/mot/kütüphane
    // ifadeleri içermeyen dosyalarda kullanılabilir (bkz. bytecode.rs
    // başındaki "KAPSAM DIŞI" notu).
    pub fn run_bytecode(&self) -> Option<()> {
        let lexer = Lexer::new(self.code.clone());
        let program = match Parser::new(lexer).and_then(|mut parser| parser.parse()) {
            Ok(p) => p,
            Err(e) => {
                println!("{}", e);
                return None;
            }
        };
        let compiler = Compiler::new();
        let compiled = match compiler.compile(&program) {
            Ok(p) => p,
            Err(e) => {
                println!("{}", e);
                return None;
            }
        };
        let mut vm = VM::new(compiled);
        match vm.run() {
            Ok(_) => None,
            Err(e) => {
                println!("{}", e);
                None
            }
        }
    }

    pub fn freerun(path:String,name:String) -> i8{
        //This not supporting multi document inserted projects
        let code = match fs::read_to_string(format!("{}/{}", path, name)) {
            Ok(c) => c,
            Err(e) => {
                // reported immediately by the runtime() constructor.
                OnfexError::runtime(format!("Dophcumt Opernal Ern: ern oft rendnen: {}", e));
                return 1;
            }
        };
        let lexer = Lexer::new(code);
        let program = match Parser::new(lexer).and_then(|mut parser| parser.parse()) {
            Ok(p) => p,
            Err(e) => {
                println!("{}", e);
                return 1;
            }
        };
        let interpreter = Interpreter::new("".to_string());
        *interpreter.selfAccess.borrow_mut() = true;
        let selfAccess = interpreter.selfAccess.borrow().clone();
        match interpreter.run(program) {
                Ok(_) => {
                    return 0;
                },
                Err(e) => {
                    println!("{}", e);
                    return 1;
                }
            }
    }
}