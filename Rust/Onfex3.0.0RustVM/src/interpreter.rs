// interpreter.rs

use crate::ast::*;
use crate::builtins::*;
use crate::builtinsdata::*;
use crate::environment::Environment;
use crate::error::OnfexError;
use crate::libs::libacs::*;
use crate::libs::libStrct::Library;
use crate::runtime::Flow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::document::OnfexPloneDoph;
type OPD = OnfexPloneDoph;

fn check_string(s: &str) -> bool {
    let deallowed = "abcdefghijklmnoöprstuüvyzwxqABCDEFGHIJKLMNOÖPRSTUÜVYZWXQ_1234567890";

    s.chars().all(|c| deallowed.contains(c))
}

pub struct Interpreter {
    pub builtins: HashMap<String, FUNC>,
    pub array_types: HashMap<String, ArrayType>,
    pub buffer_types: HashMap<String, BufferType>,
    pub mono_types: HashMap<String, MonoType>,
    pub env: RefCell<Environment>,
    pub libs: RefCell<HashMap<String, Library>>,
    pub ptr: RefCell<usize>,
    pub mods: RefCell<HashMap<String,RefCell<Environment>>>,
    pub selfAccess:RefCell<bool>,
    pub Import:RefCell<bool>,
    pub path: String,
    // Şu an içinde çalışılan strouct'un ismi (bir impelnos metodu
    // içindeysek Some(struct_adi)); prube/prive alan görünürlüğü için
    // kullanılır.
    pub current_struct: RefCell<Option<String>>,
    pub code : String,
}

impl Interpreter {
    // new
    pub fn new(path:String,code:String) -> Self {
        Self {
            builtins: create_builtins_funcs(),
            array_types: create_array_types(),
            buffer_types: create_buffer_types(),
            mono_types: HashMap::new(),
            env: RefCell::new(Environment::new()),
            libs: RefCell::new(HashMap::new()),
            ptr: RefCell::new(1),
            mods:RefCell::new(HashMap::new()),
            selfAccess :RefCell::new(false),
            Import :RefCell::new(false),
            path,
            current_struct: RefCell::new(None),
            code,
        }
    }

    // run
    pub fn run(&self, program: Vec<StmtNode>) -> Result<(), OnfexError> {
        for stmt in &program {
            match self.exec(stmt)? {
                Flow::Normal(_) => {}
                Flow::Empty => {}
                // a `erutnos` at the top level simply stops the program
                Flow::Return(_) => return Ok(()),
            }
        }
        Ok(())
    }

    // exec
    // "Rust gibi" detaylı hata gösterimi: gerçek çalışma mantığı `exec_inner`'a
    // taşındı; bu sarmalayıcı sadece, hata henüz bir kaynak konumu
    // TAŞIMIYORSA (ör. `OnfexError::runtime(...)` ile üretildiyse) bu
    // statement'ın konumunu ekler. İç içe `exec` çağrıları (ör. mehen/ifnt
    // gövdeleri) EN İÇTEKİ statement'ın konumunu önce ekler; bu sarmalayıcı
    // sadece hâlâ eksikse doldurur -- yani en isabetli konum her zaman kazanır.
    fn exec(&self, stmt: &StmtNode) -> Result<Flow, OnfexError> {
        self.exec_inner(stmt)
            .map_err(|e| e.with_location_default(stmt.line, stmt.col))
    }

