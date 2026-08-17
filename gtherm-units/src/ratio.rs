use std::ops::{Div, Mul};

use crate::errors::ZeroDenominatorError;
use crate::utils::gcd;

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

    pub const FEMTO: Ratio = Ratio::new(1, 1_000_000_000_000_000);
    pub const PICO: Ratio = Ratio::new(1, 1_000_000_000_000);
    pub const NANO: Ratio = Ratio::new(1, 1_000_000_000);
    pub const MICRO: Ratio = Ratio::new(1, 1_000_000);
    pub const MILLI: Ratio = Ratio::new(1, 1_000);
    pub const CENTI: Ratio = Ratio::new(1, 100);
    pub const DECI: Ratio = Ratio::new(1, 10);
    pub const DECA: Ratio = Ratio::new(10, 1);
    pub const HECTO: Ratio = Ratio::new(100, 1);
    pub const KILO: Ratio = Ratio::new(1_000, 1);
    pub const MEGA: Ratio = Ratio::new(1_000_000, 1);
    pub const GIGA: Ratio = Ratio::new(1_000_000_000, 1);
    pub const TERA: Ratio = Ratio::new(1_000_000_000_000, 1);
    pub const PETA: Ratio = Ratio::new(1_000_000_000_000_000, 1);

    /// Const-context equivalent of `==`; trait methods can't be called from `const fn`/`const` items on stable Rust.
    pub const fn const_eq(&self, other: &Ratio) -> bool {
        self.numerator * other.denominator == self.denominator * other.numerator
    }

    /// Const-context equivalent of `*`; trait methods can't be called from `const fn`/`const` items on stable Rust.
    pub const fn const_mul(&self, other: &Ratio) -> Ratio {
        Ratio::new(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
        )
    }

    /// Const-context equivalent of `/`; trait methods can't be called from `const fn`/`const` items on stable Rust.
    pub const fn const_div(&self, other: &Ratio) -> Ratio {
        assert!(other.numerator != 0, "cannot divide by zero ratio");
        Ratio::new(
            self.numerator * other.denominator,
            self.denominator * other.numerator,
        )
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

    /// Const-context equivalent of reducing to lowest terms; no trait bound needed, unlike `num_integer::Integer::gcd`.
    pub const fn simplify(&self) -> Ratio {
        let divisor = gcd(self.numerator, self.denominator);
        Ratio::new(self.numerator / divisor, self.denominator / divisor)
    }

    /// Const-context equivalent of exponentiation; raises `self` to the power `exp` (a negative `exp` inverts the result).
    /// Uses exponentiation by squaring, so it runs in `O(log |exp|)` regardless of how large `exp` is.
    /// Panics if `exp` is negative and `self` is zero, since that requires dividing by zero.
    pub const fn const_pow(&self, exp: i32) -> Ratio {
        // `unsigned_abs` (unlike unary `-`) has no overflow case, even for `exp == i32::MIN`.
        let mut n = exp.unsigned_abs();
        let mut base = *self;
        let mut result = Ratio::ONE;
        while n > 0 {
            if n & 1 == 1 {
                result = result.const_mul(&base);
            }
            base = base.const_mul(&base);
            n >>= 1;
        }
        if exp < 0 {
            Ratio::ONE.const_div(&result)
        } else {
            result
        }
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

const _: () = assert!(
    Ratio::new(1, 2)
        .const_mul(&Ratio::new(2, 3))
        .const_eq(&Ratio::new(1, 3))
);
const _: () = assert!(
    Ratio::new(1, 2)
        .const_div(&Ratio::new(1, 3))
        .const_eq(&Ratio::new(3, 2))
);

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

const _: () = assert!(Ratio::new(2, 4).simplify().const_eq(&Ratio::new(1, 2)));
const _: () = assert!(Ratio::new(0, 5).simplify().const_eq(&Ratio::ZERO));
const _: () = assert!(Ratio::new(7, 1).simplify().const_eq(&Ratio::new(7, 1)));
