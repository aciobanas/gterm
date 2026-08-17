use crate::qspec::QSpec;
use crate::ratio::Ratio;

pub enum UnitEq {
    Term(&'static QSpec),
    Mul(&'static UnitEq, &'static UnitEq),
    Div(&'static UnitEq, &'static UnitEq),
    Pow(&'static UnitEq, i32),
}

pub struct Unit {
    pub name: &'static str,
    pub symbol: &'static str,
    pub kind_of: &'static QSpec,
    pub ratio: Ratio,
}

impl Unit {
    pub const fn is_coherent(&self) -> bool {
        self.ratio.const_eq(&Ratio::ONE)
    }

    pub const fn const_mul(&self, other: &Unit) -> Unit {
        Unit {
            name: "derived",
            symbol: "derived",
            kind_of: self.kind_of, // TODO: this is not correct; need to compute the resulting QSpec
            ratio: self.ratio.const_mul(&other.ratio),
        }
    }

}

pub struct Quantity<T> {
    pub value: T,
    pub qspec: &'static QSpec,
    pub unit: &'static Unit,
}