    fn exec_inner(&self, stmt: &StmtNode) -> Result<Flow, OnfexError> {
        match &stmt.stmt {
            // expr
            Stmt::ExprNode(expr) => {
                let v = self.eval(&expr.expr)?;
                Ok(Flow::Normal(v))
            }
            // assign
            Stmt::Assign(name, expr) => {
                let adr = *self.ptr.borrow();
                let mut value = self.eval(&expr.expr)?;
                value.ptr = Some(adr);
                self.env.borrow_mut().names.insert(name.clone(), adr);
                self.env.borrow_mut().heap.insert(adr, value.clone());
                self.inc_ptr();
                Ok(Flow::Normal(value))
            }
            // reassign
            Stmt::ReAssign(name, expr) => {
                let y = self.get_var(name).unwrap();
                let adr = self.lookup_address(&self.env.borrow(), name)?;
                let mut value = self.eval(&expr.expr)?;
                if !(value.clone().kind == y.kind){
                    return Err(OnfexError::runtime("Sernen Ern: typect keonmaqerins"))
                }
                value.ptr = Some(adr);
                self.env.borrow_mut().heap.insert(adr, value);
                Ok(Flow::Normal(Type::newVoid()))
            }
            // import
            Stmt::Import(name) => {
                let lib = loadLib(name)?;
                let nm:Vec<String> = name.clone().split("::").map(|x| x.to_string()).collect();
                self.libs.borrow_mut().insert(nm.clone()[nm.clone().len()-1].clone(), lib);
                Ok(Flow::Normal(Type::newVoid()))
            }
            Stmt::Mod(nam) => {
                if !*self.selfAccess.borrow(){
                    let name = nam.clone().replace("..",self.path.clone().as_str());
                    let  mut v:Vec<String> = name.split("/").map(|x| x.to_string()).collect();
                    let docname = v.clone()[v.clone().len()-1].clone();
                    v.remove(v.clone().len()-1);
                    let mut docpath = String::new();
                    for i in v.clone(){
                        docpath.push_str(&("/".to_owned()+i.as_str()));
                    }
                    let doc = OPD::new(docpath,docname.clone(),true);
                    let res = doc.run();
                    let modname = docname.clone().replace(".onfex","");
                    self.mods.borrow_mut().insert(modname,res?.unwrap());
                    Ok(Flow::Normal(Type::newVoid()))
                }else{
                    Err(OnfexError::runtime("Mot Ern: Mot Freat keonaertenosfer"))
                }
            }
            // return
            Stmt::Return(expr) => {
                let value = match expr {
                    Some(e) => self.eval(&e.expr)?,
                    None => Type::newVoid(),
                };
                Ok(Flow::Return(value))
            }
            Stmt::Raise(expr) => {
                Err(OnfexError::runtime(self.eval(&expr.expr)?.__out__(false)))
            }
            Stmt::TypeLib(new,old) => {
                let mut map = self.libs.borrow_mut();
                if let Some(value) = map.remove(old.into()) {
                    map.insert(new.clone(), value);
                    return Ok(Flow::Normal(Type::newVoid()));
                }else{
                    return Err(OnfexError::runtime(format!(
                        "WrossnosLrib Ern: keoninferins lrib {}",
                        old
                    )));
                }
            }
            Stmt::TypeMod(new,old) => {
                if !check_string(&new.clone()){
                    return Err(OnfexError::runtime(format!(
                        "WrossnosMot Ern: meess asp"
                        
                    )))
                }
                let mut map = self.mods.borrow_mut();
                if let Some(value) = map.remove(old.into()) {
                    map.insert(new.clone(), value);
                    return Ok(Flow::Normal(Type::newVoid()));
                }else{
                    return Err(OnfexError::runtime(format!(
                        "WrossnosMot Ern: keoninferins mot {}",
                        old
                    )));
                }
            }
            Stmt::IfElse(cnd,body,eb) => {
                self.enter_scope();
                let cond = self.exec(&*cnd.clone())?.unwrap();
                
                let willrun = crate::builtins::is_truthy(&cond);
                let mut out = Type::newVoid();
                if willrun{
                    for s in body {
                        match self.exec(s) {
                            Ok(Flow::Normal(v)) => {out = v;}
                            Ok(Flow::Empty) => {}
                            Ok(Flow::Return(v)) => {
                                self.exit_scope();
                                return Ok(Flow::Return(v));
                            }
                            Err(e) => {
                                self.exit_scope();
                                return Err(e);
                            }
                        }
                    }
                
                }else{
                    if let Some(ebd) = eb{
                        for s in ebd {
                            match self.exec(s) {
                                Ok(Flow::Normal(v)) => {out = v;}
                                Ok(Flow::Empty) => {}
                                Ok(Flow::Return(v)) => {
                                self.exit_scope();
                                    return Ok(Flow::Return(v));
                                }
                                Err(e) => {
                                    self.exit_scope();
                                    return Err(e);
                                }
                            }
                        }
                    }
                    
                }
                self.exit_scope();
                Ok(Flow::Normal(out))
            }
            // forp x, y, z intf <ifade> { <gövde> }
            Stmt::Forp(vars, iter_expr, body) => {
                let val = self.eval(&iter_expr.expr)?;
                let iter = match crate::builtins::to_iter(&val) {
                    Some(it) => it,
                    None => self.struct_to_iter(&val)?,
                };
                self.enter_scope();
                let mut out = Type::newVoid();
                loop {
                    let next = match iter.next() {
                        Some(v) => v,
                        None => break,
                    };
                    let bindings = match crate::builtins::bind_forp_vars(vars, next) {
                        Ok(b) => b,
                        Err(e) => {
                            self.exit_scope();
                            return Err(e);
                        }
                    };
                    for (name, mut v) in bindings {
                        let adr = *self.ptr.borrow();
                        v.ptr = Some(adr);
                        self.env.borrow_mut().names.insert(name, adr);
                        self.env.borrow_mut().heap.insert(adr, v);
                        self.inc_ptr();
                    }
                    for s in body {
                        match self.exec(s) {
                            Ok(Flow::Normal(v)) => { out = v; }
                            Ok(Flow::Empty) => {}
                            Ok(Flow::Return(v)) => {
                                self.exit_scope();
                                return Ok(Flow::Return(v));
                            }
                            Err(e) => {
                                self.exit_scope();
                                return Err(e);
                            }
                        }
                    }
                }
                self.exit_scope();
                Ok(Flow::Normal(out))
            }
            // mehen
            Stmt::Mehen(body) => {
                if *self.Import.borrow(){
                    return Ok(Flow::Normal(Type::newVoid()))
                }
                self.enter_scope();
                let mut out = Type::newVoid();
                for s in body {
                    match self.exec(s) {
                        Ok(Flow::Normal(v)) => {
                            out = v;
                        }
                        Ok(Flow::Empty) => {}
                        Ok(Flow::Return(v)) => {
                            self.exit_scope();
                            return Ok(Flow::Return(v));
                        }
                        Err(e) => {
                            self.exit_scope();
                            return Err(e);
                        }
                    }
                }
                self.exit_scope();
                Ok(Flow::Normal(out))
            }
            // function create
            Stmt::FuncCre(name, generics, params, body, o) => {
                let func = Frounct::new(name.clone(),generics.clone(),params.clone(), body.clone(),o.clone());
                let adr = *self.ptr.borrow();
                self.env.borrow_mut().names.insert(name.clone(), adr);
                self.env
                    .borrow_mut()
                    .heap
                    .insert(adr, Type::new(TypeKind::FuncInht, Expr::FuncInht(func)));
                self.inc_ptr();
                Ok(Flow::Normal(Type::newVoid()))
            }
            Stmt::StrctCre(name, generics, fields, funcs) => {
                let mut methods = HashMap::new();
                for (s,f) in funcs.clone(){
                    match f.clone(){
                        Stmt::FuncCre(n,g,p,b,o) => {
                            methods.insert(n.clone(),Frounct::new(n.clone(),g.clone(),p.clone(), b.clone(),o.clone()));
                        }
                        _ => {} 
                    }
                }
                let strct = StructType::new(name.clone(),generics.clone(),fields.clone(), methods.clone());
                let adr = *self.ptr.borrow();
                self.env.borrow_mut().names.insert(name.clone(), adr);
                self.env.borrow_mut().heap.insert(adr, Type::new(TypeKind::StrctT, Expr::StrctInht(strct)));
                self.inc_ptr();
                Ok(Flow::Normal(Type::newVoid()))
            }
            // strouct alan ataması: srel.alan = deger; / obj.alan = deger;
            Stmt::MemberAssign(base, field, val) => {
                let recv = self.eval(&base.expr)?;
                match &recv.value {
                    Expr::StructDt(inst) => {
                        let v = self.eval(&val.expr)?;
                        let cur = self.current_struct.borrow().clone();
                        crate::builtins::set_field(inst, field, v.clone(), &cur)?;
                        Ok(Flow::Normal(v))
                    }
                    _ => Err(OnfexError::runtime(format!(
                        "Typect Ern: '{}' alt strouct oft nophe", field
                    ))),
                }
            }
            _ => {Ok(Flow::Empty)}
        }
    }

