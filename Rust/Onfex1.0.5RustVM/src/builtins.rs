// builtins.rs

use crate::ast::*;
use crate::error::OnfexError;
use std::collections::HashMap;
use std::cell::RefCell;
use crate::environment::*;
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

#[derive(Debug, Clone)]
pub struct StructType{
    pub name : String,
    pub fields : HashMap<String,Field>,
    pub methods : HashMap<String,Frounct>,
}

impl StructType{
    pub fn new(name:String,
    fields:HashMap<String,Field>,methods:HashMap<String,Frounct>) -> Self{
        Self{
            name,fields,methods
        }
    } 
}
#[derive(Debug, Clone)]
pub struct Struct{
    pub base : Box<StructType>,
    pub fld : RefCell<HashMap<String,Type>>
    
}
impl Struct{
    pub fn new(&self,base:Box<StructType>,fld :RefCell<HashMap<String,Type>>) -> Self{
        Self{base,fld}
    }
}

#[derive(Debug, Clone)]
pub struct Class {
    pub name : String,
    pub methods : RefCell<HashMap<String,Expr>>,
    pub fields: RefCell<HashMap<String, Type>>,
    pub inht : Option<Box<Self>>,
}

impl Class {
    pub fn new(name:String ,methods:RefCell<HashMap<String,Expr>> ,
    fields: RefCell<HashMap<String,Type>>, inht:Option<Box<Self>>) -> Self{
        
        Self{name,methods,fields,inht}
    }
}

#[derive(Debug, Clone)]
pub struct ClassObject {
    pub base : Class,
}

impl ClassObject {
    pub fn new(base:Class) -> Self{
        Self{base}
    }
    pub fn getfield(&self ,n:String) -> Option<Type>{
        self.base.fields.borrow().get(&n).cloned()
    }
    pub fn setfield(&self,n:String,t:Type){
        let mut var = self.base.fields.borrow_mut().get(&n);
        var = Some(&t);
    }
    pub fn getmethod(&self, n:String) -> Option<Expr>{
        self.base.methods.borrow().get(&n).cloned()
    }
}

// ================= ARRAY TYPE =================

#[derive(Debug, Clone)]
pub struct ArrayType {
    pub name: String,
    pub outFn: fn(&Vec<Type>) -> String,
    pub methods: RefCell<HashMap<String, FUNC>>,
}

impl ArrayType {
    pub fn new(name: String, outFn: fn(&Vec<Type>) -> String) -> Self {
        
        Self { name, outFn, methods: RefCell::new(HashMap::new()) }
    }
    pub fn isinstance(&self, x: &ArrayType) -> bool {
        self.name == x.name
    }
}

#[derive(Debug, Clone)]
pub struct MonoType {
    pub name: String,
    pub outFn: fn(&Type) -> String,
    pub methods: RefCell<HashMap<String, FUNC>>,
}

impl MonoType {
    pub fn new(name: String, outFn: fn(&Type) -> String) -> Self {
        Self { name, outFn, methods: RefCell::new(HashMap::new()) }
    }
    pub fn isinstance(&self, x: &MonoType) -> bool {
        self.name == x.name
    }
}

// ================= BUFFER TYPE =================
#[derive(Debug, Clone)]
pub struct BufferType {
    pub name: String,
    pub outFn: fn(&Vec<(Type, Type)>) -> String,
    pub methods: RefCell<HashMap<String, FUNC>>,
}

impl BufferType {
    pub fn new(name: String, outFn: fn(&Vec<(Type, Type)>) -> String) -> Self {
        Self { name, outFn, methods: RefCell::new(HashMap::new()) }
    }
    pub fn isinstance(&self, x: &str) -> bool {
        self.name == x
    }
}

// ================= ARRAY =================
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Buffer {
    pub mapp: Vec<(Type, Type)>,
    pub base: BufferType,
}

impl Buffer {
    pub fn new(mapp: Vec<(Type, Type)>, base: BufferType) -> Self {
        Self { mapp, base }
    }
}

#[derive(Debug, Clone)]
pub struct Mono {
    pub value: Type,
    pub base: MonoType,
}

