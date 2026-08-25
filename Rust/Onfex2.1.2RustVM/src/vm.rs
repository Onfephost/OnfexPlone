// - `VM`      : açık bir operand yığını (`stack: Vec<Type>`) ve isim tabanlı
//              kapsam yığını (`scopes: Vec<HashMap<String,Type>>`) kullanan
//              klasik bir stack-machine.
// =========================================================================
// VM
// =========================================================================
use crate::ast::*;
use crate::builtins::*;
use crate::builtinsdata::*;
use crate::error::OnfexError;
use crate::libs::libStrct::Library;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::cell::Cell;
use crate::bytecode::*;

#[derive(Debug, Clone)]
enum ExecResult {
    Value(Type),
    Return(Type),
}

#[derive(Clone)]
struct ModuleNS {
    program: Rc<Program>,
    namespace: HashMap<String, Type>,
    filename: String,
    code: String,
}

pub struct VM {
    program: Rc<Program>,
    scopes: Vec<HashMap<String, Type>>,
    stack: Vec<Type>,
    builtins: HashMap<String, FUNC>,
    current_struct: Option<String>,
    libs: HashMap<String, Library>,
    mods: HashMap<String, ModuleNS>,
    path: String,
    current_pos: Cell<(usize, usize)>,
}

impl VM {
    pub fn new(program: Program, path: String) -> Self {
        Self {
            program: Rc::new(program),
            scopes: vec![HashMap::new()],
            stack: Vec::new(),
            builtins: create_builtins_funcs(),
            current_struct: None,
            libs: HashMap::new(),
            mods: HashMap::new(),
            path,
            current_pos: Cell::new((0, 0)),
        }
    }

    pub fn run(&mut self) -> Result<(), OnfexError> {
        let chunk = self.program.chunks[0].clone();
        self.run_chunk(&chunk)
            .map_err(|e| e.with_location_default(self.current_pos.get().0, self.current_pos.get().1))?;
        Ok(())
    }

    fn pop_stack(&mut self) -> Result<Type, OnfexError> {
        self.stack.pop().ok_or_else(|| OnfexError::runtime("Bytecode VM Ern: yığın boşaldı (stack underflow)"))
    }

