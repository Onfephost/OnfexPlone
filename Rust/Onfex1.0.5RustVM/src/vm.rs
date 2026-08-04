// - `VM`      : açık bir operand yığını (`stack: Vec<Type>`) ve isim tabanlı
//              kapsam yığını (`scopes: Vec<HashMap<String,Type>>`) kullanan
//              klasik bir stack-machine.
// =========================================================================
// VM
// =========================================================================
use crate::ast::*;
use crate::builtins::*;
use crate::error::OnfexError;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use crate::bytecode::*;

#[derive(Debug, Clone)]
enum ExecResult {
    Value(Type),
    Return(Type),
}

pub struct VM {
    program: Program,
    scopes: Vec<HashMap<String, Type>>,
    stack: Vec<Type>,
    builtins: HashMap<String, FUNC>,
    current_struct: Option<String>,
}

impl VM {
    pub fn new(program: Program) -> Self {
        Self {
            program,
            scopes: vec![HashMap::new()],
            stack: Vec::new(),
            builtins: create_builtins_funcs(),
            current_struct: None,
        }
    }

    pub fn run(&mut self) -> Result<(), OnfexError> {
        let chunk = self.program.main.clone();
        self.run_chunk(&chunk)?;
        Ok(())
    }

    fn pop_stack(&mut self) -> Result<Type, OnfexError> {
        self.stack
            .pop()
            .ok_or_else(|| OnfexError::runtime("Bytecode VM Ern: yığın boşaldı (stack underflow)"))
    }

    fn get_var(&self, name: &str) -> Result<Type, OnfexError> {
        for scope in self.scopes.iter().rev() {
            if let Some(v) = scope.get(name) {
                return Ok(v.clone());
            }
        }
        Err(OnfexError::runtime(format!("değişken '{}' bulunamadı", name)))
    }