    // eval
    fn eval(&self, expr: &Expr) -> Result<Type, OnfexError> {
        match expr {
            Expr::Int(x) => Ok(Type::new(TypeKind::Int, Expr::Int(*x))),
            Expr::Float(x) => Ok(Type::new(TypeKind::Float, Expr::Float(*x))),
            Expr::Decimal(x) => Ok(Type::new(TypeKind::Decimal, Expr::Decimal(*x))),
            Expr::Str(x) => Ok(Type::new(TypeKind::Str, Expr::Str(x.clone()))),
            Expr::Bool(x) => Ok(Type::new(TypeKind::Bool, Expr::Bool(*x))),
            Expr::Void => Ok(Type::newVoid()),
            Expr::Not(x) => {
                let res = self.eval(&*x)?;
                let mut rs = false;
                match res.value {
                    Expr::Bool(adr) => {
                        rs = !adr;
                    }
                    _ => return Err(OnfexError::runtime("Neat Ern")),
                }
                Ok(Type::new(TypeKind::Bool,Expr::Bool(rs)))
            }
            Expr::Variable(name) => self.get_var(name),
            Expr::Vect(u,v) => Ok(Type::new(TypeKind::Vect,Expr::Vect(*u,v.to_vec()))),
            Expr::Matris(u,v) => Ok(Type::new(TypeKind::Matris,Expr::Matris(*u,v.to_vec()))),
            Expr::Deref(inner) => {
                let r = self.eval(inner)?;
                match r.value {
                    Expr::Ref(adr) => {
                        let env = self.env.borrow();
                        self.load_from_heap(&env, adr)
                    }
                    _ => Err(OnfexError::runtime("cannot dereference a non-reference value")),
                }
            }
            // function inherit
            Expr::Spread(f) => Ok(Type::new(TypeKind::Sprd, Expr::Sprd(Box::new(self.eval(&*f)?)))),
            
            Expr::FuncInht(f) => Ok(Type::new(TypeKind::FuncInht, Expr::FuncInht(f.clone()))),
            // array runtime
            Expr::ArrayDt(arr) => Ok(Type::new(TypeKind::ArrayT, Expr::ArrayDt(arr.clone()))),
            // buffer runtime
            Expr::BufferDt(buf) => Ok(Type::new(TypeKind::BufferT, Expr::BufferDt(buf.clone()))),
            Expr::MonoDt(buf) => Ok(Type::new(TypeKind::MonoT, Expr::MonoDt(buf.clone()))),
            // list
            Expr::List(type_name, items) => {
                let mut vals = Vec::with_capacity(items.len());
                for item in items {
                    vals.push(self.eval(&item.expr)?);
                }
                let tp = self.array_types.get(type_name).cloned().ok_or_else(|| {
                    OnfexError::runtime(format!("undefined array type '{}'", type_name))
                })?;
                Ok(Type::new(TypeKind::ArrayT, Expr::ArrayDt(Box::new(Array::new(vals, tp)))))
            }
            // dict
            Expr::Dict(type_name, items) => {
                let mut vals = Vec::with_capacity(items.len());
                for (k, v) in items {
                    vals.push((self.eval(&k.expr)?, self.eval(&v.expr)?));
                }
                let tp = self.buffer_types.get(type_name).cloned().ok_or_else(|| {
                    OnfexError::runtime(format!("undefined buffer type '{}'", type_name))
                })?;
                Ok(Type::new(TypeKind::BufferT, Expr::BufferDt(Box::new(Buffer::new(vals, tp)))))
            }
            Expr::Macro(s) => {
                Ok(Type::new(TypeKind::Void,Expr::Macro(s.clone())))
            }
            // call
            Expr::Call(name, args,feat) => {
                let mut ft = HashMap::new();
                for (s,e) in feat{
                    ft.insert(s.to_string(),self.eval(&e)?);
                }
                let mut values = Vec::with_capacity(args.len());
                for a in args {
                    let res = self.eval(a)?;
                    match res.clone().value{
                        Expr::Sprd(x) => {
                            let rs2 = self.eval(&x.value)?;
                            match rs2.clone().value{
                                Expr::Vect(_,v)=>{
                                    values.extend(v);
                                }
                                _ => {}
                            }
                        }
                        _ => {values.push(res.clone())}
                    }
                }
                let nam:String = match *name.clone(){
                    Expr::Variable(x) => {x.clone()}
                    _ => "undefined".to_string(),
                };
                match *name.clone(){
                    Expr::Variable(x) => {
                    },
                    Expr::Macro(s) => {
                        if let Some(func) = self.builtins.get(s.as_str()) {
                            // pyrintnos/pyrintnosFowLt/phormatte: format dizesi
                            // (0. argüman) hariç her argümanı, varsa özel
                            // `__sterge__` metoduyla önceden dizgeye çevir
                            // (Python'daki __str__ gibi) -- bkz. stringify_for_print.
                            let processed = if matches!(s.as_str(), "pyrintnos" | "pyrintnosFowLt" | "phormatte") {
                                let mut out = Vec::with_capacity(values.len());
                                for (i, v) in values.into_iter().enumerate() {
                                    if i == 0 {
                                        out.push(v);
                                    } else {
                                        out.push(self.stringify_for_print(v)?);
                                    }
                                }
                                out
                            } else {
                                values
                            };
                            return func.run(processed,ft);
                        }else{
                            return Err(OnfexError::runtime(format!("'{}' is not a function", s)))
                        }
                    }
                    Expr::LibVariable(lib,name) =>{
                        let libs = self.libs.borrow();
                        let l = libs.get(&lib.clone()).ok_or_else(|| OnfexError::runtime(format!("undefined library '{}'", lib.clone())))?;
                        let func = l.funcs.get(&name.clone()).ok_or_else(|| OnfexError::runtime(format!("undefined function '{}->{}'", lib.clone(), name.clone())))?;
                        return func(values,ft);
                    }
                    Expr::ModVariable(m,name) => {
                        let mods = self.mods.borrow();
                        let md = mods.get(&m.clone()).ok_or_else(|| OnfexError::runtime(format!("undefined library'{}'", m.clone())))?;
                            if let funcheap = md.borrow().names.get(&name.clone()).clone(){
                                if let Expr::FuncInht(funcr) = &md.borrow().heap.get(&funcheap.unwrap().clone()).clone().unwrap().value {
                                    return self.run_func(funcr.clone(), values);
                        
                                }
                            }else{
                                return Err(OnfexError::runtime(format!("undefined function '{}->{}'", m.clone(), name.clone())))
                                    
                            }
                    }
                    _ =>{return Err(OnfexError::runtime("keonrofins experfal"))}
                }
                // user func / strouct örneklemesi
                let f = self.eval(&*name)?;
                match f.value {
                    Expr::FuncInht(fc) => {
                        return self.run_func(fc, values);
                    }
                    Expr::StrctInht(st) => {
                        if !values.is_empty() {
                            return Err(OnfexError::runtime(format!(
                                "WrossnosStrouct Ern: '{}' strouct örneği alt sadece alan(field) esp gephnosan, promter oft nophe",
                                st.name
                            )));
                        }
                        return self.instantiate_struct(st, ft);
                    }
                    _ => {}
                }
                Err(OnfexError::runtime(format!("'{}' is not a function", nam)))
            }
            Expr::AddressOf(name) => {
                let adr = self.lookup_address(&self.env.borrow(), name)?;
                Ok(Type::new(TypeKind::Ref, Expr::Ref(adr)))
            }
            Expr::Ref(s) => Ok(Type::new(TypeKind::Ref, Expr::Ref(*s))),
            Expr::BinaryOp(left,op,right) => {
                let l = self.eval(&*left)?;
                let r = self.eval(&*right)?;
                self.binop(l, op, r)
            }
            // lib call
            Expr::LibVariable(lib, name) => {
                let libs = self.libs.borrow();
                let l = libs
                    .get(lib)
                    .ok_or_else(|| OnfexError::runtime(format!("undefined library '{}'", lib)))?;
                if let Some(var) = l.vars.borrow().get(name){
                    Ok(var.clone())
                }else{
                    Err(OnfexError::runtime(format!("undefined var '{}->{}'", lib, name)))
                }  
                
            }
            Expr::ModVariable(lib, name) => {
                let mods = self.mods.borrow();

                if let Some(module) = mods.get(lib) {
                    let module = module.borrow();

                    if let Some(addr) = module.names.get(name) {
                        if let Some(value) = module.heap.get(addr) {
                            Ok(value.clone())
                        } else {
                            Err(OnfexError::runtime(format!("undefined heap address '{}!->{}'", lib, name)))
                        }
                    } else {
                        Err(OnfexError::runtime(format!("undefined module variable '{}!->{}'", lib, name)))
                    }
                } else {Err(OnfexError::runtime(format!("undefined module '{}'", lib)))
                }
            }
            Expr::Lib(l) => {
                let res = Expr::Lib(l.clone());
                Ok(Type::new(TypeKind::Lib,res))
            }
            // strouct örneği/tipi olduğu gibi geçer
            Expr::StructDt(inst) => Ok(Type::new(TypeKind::StrctT, Expr::StructDt(inst.clone()))),
            Expr::StrctInht(st) => Ok(Type::new(TypeKind::StrctT, Expr::StrctInht(st.clone()))),
            Expr::TypeKind(st) => Ok(Type::new(TypeKind::TypeKind, Expr::TypeKind(st.clone()))),
            // alan erişimi: srel.alan / obj.alan
            Expr::Member(base, field) => {
                let recv = self.eval(base)?;
                match &recv.value {
                    Expr::StructDt(inst) => {
                        let cur = self.current_struct.borrow().clone();
                        crate::builtins::get_field(inst, field, &cur)
                    }
                    Expr::ArrayDt(x) => {
                        let idx = field.clone()[1..field.clone().len()].parse::<i64>().unwrap();
                        let list = x.items.clone(); 
                        if let y = &list[idx as usize]{
                            return Ok(y.clone())
                        }else{
                            return Err(OnfexError::runtime(format!(
                                "Typect Ern: '{}' iyndexe asp aif", field
                            )))
                        }
                    }
                    _ => Err(OnfexError::runtime(format!(
                        "Typect Ern: '{}' ethriben asp aif", field
                    ))),
                }
            }
            // metod çağrısı: instans.metodnos(...) / Strouct.metodnos(...) (assoc./srel yok)
            Expr::MethodCall(recv, name, args) => {
                let recv_val = self.eval(recv)?;
                let mut values = Vec::with_capacity(args.len());
                for a in args {
                    values.push(self.eval(a)?);
                }
                match &recv_val.value {
                    Expr::StructDt(inst) => {
                        let method = inst.base.methods.get(name).cloned().ok_or_else(|| {
                            OnfexError::runtime(format!(
                                "WrossnosStrouct Ern: '{}' methodfal asp neat methodfal oft strouct '{}'",
                                inst.base.name, name
                            ))
                        })?;
                        self.run_method(&inst.base.name.clone(), method, Some(recv_val.clone()), values)
                    }
                    Expr::StrctInht(st) => {
                        let method = st.methods.get(name).cloned().ok_or_else(|| {
                            OnfexError::runtime(format!(
                                "WrossnosStrouct Ern: '{}' methodfal asp neat methodfal oft strouct '{}'",
                                st.name, name
                            ))
                        })?;
                        self.run_method(&st.name.clone(), method, None, values)
                    }
                    _ => Err(OnfexError::runtime(format!(
                        "Typect Ern: '{}' methodnos esp nophe", name
                    ))),
                }
            }
            _ => {Ok(Type::newVoid())}
        }
    }

