// builtins.rs

use crate::ast::*;
use crate::error::OnfexError;
use std::collections::HashMap;
use std::cell::RefCell;

use std::any::type_name;
pub type BufferTypeFn = fn(Vec<(Type,Type)>,Vec<Type>,HashMap<String,Type>) -> Result<Type,OnfexError>;
pub type ArrayTypeFn = fn(Vec<Type>,Vec<Type>,HashMap<String,Type>) -> Result<Type,OnfexError>;
pub type MonoTypeFn = fn(Type,Vec<Type>,HashMap<String,Type>) -> Result<Type,OnfexError>;

pub fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

#[derive(Debug, Clone, PartialEq)]
pub struct Iter {
    pub items: Vec<Type>,
    pub pos: RefCell<usize>,
}

impl Iter {
    pub fn new(items: Vec<Type>) -> Self {
        Self { items, pos: RefCell::new(0) }
    }
    pub fn has_next(&self) -> bool {
        *self.pos.borrow() < self.items.len()
    }
    pub fn next(&self) -> Option<Type> {
        let mut p = self.pos.borrow_mut();
        if *p < self.items.len() {
            let v = self.items[*p].clone();
            *p += 1;
            Some(v)
        } else {
            None
        }
    }
    pub fn collect(&self) -> Vec<Type>{
        return self.items.clone()
    }
}

pub fn to_iter(v: &Type) -> Option<Iter> {
    match &v.value {
        Expr::Iter(it) => Some((**it).clone()),
        Expr::ArrayDt(a) => Some(Iter::new(a.items.clone())),
        Expr::Vect(_, items) => Some(Iter::new(items.clone())),
        Expr::Matris(_, pairs) => {
            let items = pairs.iter().map(|(k, vv)| Type::new(TypeKind::Vect, Expr::Vect(2, vec![k.clone(), vv.clone()]))).collect();
            Some(Iter::new(items))
        }
        _ => None,
    }
}

pub fn autoConvert(x:f64) -> (bool,i64,f64){
    if x.clone() % 1.0 == 0.0{
        return (true,x.clone() as i64,x);
    }else{
        return (false,x.clone() as i64,x);
    }
}

// ================= FUNC =================
pub type Fnc = fn(Vec<Type>, HashMap<String, Type>) -> Result<Type, OnfexError>;
#[derive(Debug, Clone)]
pub struct FUNC {
    pub func: Fnc,
}

impl FUNC{
    pub fn run(&self,args:Vec<Type>,ft:HashMap<String,Type>) -> Result<Type, OnfexError> {
        return (self.func)(args,ft)
    }
}

#[derive(Debug, Clone,PartialEq)]
pub struct StructType{
    pub name : String,
    pub generics : Vec<String>,
    pub fields : HashMap<String,Field>,
    pub methods : HashMap<String,Frounct>,
}

impl StructType{
    pub fn new(name:String, generics:Vec<String>,
    fields:HashMap<String,Field>,methods:HashMap<String,Frounct>) -> Self{
        Self{
            name,generics,fields,methods
        }
    }
    pub fn is_generic(&self, x: &str) -> bool {
        self.generics.iter().any(|g| g == x)
    }
}

#[derive(Debug, Clone,PartialEq)]
pub struct Struct{
    pub base : Box<StructType>,
    pub fld : RefCell<HashMap<String,Type>>
}

impl Struct{
    pub fn new(base:Box<StructType>,fld :RefCell<HashMap<String,Type>>) -> Self{
        Self{base,fld}
    }
}

// ================= ARRAY TYPE =================

#[derive(Debug, Clone,PartialEq)]
pub struct ArrayType {
    pub name: String,
    pub outFn: fn(&Vec<Type>) -> String,
    pub methods: RefCell<HashMap<String, ArrayTypeFn>>,
}

impl ArrayType {
    pub fn new(name: String, outFn: fn(&Vec<Type>) -> String) -> Self {
        
        Self { name, outFn, methods: RefCell::new(HashMap::new()) }
    }
    pub fn isinstance(&self, x: &ArrayType) -> bool {
        self.name == x.name
    }
    pub fn insert(&self,x:&str,f:ArrayTypeFn){
        self.methods.borrow_mut().insert(x.to_string(),f);
    } 
}

#[derive(Debug, Clone,PartialEq)]
pub struct MonoType {
    pub name: String,
    pub outFn: fn(&Type) -> String,
    pub methods: RefCell<HashMap<String, MonoTypeFn>>,
}

impl MonoType {
    pub fn new(name: String, outFn: fn(&Type) -> String) -> Self {
        Self { name, outFn, methods: RefCell::new(HashMap::new()) }
    }
    pub fn isinstance(&self, x: &MonoType) -> bool {
        self.name == x.name
    }
    pub fn insert(&self,x:&str,f:MonoTypeFn){
        self.methods.borrow_mut().insert(x.to_string(),f); 
    } 
}

