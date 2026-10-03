use crate::environment::*;
use crate::runtime;
use crate::token::*;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::Interpreter;
use crate::bytecode::*;
use crate::compiler::*;
use crate::vm::*;
use crate::error::OnfexError;
use crate::ast::{Stmt, StmtNode};
use std::fs;
use std::cell::RefCell;

/// "Self-hosted" standart kütüphane dosyası: Onfex'in KENDİ dilinde
/// (Rust'ta değil) yazılır ve her üst seviye (import olmayan) programın
/// başına otomatik olarak eklenir -- tıpkı Rust'ın prelude'u gibi, `mot`
/// ile açıkça içeri almaya gerek kalmadan `Vektöre` gibi std tanımları
/// doğrudan kullanılabilir olur. Bkz. `OnfexPloneDoph::run`/`run_bytecode`.
const STD_LIB_NAME: &str = "std.onfex";

pub struct OnfexPloneDoph{
    pub version:String,
    pub path:String,
    pub name:String,
    pub isImport:bool,
    pub code:String,
}

impl OnfexPloneDoph{
    pub fn new(path:String,name:String,isImport:bool)->Self{
        let full = format!("{}/{}", path, name);
        let code = match fs::read_to_string(&full) {
            Ok(c) => c,
            Err(_e) => {
                println!("{}", OnfexError::runtime(format!("Dophcumt Opernal Ern: ern oft rendnen dophcumt: {}", full)));
                String::new()
            }
        };
        Self{version:"2.0.0".to_string(),path,name,isImport,code,}
    }
    
    pub fn verifyVersion(&self,v:String) -> bool{
        return self.version == v;
    }

    /// Bu dosya, std kütüphanesinin otomatik olarak kendine eklenmesini
    /// İSTEMEYEN bir dosya mı? (std.onfex'in kendisi, ya da `mot`/`urso`
    /// ile içeri alınan herhangi bir dosya -- bkz. `STD_LIB_NAME` ve
    /// `prepend_std`/`load_std_stmts`.) Bu olmadan std.onfex kendini
    /// sonsuz özyinelemeli biçimde içeri almaya çalışırdı.
    fn skip_std_injection(&self) -> bool {
        self.isImport || self.name == STD_LIB_NAME
    }

    /// `std.onfex`'i (bu dosyayla aynı klasörden, varsa) ayrıştırır ve
    /// programın BAŞINA eklenecek statement listesini döner (yoksa `None`).
    /// std.onfex'in kendi `mehen{}` gövdesi (varsa) hiçbir zaman
    /// ÇALIŞTIRILMAZ -- bir kütüphane olarak sadece üst seviye tanımları
    /// (valt/frounct/strouct) katkı sağlar.
    fn load_std_stmts(&self) -> Result<Option<Vec<StmtNode>>, OnfexError> {
        let std_path = format!("{}/{}", self.path, STD_LIB_NAME);
        if !std::path::Path::new(&std_path).exists() {
            return Ok(None);
        }
        let std_code = fs::read_to_string(&std_path).map_err(|_| {
            OnfexError::runtime(format!("Dophcumt Opernal Ern: ern oft rendnen dophcumt: {}", std_path))
        })?;
        let _std_src = crate::error::set_source_context(std_path, &std_code);
        let std_lexer = Lexer::new(std_code);
        let std_program = Parser::new(std_lexer).and_then(|mut p| p.parse())?;
        Ok(Some(
            std_program
                .into_iter()
                .filter(|s| !matches!(s.stmt, Stmt::Mehen(_)))
                .collect(),
        ))
    }
    
    pub fn run(&self) -> Result<Option<RefCell<Environment>>,OnfexError>{
        let _src = crate::error::set_source_context(format!("{}/{}", self.path, self.name), &self.code);
        let lexer = Lexer::new(self.code.clone());
        let mut program = match Parser::new(lexer).and_then(|mut parser| parser.parse()) {
            Ok(p) => Ok(p),
            Err(e) => {
                return Err(e)
            }
        }?;
        if !self.skip_std_injection() {
            if let Some(std_stmts) = self.load_std_stmts()? {
                let mut merged = std_stmts;
                merged.extend(program);
                program = merged;
            }
        }
        let interpreter = Interpreter::new(self.path.clone(),self.code.clone());
        *interpreter.Import.borrow_mut() = self.isImport.clone();
        if self.isImport.clone(){
            match interpreter.run(program) {
                Ok(_) => Ok(Some(interpreter.env)),
                Err(e) => {
                    Err(e)
                }
            }
        }else{
            match interpreter.run(program) {
                Ok(_) => return Ok(None),
                Err(e) => {
                Err(e)
                }
            }
        }
    }
    // Aynı `run()` gibi çalışır, ancak ağaç-yürüten Interpreter yerine
    // bytecode derleyicisi + VM kullanır. Şu an için urso/mot/kütüphane
    // ifadeleri içermeyen dosyalarda kullanılabilir (bkz. bytecode.rs
    // başındaki "KAPSAM DIŞI" notu).
    pub fn run_bytecode(&self) -> Option<()> {
        let _src = crate::error::set_source_context(format!("{}/{}", self.path, self.name), &self.code);
        let lexer = Lexer::new(self.code.clone());
        let program = match Parser::new(lexer).and_then(|mut parser| parser.parse()) {
            Ok(p) => p,
            Err(e) => {
                println!("{}", e);
                return None;
            }
        };
        let compiler = Compiler::new();
        let mut compiled = match compiler.compile(&program) {
            Ok(p) => p,
            Err(e) => {
                println!("{}", e);
                return None;
            }
        };
        if !self.skip_std_injection() {
            if let Err(e) = self.prepend_std(&mut compiled) {
                println!("{}", e);
                return None;
            }
        }
        let mut vm = VM::new(compiled,self.path.clone());
        match vm.run() {
            Ok(_) => None,
            Err(e) => {
                println!("{}", e);
                None
            }
        }
    }