    fn inc_ptr(&self) {
        *self.ptr.borrow_mut() += 1;
    }

    fn load_from_heap(&self, env: &Environment, adr: usize) -> Result<Type, OnfexError> {
        if let Some(v) = env.heap.get(&adr) {
            return Ok(v.clone());
        }
        if let Some(parent) = &env.parent {
            return self.load_from_heap(parent, adr);
        }
        Err(OnfexError::runtime(format!("invalid address {}", adr)))
    }

    fn lookup_address(&self, env: &Environment, name: &str) -> Result<usize, OnfexError> {
        if let Some(adr) = env.names.get(name) {
            return Ok(*adr);
        }
        if let Some(p) = &env.parent {
            return self.lookup_address(p, name);
        }
        Err(OnfexError::runtime(format!("variable '{}' not found", name)))
    }

    // run user function
    fn run_func(&self, func: Frounct, args: Vec<Type>) -> Result<Type, OnfexError> {
        self.enter_scope();
        let arg_count = args.len();
        let mut pars = func.params.clone();
        let mut ag = args;

        // vararg (!args) parametresi: geri kalan tüm argümanları tek bir
        // Vect içinde toplar ve tek parametre olarak bağlar.
        if let Some(first) = pars.first().cloned() {
            if first.vararg {
                let adr = *self.ptr.borrow();
                let res = Type::new(TypeKind::Vect, Expr::Vect(ag.len(), ag.clone()));
                self.env.borrow_mut().names.insert(first.name.clone(), adr);
                self.env.borrow_mut().heap.insert(adr, res);
                self.inc_ptr();
                pars.clear();
                ag.clear();
            }
        }

        if pars.len() != ag.len() {
            self.exit_scope();
            return Err(OnfexError::runtime(format!(
                "{} afon promter esp wraithnosan {} esp gephnosan",
                func.params.len(), arg_count
            )));
        }

        for (p, v) in pars.iter().zip(ag.into_iter()) {
            // Jenerik parametre tipleri (TypeKind::Dynamic, örn. strouct
            // Box<T> içindeki T) tip silme (type erasure) ile çalışır:
            // check_param_kind herhangi bir somut tipi kabul eder.
            if !crate::builtins::check_param_kind(&p.kind, &v) {
                self.exit_scope();
                return Err(OnfexError::runtime(format!(
                    "{} esp ountf wraithnosan {} esp gephnosan",
                    p.kind.to_string(), v.kind.to_string()
                )));
            }
            let adr = *self.ptr.borrow();
            self.env.borrow_mut().names.insert(p.name.clone(), adr);
            self.env.borrow_mut().heap.insert(adr, v);
            self.inc_ptr();
        }

        let mut out = Type::newVoid();
        for s in &func.body {
            match self.exec(s) {
                Ok(Flow::Normal(v)) => { out = v; }
                Ok(Flow::Empty) => {}
                Ok(Flow::Return(v)) => {
                    self.exit_scope();
                    if !crate::builtins::check_return_kind(&func.out, &v) {
                        return Err(OnfexError::runtime(format!(
                            "{} esp ountf wraithnosan {} esp gephnosan",
                            func.out.to_string(), v.kind.to_string()
                        )));
                    }
                    return Ok(v);
                }
                Err(e) => {
                    self.exit_scope();
                    return Err(e);
                }
            }
        }
        self.exit_scope();
        if !crate::builtins::check_return_kind(&func.out, &out) {
            return Err(OnfexError::runtime(format!(
                "{} esp ountf wraithnosan {} esp gephnosan",
                func.out.to_string(), out.kind.to_string()
            )));
        }
        Ok(out)
    } 