// ================= BUFFER TYPE =================

#[derive(Debug, Clone,PartialEq)]
pub struct BufferType {
    pub name: String,
    pub outFn: fn(&Vec<(Type, Type)>) -> String,
    pub methods: RefCell<HashMap<String, BufferTypeFn>>,
}

impl BufferType {
    pub fn new(name: String, outFn: fn(&Vec<(Type, Type)>) -> String) -> Self {
        Self { name, outFn, methods: RefCell::new(HashMap::new()) }
    }
    pub fn isinstance(&self, x: &str) -> bool {
        self.name == x
    }
    pub fn insert(&self,x:&str,f:BufferTypeFn){
        self.methods.borrow_mut().insert(x.to_string(),f);
    } 
}

// ================= ARRAY =================

#[derive(Debug, Clone,PartialEq)]
pub struct Array {
    pub items: Vec<Type>,
    pub base: ArrayType,
}

impl Array {
    pub fn new(items: Vec<Type>, base: ArrayType) -> Self {
        Self { items, base }
    }
    pub fn runM(&self, func: String, _items: Vec<Type>, base: ArrayType) -> Result<Type, OnfexError> {
        if base.methods.borrow().get(&func).is_some() {
            Ok(Type::newVoid())
        } else {
            Err(OnfexError::runtime(format!("undefined method '{}'", func)))
        }
    }
}

// ================= BUFFER =================

#[derive(Debug, Clone,PartialEq)]
pub struct Buffer {
    pub mapp: Vec<(Type, Type)>,
    pub base: BufferType,
}

impl Buffer {
    pub fn new(mapp: Vec<(Type, Type)>, base: BufferType) -> Self {
        Self { mapp, base }
    }
}

#[derive(Debug, Clone,PartialEq)]
pub struct Mono {
    pub value: Type,
    pub base: MonoType,
}

impl Mono {
    pub fn new(value:Type, base: MonoType) -> Self {
        Self { value, base }
    }
}

// ================= ITER =================
//
// `forp x intf <ifade> { ... }` döngüsünün üzerinde çalıştığı, native
// (Rust tarafında tanımlı) iterasyon durumu. Bir vektöre/mappe/dizi gibi
// bir değerin ÜZERİNDE tek geçişlik (single-pass), imleçli (cursor'lu)
// bir gezinti sağlar; `pos` `RefCell` içindedir çünkü Iter değeri döngü
// boyunca (bytecode tarafında yığında, ağaç-yürütende ise değişkende)
// PAYLAŞILAN bir referans olarak durur ve her `next()` çağrısı onu
// İLERLETMELİDİR -- taşınabilir (Clone) ama imleci PAYLAŞAN bir durum.


/// Bir değeri (zaten-Iter/vektöre/dizi/mappe) `forp` döngüsü için bir
/// `Iter`'a çevirir. `Expr::Matris` (mappe) her (anahtar,değer) çiftini
/// 2 elemanlı bir `Vect` olarak akıtır -- `forp k, v intf mappe {...}`
/// ile doğrudan destructure edilebilsin diye.
///
/// Bir strouct örneği (ör. Vektöre) ise ve `gephnosArrey` adında bir
/// metodu varsa BURADA çağrılamaz (bu saf/motor erişimsiz bir
/// fonksiyondur) -- bu durumda çağıran taraf (interpreter/vm) önce o
/// metodu çalıştırıp SONRA sonucu tekrar bu fonksiyona vermelidir.




/// `forp x, y, z intf ... { }` döngüsünde tek bir `next()` sonucunu
/// verilen değişken isimlerine bağlar: TEK isim varsa değer doğrudan ona
/// bağlanır; BİRDEN FAZLA isim varsa değerin tam olarak o kadar elemanlı
/// bir `Vect` olması beklenir (elemanlar sırayla bağlanır) -- ör.
/// `forp k, v intf mappe {...}`.
pub fn bind_forp_vars(vars: &[String], val: Type) -> Result<Vec<(String, Type)>, OnfexError> {
    if vars.len() == 1 {
        return Ok(vec![(vars[0].clone(), val)]);
    }
    match &val.value {
        Expr::Vect(_, items) if items.len() == vars.len() => {
            Ok(vars.iter().cloned().zip(items.iter().cloned()).collect())
        }
        _ => Err(OnfexError::runtime(format!(
            "Forp Ern: {} promter esp wraithnosan, ama afon değeri {} eleman oft nophe",
            vars.len(),
            vars.len()
        ))),
    }
}

// ================= DEFAULT OUT =================

