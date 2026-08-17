use crate::dims::Dims;
use crate::qcharacter::QCharacter;
use std::collections::HashSet;

/// A dependency graph node describing how a derived quantity is built from others, e.g. velocity = length / time.
// `Box` can't be used here since heap allocation isn't allowed in const context; use `&'static` references instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QSpecEq {
    Term(&'static QSpec),
    Mul(&'static QSpecEq, &'static QSpecEq),
    Div(&'static QSpecEq, &'static QSpecEq),
    Pow(&'static QSpecEq, i32),
}

impl QSpecEq {
    /// Computes the resulting `Dims` of this equation by recursively combining its operands' dimensions.
    pub const fn dims(&self) -> Dims {
        match self {
            QSpecEq::Term(spec) => spec.dims,
            QSpecEq::Mul(a, b) => a.dims().const_mul(&b.dims()),
            QSpecEq::Div(a, b) => a.dims().const_div(&b.dims()),
            QSpecEq::Pow(a, exp) => a.dims().pow(*exp),
        }
    }

    pub const fn character(&self) -> QCharacter {
        match self {
            QSpecEq::Term(spec) => spec.character,
            QSpecEq::Mul(a, b) => a.character().mul(b.character()),
            QSpecEq::Div(a, b) => a.character().div(b.character()),
            QSpecEq::Pow(a, exp) => a.character().pow(*exp),
        }
    }
}

/// A physical quantity's specification: its name, dimensionality, and (if derived) the equation it comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QSpec {
    pub name: &'static str,
    pub dims: Dims,
    pub character: QCharacter,
    pub equation: Option<&'static QSpecEq>,
}

impl QSpec {
    /// Starts a builder for a base spec (real scalar, no equation); chain `.character()`
    /// and/or `.equation()` to override.
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            dims: Dims::ZERO,                   // default to dimensionless
            character: QCharacter::REAL_SCALAR, // default to real scalar
            equation: None,                     // default to base quantity
        }
    }

    /// Overrides the default `Dims` for this spec.
    pub const fn dims(mut self, dims: Dims) -> Self {
        self.dims = dims;
        self
    }

    /// Overrides the default `REAL_SCALAR` character.
    pub const fn character(mut self, character: QCharacter) -> Self {
        self.character = character;
        self
    }

    /// Marks this as a derived spec built from `equation`.
    pub const fn equation(mut self, equation: &'static QSpecEq) -> Self {
        self.equation = Some(equation);
        self.dims = equation.dims();
        self
    }

    /// Whether this is one of the seven SI base quantities, as opposed to derived from an equation.
    pub const fn is_base(&self) -> bool {
        self.equation.is_none()
    }

    /// Whether this is derived from an equation, as opposed to one of the seven SI base quantities.
    pub const fn is_derived(&self) -> bool {
        self.equation.is_some()
    }

    /// Finds the nearest common sub-kind ancestor (via `Term` links), if any.
    pub fn find_same_kind(&'static self, other: &'static QSpec) -> Option<&'static QSpec> {
        if self.dims != other.dims {
            return None;
        }

        let mut self_ancestors: HashSet<&'static QSpec> = HashSet::new();

        let mut current: &'static QSpec = self;
        loop {
            self_ancestors.insert(current);
            let Some(QSpecEq::Term(sub)) = current.equation else {
                break;
            };
            current = sub;
        }

        let mut current: &'static QSpec = other;
        loop {
            if self_ancestors.contains(current) {
                return Some(current);
            }
            let Some(QSpecEq::Term(sub)) = current.equation else {
                break;
            };
            current = sub;
        }

        None
    }
}

// some compile-time assertions to ensure that the const functions are working as expected
// (`find_same_kind` isn't tested here since it uses `HashSet` and isn't a `const fn`)
use crate::qcharacter::TensorOrder;

const LEN: QSpec = QSpec::new("len").dims(Dims::L);
const TIME: QSpec = QSpec::new("time").dims(Dims::T);
const FORCE: QSpec = QSpec::new("force").character(QCharacter::REAL_VECTOR);

const VELOCITY_EQ: QSpecEq = QSpecEq::Div(&QSpecEq::Term(&LEN), &QSpecEq::Term(&TIME));
const VELOCITY: QSpec = QSpec::new("velocity").equation(&VELOCITY_EQ);

const _: () = assert!(LEN.is_base());
const _: () = assert!(!LEN.is_derived());

const _: () = assert!(VELOCITY_EQ.dims().const_eq(&Dims::L.const_div(&Dims::T)));
const _: () = assert!(VELOCITY.dims.const_eq(&VELOCITY_EQ.dims()));
const _: () = assert!(VELOCITY.is_derived());
const _: () = assert!(!VELOCITY.is_base());

const _: () = assert!(matches!(
    FORCE.character.tensor_order,
    Some(TensorOrder::Vector)
));

// `Mul`/`Div`/`Pow` on `QSpecEq` should combine dims and character the same way as on `Dims`/`QCharacter` directly
const MOMENTUM_EQ: QSpecEq = QSpecEq::Mul(&QSpecEq::Term(&FORCE), &QSpecEq::Term(&TIME));
const _: () = assert!(MOMENTUM_EQ.dims().const_eq(&Dims::T));
const _: () = assert!(matches!(
    MOMENTUM_EQ.character().tensor_order,
    Some(TensorOrder::Vector)
));

const AREA_EQ: QSpecEq = QSpecEq::Pow(&QSpecEq::Term(&LEN), 2);
const _: () = assert!(AREA_EQ.dims().const_eq(&Dims::L.pow(2)));
