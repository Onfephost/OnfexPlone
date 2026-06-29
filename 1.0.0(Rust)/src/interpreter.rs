// interpreter.rs

use crate::ast::*;
use crate::builtins::*;
use crate::builtinsdata::*;
use crate::environment::Environment;
use crate::libs::libacs::*;
use crate::libs::libStrct::Library;
use crate::runtime::*;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct Interpreter{
    pub builtins: HashMap<String,FUNC>,
    pub array_types: HashMap<String,ArrayType>,
    pub buffer_types: HashMap<String,BufferType>,
    pub env: RefCell<Environment>,
    pub libs: RefCell<HashMap<String,Library>>,
    pub ptr: RefCell<usize>,
}

impl Interpreter{
    // new
    pub fn new()->Self{
        Self{
            builtins: create_builtins_funcs(),
            array_types: create_array_types(),
            buffer_types: create_buffer_types(),
            env: RefCell::new(Environment::new()),
            libs: RefCell::new(HashMap::new()),
            ptr: RefCell::new(1),
        }
    }
    // run
    pub fn run(&self,program:Vec<Stmt>){
        for stmt in program{
            match self.exec(stmt){
                Ok(_)=>{}
                Err(RuntimeSignal::Error(msg))=>{
                    panic!("{}",msg);
                }
                Err(RuntimeSignal::Return(_))=>{
                    panic!("return outside function");
                }
            }
        }
    }
    // exec
    fn exec(&self,stmt:Stmt)->OnfexResult{
        match stmt{
            // expr
            Stmt::Expr(expr)=>{Ok(self.eval(expr))}
            // assign
            Stmt::Assign(name,expr)=>{
                let adr = *self.ptr.borrow();
                let mut value = self.eval(expr);
                value.ptr = Some(adr);
                self.env.borrow_mut().names.insert(name,adr);
                self.env.borrow_mut().heap.insert(adr,value);
                self.inc_ptr();
                Ok(Type::newVoid())
            }
            // reassign
            Stmt::ReAssign(name,expr)=>{
                let adr = self.lookup_address(&self.env.borrow(),&name);
                let mut value = self.eval(expr);
                value.ptr = Some(adr);
                self.env.borrow_mut().heap.insert(adr,value);
                Ok(Type::newVoid())
            }
            // import
            Stmt::Import(name)=>{
                let lib = loadLib(name.clone());
                self.libs.borrow_mut().insert(name,lib);
                Ok(Type::newVoid())
            }
            Stmt::ImportAs(name,a)=>{
                let lib =loadLib(name);
                self.libs.borrow_mut().insert(a,lib);
                Ok(Type::newVoid())
            }
            // return
            Stmt::Return(expr)=>{
                let value =if let Some(e)=expr{
                    self.eval(e)
                    }else{Type::newVoid()};
                Err(RuntimeSignal::Return(value))
            }
            // mehen
            Stmt::Mehen(body)=>{
                self.enter_scope();
                let mut out = Type::newVoid();
                for stmt in body{
                    match self.exec(stmt){
                        Ok(v)=>{out=v;}
                        Err(sig)=>{
                            self.exit_scope();
                            return Err(sig);
                        }
                    }
                }
                self.exit_scope();
                Ok(out)
            }
            // function create
            Stmt::FuncCre(name,params,body,_)=>{
                let func = Frounct::new(params,body);
                let adr = *self.ptr.borrow();
                self.env.borrow_mut().names.insert(name, adr);
                self.env.borrow_mut().heap.insert(adr,Type::new(TypeKind::FuncInht,Expr::FuncInht(func)));
                self.inc_ptr();
                Ok(Type::newVoid())
            }
        }
    }
    // eval
    fn eval(&self,expr:Expr)->Type{
        match expr{
            Expr::Int(x)=>{
                Type::new(TypeKind::Int,Expr::Int(x),)
            }
            Expr::Float(x)=>{
                Type::new(TypeKind::Float,Expr::Float(x))
            }
            Expr::Str(x)=>{
                Type::new(TypeKind::Str,Expr::Str(x),)
            }
            Expr::Bool(x)=>{
                Type::new(TypeKind::Bool,Expr::Bool(x),)
            }
            Expr::Void=>{
                Type::newVoid()
            }
            Expr::Variable(name)=>{
                self.get_var(&name)
            }
            Expr::Deref(expr)=>{
                let r = self.eval(*expr);
                match r.value{
                    Expr::Ref(adr)=>{
                        let env = self.env.borrow();
                        self.load_from_heap(&env,adr)
                    }
                    _=>{
                        panic!("not reference")
                    }       
                }
            }
            // function inherit
            Expr::FuncInht(f)=>{
                Type::new(TypeKind::FuncInht,Expr::FuncInht(f),)
            }
            // array runtime
            Expr::ArrayDt(arr)=>{
                Type::new(TypeKind::ArrayT,Expr::ArrayDt(arr))
            }
            // buffer runtime
            Expr::BufferDt(buf)=>{
                Type::new(TypeKind::BufferT,Expr::BufferDt(buf))
            }
            // list
            Expr::List(type_name,items)=>{
                let vals:Vec<Type> =items.into_iter().map(|x|self.eval(x)).collect();
                let tp =self.array_types.get(&type_name).unwrap().clone();
                Type::new(TypeKind::ArrayT,Expr::ArrayDt(Box::new(Array::new(vals,tp))))
            }
            // dict
            Expr::Dict(type_name,items)=>{
                let mut vals = vec![];
                for (k,v) in items{
                    vals.push((self.eval(k),self.eval(v)));
                }
                let tp =self.buffer_types.get(&type_name).unwrap().clone();
                Type::new(TypeKind::BufferT,Expr::BufferDt(Box::new(Buffer::new(vals,tp))))
            }
            // call
            Expr::Call(name,args)=>{
                let values:Vec<Type> =args.into_iter().map(|x|self.eval(x)).collect();
                // builtin
                if let Some(func) = self.builtins.get(&name){
                    return func.run(values);
                }
                // user func
                let f = self.get_var(&name);
                if let Expr::FuncInht(func)=f.value{
                    return self.run_func(func,values);
                }
                panic!("{} not function",name);
            }
            Expr::AddressOf(name)=>{
                let adr = self.lookup_address(&self.env.borrow(),&name);
                Type::new(TypeKind::Ref,Expr::Ref(adr))
            }
            Expr::Ref(s) => {
                Type::new(TypeKind::Ref,Expr::Ref(s))
            }
            // lib call
            Expr::LibCall(lib,name,args)=>{
                let values:Vec<Type> = args.into_iter().map(|x|self.eval(x)).collect();
                let libs = self.libs.borrow();
                if let Some(l) = libs.get(&lib){
                    if let Some(func) = l.funcs.get(&name){return func(values);}
                    panic!("undefined func");
                }
                panic!("undefined lib");
            }
            // lib list
            Expr::LibList(lib,name,items)=>{
                self.inc_ptr();
                let values:Vec<Type> = items.into_iter().map(|x| self.eval(x)).collect();
                let libs = self.libs.borrow();
                let library = libs.get(&lib).unwrap_or_else(||{panic!("undefined lib {}",lib)});
                let arr_type = library.array_types.get(&name).unwrap_or_else(||{panic!("undefined array type {}::{}",lib,name)}).clone();
                Type::new(TypeKind::ArrayT,Expr::ArrayDt(Box::new(Array::new(values,arr_type))))
            }
            Expr::LibDict(lib,name,items)=>{
                let mut values = vec![];
                for (k,v) in items{
                    values.push((self.eval(k),self.eval(v)));
                }
                let libs = self.libs.borrow();
                let library = libs.get(&lib).unwrap_or_else(||{panic!("undefined lib {}",lib)});
                let buf_type = library.buffer_types.get(&name).unwrap_or_else(||{panic!("undefined buffer type {}::{}",lib,name)}).clone();
                Type::new(TypeKind::BufferT,Expr::BufferDt(Box::new(Buffer::new(values,buf_type))))
            }
        }
    }
    fn inc_ptr(&self){
        *self.ptr.borrow_mut() += 1;
    }
    fn load_from_heap(&self,env:&Environment,adr:usize) -> Type {
        if let Some(v)=env.heap.get(&adr){
            return v.clone();
        }
        if let Some(parent)=&env.parent{
            return self.load_from_heap(parent,adr);
        }
        panic!("invalid address {}",adr);
    }
    fn lookup_address(&self,env:&Environment,name:&str)->usize{
        if let Some(adr)=env.names.get(name){
            return *adr;
        }
        if let Some(p) = &env.parent{
            return self.lookup_address(p,name);
        }
        panic!("variable not found {}",name);
    }
    // run user function
    fn run_func(&self,func:Frounct,args:Vec<Type>)->Type{
        self.enter_scope();
        for (par,arg)in func.params.iter().zip(args){
            let adr = *self.ptr.borrow();
                self.env.borrow_mut().names.insert(par.name.clone(),adr);
                self.env.borrow_mut().heap.insert(adr,arg);
                self.inc_ptr();
            }
        let mut out = Type::newVoid();
        for stmt in func.body{
            match self.exec(stmt){
                Ok(v)=>{
                    out=v;
                }
                Err(RuntimeSignal::Return(v))=>{
                    self.exit_scope();
                    return v;
                }
                Err(RuntimeSignal::Error(msg))=>{panic!("{}",msg);}
            }
        }
        self.exit_scope();
        out
    }
    // variable
    fn get_var(&self,name:&str)->Type{
        let env = self.env.borrow();
        self.lookup(&env,name)
    }

    fn lookup(&self,env:&Environment,name:&str)->Type{
        if let Some(adr)=env.names.get(name){
            if let Some(v)=env.heap.get(adr){
                return v.clone();
            }
        }
        if let Some(p)=&env.parent{
            return self.lookup(p,name);
        }
        panic!("variable not found {}",name);
    }
    // scope
    fn enter_scope(&self){
        let current =self.env.replace(Environment::new());
        let child =Environment::child(current);
        self.env.replace(child);
    }
    fn exit_scope(&self){
        let current =self.env.replace(Environment::new());
        if let Some(p)=current.parent{
            self.env.replace(*p);
        }
    }
}