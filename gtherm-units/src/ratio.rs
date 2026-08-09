use std::ops::{Div, Mul};
use num_integer::Integer;

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

    /// Const-context equivalent of `==`; trait methods can't be called from `const fn`/`const` items on stable Rust.
    pub const fn const_eq(&self, other: &Ratio) -> bool {
        self.numerator * other.denominator == self.denominator * other.numerator
    }

    /// Const-context equivalent of `*`; trait methods can't be called from `const fn`/`const` items on stable Rust.
    pub const fn const_mul(&self, other: &Ratio) -> Ratio {
        Ratio::new(self.numerator * other.numerator, self.denominator * other.denominator)
    }

    /// Const-context equivalent of `/`; trait methods can't be called from `const fn`/`const` items on stable Rust.
    pub const fn const_div(&self, other: &Ratio) -> Ratio {
        assert!(other.numerator != 0, "cannot divide by zero ratio");
        Ratio::new(self.numerator * other.denominator, self.denominator * other.numerator)
    }

    pub const fn is_zero(&self) -> bool {
        self.numerator == 0
    }

    pub const fn is_one(&self) -> bool {
        self.numerator == self.denominator
    }

    pub const fn to_double(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }

}

impl Div for Ratio {
    type Output = Ratio;

    fn div(self, rhs: Ratio) -> Self::Output {
        self.const_div(&rhs)
    }
}
impl Mul for Ratio {
    type Output = Ratio;

    fn mul(self, rhs: Ratio) -> Self::Output {
        self.const_mul(&rhs)
    }
}

// some compile-time assertions to ensure that the const functions are working as expected
const _: () = assert!(Ratio::ONE.const_eq(&Ratio::ONE));
const _: () = assert!(!Ratio::ONE.const_eq(&Ratio::ZERO));
const _: () = assert!(Ratio::new(1, 2).const_eq(&Ratio::new(2, 4)));

const _: () = assert!(Ratio::new(1, 2).const_mul(&Ratio::new(2, 3)).const_eq(&Ratio::new(1, 3)));
const _: () = assert!(Ratio::new(1, 2).const_div(&Ratio::new(1, 3)).const_eq(&Ratio::new(3, 2)));

const _: () = assert!(matches!(Ratio::try_new(1, 2), Ok(_)));
const _: () = assert!(matches!(Ratio::try_new(1, 0), Err(ZeroDenominatorError)));

const _: () = assert!(Ratio::ONE.to_double() == 1.0);
const _: () = assert!(Ratio::ZERO.to_double() == 0.0);
const _: () = assert!(Ratio::new(1, 2).to_double() == 0.5);

const _: () = assert!(Ratio::new(2, 3).const_pow(0).const_eq(&Ratio::ONE));
const _: () = assert!(Ratio::new(2, 3).const_pow(3).const_eq(&Ratio::new(8, 27)));
const _: () = assert!(Ratio::new(2, 3).const_pow(-2).const_eq(&Ratio::new(9, 4)));
const _: () = assert!(Ratio::ZERO.const_pow(3).const_eq(&Ratio::ZERO));
const _: () = assert!(Ratio::ONE.const_pow(i32::MIN).const_eq(&Ratio::ONE));