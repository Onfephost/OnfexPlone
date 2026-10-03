// bytecode.rs

use crate::ast::*;
use crate::builtins::*;
use crate::error::OnfexError;
use crate::OnfexDecimal::*;

#[derive(Debug, Clone)]
pub enum OpCode {
    //Vektör / Matris
    PushVec(usize),
    PushMatris(usize),
    GetIndex,
    // -- sabitler / yığın --
    PushInt(i64),
    PushFloat(f64),
    PushDecimal(OnfexDecimal),
    PushStr(String),
    PushBool(bool),
    PushType(TypeKind),
    PushVoid,
    Pop,
    Dup,
    Not,

    // -- değişkenler (isim tabanlı, kapsam zinciri üzerinden) --
    DefineVar(String),
    SetVar(String),
    GetVar(String),

    // -- strouct alanları --
    GetField(String),
    SetField(String),

    // -- aritmetik / karşılaştırma --
    BinOp(String),

    // -- kontrol akışı (aynı chunk içinde, mutlak pc hedefli) --
    JumpIfFalse(usize),
    Jump(usize),

    // -- forp döngüsü (bkz. builtins::Iter) --
    MakeIter,
    IterHasNext,
    IterNext,
    BindForpVars(Vec<String>), 

    // -- kapsam (yalnızca değişken bağlamaları için; operand yığınını etkilemez) --
    EnterScope,
    ExitScope,

    // -- çağrılar --
    CallFunction(String, usize),
    CallBuiltin(String, usize),
    CallMethod(String, usize),
    CallMacroMethod(String, usize),
    NewStruct(String, Vec<String>),

    // -- tanımlar (mevcut kapsama bağlar, Void iter) --
    DefineFunction(String),
    DefineStruct(String),

    // -- kütüphane / modül sistemi --
    ImportLib(String),        // urso a::b::c;             -> libs["c"] yüklenir
    ImportMod(String),        // mot "../add.onfex";       -> mods["add"] yüklenir
    AliasLib(String, String), // wrossnosLrib yeni = eski; -> libs içinde yeniden adlandır
    AliasMod(String, String), // wrossnosMot yeni = eski;  -> mods içinde yeniden adlandır
    GetLibVar(String, String),      // lib->degisken
    GetModVar(String, String),      // mod!->degisken
    CallLibFunc(String, String, usize), // lib::fonk(...)
    CallModFunc(String, String, usize), // mod->fonk(...)

    // -- dönüş --
    Return,
    Panic,
}

#[derive(Debug, Clone, Default)]
pub struct Chunk {
    pub name: String,
    pub code: Vec<OpCode>,
    pub positions: Vec<(usize, usize)>,
    pub meta: Option<Frounct>,
    pub struct_meta: Option<StructType>,
}

impl Chunk {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn named(name: String) -> Self {
        Self { name, ..Self::default() }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Program {
    pub chunks: Vec<Chunk>,
    pub cn: usize,
}

impl Program {
    pub fn new() -> Self {
        let mut p = Self::default();
        p.chunks.push(Chunk::named(String::new()));
        p
    }
    
    pub fn find(&self, name: &str) -> Option<&Chunk> {
        self.chunks.iter().find(|c| c.name == name)
    }
}
