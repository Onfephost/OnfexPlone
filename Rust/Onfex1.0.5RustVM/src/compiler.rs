// - `Compiler`: AST'yi bir kez gezip yukarıdaki Chunk'ları üretir. Aynı
//              fonksiyon/metod bir daha asla AST üzerinden yürünmez; VM
//              sadece derlenmiş OpCode dizisini "koşar".

// =========================================================================
// COMPILER
// =========================================================================
use crate::ast::*;
use crate::builtins::*;
use crate::error::OnfexError;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use crate::bytecode::*;

pub struct Compiler {
    program: Program,
    chunk_stack: Vec<Chunk>,
    known_functions: HashSet<String>,
    known_structs: HashSet<String>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            program: Program::new(),
            chunk_stack: vec![Chunk::new()],
            known_functions: HashSet::new(),
            known_structs: HashSet::new(),
        }
    }

    pub fn compile(mut self, stmts: &[StmtNode]) -> Result<Program, OnfexError> {
        println!("Onfex Compiling...");
        self.compile_block(stmts)?;
        self.program.main = self.chunk_stack.pop().unwrap();
        Ok(self.program)
    }

    fn emit(&mut self, op: OpCode) -> usize {
        let chunk = self.chunk_stack.last_mut().unwrap();
        chunk.code.push(op);
        chunk.code.len() - 1
    }

    fn here(&self) -> usize {
        self.chunk_stack.last().unwrap().code.len()
    }

    fn patch_jump(&mut self, idx: usize, target: usize) {
        let chunk = self.chunk_stack.last_mut().unwrap();
        chunk.code[idx] = match &chunk.code[idx] {
            OpCode::Jump(_) => OpCode::Jump(target),
            OpCode::JumpIfFalse(_) => OpCode::JumpIfFalse(target),
            other => other.clone(),
        };
    }

    /// Bir fonksiyon/metod gövdesini KENDİ (ayrı) Chunk'ına derler; bu Chunk
    /// o fonksiyonun/metodun çağrı sınırıdır (call boundary).
    fn compile_function_body(&mut self, body: &[StmtNode]) -> Result<Chunk, OnfexError> {
        self.chunk_stack.push(Chunk::new());
        let res = self.compile_block(body);
        let chunk = self.chunk_stack.pop().unwrap();
        res?;
        Ok(chunk)
    }

    /// Bir statement dizisini derler. Kural: net olarak TAM OLARAK BİR
    /// değer stack'te bırakır (son statement'ın "Flow" değeri) -- bu,
    /// interpreter'daki `out = v` izlemesinin bytecode karşılığıdır.
    fn compile_block(&mut self, stmts: &[StmtNode]) -> Result<(), OnfexError> {
        if stmts.is_empty() {
            self.emit(OpCode::PushVoid);
            return Ok(());
        }
        for (i, s) in stmts.iter().enumerate() {
            if i > 0 {
                self.emit(OpCode::Pop);
            }
            self.compile_stmt(s)?;
        }
        Ok(())
    }

    fn compile_stmt(&mut self, s: &StmtNode) -> Result<(), OnfexError> {
        match &s.stmt {
            Stmt::ExprNode(e) => {
                self.compile_expr(&e.expr)?;
            }
            Stmt::Assign(name, e) => {
                self.compile_expr(&e.expr)?;
                self.emit(OpCode::Dup);
                self.emit(OpCode::DefineVar(name.clone()));
            }
            Stmt::ReAssign(name, e) => {
                self.compile_expr(&e.expr)?;
                self.emit(OpCode::Dup);
                self.emit(OpCode::SetVar(name.clone()));
            }
            Stmt::MemberAssign(base, field, val) => {
                self.compile_expr(&base.expr)?;
                self.compile_expr(&val.expr)?;
                self.emit(OpCode::SetField(field.clone()));
            }
            Stmt::Return(opt) => {
                match opt {
                    Some(e) => self.compile_expr(&e.expr)?,
                    None => {
                        self.emit(OpCode::PushVoid);
                    }
                }
                self.emit(OpCode::Return);
            }
            Stmt::FuncCre(name, params, body, out) => {
                if params.iter().any(|p| p.vararg) {
                    return Err(OnfexError::runtime(format!(
                        "Bytecode Ern: '{}' afon '...' (vararg) promterlnos esp desteklenmiyor",
                        name
                    )));
                }
                // Kendi ismini önceden tanıyarak recursion'a izin ver.
                self.known_functions.insert(name.clone());
                let chunk = self.compile_function_body(body)?;
                self.program.functions.insert(name.clone(), chunk);
                self.program
                    .function_meta
                    .insert(name.clone(), Frounct::new(params.clone(), body.clone(), out.clone()));
                self.emit(OpCode::DefineFunction(name.clone()));
            }
            Stmt::StrctCre(name, fields, funcs) => {
                self.known_structs.insert(name.clone());
                let mut methods: HashMap<String, Frounct> = HashMap::new();
                for (mname, f) in funcs {
                    if let Stmt::FuncCre(n, p, b, o) = f {
                        if p.iter().any(|pp| pp.vararg) {
                            return Err(OnfexError::runtime(format!(
                                "Bytecode Ern: '{}::{}' metodnos '...' (vararg) esp desteklenmiyor",
                                name, n
                            )));
                        }
                        let chunk = self.compile_function_body(b)?;
                        self.program.methods.insert(format!("{}::{}", name, n), chunk);
                        methods.insert(mname.clone(), Frounct::new(p.clone(), b.clone(), o.clone()));
                    }
                }
                let st = StructType::new(name.clone(), fields.clone(), methods);
                self.program.struct_types.insert(name.clone(), st);
                self.emit(OpCode::DefineStruct(name.clone()));
            }
            Stmt::IfElse(cond, body, else_body) => {
                self.emit(OpCode::EnterScope);
                self.compile_stmt(cond)?;
                let jf = self.emit(OpCode::JumpIfFalse(usize::MAX));
                self.compile_block(body)?;
                let jend = self.emit(OpCode::Jump(usize::MAX));
                let else_start = self.here();
                self.patch_jump(jf, else_start);
                match else_body {
                    Some(eb) => self.compile_block(eb)?,
                    None => {
                        self.emit(OpCode::PushVoid);
                    }
                }
                let end = self.here();
                self.patch_jump(jend, end);
                self.emit(OpCode::ExitScope);
            }
            Stmt::Mehen(body) => {
                self.emit(OpCode::EnterScope);
                self.compile_block(body)?;
                self.emit(OpCode::ExitScope);
            }
            Stmt::Import(_) | Stmt::Mod(_) | Stmt::TypeLib(_, _) | Stmt::TypeMod(_, _) => {
                return Err(OnfexError::runtime(
                    "Bytecode Ern: 'urso' / 'mot' / 'wrossnosLrib' / 'wrossnosMot' şu anda bytecode derleyicisi tarafından desteklenmiyor; bu dosya için yorum ağacı (interpreter) motorunu kullanın.",
                ));
            }
        }
        Ok(())
    }

    fn compile_expr(&mut self, e: &Expr) -> Result<(), OnfexError> {
        match e {
            Expr::Int(x) => {
                self.emit(OpCode::PushInt(*x));
            }
            Expr::Float(x) => {
                self.emit(OpCode::PushFloat(*x));
            }
            Expr::Str(x) => {
                self.emit(OpCode::PushStr(x.clone()));
            }
            Expr::Bool(x) => {
                self.emit(OpCode::PushBool(*x));
            }
            Expr::Void => {
                self.emit(OpCode::PushVoid);
            }
            Expr::Variable(name) => {
                self.emit(OpCode::GetVar(name.clone()));
            }
            Expr::BinaryOp(l, op, r) => {
                self.compile_expr(l)?;
                self.compile_expr(r)?;
                self.emit(OpCode::BinOp(op.clone()));
            }
            Expr::Member(base, field) => {
                self.compile_expr(base)?;
                self.emit(OpCode::GetField(field.clone()));
            }
            Expr::MethodCall(recv, name, args) => {
                self.compile_expr(recv)?;
                for a in args {
                    self.compile_expr(a)?;
                }
                self.emit(OpCode::CallMethod(name.clone(), args.len()));
            }
            Expr::Call(callee, args, ft) => match &**callee {
                Expr::Macro(mname) => {
                    for a in args {
                        self.compile_expr(a)?;
                    }
                    self.emit(OpCode::CallBuiltin(mname.clone(), args.len()));
                }
                Expr::Variable(vname) if self.known_structs.contains(vname) => {
                    if !args.is_empty() {
                        return Err(OnfexError::runtime(format!(
                            "WrossnosStrouct Ern: '{}' strouct örneği alt sadece alan(field) esp gephnosan",
                            vname
                        )));
                    }
                    let mut names: Vec<String> = Vec::with_capacity(ft.len());
                    for (fname, fexpr) in ft {
                        self.compile_expr(fexpr)?;
                        names.push(fname.clone());
                    }
                    self.emit(OpCode::NewStruct(vname.clone(), names));
                }
                Expr::Variable(vname) if self.known_functions.contains(vname) => {
                    for a in args {
                        self.compile_expr(a)?;
                    }
                    self.emit(OpCode::CallFunction(vname.clone(), args.len()));
                }
                _ => {
                    return Err(OnfexError::runtime(
                        "Bytecode Ern: bu çağrı türü (lrib/mot değişkeni, ya da henüz tanımlanmamış/dinamik bir fonksiyon) şu anda bytecode derleyicisi tarafından desteklenmiyor.",
                    ));
                }
            },
            _ => {
                return Err(OnfexError::runtime(
                    "Bytecode Ern: bu ifade türü (vektöre/mappe literali, urso/mot değişkeni, referans/spread, vb.) şu anda bytecode derleyicisi tarafından desteklenmiyor.",
                ));
            }
        }
        Ok(())
    }
}