    /// `std.onfex`'i (bu dosyayla aynı klasörden, varsa) derler ve TÜM
    /// chunk'larını ana programa katar; std'nin ana (mehen) chunk'ının
    /// KODU, asıl programın ana chunk'ının EN BAŞINA eklenir -- böylece
    /// std'nin üst seviye valt/frounct/strouct tanımları, programın geri
    /// kalanı çalışmadan ÖNCE çalışıp aynı global kapsama yerleşir (bir
    /// önek gerekmeden doğrudan kullanılabilirler, tıpkı Rust'ın prelude'u
    /// gibi -- örn. `std->Vektöre.meess(...)` yerine artık doğrudan
    /// `Vektöre.meess(...)`).
    ///
    /// std'nin kodu asıl kodun ÖNÜNE eklendiği için, asıl programın ana
    /// chunk'ındaki sıçrama (Jump/JumpIfFalse) hedefleri std'nin kod
    /// uzunluğu kadar kaydırılır; std'nin KENDİ sıçramaları (0'dan
    /// başladığı için) hiç değişmeden doğru kalır.
    fn prepend_std(&self, program: &mut Program) -> Result<(), OnfexError> {
        let std_path = format!("{}/{}", self.path, STD_LIB_NAME);
        if !std::path::Path::new(&std_path).exists() {
            return Ok(());
        }
        let std_doc = OnfexPloneDoph::new(self.path.clone(), STD_LIB_NAME.to_string(), true);
        let std_program = std_doc.compile_project()?;

        let mut std_chunks = std_program.chunks;
        if std_chunks.is_empty() {
            return Ok(());
        }
        let std_main = std_chunks.remove(0);
        let offset = std_main.code.len();

        let main_chunk = &mut program.chunks[0];
        for op in main_chunk.code.iter_mut() {
            match op {
                OpCode::Jump(t) | OpCode::JumpIfFalse(t) => {
                    *t += offset;
                }
                _ => {}
            }
        }
        let mut new_code = std_main.code;
        new_code.append(&mut main_chunk.code);
        main_chunk.code = new_code;

        let mut new_positions = std_main.positions;
        new_positions.append(&mut main_chunk.positions);
        main_chunk.positions = new_positions;

        // std'nin diğer chunk'larını (fonksiyon/metod/strouct tanımları) da
        // programa ekle -- DefineFunction/DefineStruct opcode'ları bunları
        // isimle bulabilsin diye.
        program.chunks.extend(std_chunks);
        Ok(())
    }

    /// `run_bytecode()` ile aynı ilk adımları atar (dosyayı okur, ayrıştırır,
    /// derler) ama SONUCU ÇALIŞTIRMAZ -- derlenmiş `Program`'ı döner.
    /// "mot" ile içeri alınan bytecode modülleri (`vm.rs::VM::load_module`)
    /// bunu kullanır. `self.isImport` true olduğunda derleyici `mehen{}`
    /// gövdesini atlar (bkz. `Compiler::new_import`), tıpkı `run()`'ın
    /// ağaç-yürüten Interpreter için yaptığı gibi. `mot` ile içeri alınan
    /// dosyalara std OTOMATİK EKLENMEZ (sadece üst seviye ana dosyaya) --
    /// bkz. `skip_std_injection`.
    pub fn compile_project(&self) -> Result<Program, OnfexError> {
        let _src = crate::error::set_source_context(format!("{}/{}", self.path, self.name), &self.code);
        let lexer = Lexer::new(self.code.clone());
        let program = Parser::new(lexer).and_then(|mut parser| parser.parse())?;
        let compiler = if self.isImport {
            Compiler::new_import()
        } else {
            Compiler::new()
        };
        compiler.compile(&program)
    }

    pub fn freerun(&self,path:String,name:String) -> i8{
        //This not supporting multi document inserted projects
        let full = format!("{}/{}", path, name);
        let code = match fs::read_to_string(&full) {
            Ok(c) => c,
            Err(e) => {
                println!("{}", OnfexError::runtime(format!("Dophcumt Opernal Ern: ern oft rendnen: {} ({})", full, e)));
                return 1;
            }
        };
        let _src = crate::error::set_source_context(full, &code);
        let lexer = Lexer::new(code.clone());
        let program = match Parser::new(lexer).and_then(|mut parser| parser.parse()) {
            Ok(p) => p,
            Err(e) => {
                println!("{}", e);
                return 1;
            }
        };
        let interpreter = Interpreter::new("".to_string(),code.clone());
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