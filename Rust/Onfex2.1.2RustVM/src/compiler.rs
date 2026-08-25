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
    known_functions: HashSet<String>,
    known_structs: HashSet<String>,
    is_import: bool,
    current_pos: (usize, usize),
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            program: Program::new(), // chunks = [<ana chunk, index 0>], cn = 0
            known_functions: HashSet::new(),
            known_structs: HashSet::new(),
            is_import: false,
            current_pos: (0, 0),
        }
    }

    /// `mot "yol.onfex";` ile içeri alınan bir dosya için derleyici: bkz.
    /// `is_import` alanı.
    pub fn new_import() -> Self {
        Self {
            is_import: true,
            ..Self::new()
        }
    }

    pub fn compile(mut self, stmts: &[StmtNode]) -> Result<Program, OnfexError> {
        if !self.is_import.clone(){
            println!("Onfex Compiling...");
        }
        self.program.cn = 0; // ana chunk hedefte
        self.compile_block(stmts)?;
        Ok(self.program)
    }

    fn emit(&mut self, op: OpCode) -> usize {
        let cn = self.program.cn;
        self.program.chunks[cn].code.push(op);
        self.program.chunks[cn].positions.push(self.current_pos);
        self.program.chunks[cn].code.len() - 1
    }

    fn here(&self) -> usize {
        self.program.chunks[self.program.cn].code.len()
    }

    fn patch_jump(&mut self, idx: usize, target: usize) {
        let cn = self.program.cn;
        self.program.chunks[cn].code[idx] = match &self.program.chunks[cn].code[idx] {
            OpCode::Jump(_) => OpCode::Jump(target),
            OpCode::JumpIfFalse(_) => OpCode::JumpIfFalse(target),
            other => other.clone(),
        };
    }
    
    fn compile_named_chunk(&mut self, name: String, body: &[StmtNode]) -> Result<usize, OnfexError> {
        let idx = self.program.chunks.len();
        self.program.chunks.push(Chunk::named(name));
        let prev_cn = self.program.cn;
        self.program.cn = idx;
        let res = self.compile_block(body);
        self.program.cn = prev_cn;
        res?;
        Ok(idx)
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
        self.current_pos = (s.line, s.col);
        match &s.stmt {
            Stmt::ExprNode(e) => {
                self.compile_expr(&e.expr)?;
            }
            Stmt::Assign(name, e) => {
                self.compile_expr(&e.expr)?;
                // DefineVar kendi içinde pop+tanımla+değeri geri it yapar
                // (bkz. bytecode.rs), bu yüzden burada AYRICA Dup GEREKMEZ
                // -- fazladan bir Dup, statement başına net +1 yerine +2
                // bırakıp yığının sessizce büyümesine (ve sonunda "yığın
                // boşaldı" hatalarına) yol açardı.
                self.emit(OpCode::DefineVar(name.clone()));
            }
            Stmt::ReAssign(name, e) => {
                self.compile_expr(&e.expr)?;
                // Aynı gerekçe: SetVar de kendi içinde pop+güncelle+geri it
                // yapıyor, ayrı bir Dup fazladan.
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
            Stmt::FuncCre(name, generics, params, body, out) => {
                if params.iter().any(|p| p.vararg) {
                    return Err(OnfexError::runtime(format!("Bytecode Ern: '{}' afon '...' (vararg) promterlnos esp desteklenmiyor",name)));
                }
                // Kendi ismini önceden tanıyarak recursion'a izin ver.
                self.known_functions.insert(name.clone());
                let idx = self.compile_named_chunk(name.clone(), body)?;
                self.program.chunks[idx].meta = Some(Frounct::new(
                    name.clone(), generics.clone(), params.clone(), body.clone(), out.clone(),
                ));
                self.emit(OpCode::DefineFunction(name.clone()));
            }
            Stmt::StrctCre(name, generics, fields, funcs) => {
                self.known_structs.insert(name.clone());
                let mut methods: HashMap<String, Frounct> = HashMap::new();
                for (mname, f) in funcs {
                    if let Stmt::FuncCre(n, g, p, b, o) = f {
                        if p.iter().any(|pp| pp.vararg) {
                            return Err(OnfexError::runtime(format!("Bytecode Ern: '{}::{}' metodnos '...' (vararg) esp desteklenmiyor",name, n)));
                        }
                        self.compile_named_chunk(format!("{}::{}", name, n), b)?;
                        methods.insert(mname.clone(), Frounct::new(n.clone(),g.clone(),p.clone(), b.clone(), o.clone()));
                    }
                }
                let st = StructType::new(name.clone(), generics.clone(), fields.clone(), methods);
                let mut struct_chunk = Chunk::named(name.clone());
                struct_chunk.struct_meta = Some(st);
                self.program.chunks.push(struct_chunk);
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
            Stmt::Forp(vars, iter_expr, body) => {
                // forp x, y, z intf <ifade> { <gövde> } -- bkz. bytecode.rs'deki
                // MakeIter/IterHasNext/IterNext/BindForpVars yorumları.
                self.compile_expr(&iter_expr.expr)?;
                self.emit(OpCode::MakeIter);
                let loop_start = self.here();
                self.emit(OpCode::IterHasNext);
                let jf = self.emit(OpCode::JumpIfFalse(usize::MAX));
                self.emit(OpCode::IterNext);
                self.emit(OpCode::BindForpVars(vars.clone()));
                self.emit(OpCode::EnterScope);
                self.compile_block(body)?;
                self.emit(OpCode::ExitScope);
                self.emit(OpCode::Pop); // gövdenin net +1 değerini at
                self.emit(OpCode::Jump(loop_start));
                let end = self.here();
                self.patch_jump(jf, end);
                self.emit(OpCode::Pop); // Iter'ı yığından at
                self.emit(OpCode::PushVoid); // forp ifadesinin kendi net +1'i
            }
            Stmt::Mehen(body) => {
                if self.is_import {
                    // "mot" ile içeri alınan dosyalarda mehen{} gövdesi
                    // ÇALIŞTIRILMAZ -- ağaç-yürüten motordaki isImport
                    // davranışıyla birebir (bkz. interpreter.rs Stmt::Mehen).
                    self.emit(OpCode::PushVoid);
                } else {
                    self.emit(OpCode::EnterScope);
                    self.compile_block(body)?;
                    self.emit(OpCode::ExitScope);
                }
            }
            Stmt::Raise(st) => {
                self.compile_expr(&st.expr)?;
                self.emit(OpCode::Panic);
            } 
            Stmt::Import(name) => {
                self.emit(OpCode::ImportLib(name.clone()));
            }
            Stmt::Mod(path) => {
                self.emit(OpCode::ImportMod(path.clone()));
            }
            Stmt::TypeLib(new_name, old_name) => {
                self.emit(OpCode::AliasLib(new_name.clone(), old_name.clone()));
            }
            Stmt::TypeMod(new_name, old_name) => {
                self.emit(OpCode::AliasMod(new_name.clone(), old_name.clone()));
            }
            _ => return Err(OnfexError::runtime(
                    "BytecodeC Ern: bu ifade türü (vektöre/mappe literali, referans/spread, vb.) şu anda bytecode derleyicisi tarafından desteklenmiyor.",
                )),
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
            Expr::Decimal(x) => {
                self.emit(OpCode::PushDecimal(*x));
            }
            Expr::TypeKind(st) => {
                self.emit(OpCode::PushType(*st.clone()));
            }
            Expr::Str(x) => {
                self.emit(OpCode::PushStr(x.clone()));
            }
            Expr::Bool(x) => {
                self.emit(OpCode::PushBool(*x));
            }
            Expr::Not(x) => {
                self.compile_expr(&*x)?;
                self.emit(OpCode::Not);
            }
            Expr::ArrayDt(x) => {
                let arr = x.items.clone();
                let ln = arr.len();
                for i in 0..ln.clone(){
                    let er = arr[i].clone().value;
                    self.compile_expr(&er)?;
                }
                self.emit(OpCode::PushVec(ln));
            }
            Expr::BufferDt(x) => {
                let arr = x.mapp.clone();
                let ln = arr.len();
                for i in 0..ln.clone(){
                    let key = arr[i].clone().0;
                    let value = arr[i].clone().1;
                    self.compile_expr(&value.value)?;
                    self.compile_expr(&key.value)?;
                }
                self.emit(OpCode::PushMatris(ln));
            }
            Expr::List(_,items) => {
                let arr = items.clone();
                let ln = arr.len();
                for i in 0..ln.clone(){
                    let er = arr[i].clone().expr;
                    self.compile_expr(&er)?;
                }
                self.emit(OpCode::PushVec(ln));
            }
            Expr::Dict(_,mapp) => {
                let arr = mapp.clone();
                let ln = arr.len();
                for i in 0..ln.clone(){
                    let key = arr[i].clone().0;
                    let value = arr[i].clone().1;
                    self.compile_expr(&value.expr)?;
                    self.compile_expr(&key.expr)?;
                }
                self.emit(OpCode::PushMatris(ln));
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
            Expr::LibVariable(lib, name) => {
                self.emit(OpCode::GetLibVar(lib.clone(), name.clone()));
            }
            Expr::ModVariable(m, name) => {
                self.emit(OpCode::GetModVar(m.clone(), name.clone()));
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
                Expr::LibVariable(lib, fname) => {
                    for a in args {
                        self.compile_expr(a)?;
                    }
                    self.emit(OpCode::CallLibFunc(lib.clone(), fname.clone(), args.len()));
                }
                Expr::ModVariable(m, fname) => {
                    for a in args {
                        self.compile_expr(a)?;
                    }
                    self.emit(OpCode::CallModFunc(m.clone(), fname.clone(), args.len()));
                }
                _ => {
                    return Err(OnfexError::runtime(
                        "BytecodeC Ern: bu çağrı türü (henüz tanımlanmamış/dinamik bir fonksiyon) şu anda bytecode derleyicisi tarafından desteklenmiyor.",
                    ));
                }
            },
            _ => {
                println!("{:?}",e);
                return Err(OnfexError::runtime(
                    "BytecodeC Ern: bu ifade türü (vektöre/mappe literali, referans/spread, vb.) şu anda bytecode derleyicisi tarafından desteklenmiyor.",
                ));
            }
        }
        Ok(())
    }
}