    fn set_var(&mut self, name: &str, value: Type) -> Result<(), OnfexError> {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), value);
                return Ok(());
            }
        }
        Err(OnfexError::runtime(format!("değişken '{}' bulunamadı", name)))
    }

    fn define_var(&mut self, name: &str, value: Type) {
        self.scopes.last_mut().unwrap().insert(name.to_string(), value);
    }

    /// Bir chunk'ı çalıştırır. Chunk `compile_block` kuralına uyar: normal
    /// bitişte tam olarak bir değer stack'te kalır (ExecResult::Value);
    /// bir `Return` opcode'una çarpınca hemen `ExecResult::Return` ile
    /// döner (fonksiyon/metod çağrı sınırı burada kapanır).
    fn run_chunk(&mut self, chunk: &Chunk) -> Result<ExecResult, OnfexError> {
        let mut pc: usize = 0;
        loop {
            if pc >= chunk.code.len() {
                break;
            }
            let op = chunk.code[pc].clone();
            pc += 1;
            match op {
                OpCode::PushInt(x) => self.stack.push(Type::new(TypeKind::Int, Expr::Int(x))),
                OpCode::PushFloat(x) => self.stack.push(Type::new(TypeKind::Float, Expr::Float(x))),
                OpCode::PushStr(s) => self.stack.push(Type::new(TypeKind::Str, Expr::Str(s))),
                OpCode::PushBool(b) => self.stack.push(Type::new(TypeKind::Bool, Expr::Bool(b))),
                OpCode::PushVoid => self.stack.push(Type::newVoid()),
                OpCode::Pop => {
                    self.pop_stack()?;
                }
                OpCode::Dup => {
                    let v = self
                        .stack
                        .last()
                        .cloned()
                        .ok_or_else(|| OnfexError::runtime("Bytecode VM Ern: yığın boş (Dup)"))?;
                    self.stack.push(v);
                }
                OpCode::DefineVar(name) => {
                    let v = self.pop_stack()?;
                    self.define_var(&name, v.clone());
                    self.stack.push(v);
                }
                OpCode::SetVar(name) => {
                    let v = self.pop_stack()?;
                    self.set_var(&name, v.clone())?;
                    self.stack.push(v);
                }
                OpCode::GetVar(name) => {
                    let v = self.get_var(&name)?;
                    self.stack.push(v);
                }
                OpCode::GetField(field) => {
                    let recv = self.pop_stack()?;
                    match &recv.value {
                        Expr::StructDt(inst) => {
                            let v = crate::builtins::get_field(inst, &field, &self.current_struct)?;
                            self.stack.push(v);
                        }
                        _ => {
                            return Err(OnfexError::runtime(format!(
                                "Typect Ern: '{}' alt strouct oft nophe",
                                field
                            )))
                        }
                    }
                }
                OpCode::SetField(field) => {
                    let val = self.pop_stack()?;
                    let recv = self.pop_stack()?;
                    match &recv.value {
                        Expr::StructDt(inst) => {
                            crate::builtins::set_field(inst, &field, val.clone(), &self.current_struct)?;
                            self.stack.push(val);
                        }
                        _ => {
                            return Err(OnfexError::runtime(format!(
                                "Typect Ern: '{}' alt strouct oft nophe",
                                field
                            )))
                        }
                    }
                }
                OpCode::BinOp(op_str) => {
                    let r = self.pop_stack()?;
                    let l = self.pop_stack()?;
                    let v = crate::builtins::binop(l, &op_str, r)?;
                    self.stack.push(v);
                }
                OpCode::JumpIfFalse(target) => {
                    let cond = self.pop_stack()?;
                    if !crate::builtins::is_truthy(&cond) {
                        pc = target;
                    }
                }
                OpCode::Jump(target) => {
                    pc = target;
                }
                OpCode::EnterScope => {
                    self.scopes.push(HashMap::new());
                }
                OpCode::ExitScope => {
                    self.scopes.pop();
                }
                OpCode::CallFunction(name, argc) => {
                    let mut args = Vec::with_capacity(argc);
                    for _ in 0..argc {
                        args.push(self.pop_stack()?);
                    }
                    args.reverse();
                    let result = self.call_function(&name, args)?;
                    self.stack.push(result);
                }
                OpCode::CallBuiltin(name, argc) => {
                    let mut args = Vec::with_capacity(argc);
                    for _ in 0..argc {
                        args.push(self.pop_stack()?);
                    }
                    args.reverse();
                    let func = self.builtins.get(&name).cloned().ok_or_else(|| {
                        OnfexError::runtime(format!("'{}' bir fonksiyon oft nophe", name))
                    })?;
                    let result = func.run(args, HashMap::new())?;
                    self.stack.push(result);
                }
                OpCode::CallMethod(name, argc) => {
                    let mut args = Vec::with_capacity(argc);
                    for _ in 0..argc {
                        args.push(self.pop_stack()?);
                    }
                    args.reverse();
                    let recv = self.pop_stack()?;
                    let result = self.call_method(&recv, &name, args)?;
                    self.stack.push(result);
                }
                OpCode::NewStruct(name, field_names) => {
                    let mut ft = HashMap::new();
                    for fname in field_names.iter().rev() {
                        let v = self.pop_stack()?;
                        ft.insert(fname.clone(), v);
                    }
                    let st = self.program.struct_types.get(&name).cloned().ok_or_else(|| {
                        OnfexError::runtime(format!("'{}' strouct esp gephnosan", name))
                    })?;
                    let inst = crate::builtins::instantiate_struct(&st, ft)?;
                    self.stack.push(Type::new(TypeKind::StrctT, Expr::StructDt(Rc::new(inst))));
                }
                OpCode::DefineFunction(name) => {
                    let meta = self.program.function_meta.get(&name).cloned().ok_or_else(|| {
                        OnfexError::runtime(format!("'{}' fonksiyon meta oft gephnosan", name))
                    })?;
                    self.define_var(&name, Type::new(TypeKind::FuncInht, Expr::FuncInht(meta)));
                    self.stack.push(Type::newVoid());
                }
                OpCode::DefineStruct(name) => {
                    let st = self.program.struct_types.get(&name).cloned().ok_or_else(|| {
                        OnfexError::runtime(format!("'{}' strouct meta oft gephnosan", name))
                    })?;
                    self.define_var(&name, Type::new(TypeKind::StrctT, Expr::StrctInht(st)));
                    self.stack.push(Type::newVoid());
                }
                OpCode::Return => {
                    let v = self.pop_stack()?;
                    return Ok(ExecResult::Return(v));
                }
            }
        }
        let v = self.stack.pop().unwrap_or_else(Type::newVoid);
        Ok(ExecResult::Value(v))
    }

    fn call_function(&mut self, name: &str, args: Vec<Type>) -> Result<Type, OnfexError> {
        let meta = self
            .program
            .function_meta
            .get(name)
            .cloned()
            .ok_or_else(|| OnfexError::runtime(format!("'{}' fonksiyon oft nophe", name)))?;
        let chunk = self.program.functions.get(name).cloned().ok_or_else(|| {
            OnfexError::runtime(format!("'{}' afon dernos esp gephnosan (derlenmemiş)", name))
        })?;
        if meta.params.len() != args.len() {
            return Err(OnfexError::runtime(format!(
                "{} afon promter esp wraithnosan {} esp gephnosan",
                meta.params.len(),
                args.len()
            )));
        }
        let scope_depth = self.scopes.len();
        let mut frame: HashMap<String, Type> = HashMap::new();
        for (p, v) in meta.params.iter().zip(args.into_iter()) {
            if !p.kind.clone().equal(v.kind.clone()) {
                return Err(OnfexError::runtime(format!(
                    "{} esp ountf wraithnosan {} esp gephnosan",
                    p.kind.to_string(),
                    v.kind.to_string()
                )));
            }
            frame.insert(p.name.clone(), v);
        }
        self.scopes.push(frame);
        let result = self.run_chunk(&chunk);
        self.scopes.truncate(scope_depth);
        let out = match result? {
            ExecResult::Value(v) => v,
            ExecResult::Return(v) => v,
        };
        if !crate::builtins::check_return_kind(&meta.out, &out) {
            return Err(OnfexError::runtime(format!(
                "{} esp ountf wraithnosan {} esp gephnosan",
                meta.out.to_string(),
                out.kind.to_string()
            )));
        }
        Ok(out)
    }

    fn call_method(&mut self, recv: &Type, name: &str, args: Vec<Type>) -> Result<Type, OnfexError> {
        let (struct_name, methods, this): (String, HashMap<String, Frounct>, Option<Type>) = match &recv.value
        {
            Expr::StructDt(inst) => (inst.base.name.clone(), inst.base.methods.clone(), Some(recv.clone())),
            Expr::StrctInht(st) => (st.name.clone(), st.methods.clone(), None),
            _ => {
                return Err(OnfexError::runtime(format!(
                    "Typect Ern: '{}' metodnos strouct esp nophe",
                    name
                )))
            }
        };
        let meta = methods.get(name).cloned().ok_or_else(|| {
            OnfexError::runtime(format!(
                "WrossnosStrouct Ern: '{}' metodnos '{}' esp gephnosan",
                struct_name, name
            ))
        })?;
        let key = format!("{}::{}", struct_name, name);
        let chunk = self
            .program
            .methods
            .get(&key)
            .cloned()
            .ok_or_else(|| OnfexError::runtime(format!("'{}' metodnos derlenmemiş", key)))?;

        let mut pars = meta.params.clone();
        let scope_depth = self.scopes.len();
        let mut frame: HashMap<String, Type> = HashMap::new();
        if let Some(p0) = pars.first().cloned() {
            if matches!(p0.kind, TypeKind::srel) {
                let me = this.clone().ok_or_else(|| {
                    OnfexError::runtime(format!(
                        "WrossnosStrouct Ern: '{}' metodnos srel esp wraithnosan afma instans oft gephnosan",
                        p0.name
                    ))
                })?;
                frame.insert(p0.name.clone(), me);
                pars.remove(0);
            }
        }
        if pars.len() != args.len() {
            return Err(OnfexError::runtime(format!(
                "{} afon promter esp wraithnosan {} esp gephnosan",
                pars.len(),
                args.len()
            )));
        }
        for (p, v) in pars.iter().zip(args.into_iter()) {
            frame.insert(p.name.clone(), v);
        }

        self.scopes.push(frame);
        let prev_struct = self.current_struct.take();
        self.current_struct = Some(struct_name);
        let result = self.run_chunk(&chunk);
        self.current_struct = prev_struct;
        self.scopes.truncate(scope_depth);

        match result? {
            ExecResult::Value(v) => Ok(v),
            ExecResult::Return(v) => Ok(v),
        }
    }
}