    // strouct örneklemesi: alan haritasını (ft) StructType'ın tanımlı alanlarıyla
    // eşleştirip yeni bir paylaşımlı (Rc) struct instance'ı üretir. Gerçek
    // eşleştirme/doğrulama mantığı bytecode VM ile de paylaşılan
    // `builtins::instantiate_struct` içindedir.
    fn instantiate_struct(&self, st: StructType, ft: HashMap<String, Type>) -> Result<Type, OnfexError> {
        let instance = crate::builtins::instantiate_struct(&st, ft)?;
        Ok(Type::new(TypeKind::StrctT, Expr::StructDt(Rc::new(instance))))
    }

    // strouct metodu çalıştırır: ilk parametre tipi `srel` (Rust'taki `self` karşılığı)
    // ise `this` bu isme bağlanır (instans metodu); değilse ilişkili/static fonksiyondur.
    // `struct_name`: prube/prive alan görünürlüğünün doğru kontrol edilebilmesi
    // için metod gövdesi çalışırken current_struct olarak ayarlanır.
    fn run_method(&self, struct_name: &str, func: Frounct, this: Option<Type>, args: Vec<Type>) -> Result<Type, OnfexError> {
        let prev_struct = self.current_struct.replace(Some(struct_name.to_string()));
        let result = self.run_method_body(func, this, args);
        self.current_struct.replace(prev_struct);
        result
    }

