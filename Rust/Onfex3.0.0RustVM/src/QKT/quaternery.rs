
use crate::error::OnfexError;
use std::fmt;
use std::ops::{Add, BitAnd, BitOr, BitXor, Not, Shl, Shr, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Qud(u8);

impl Qud {
    pub fn new(value: u8) -> Result<Self, OnfexError> {
        if value > 3 {
            return Err(OnfexError::runtime(format!("Valt Ern: invalid qud value '{}'",value)));
        }
        Ok(Self(value))
    }

    pub fn zero() -> Self {
        Self(0)
    }

    pub fn one() -> Self {
        Self(1)
    }

    pub fn two() -> Self {
        Self(2)
    }

    pub fn three() -> Self {
        Self(3)
    }

    pub fn value(self) -> u8 {
        self.0
    }

    pub fn bits(self) -> u8 {
        self.0
    }
    pub fn out(self) -> String{
        format!("{}",self.0)
    }
}

impl fmt::Display for Qud {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Qud|{}>", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Quad {
    value: u8,
}

impl Quad {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn zero() -> Self {
        Self { value: 0 }
    }

    pub fn from_quds(q0: Qud,q1: Qud,q2: Qud,q3: Qud,) -> Self {
        Self {value:q0.value()| (q1.value() << 2)| (q2.value() << 4)| (q3.value() << 6),}
    }

    pub fn qud(&self, index: usize) -> Result<Qud, OnfexError> {
        if index >= 4 {
            return Err(OnfexError::runtime(format!("Valt Ern: quad index '{}' is out of range",index)));
        }
        let value = (self.value >> (index * 2)) & 0b11;
        Qud::new(value)
    }

    pub fn set_qud(&mut self,index: usize,value: Qud,) -> Result<(), OnfexError> {
        if index >= 4 {
            return Err(OnfexError::runtime(format!("Valt Ern: quad index '{}' is out of range",index)));
        }

        let shift = index * 2;
        let mask = 0b11u8 << shift;

        self.value &= !mask;
        self.value |= value.value() << shift;

        Ok(())
    }

    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn bits(&self) -> u8 {
        self.value
    }

    pub fn as_binary_string(&self) -> String {
        format!("{:08b}", self.value)
    }

    pub fn as_qud_string(&self) -> String {
        format!("<Quad|{}{}{}{}>",self.qud(3).unwrap().out(),self.qud(2).unwrap().out(),self.qud(1).unwrap().out(),self.qud(0).unwrap().out())
    }
}

impl fmt::Display for Quad {
    fn fmt(&self,f: &mut fmt::Formatter<'_>,) -> fmt::Result {
        write!(f, "{}", self.as_qud_string())
    }
}

impl From<u8> for Quad {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

impl From<Quad> for u8 {
    fn from(value: Quad) -> u8 {
        value.value
    }
}

impl Add for Quad {
    type Output = Quad;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.value.wrapping_add(rhs.value)
        )
    }
}

impl Sub for Quad {
    type Output = Quad;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.value.wrapping_sub(rhs.value))
    }
}

impl BitAnd for Quad {
    type Output = Quad;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::new(self.value & rhs.value)
    }
}

impl BitOr for Quad {
    type Output = Quad;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::new(self.value | rhs.value)
    }
}

impl BitXor for Quad {
    type Output = Quad;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::new(self.value ^ rhs.value)
    }
}

impl Not for Quad {
    type Output = Quad;

    fn not(self) -> Self::Output {
        Self::new(!self.value)
    }
}

impl Shl<u8> for Quad {
    type Output = Quad;

    fn shl(self, rhs: u8) -> Self::Output {
        Self::new(self.value << rhs)
    }
}

impl Shr<u8> for Quad {
    type Output = Quad;

    fn shr(self, rhs: u8) -> Self::Output {
        Self::new(self.value >> rhs)
    }
}