impl Mono {
    pub fn new(value:Type, base: MonoType) -> Self {
        Self { value, base }
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

pub fn create_builtins_funcs() -> HashMap<String, FUNC> {
    use crate::builtinsdata::*;
    let mut builtins: HashMap<String, FUNC> = HashMap::new();
    builtins.insert("pyrintnos".to_string(), FUNC { func: pr });
    builtins.insert("pyrintnosFowLt".to_string(), FUNC { func: prln });
    builtins.insert("morfenlnos".to_string(), FUNC { func: ask });
    builtins
}

// =========================================================================
// PAYLAŞILAN STROUCT/ÇALIŞTIRMA ÇEKİRDEĞİ
// -------------------------------------------------------------------------
// Bu fonksiyonlar hem `interpreter.rs` (ağaç-yürüten yorumlayıcı) hem de
// `bytecode.rs` (bytecode derleyici + VM) tarafından ORTAK olarak kullanılır.
// Amaç: strouct örnekleme, alan erişimi, ikili operatörler ve dönüş tipi
// kontrolünün iki motorda da AYNI davranmasını garanti etmek -- mantığı iki
// yerde ayrı ayrı yazıp zamanla birbirinden sapmasını önlemek.
// =========================================================================

/// strouct örneklemesi: alan haritasını (ft) StructType'ın tanımlı
/// alanlarıyla eşleştirip yeni bir `Struct` üretir. Çağıran taraf bunu
/// genelde `Rc::new(...)` ile sarıp `Expr::StructDt` içine koyar.
pub fn instantiate_struct(st: &StructType, ft: HashMap<String, Type>) -> Result<Struct, OnfexError> {
    let mut fld: HashMap<String, Type> = HashMap::new();
    for fname in st.fields.keys() {
        match ft.get(fname) {
            Some(v) => {
                fld.insert(fname.clone(), v.clone());
            }
            None => {
                return Err(OnfexError::runtime(format!(
                    "WrossnosStrouct Ern: '{}' alt strouct '{}' afma gephnosan nophe",
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

/// prube/prive (public/private) alan görünürlüğünü kontrol eder.
/// `current_struct`: şu an içinde çalışılan strouct'un ismi (bir impelnos
/// bloğu içindeysek Some(struct_adi), değilsek None).
pub fn check_field_visibility(
    struct_name: &str,
    field: &Field,
    current_struct: &Option<String>,
) -> Result<(), OnfexError> {
    if field.glb {
        return Ok(());
    }
    if current_struct.as_deref() == Some(struct_name) {
        return Ok(());
    }
    Err(OnfexError::runtime(format!(
        "WrossnosStrouct Ern: '{}' alt '{}' priv esp gephnosan (impelnos disponden erisim)",
        field.name, struct_name
    )))
}

/// Bir strouct örneğinden alan okur (görünürlük kontrolü dahil).
pub fn get_field(inst: &Struct, field: &str, current_struct: &Option<String>) -> Result<Type, OnfexError> {
    let f = inst.base.fields.get(field).ok_or_else(|| {
        OnfexError::runtime(format!(
            "WrossnosStrouct Ern: '{}' alt strouct '{}' meovinden oft nophe",
            field, inst.base.name
        ))
    })?;
    check_field_visibility(&inst.base.name, f, current_struct)?;
    inst.fld.borrow().get(field).cloned().ok_or_else(|| {
        OnfexError::runtime(format!("WrossnosStrouct Ern: '{}' alt tanphelanmes nophe", field))
    })
}

/// Bir strouct örneğinin alanına yazar (görünürlük kontrolü dahil).
pub fn set_field(
    inst: &Struct,
    field: &str,
    value: Type,
    current_struct: &Option<String>,
) -> Result<(), OnfexError> {
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

/// Bir metodun/fonksiyonun bildirilen dönüş tipiyle (`out`) döndürdüğü
/// gerçek değerin (`v`) eşleşip eşleşmediğini kontrol eder. `Srel` (Self)
/// özel bir durumdur: gerçek strouct ismini TypeKind seviyesinde
/// takip edemediğimiz için, "herhangi bir strouct örneği dönmüş mü" diye
/// bakılır.
pub fn check_return_kind(out: &TypeKind, v: &Type) -> bool {
    if matches!(out, TypeKind::Srel) {
        matches!(v.kind, TypeKind::StrctT)
    } else {
        v.kind.clone().equal(out.clone())
    }
}

/// ikili operatör değerlendirmesi (+ - * / < >). int/float otomatik
/// uyumlanır, sterge için sadece '+' (birleştirme) desteklenir.
pub fn binop(l: Type, op: &str, r: Type) -> Result<Type, OnfexError> {
    match (l.value.clone(), r.value.clone()) {
        
        (Expr::Int(a), Expr::Int(b)) => match op {
            "+" => Ok(Type::new(TypeKind::Int, Expr::Int(a + b))),
            "-" => Ok(Type::new(TypeKind::Int, Expr::Int(a - b))),
            "*" => Ok(Type::new(TypeKind::Int, Expr::Int(a * b))),
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
            "*" => Ok(Type::new(TypeKind::Float, Expr::Float(a * b))),
            "/" => Ok(Type::new(TypeKind::Float, Expr::Float(a / b))),
            "<" => Ok(Type::new(TypeKind::Bool, Expr::Bool(a < b))),
            ">" => Ok(Type::new(TypeKind::Bool, Expr::Bool(a > b))),
            "<=" => Ok(Type::new(TypeKind::Bool, Expr::Bool(a <= b))),
            ">=" => Ok(Type::new(TypeKind::Bool, Expr::Bool(a >= b))),
            "==" => Ok(Type::new(TypeKind::Bool, Expr::Bool(a == b))),
            _ => Err(OnfexError::runtime(format!("WrossnosOp Ern: '{}' oft alnos nophe", op))),
        },
        
        (Expr::Int(a), Expr::Float(_)) => binop(Type::new(TypeKind::Float, Expr::Float(a as f64)), op, r),
        
        (Expr::Float(_), Expr::Int(b)) => binop(l, op, Type::new(TypeKind::Float, Expr::Float(b as f64))),
        
        (Expr::Str(a), Expr::Str(b)) => match op {
            "+" => Ok(Type::new(TypeKind::Str, Expr::Str(format!("{}{}", a, b)))),
            _ => Err(OnfexError::runtime(format!("WrossnosOp Ern: sterge alt '{}' oft alnos nophe", op))),
        },
        _ => Err(OnfexError::runtime(format!(
            "Typect Ern: '{}' oft {} alt {} arasalt alnos nophe",
            op, l.kind.to_string(), r.kind.to_string()
        ))),
    }
}

/// `ifnt (cond)` koşulunun "truthy" olup olmadığını belirler: `nophe`
/// (Void) ve Bool haricindeki her tip truthy sayılır; Bool ise kendi
/// değerine bakılır.
pub fn is_truthy(v: &Type) -> bool {
    match &v.kind {
        TypeKind::Void => true,
        TypeKind::Bool => matches!(v.value, Expr::Bool(true)),
        _ => true,
    }
}
