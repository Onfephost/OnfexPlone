use crate::error::OnfexError;
use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Sub,Rem};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OnfexDecimal {
    pub value: i128,
    pub scale: u32,
    pub size: u8,
}

impl OnfexDecimal {
    pub fn new(value: i128, scale: u32, size: u8) -> Result<Self, OnfexError> {
        if !matches!(size,16 |64 ) {
            return Err(OnfexError::runtime(format!("Valt Ern: keonvald decmal seznev '{}'", size)));
        }
        let mut result = Self { value, scale, size };
        result.normalize();
        result.check_size()?;
        Ok(result)
    }

    pub fn zero() -> Self {
        Self { value: 0, scale: 0, size: 64 }
    }

    pub fn zero_with_size(size: u8) -> Result<Self, OnfexError> {
        Self::new(0, 0, size)
    }

    pub fn from_i128(value: i128) -> Result<Self, OnfexError> {
        Self::new(value, 0, 64)
    }

    pub fn from_i128_with_size(value: i128, size: u8) -> Result<Self, OnfexError> {
        Self::new(value, 0, size)
    }

    fn check_size(&self) -> Result<(), OnfexError> {
        let (min, max) = match self.size {
            16 => (i16::MIN as i128, i16::MAX as i128),
            64 => (i64::MIN as i128, i64::MAX as i128),
            _ => return Err(OnfexError::runtime(format!("Valt Ern: invalid decimal size '{}'", self.size))),
        };
        if self.value < min || self.value > max {
            return Err(OnfexError::runtime(format!("Valt Ern: decmal valtue '{}' asp intf '{}' quad neat fertnosfer ", self, self.size)));
        }
        Ok(())
    }

    pub fn normalize(&mut self) {
        if self.value == 0 {
            self.scale = 0;
            return;
        }
        while self.scale > 0 && self.value % 10 == 0 {
            self.value /= 10;
            self.scale -= 1;
        }
    }

    fn pow10(n: u32) -> Result<i128, OnfexError> {
        match 10_i128.checked_pow(n) {
            Some(value) => Ok(value),
            None => Err(OnfexError::runtime(format!("Valt Ern: decmal skalnev '{}' asp banev fras ", n))),
        }
    }

    fn align(self, other: Self) -> Result<(i128, i128, u32), OnfexError> {
        if self.scale == other.scale {
            return Ok((self.value, other.value, self.scale));
        }
        if self.scale > other.scale {
            let diff = self.scale - other.scale;
            let multiplier = Self::pow10(diff)?;
            let other_value = match other.value.checked_mul(multiplier) {
                Some(value) => value,
                None => return Err(OnfexError::runtime("Valt Ern: decimal algernen esp opherflarnosan".to_string())),
            };
            Ok((self.value, other_value, self.scale))
        } else {
            let diff = other.scale - self.scale;
            let multiplier = Self::pow10(diff)?;
            let self_value = match self.value.checked_mul(multiplier) {
                Some(value) => value,
                None => return Err(OnfexError::runtime("Valt Ern: decimal alignment overflow".to_string())),
            };
            Ok((self_value, other.value, other.scale))
        }
    }

    pub fn from_f64(value: f64, scale: u32, size: u8) -> Result<Self, OnfexError> {
        if !value.is_finite() {
            return Err(OnfexError::runtime("Valt Ern: invalid f64 value".to_string()));
        }
        if !matches!(size, 16 | 64 ) {
            return Err(OnfexError::runtime(format!("Valt Ern: keoninferins decmal seznev '{}'", size)));
        }
        let multiplier = 10_f64.powi(scale as i32);
        let converted = (value * multiplier).round();
        if !converted.is_finite() {
            return Err(OnfexError::runtime("Valt Ern: f64 conversion overflow".to_string()));
        }
        if converted < i128::MIN as f64 || converted > i128::MAX as f64 {
            return Err(OnfexError::runtime("Valt Ern: f64 value is too large".to_string()));
        }
        let value = converted as i128;
        Self::new(value, scale, size)
    }
    pub fn from_f64_auto(val:f64) -> Result<Self,OnfexError>{
        let mut value = val.clone();
        let defl = "0".to_string();
        let rss = format!("{}",value.clone()).split(".").map(|x| x.to_string()).collect::<Vec<String>>();
        let tm = rss[0].clone();
        let md = rss.get(1).unwrap_or(&defl);
        let ln = md.len();
        let res = tm+&md;
        let i = res.parse::<i128>().unwrap();
        Self::new(i, ln as u32, 64)
    }

    pub fn to_f64(self) -> Result<f64, OnfexError> {
        let power = Self::pow10(self.scale)?;
        Ok(self.value as f64 / power as f64)
    }

    pub fn to_string(&self) -> String {
        format!("{}", self)
    }