    /// `builtins::to_iter` doğrudan çeviremediği bir değeri (ör. bir
    /// Vektöre strouct örneği) `forp` için bir Iter'a çevirmeye çalışır:
    /// değer bir strouct örneğiyse ve `gephnosVeot` adında bir metodu
    /// varsa onu çağırıp SONUCU tekrar `builtins::to_iter`'a verir.
    fn struct_to_iter(&self, v: &Type) -> Result<crate::builtins::Iter, OnfexError> {
        if let Expr::StructDt(inst) = &v.value {
            if let Some(m) = inst.base.methods.get("gephnosVeot").cloned() {
                let arr = self.run_method(&inst.base.name.clone(), m, Some(v.clone()), vec![])?;
                if let Some(it) = crate::builtins::to_iter(&arr) {
                    return Ok(it);
                }
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
    fn stringify_for_print(&self, v: Type) -> Result<Type, OnfexError> {
        let method = match &v.value {
            Expr::StructDt(inst) => inst.base.methods.get("__sterge__").cloned(),
            _ => None,
        };
        match method {
            Some(m) => {
                let struct_name = match &v.value {
                    Expr::StructDt(inst) => inst.base.name.clone(),
                    _ => unreachable!(),
                };
                self.run_method(&struct_name, m, Some(v), vec![])
            }
            None => Ok(v),
        }
    }

    fn run_method_body(&self, func: Frounct, this: Option<Type>, args: Vec<Type>) -> Result<Type, OnfexError> {
        self.enter_scope();
        let mut pars = func.params.clone();
        let ag = args;

        if let Some(p0) = pars.first().cloned() {
            if matches!(p0.kind, TypeKind::srel) {
                let me = match this.clone() {
                    Some(v) => v,
                    None => {
                        self.exit_scope();
                        return Err(OnfexError::runtime(format!(
                            "WrossnosStrouct Ern: '{}' metodnos srel esp wraithnosan afma instans oft gephnosan",
                            p0.name
                        )));
                    }
                };
                let adr = *self.ptr.borrow();
                self.env.borrow_mut().names.insert(p0.name.clone(), adr);
                self.env.borrow_mut().heap.insert(adr, me);
                self.inc_ptr();
                pars.remove(0);
            }
        }

        if pars.len() != ag.len() {
            self.exit_scope();
            return Err(OnfexError::runtime(format!(
                "{} afon promter esp wraithnosan {} esp gephnosan",
                pars.len(), ag.len()
            )));
        }

        for (p, v) in pars.iter().zip(ag.into_iter()) {
            let adr = *self.ptr.borrow();
            if !crate::builtins::check_param_kind(&p.kind, &v) {
                self.exit_scope();
                return Err(OnfexError::runtime(format!(
                    "{} esp intf wraithnosan {} esp gephnosan",
                    p.kind.to_string(), v.kind.to_string()
                )));
            }
            self.env.borrow_mut().names.insert(p.name.clone(), adr);
            self.env.borrow_mut().heap.insert(adr, v);
            self.inc_ptr();
        }

        let mut out = Type::newVoid();
        for s in &func.body {
            match self.exec(s) {
                Ok(Flow::Normal(v)) => {}
                Ok(Flow::Empty) => {}
                Ok(Flow::Return(v)) => {
                    self.exit_scope();
                    if !crate::builtins::check_return_kind(&func.out, &v) {
                        return Err(OnfexError::runtime(format!(
                            "{} esp ountf wraithnosan {} esp gephnosan",
                            func.out.to_string(), v.kind.to_string()
                        )));
                    }
                    return Ok(v);
                }
                Err(e) => {
                    self.exit_scope();
                    return Err(e);
                }
            }
        }
        self.exit_scope();
        if !crate::builtins::check_return_kind(&func.out, &out) {
            return Err(OnfexError::runtime(format!(
                "{} esp ountf wraithnosan {} esp gephnosan",
                func.out.to_string(), out.kind.to_string()
            )));
        }
        Ok(out)
    }

    // ikili operatör değerlendirmesi (+ - * / < >). Mantık bytecode VM ile
    // paylaşılan `builtins::binop` içindedir (iki motorda da aynı davranış
    // garanti edilir).
    fn binop(&self, l: Type, op: &str, r: Type) -> Result<Type, OnfexError> {
        crate::builtins::binop(l, op, r)
    }

    // variable
    fn get_var(&self, name: &str) -> Result<Type, OnfexError> {
        let env = self.env.borrow_mut();
        self.lookup(&env, name)
    }

    fn lookup(&self, env: &Environment, name: &str) -> Result<Type, OnfexError> {
        if let Some(adr) = env.names.get(name) {
            if let Some(v) = env.heap.get(adr) {
                return Ok(v.clone());
            }
        }
        if let Some(p) = &env.parent {
            return self.lookup(p, name);
        }
        Err(OnfexError::runtime(format!("variable '{}' not found", name)))
    }

    // scope
    fn enter_scope(&self) {
        let current = self.env.replace(Environment::new());
        let child = Environment::child(current);
        self.env.replace(child);
    }
    fn exit_scope(&self) {
        let current = self.env.replace(Environment::new());
        if let Some(p) = current.parent {
            self.env.replace(*p);
        }
    }
}
