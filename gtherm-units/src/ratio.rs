use std::ops::{Div, Mul};

use crate::errors::ZeroDenominatorError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ratio {
    pub numerator: u64,
    pub denominator: u64,
}

impl Ratio {
    /// Panics if `denominator` is zero. Use `try_new` to handle this without panicking.
    pub const fn new(numerator: u64, denominator: u64) -> Self {
        assert!(denominator != 0, "ratio denominator cannot be zero");
        Ratio {
            numerator,
            denominator,
        }
    }

    pub const fn try_new(numerator: u64, denominator: u64) -> Result<Self, ZeroDenominatorError> {
        if denominator == 0 {
            return Err(ZeroDenominatorError);
        }
        Ok(Ratio {
            numerator,
            denominator,
        })
    }

    pub const ONE: Ratio = Ratio::new(1, 1);
    pub const ZERO: Ratio = Ratio::new(0, 1);

    pub const fn const_eq(&self, other: &Ratio) -> bool {
        self.numerator * other.denominator == self.denominator * other.numerator
    }

    


}