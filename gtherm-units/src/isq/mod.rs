//! International System of Quantities (ISQ) namespace.
//!
//! `Isq` is a zero-sized "namespace" struct. Each ISQ chapter lives in a submodule
//! and extends `Isq` with a set of associated `const` [`QSpec`] items, so callers
//! write `Isq::L`, `Isq::V`, `Isq::E`, etc.
//!

use crate::qspec::{QSpec, QSpecEq};
use crate::dims::Dims;

/// ISQ namespace. Access quantities as associated constants, e.g. `Isq::LENGTH`.
pub struct Isq;

/// ISO 80000-1: General
pub mod si_quantities;

/// ISO 80000-3: Space and time
pub mod space_and_time;

/// ISO 80000-4: Mechanics
pub mod mechanics;

/// ISO 80000-5: Thermodynamics
pub mod thermodynamics;


impl Isq {

    // base quantity spec constants for the seven SI base quantities

    /// Time.
    pub const T: QSpec = QSpec::new("time").dims(Dims::T);
    /// Length.
    pub const L: QSpec = QSpec::new("length").dims(Dims::L);
    /// Mass.
    pub const M: QSpec = QSpec::new("mass").dims(Dims::M);
    /// Electric current.
    pub const I: QSpec = QSpec::new("electric_current").dims(Dims::I);
    /// Thermodynamic temperature.
    pub const THETA: QSpec = QSpec::new("thermodynamic_temperature").dims(Dims::THETA);
    /// Amount of substance.
    pub const N: QSpec = QSpec::new("amount_of_substance").dims(Dims::N);
    /// Luminous intensity.
    pub const J: QSpec = QSpec::new("luminous_intensity").dims(Dims::J);

    /// dimensionless quantity.
    pub const DIMENSIONLESS: QSpec = QSpec::new("dimensionless").dims(Dims::ZERO);
    
}

// some compile-time assertions to ensure that the const functions are working as expected
const _: () = assert!(Isq::L.is_base());
const _: () = assert!(Isq::T.dims.const_eq(&crate::dims::Dims::T));

const _: () = assert!(Isq::T.is_base());
const _: () = assert!(!Isq::T.is_derived());
const _: () = assert!(Isq::L.dims.const_eq(&Dims::L));

const _: () = assert!(QSpecEq::Term(&Isq::L).dims().const_eq(&Dims::L));
const _: () = assert!(
    QSpecEq::Mul(&QSpecEq::Term(&Isq::L), &QSpecEq::Term(&Isq::T))
        .dims()
        .const_eq(&Dims::L.const_mul(&Dims::T))
);
const _: () = assert!(
    QSpecEq::Div(&QSpecEq::Term(&Isq::L), &QSpecEq::Term(&Isq::T))
        .dims()
        .const_eq(&Dims::L.const_div(&Dims::T))
);
const _: () = assert!(
    QSpecEq::Pow(&QSpecEq::Term(&Isq::L), 3)
        .dims()
        .const_eq(&Dims::L.pow(3))
);