pub fn default_array_out(v: &Vec<Type>) -> String {
    let mut vals = String::new();
    for i in v {
        vals.push_str(&format!("{}, ", i.__out__(true)));
    }
    if vals.len() >= 2 {
        vals.truncate(vals.len() - 2);
    }
    format!("[{}]", vals)
}

pub fn default_buffer_out(v: &Vec<(Type, Type)>) -> String {
    let mut vals = String::new();
    for (k, val) in v {
        vals.push_str(&format!("{}:{}, ", k.__out__(true), val.__out__(true)));
    }
    if vals.len() >= 2 {
        vals.truncate(vals.len() - 2);
    }
    format!("{{{}}}", vals)
}
pub fn default_mono_out(v: Type) -> String {
    let mut vals = String::new();
    let res = v.clone();
    format!("{:#?}", res.__out__(true))
}



// PAYLAŞILAN STROUCT/ÇALIŞTIRMA ÇEKİRDEĞİ
pub fn instantiate_struct(st: &StructType, ft: HashMap<String, Type>) -> Result<Struct, OnfexError> {
    let mut fld: HashMap<String, Type> = HashMap::new();
    for (fname,fbase) in st.fields.clone() {
        match ft.get(&fname) {
            Some(v) => {
                fld.insert(fname.clone(), v.clone());
            }
            None => {
                return Err(OnfexError::runtime(format!(
                    "WrossnosStrouct Ern: '{}' freld asp neat inferins oft strouct '{}'",
                    fname, st.name
                )));
            }
        }
    }
    for k in ft.keys() {
        if !st.fields.contains_key(k) {
            return Err(OnfexError::runtime(format!(
                "WrossnosStrouct Ern: '{}' alt strouct '{}' meovinden oft nophe",
                k, st.name
            )));
        }
    }
    Ok(Struct { base: Box::new(st.clone()), fld: RefCell::new(fld) })
}

pub fn check_field_visibility(struct_name: &str,field: &Field,current_struct: &Option<String>,) -> Result<(), OnfexError> {
    if field.glb {
        return Ok(());
    }
    if current_struct.as_deref() == Some(struct_name) {
        return Ok(());
    }
    Err(OnfexError::runtime(format!(
        "Valt Ern: '{}' asp '{}' prive freld",
        field.name, struct_name
    )))
}

pub fn get_field(inst: &Struct, field: &str, current_struct: &Option<String>) -> Result<Type, OnfexError> {
    let f = inst.base.fields.get(field).ok_or_else(|| {
        OnfexError::runtime(format!(
            "WrossnosStrouct Ern: '{}' alt strouct '{}' meovinden oft nophe",
            field, inst.base.name
        ))
    })?;
    check_field_visibility(&inst.base.name, f, current_struct)?;
    inst.fld.borrow().get(field).cloned().ok_or_else(|| {
        OnfexError::runtime(format!("WrossnosStrouct Ern: '{}' alt freld nophe", field))
    })
}

pub fn set_field(inst: &Struct,field: &str,value: Type,current_struct: &Option<String>,) -> Result<(), OnfexError> {
    let f = inst.base.fields.get(field).cloned().ok_or_else(|| {
        OnfexError::runtime(format!(
            "WrossnosStrouct Ern: '{}' alt strouct '{}' meovinden oft nophe",
            field, inst.base.name
        ))
    })?;
    check_field_visibility(&inst.base.name, &f, current_struct)?;
    inst.fld.borrow_mut().insert(field.to_string(), value);
    Ok(())
}

pub fn check_return_kind(out: &TypeKind, v: &Type) -> bool {
    if matches!(out, TypeKind::Srel) {
        return matches!(v.kind, TypeKind::StrctT);
    }
    // Jenerik dönüş tipi (T, U, ...): tip silme (type erasure) -- Rust'ta
    // jenerik bir fonksiyonun herhangi bir somut tip için geçerli olması
    // gibi, burada da hangi somut değer dönerse dönsün kabul edilir.
    if matches!(out, TypeKind::Dynamic(_)) {
        return true;
    }
    v.kind.clone().equal(out.clone())
}

pub fn check_param_kind(p: &TypeKind, v: &Type) -> bool {
    if matches!(p, TypeKind::Dynamic(_)) {
        return true;
    }
    p.clone().equal(v.kind.clone())
}