    pub fn with_size(self, size: u8) -> Result<Self, OnfexError> {
        Self::new(self.value, self.scale, size)
    }

    pub fn size(&self) -> u8 {
        self.size.clone()
    }
    pub fn set_size(&mut self,sz:u8) -> Result<(),OnfexError>{
        if !matches!(sz, 16 | 64 ) {
            return Err(OnfexError::runtime(format!("Valt Ern: invalid decimal size '{}'", sz)));
        }
        self.size = sz;
        return self.check_size();
    }
}

impl Add for OnfexDecimal {
    type Output = Result<Self, OnfexError>;

    fn add(self, rhs: Self) -> Self::Output {
        let (a, b, scale) = self.align(rhs)?;
        let value = match a.checked_add(b) {
            Some(value) => value,
            None => return Err(OnfexError::runtime("Valt Ern: decimal addition overflow".to_string())),
        };
        Self::new(value, scale, self.size)
    }
}

impl Sub for OnfexDecimal {
    type Output = Result<Self, OnfexError>;

    fn sub(self, rhs: Self) -> Self::Output {
        let (a, b, scale) = self.align(rhs)?;
        let value = match a.checked_sub(b) {
            Some(value) => value,
            None => return Err(OnfexError::runtime("Valt Ern: decimal subtraction overflow".to_string())),
        };
        Self::new(value, scale, self.size)
    }
}

impl Mul for OnfexDecimal {
    type Output = Result<Self, OnfexError>;

    fn mul(self, rhs: Self) -> Self::Output {
        let value = match self.value.checked_mul(rhs.value) {
            Some(value) => value,
            None => return Err(OnfexError::runtime("Valt Ern: decimal multiplication overflow".to_string())),
        };
        let scale = match self.scale.checked_add(rhs.scale) {
            Some(scale) => scale,
            None => return Err(OnfexError::runtime("Valt Ern: decimal scale overflow".to_string())),
        };
        Self::new(value, scale, self.size)
    }
}

impl Div for OnfexDecimal {
    type Output = Result<Self, OnfexError>;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs.value == 0 {
            return Err(OnfexError::runtime("Valt Ern: ernen bron kleün".to_string()));
        }
        const PRECISION: u32 = 18;
        let multiplier = Self::pow10(PRECISION)?;
        let numerator = match self.value.checked_mul(multiplier) {
            Some(value) => value,
            None => return Err(OnfexError::runtime("Valt Ern: decimal division overflow".to_string())),
        };
        let value = numerator / rhs.value;
        let scale = match self.scale.checked_add(PRECISION).and_then(|x| x.checked_sub(rhs.scale)) {
            Some(scale) => scale,
            None => return Err(OnfexError::runtime("Valt Ern: decimal scale overflow".to_string())),
        };
        Self::new(value, scale, self.size)
    }
}

impl Neg for OnfexDecimal {
    type Output = Result<Self, OnfexError>;

    fn neg(self) -> Self::Output {
        let value = match self.value.checked_neg() {
            Some(value) => value,
            None => return Err(OnfexError::runtime("Valt Ern: decimal negation overflow".to_string())),
        };
        Self::new(value, self.scale, self.size)
    }
}

impl PartialOrd for OnfexDecimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (a, b, _) = (*self).align(*other).ok()?;
        Some(a.cmp(&b))
    }
}

impl Ord for OnfexDecimal {
    fn cmp(&self, other: &Self) -> Ordering {
        match (*self).align(*other) {
            Ok((a, b, _)) => a.cmp(&b),
            Err(_) => Ordering::Equal,
        }
    }
}

impl Rem for OnfexDecimal {
    type Output = Result<Self, OnfexError>;

    fn rem(self, rhs: Self) -> Self::Output {
        if rhs.value == 0 {
            return Err(OnfexError::runtime(
                "Valt Ern: division by zero".to_string()
            ));
        }

        let (a, b, scale) = self.align(rhs)?;

        let value = a % b;

        Self::new(value, scale, self.size)
    }
}

impl std::fmt::Display for OnfexDecimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.value == 0 {
            return write!(f, "0");
        }
        let negative = self.value < 0;
        let digits = self.value.abs().to_string();
        if self.scale == 0 {
            if negative {
                write!(f, "-{}", digits)
            } else {
                write!(f, "{}", digits)
            }
        } else {
            let scale = self.scale as usize;
            let result = if digits.len() <= scale {
                format!("0.{}{}", "0".repeat(scale - digits.len()), digits)
            } else {
                let split = digits.len() - scale;
                format!("{}.{}", &digits[..split], &digits[split..])
            };
            if negative {
                write!(f, "-{}", result)
            } else {
                write!(f, "{}", result)
            }
        }
    }
}