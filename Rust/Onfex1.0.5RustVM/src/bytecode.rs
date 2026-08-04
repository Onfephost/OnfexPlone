// bytecode.rs
//
// Onfex için gerçek bir bytecode derleme + çalıştırma sistemi.
//
// Genel mimari:
//   AST (Vec<StmtNode>)  --[Compiler]-->  Program (Chunk'lar)  --[VM]--> sonuç
//
// - `Chunk`  : düz (flat), doğrusal bir OpCode dizisi. `ifnt/elsnt` gibi
//              bloklar ayrı bir Chunk almaz; JUMP komutlarıyla AYNI chunk
//              içine "inline" derlenir. Sadece `frounct` gövdeleri ve
//              `impelnos` metod gövdeleri kendi Chunk'larını alır (bunlar
//              gerçek çağrı sınırlarıdır).


// STROUCT DESTEĞİ: struct örnekleri/tipleri (`Struct`/`StructType`/`Field`)
// ve alan görünürlüğü/örnekleme/dönüş-tipi kontrolü mantığı `builtins.rs`
// içindeki PAYLAŞILAN yardımcı fonksiyonlardan gelir (`instantiate_struct`,
// `get_field`, `set_field`, `binop`, `is_truthy`, `check_return_kind`).
// Böylece bytecode VM'in ürettiği bir struct örneği, ağaç-yürüten
// `interpreter.rs`'in ürettiğiyle BİREBİR aynı temsile ve davranışa sahiptir.
//
// KAPSAM DIŞI (v1): urso/mot/wrossnosLrib/wrossnosMot (kütüphane/modül)
// sistemi), vararg (`...`) parametreler, dizi/mappe (vektöre/mappe)
// literalleri, referans/deref (`&`/`*`), spread (`!`). Bunlarla karşılaşan
// derleyici, sessizce yanlış kod üretmek yerine AÇIK bir OnfexError döner.
// Bu ifadeler için hâlâ `interpreter.rs` (ağaç-yürüten motor) kullanılabilir.

use crate::ast::*;
use crate::builtins::*;
use crate::error::OnfexError;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

// =========================================================================
// OPCODE / CHUNK
// =========================================================================

#[derive(Debug, Clone)]
pub enum OpCode {
    // -- sabitler / yığın --
    PushInt(i64),
    PushFloat(f64),
    PushStr(String),
    PushBool(bool),
    PushVoid,
    Pop,
    Dup,

    // -- değişkenler (isim tabanlı, kapsam zinciri üzerinden) --
    DefineVar(String), // valt x = <tos>;  (pop + tanımla + değeri geri it)
    SetVar(String),    // x = <tos>;       (pop + var olanı güncelle + geri it)
    GetVar(String),    // değişken değerini it

    // -- strouct alanları --
    GetField(String),
    SetField(String), // pop(val), pop(recv), ata, val'i geri it

    // -- aritmetik / karşılaştırma --
    BinOp(String),

    // -- kontrol akışı (aynı chunk içinde, mutlak pc hedefli) --
    JumpIfFalse(usize),
    Jump(usize),

    // -- kapsam (yalnızca değişken bağlamaları için; operand yığınını etkilemez) --
    EnterScope,
    ExitScope,

    // -- çağrılar --
    CallFunction(String, usize), // isim + argüman sayısı
    CallBuiltin(String, usize),  // name!(...) makro çağrısı
    CallMethod(String, usize),   // .metodnos(...) -- alıcı + argümanlar yığından
    NewStruct(String, Vec<String>), // strouct ismi + alan isimleri (push sırasıyla eşleşir)

    // -- tanımlar (mevcut kapsama bağlar, Void iter) --
    DefineFunction(String),
    DefineStruct(String),

    // -- dönüş --
    Return,
}

#[derive(Debug, Clone, Default)]
pub struct Chunk {
    pub code: Vec<OpCode>,
}

impl Chunk {
    pub fn new() -> Self {
        Self { code: Vec::new() }
    }
}

// =========================================================================
// PROGRAM (derleyici çıktısı)
// =========================================================================

#[derive(Debug, Clone, Default)]
pub struct Program {
    pub main: Chunk,
    pub functions: HashMap<String, Chunk>,
    pub function_meta: HashMap<String, Frounct>,
    pub methods: HashMap<String, Chunk>, // key: "StructAdi::metodAdi"
    pub struct_types: HashMap<String, StructType>,
}

impl Program {
    pub fn new() -> Self {
        Self::default()
    }
}
