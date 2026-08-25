// bytecode.rs

use crate::ast::*;
use crate::builtins::*;
use crate::error::OnfexError;
use crate::OnfexDecimal::*;
// =========================================================================
// OPCODE / CHUNK
// =========================================================================

#[derive(Debug, Clone)]
pub enum OpCode {
    //Vektör / Matris
    PushVec(usize),
    PushMatris(usize),
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

    // -- forp döngüsü (bkz. builtins::Iter) --
    MakeIter,            // pop(değer), Iter'a çevir, it
    IterHasNext,          // yığının tepesindeki Iter'a BAKAR (pop etmez), bool it
    IterNext,              // yığının tepesindeki Iter'a BAKAR (pop etmez), sıradaki değeri it
    BindForpVars(Vec<String>), // pop(değer), döngü değişken(ler)ine bağla

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

// =========================================================================
// PROGRAM (derleyici çıktısı)
// =========================================================================

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

    /// İsme göre bir chunk arar (fonksiyon, "Struct::metod" ya da strouct
    /// tanımı). Doğrusal arama: chunk sayısı tipik bir program için küçük
    /// olduğundan ayrı bir isim->indeks tablosuna gerek yok.
    pub fn find(&self, name: &str) -> Option<&Chunk> {
        self.chunks.iter().find(|c| c.name == name)
    }
}