pub fn binop(l: Type, op: &str, r: Type) -> Result<Type, OnfexError> {
    match (l.value.clone(), r.value.clone()) {
        
        (Expr::Int(a), Expr::Int(b)) => match op {
            "+" => Ok(Type::new(TypeKind::Int, Expr::Int(a + b))),
            "-" => Ok(Type::new(TypeKind::Int, Expr::Int(a - b))),
            "%" => Ok(Type::new(TypeKind::Int, Expr::Int(a % b))),
            "*" => {Ok(Type::new(TypeKind::Int, Expr::Int(a * b)))}
            "/" => {
                if b == 0 {
                    return Err(OnfexError::runtime("Sifrhen Ern: sifrhen tarafalt bölnos oft nophe"));
                }
                Ok(Type::new(TypeKind::Int, Expr::Int(a / b)))
            }
            "<" => Ok(Type::new(TypeKind::Bool, Expr::Bool(a < b))),
            ">" => Ok(Type::new(TypeKind::Bool, Expr::Bool(a > b))),
            "<=" => Ok(Type::new(TypeKind::Bool, Expr::Bool(a <= b))),
            ">=" => Ok(Type::new(TypeKind::Bool, Expr::Bool(a >= b))),
            "==" => Ok(Type::new(TypeKind::Bool, Expr::Bool(a == b))),
            _ => Err(OnfexError::runtime(format!("WrossnosOp Ern: '{}' oft alnos nophe", op))),
        },
        
        (Expr::Float(a), Expr::Float(b)) => match op {
            "+" => Ok(Type::new(TypeKind::Float, Expr::Float(a + b))),
            "-" => Ok(Type::new(TypeKind::Float, Expr::Float(a - b))),
            "%" => Ok(Type::new(TypeKind::Float, Expr::Float(a % b))),
            "*" => Ok(Type::new(TypeKind::Float, Expr::Float(a * b))),
            "/" => Ok(Type::new(TypeKind::Float, Expr::Float(a / b))),
            "<" => Ok(Type::new(TypeKind::Bool, Expr::Bool(a < b))),
            ">" => Ok(Type::new(TypeKind::Bool, Expr::Bool(a > b))),
            "<=" => Ok(Type::new(TypeKind::Bool, Expr::Bool(a <= b))),
            ">=" => Ok(Type::new(TypeKind::Bool, Expr::Bool(a >= b))),
            "==" => Ok(Type::new(TypeKind::Bool, Expr::Bool(a == b))),
            _ => Err(OnfexError::runtime(format!("WrossnosOp Ern: '{}' oft alnos nophe", op))),
        },
        (Expr::Decimal(a), Expr::Decimal(b)) => match op {
            "+" => Ok(Type::new(TypeKind::Decimal,Expr::Decimal((a + b)?),)),
            "-" => Ok(Type::new(TypeKind::Decimal,Expr::Decimal((a - b)?),)),
            "%" => Ok(Type::new(TypeKind::Decimal,Expr::Decimal((a % b)?),)),
            "*" => Ok(Type::new(TypeKind::Decimal,Expr::Decimal((a * b)?),)),
            "/" => Ok(Type::new(TypeKind::Decimal,Expr::Decimal((a / b)?),)),
            "<" => Ok(Type::new(TypeKind::Bool,Expr::Bool(a < b),)),
            ">" => Ok(Type::new(TypeKind::Bool,Expr::Bool(a > b),)),
            "<=" => Ok(Type::new(TypeKind::Bool,Expr::Bool(a <= b),)),
            ">=" => Ok(Type::new(TypeKind::Bool,Expr::Bool(a >= b),)),
            "==" => Ok(Type::new(TypeKind::Bool,Expr::Bool(a == b),)),
            _ => Err(OnfexError::runtime(format!("WrossnosOp Ern: '{}' oft alnos nophe",op))),
        },
        
        (Expr::Int(a), Expr::Float(_)) => binop(Type::new(TypeKind::Float, Expr::Float(a as f64)), op, r),
        
        (Expr::Float(_), Expr::Int(b)) => binop(l, op, Type::new(TypeKind::Float, Expr::Float(b as f64))),
        
        (Expr::Str(a), Expr::Str(b)) => match op {
            "+" => Ok(Type::new(TypeKind::Str, Expr::Str(format!("{}{}", a, b)))),
                "==" => Ok(Type::new(TypeKind::Bool, Expr::Bool(a == b))),
            _ => Err(OnfexError::runtime(format!("WrossnosOp Ern: sterge alt '{}' oft alnos nophe", op))),
        },
        (Expr::TypeKind(a),Expr::TypeKind(b)) => match op{
            "==" => Ok(Type::new(TypeKind::Bool, Expr::Bool(*a == *b))),
            _ => Err(OnfexError::runtime(format!(
                "Typect Ern: '{}' asp gerl '{}' brof '{}' neat opernosfer",
                    op, l.kind.to_string(), r.kind.to_string()
                ))), 
        },
        _ => Err(OnfexError::runtime(format!(
            "Typect Ern: '{}' asp gerl '{}' brof '{}' neat opernosfer",
            op, l.kind.to_string(), r.kind.to_string()
        ))),
    }
}

pub fn is_truthy(v: &Type) -> bool {
    match &v.kind {
        TypeKind::Void => true,
        TypeKind::Bool => matches!(v.value, Expr::Bool(true)),
        _ => true,
    }
}