    fn get_var(&self, name: &str) -> Result<Type, OnfexError> {
        for scope in self.scopes.iter().rev() {
            if let Some(v) = scope.get(name) {
                return Ok(v.clone());
            }
        }
        Err(OnfexError::runtime(format!("Valt Ern: '{}' asp neat", name)))
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
    
    fn run_chunk(&mut self, chunk: &Chunk) -> Result<ExecResult, OnfexError> {
        let mut pc: usize = 0;
        loop {
            if pc >= chunk.code.len() {
                break;
            }
            let op = chunk.code[pc].clone();
            if let Some(&pos) = chunk.positions.get(pc) {
                self.current_pos.set(pos);
            }
            pc += 1;
            match op {
                OpCode::PushVec(sz) => {
                    if sz == 0{
                        self.stack.push(Type::new(TypeKind::Vect, Expr::Vect(sz.clone(),vec![])));
                    }else{
                        let typ = self.pop_stack()?;
                        let mut res = Vec::with_capacity(sz.clone());
                        res.push(typ.clone());
                        for _ in 1..sz.clone() {
                            let st = self.pop_stack()?;
                            if !crate::builtins::check_param_kind(&st.kind, &typ){
                                return Err(OnfexError::runtime(format!("{} esp intf wraithnosan {} esp gephnosan",typ.kind.to_string(),st.kind.to_string())));
                            }
                            res.push(st);
                        }
                        res.reverse();
                        self.stack.push(Type::new(TypeKind::Vect, Expr::Vect(sz.clone(),res)));
                    }
                }
                OpCode::PushMatris(sz) => {
                    if sz == 0{
                        self.stack.push(Type::new(TypeKind::Matris, Expr::Matris(sz.clone(),vec![])));
                    }else{
                        let typ1 = self.pop_stack()?;//key
                        let typ2 = self.pop_stack()?;
                        let mut res = Vec::with_capacity(sz.clone());
                        res.push((typ1.clone(),typ2.clone()));
                        for _ in 1..sz.clone() {
                            let st = self.pop_stack()?;
                            if !crate::builtins::check_param_kind(&st.kind, &typ1){
                            return Err(OnfexError::runtime(format!("{} esp keontpher wraithnosan {} esp gephnosan",typ1.kind.to_string(),st.kind.to_string())));
                            }
                            let vl = self.pop_stack()?;
                            if !crate::builtins::check_param_kind(&vl.kind, &typ2){
                                return Err(OnfexError::runtime(format!("{} esp valtue wraithnosan {} esp gephnosan",typ2.kind.to_string(),st.kind.to_string())));
                            }
                            res.push((st,vl));
                        }
                        res.reverse();
                        self.stack.push(Type::new(TypeKind::Matris, Expr::Matris(sz.clone(),res)));
                    }
                }
                OpCode::PushInt(x) => self.stack.push(Type::new(TypeKind::Int, Expr::Int(x))),
                OpCode::PushFloat(x) => self.stack.push(Type::new(TypeKind::Float, Expr::Float(x))),
                OpCode::PushDecimal(x) => self.stack.push(Type::new(TypeKind::Decimal, Expr::Decimal(x))),
                OpCode::PushStr(s) => self.stack.push(Type::new(TypeKind::Str, Expr::Str(s))),
                OpCode::PushBool(b) => self.stack.push(Type::new(TypeKind::Bool, Expr::Bool(b))),
                OpCode::PushVoid => self.stack.push(Type::newVoid()),
                OpCode::Pop => {
                    self.pop_stack()?;
                }
                OpCode::Dup => {
                    let v = self.stack.last().cloned()
                        .ok_or_else(|| OnfexError::runtime("Bytecode Staker Ern"))?;
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
                            return Err(OnfexError::runtime(format!("Typect Ern: '{}' alt strouct oft nophe",field)))
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
                            return Err(OnfexError::runtime(format!("Typect Ern: '{}' alt strouct oft nophe",field)))
                        }
                    }
                }
                OpCode::BinOp(op_str) => {
                    let r = self.pop_stack()?;
                    let l = self.pop_stack()?;
                    let v = crate::builtins::binop(l, &op_str, r)?;
                    self.stack.push(v);
                }
                OpCode::Not => {
                    let r = self.pop_stack()?;
                    let v = Ok(Type::new(TypeKind::Bool,Expr::Bool(!r.boolout()?)));
                    self.stack.push(v?);
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
                OpCode::MakeIter => {
                    let v = self.pop_stack()?;
                    let it = match crate::builtins::to_iter(&v) {
                        Some(it) => it,
                        None => self.struct_to_iter(&v)?,
                    };
                    self.stack.push(Type::new(TypeKind::Iter, Expr::Iter(Rc::new(it))));
                }
                OpCode::IterHasNext => {
                    let v = self.stack.last().cloned().ok_or_else(|| {
                        OnfexError::runtime("Bytecode VM Ern: yığın boş (IterHasNext)")
                    })?;
                    let has = match &v.value {
                        Expr::Iter(it) => it.has_next(),
                        _ => return Err(OnfexError::runtime("Typect Ern: Iter esp wraithnosan")),
                    };
                    self.stack.push(Type::new(TypeKind::Bool, Expr::Bool(has)));
                }
                OpCode::IterNext => {
                    let v = self.stack.last().cloned().ok_or_else(|| {
                        OnfexError::runtime("Bytecode VM Ern: yığın boş (IterNext)")
                    })?;
                    let next = match &v.value {
                        Expr::Iter(it) => it.next().ok_or_else(|| {
                            OnfexError::runtime("Forp Ern: iterasyon zaten bitmiş")
                        })?,
                        _ => return Err(OnfexError::runtime("Typect Ern: Iter esp wraithnosan")),
                    };
                    self.stack.push(next);
                }
                OpCode::BindForpVars(vars) => {
                    let v = self.pop_stack()?;
                    let bindings = crate::builtins::bind_forp_vars(&vars, v)?;
                    for (name, val) in bindings {
                        self.define_var(&name, val);
                    }
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
                    // pyrintnos/pyrintnosFowLt/phormatte: format dizesi (0.
                    // argüman) hariç her argümanı, varsa özel `__sterge__`
                    // metoduyla önceden dizgeye çevir (Python'daki __str__
                    // gibi) -- bkz. stringify_for_print.
                    if matches!(name.as_str(), "pyrintnos" | "pyrintnosFowLt" | "phormatte") {
                        let mut processed = Vec::with_capacity(args.len());
                        for (i, a) in args.into_iter().enumerate() {
                            if i == 0 {
                                processed.push(a);
                            } else {
                                processed.push(self.stringify_for_print(a)?);
                            }
                        }
                        args = processed;
                    }
                    let func = self.builtins.get(&name).cloned().ok_or_else(|| {
                        OnfexError::runtime(format!("'{}' asp neat inferins frounct", name))
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
                    let st = self.program.find(&name).and_then(|c| c.struct_meta.clone()).ok_or_else(|| {
                        OnfexError::runtime(format!("'{}' strouct esp gephnosan", name))
                    })?;
                    let inst = crate::builtins::instantiate_struct(&st, ft)?;
                    self.stack.push(Type::new(TypeKind::StrctT, Expr::StructDt(Rc::new(inst))));
                }
                OpCode::DefineFunction(name) => {
                    let meta = self.program.find(&name).and_then(|c| c.meta.clone()).ok_or_else(|| {
                        OnfexError::runtime(format!("'{}' fonksiyon meta oft gephnosan", name))
                    })?;
                    self.define_var(&name, Type::new(TypeKind::FuncInht, Expr::FuncInht(meta)));
                    self.stack.push(Type::newVoid());
                }
                OpCode::DefineStruct(name) => {
                    let st = self.program.find(&name).and_then(|c| c.struct_meta.clone()).ok_or_else(|| {
                        OnfexError::runtime(format!("'{}' strouct meta oft gephnosan", name))
                    })?;
                    self.define_var(&name, Type::new(TypeKind::StrctT, Expr::StrctInht(st)));
                    self.stack.push(Type::newVoid());
                }
                OpCode::ImportLib(name) => {
                    let lib = crate::libs::libacs::loadLib(&name)?;
                    let key = name.rsplit("::").next().unwrap_or(name.as_str()).to_string();
                    self.libs.insert(key, lib);
                    self.stack.push(Type::newVoid());
                }
                OpCode::ImportMod(raw_path) => {
                    let modname = std::path::Path::new(&raw_path)
                        .file_stem()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_else(|| raw_path.clone());
                    let ns = self.load_module(&raw_path)?;
                    self.mods.insert(modname, ns);
                    self.stack.push(Type::newVoid());
                }
                OpCode::AliasLib(new_name, old_name) => {
                    let v = self.libs.remove(&old_name).ok_or_else(|| {
                        OnfexError::runtime(format!("WrossnosLrib Ern: '{}' lrib asp aif", old_name))
                    })?;
                    self.libs.insert(new_name, v);
                    self.stack.push(Type::newVoid());
                }
                OpCode::AliasMod(new_name, old_name) => {
                    let v = self.mods.remove(&old_name).ok_or_else(|| {
                        OnfexError::runtime(format!("WrossnosMot Ern: '{}' esp gephnosan mot", old_name))
                    })?;
                    self.mods.insert(new_name, v);
                    self.stack.push(Type::newVoid());
                }
                OpCode::GetLibVar(lib, name) => {
                    let l = self.libs.get(&lib).ok_or_else(|| {
                        OnfexError::runtime(format!("WrossnosLrib Ern: '{}' esp neat gephnosan lrib", lib))
                    })?;
                    let v = l.vars.borrow().get(&name).cloned().ok_or_else(|| {
                        OnfexError::runtime(format!("'{}::{}' esp gephnosan degisken", lib, name))
                    })?;
                    self.stack.push(v);
                }
                OpCode::GetModVar(m, name) => {
                    let ns = self.mods.get(&m).ok_or_else(|| {
                        OnfexError::runtime(format!("Mot Ern: '{}' esp gephnosan mot", m))
                    })?;
                    let v = ns.namespace.get(&name).cloned().ok_or_else(|| {
                        OnfexError::runtime(format!("'{}!->{}' esp gephnosan degisken", m, name))
                    })?;
                    self.stack.push(v);
                }
                OpCode::CallLibFunc(lib, fname, argc) => {
                    let mut args = Vec::with_capacity(argc);
                    for _ in 0..argc {
                        args.push(self.pop_stack()?);
                    }
                    args.reverse();
                    let l = self.libs.get(&lib).ok_or_else(|| {
                        OnfexError::runtime(format!("WrossnosLrib Ern: '{}' esp neat gephnosan lrib", lib))
                    })?;
                    let func = l.funcs.get(&fname).cloned().ok_or_else(|| {
                        OnfexError::runtime(format!("'{}::{}' frounct asp nophe", lib, fname))
                    })?;
                    let result = func(args, HashMap::new())?;
                    self.stack.push(result);
                }
                OpCode::CallModFunc(m, fname, argc) => {
                    let mut args = Vec::with_capacity(argc);
                    for _ in 0..argc {
                        args.push(self.pop_stack()?);
                    }
                    args.reverse();
                    let result = self.call_module_function(&m, &fname, args)?;
                    self.stack.push(result);
                }
                OpCode::Return => {
                    let v = self.pop_stack()?;
                    return Ok(ExecResult::Return(v));
                }
                OpCode::Panic => {
                    let x = self.pop_stack()?;
                    return Err(OnfexError::runtime(format!("{}", x.__out__(false))))
                }
                OpCode::PushType(x) => {
                    self.stack.push(Type::new(TypeKind::TypeKind, Expr::TypeKind(Box::new(x))));
                }
            }
        }
        
        let v = self.stack.pop().unwrap_or_else(Type::newVoid);
        Ok(ExecResult::Value(v))
    }

    fn call_function(&mut self, name: &str, args: Vec<Type>) -> Result<Type, OnfexError> {
        let found = self.program.find(name).cloned().ok_or_else(|| {
            OnfexError::runtime(format!("'{}' afon dernos esp gephnosan (derlenmemiş)", name))
        })?;
        let meta = found
            .meta
            .clone()
            .ok_or_else(|| OnfexError::runtime(format!("'{}' fonksiyon oft nophe", name)))?;
        let chunk = found;
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
            // Jenerik parametre tipleri (TypeKind::Dynamic) tip silme ile
            // çalışır; bkz. builtins::check_param_kind.
            if !crate::builtins::check_param_kind(&p.kind, &v) {
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

    /// `builtins::to_iter` doğrudan çeviremediği bir değeri (ör. bir
    /// Vektöre strouct örneği) `forp` için bir Iter'a çevirmeye çalışır:
    /// değer bir strouct örneğiyse ve `gephnosVeot` adında bir metodu
    /// varsa onu çağırıp SONUCU tekrar `builtins::to_iter`'a verir.
    fn struct_to_iter(&mut self, v: &Type) -> Result<crate::builtins::Iter, OnfexError> {
        let has_method = match &v.value {
            Expr::StructDt(inst) => inst.base.methods.contains_key("iterfal"),
            _ => false,
        };
        if has_method {
            let arr = self.call_method(v, "iterfal", vec![])?;
            if let Some(it) = crate::builtins::to_iter(&arr) {
                return Ok(it);
            }
        }
        Err(OnfexError::runtime(
            "Typect Ern: forp 'intf' esp bir Iter/vektöre/mappe (ya da 'gephnosVeot' metodnos strouct) wraithnosan",
        ))
    }

    /// Bir değeri, çıktı/format fonksiyonlarına (pyrintnos, pyrintnosFowLt,
    /// phormatte) geçirmeden ÖNCE "ön işler": eğer bu bir strouct örneğiyse
    /// ve tipi `__sterge__` adında bir metod tanımlıyorsa (Python'daki
    /// `__str__` gibi), o metod çağrılır ve DÖNEN sterg değeri kullanılır.
    /// Aksi halde değer OLDUĞU GİBİ döner -- varsayılan `Type::__out__`
    /// biçimlendirmesi (`Test { x: 5 }`) devreye girer.
    fn stringify_for_print(&mut self, v: Type) -> Result<Type, OnfexError> {
        let has_sterge = match &v.value {
            Expr::StructDt(inst) => inst.base.methods.contains_key("__sterge__"),
            _ => false,
        };
        if has_sterge {
            self.call_method(&v, "__sterge__", vec![])
        } else {
            Ok(v)
        }
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
        // Metodun chunk'ı önce kendi programımızda aranır. Struct bir "mot"
        // ile içeri alınmış bir modülden geliyorsa (örn. `std->Vektöre`),
        // metodun derlenmiş kodu KENDİ programımızda DEĞİL, modülün kendi
        // derlenmiş programındadır -- bu yüzden bulunamazsa içeri alınmış
        // tüm modüllerin programlarına da bakılır.
        let (chunk, method_program, method_src): (Chunk, Rc<Program>, Option<(String, String)>) =
            match self.program.find(&key) {
                Some(c) => (c.clone(), self.program.clone(), None),
                None => self
                    .mods
                    .values()
                    .find_map(|m| {
                        m.program
                            .find(&key)
                            .map(|c| (c.clone(), m.program.clone(), Some((m.filename.clone(), m.code.clone()))))
                    })
                    .ok_or_else(|| OnfexError::runtime(format!("'{}' metodnos derlenmemiş", key)))?,
            };

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
            if !crate::builtins::check_param_kind(&p.kind, &v) {
                return Err(OnfexError::runtime(format!(
                    "{} esp ountf wraithnosan {} esp gephnosan",
                    p.kind.to_string(),
                    v.kind.to_string()
                )));
            }
            frame.insert(p.name.clone(), v);
        }

        self.scopes.push(frame);
        let prev_struct = self.current_struct.take();
        self.current_struct = Some(struct_name);
        // Metod başka bir modülden geldiyse, çalışırken KENDİ programına
        // geçici olarak geçilir (kendi struct'larına/diğer fonksiyonlarına
        // başvurabilmesi için) -- `call_module_function`'daki Rc değişimiyle
        // birebir aynı mantık; `Rc` sayesinde ucuz bir referans değişimi.
        // Kaynak bağlamı da (varsa `method_src`) aynı şekilde geçici olarak
        // o modüle çevrilir, böylece bir hata doğru dosyayı gösterir.
        let prev_program = std::mem::replace(&mut self.program, method_program);
        let _src = method_src
            .as_ref()
            .map(|(filename, code)| crate::error::set_source_context(filename.clone(), code));
        let result = self.run_chunk(&chunk);
        drop(_src);
        self.program = prev_program;
        self.current_struct = prev_struct;
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
        Ok(out )
    }

    /// "mot" hedefi olan bir `.onfex` dosyasını yükler: yolu çözer
    /// (ağaç-yürüten motordaki `Stmt::Mod` ile aynı ".." kuralı), dosyayı
    /// `is_import` modunda derler (mehen{} gövdesi atlanır -- bkz.
    /// `Compiler::new_import`), İZOLE bir alt-VM'de bir kez çalıştırıp
    /// üst seviye bağlamalarını (frounct/strouct/valt) yakalar.
    fn load_module(&self, raw_path: &str) -> Result<ModuleNS, OnfexError> {
        let resolved = raw_path.replace("..", &self.path);
        let p = std::path::Path::new(&resolved);
        let docname = p
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .ok_or_else(|| OnfexError::runtime(format!("Mot Ern: geçersiz yol '{}'", raw_path)))?;
        let docpath = p
            .parent()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();

        let doc = crate::document::OnfexPloneDoph::new(docpath, docname, true);
        let program = doc.compile_project()?;

        let mut sub_vm = VM::new(program.clone(), doc.path.clone());
        // `compile_project()` kendi kaynak bağlamını sadece lex/parse/derleme
        // süresince açık tutar (fonksiyonu bitince kapanır); burada modülü
        // ÇALIŞTIRIRKEN oluşacak hataların da doğru dosyayı göstermesi için
        // ayrıca bir bağlam açılır.
        let _src = crate::error::set_source_context(format!("{}/{}", doc.path, doc.name), &doc.code);
        sub_vm.run()?;

        Ok(ModuleNS {
            program: Rc::new(program),
            namespace: sub_vm.scopes.into_iter().next().unwrap_or_default(),
            filename: format!("{}/{}", doc.path, doc.name),
            code: doc.code.clone(),
        })
    }

    /// `mod!->fonk(...)` çağrısı: modülün KENDİ derlenmiş programı
    /// bağlamında (kendi struct'larına/diğer fonksiyonlarına başvurabilmesi
    /// için) çalıştırılır. `self.program` bir `Rc` olduğundan bu geçici
    /// değişim ucuz bir referans değişimidir, derin kopya değildir.
    fn call_module_function(&mut self, modname: &str, fname: &str, args: Vec<Type>) -> Result<Type, OnfexError> {
        let module = self.mods.get(modname).cloned().ok_or_else(|| {
            OnfexError::runtime(format!("Mot Ern: '{}' esp gephnosan mot", modname))
        })?;
        let func_val = module.namespace.get(fname).cloned().ok_or_else(|| {
            OnfexError::runtime(format!("'{}->{}' esp gephnosan afon", modname, fname))
        })?;
        let meta = match &func_val.value {
            Expr::FuncInht(f) => f.clone(),
            _ => {
                return Err(OnfexError::runtime(format!(
                    "Typect Ern: '{}->{}' bir fonksiyon oft nophe",
                    modname, fname
                )))
            }
        };
        let chunk = module.program.find(fname).cloned().ok_or_else(|| {
            OnfexError::runtime(format!(
                "'{}->{}' afon dernos esp gephnosan (derlenmemiş)",
                modname, fname
            ))
        })?;
        if meta.params.len() != args.len() {
            return Err(OnfexError::runtime(format!(
                "{} afon promter esp wraithnosan {} esp gephnosan",
                meta.params.len(),
                args.len()
            )));
        }
        let mut frame: HashMap<String, Type> = HashMap::new();
        for (p, v) in meta.params.iter().zip(args.into_iter()) {
            if !crate::builtins::check_param_kind(&p.kind, &v) {
                return Err(OnfexError::runtime(format!(
                    "{} esp ountf wraithnosan {} esp gephnosan",
                    p.kind.to_string(),
                    v.kind.to_string()
                )));
            }
            frame.insert(p.name.clone(), v);
        }

        let prev_program = std::mem::replace(&mut self.program, module.program.clone());
        let prev_struct = self.current_struct.take();
        let scope_depth = self.scopes.len();
        self.scopes.push(frame);
        // Bu çağrı sırasında bir hata olursa doğru dosyayı gösterebilmesi
        // için modülün kaynak bağlamı geçici olarak açılır.
        let _src = crate::error::set_source_context(module.filename.clone(), &module.code);
        let result = self.run_chunk(&chunk);
        drop(_src);
        self.scopes.truncate(scope_depth);
        self.current_struct = prev_struct;
        self.program = prev_program;

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
}
