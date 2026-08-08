use std::ops::{Div, Mul};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dims {
    pub time: i32,
    pub length: i32,
    pub mass: i32,
    pub electric_current: i32,
    pub thermodynamic_temperature: i32,
    pub amount_of_substance: i32,
    pub luminous_intensity: i32,
}

impl Dims {
    pub const fn new(
        time: i32,
        length: i32,
        mass: i32,
        electric_current: i32,
        thermodynamic_temperature: i32,
        amount_of_substance: i32,
        luminous_intensity: i32,
    ) -> Self {
        Dims {
            time,
            length,
            mass,
            electric_current,
            thermodynamic_temperature,
            amount_of_substance,
            luminous_intensity,
        }
    }

    pub const ZERO: Dims = Dims::new(0, 0, 0, 0, 0, 0, 0);
    pub const T: Dims = Dims::new(1, 0, 0, 0, 0, 0, 0);
    pub const L: Dims = Dims::new(0, 1, 0, 0, 0, 0, 0);
    pub const M: Dims = Dims::new(0, 0, 1, 0, 0, 0, 0);
    pub const I: Dims = Dims::new(0, 0, 0, 1, 0, 0, 0);
    pub const THETA: Dims = Dims::new(0, 0, 0, 0, 1, 0, 0);
    pub const N: Dims = Dims::new(0, 0, 0, 0, 0, 1, 0);
    pub const J: Dims = Dims::new(0, 0, 0, 0, 0, 0, 1);

    pub const fn const_eq(&self, other: &Dims) -> bool {
        self.time == other.time
            && self.length == other.length
            && self.mass == other.mass
            && self.electric_current == other.electric_current
            && self.thermodynamic_temperature == other.thermodynamic_temperature
            && self.amount_of_substance == other.amount_of_substance
            && self.luminous_intensity == other.luminous_intensity
    }

    pub const fn const_div(&self, other: &Dims) -> Dims {
        Dims {
            time: self.time - other.time,
            length: self.length - other.length,
            mass: self.mass - other.mass,
            electric_current: self.electric_current - other.electric_current,
            thermodynamic_temperature: self.thermodynamic_temperature - other.thermodynamic_temperature,
            amount_of_substance: self.amount_of_substance - other.amount_of_substance,
            luminous_intensity: self.luminous_intensity - other.luminous_intensity,
        }
    }

    pub const fn const_mul(&self, other: &Dims) -> Dims {
        Dims {
            time: self.time + other.time,
            length: self.length + other.length,
            mass: self.mass + other.mass,
            electric_current: self.electric_current + other.electric_current,
            thermodynamic_temperature: self.thermodynamic_temperature + other.thermodynamic_temperature,
            amount_of_substance: self.amount_of_substance + other.amount_of_substance,
            luminous_intensity: self.luminous_intensity + other.luminous_intensity,
        }
    }

    pub const fn pow(&self, exponent: i32) -> Dims {
        Dims {
            time: self.time * exponent,
            length: self.length * exponent,
            mass: self.mass * exponent,
            electric_current: self.electric_current * exponent,
            thermodynamic_temperature: self.thermodynamic_temperature * exponent,
            amount_of_substance: self.amount_of_substance * exponent,
            luminous_intensity: self.luminous_intensity * exponent,
        }
    }

    pub const fn are_base(&self) -> bool {
        self.const_eq(&Dims::T)
            || self.const_eq(&Dims::L)
            || self.const_eq(&Dims::M)
            || self.const_eq(&Dims::I)
            || self.const_eq(&Dims::THETA)
            || self.const_eq(&Dims::N)
            || self.const_eq(&Dims::J)
    }
}

impl Div for Dims {
    type Output = Dims;

    fn div(self, rhs: Dims) -> Self::Output {
        self.const_div(&rhs)
    }
}

impl Mul for Dims {
    type Output = Dims;

    fn mul(self, rhs: Dims) -> Self::Output {
        self.const_mul(&rhs)
    }
}

// some compile-time assertions to ensure that the const functions are working as expected
const _: () = assert!(Dims::T.const_eq(&Dims::T));
const _: () = assert!(!Dims::T.const_eq(&Dims::L));
const _: () = assert!(Dims::T.are_base());
const _: () = assert!(!Dims::ZERO.are_base());

const _: () = assert!(Dims::T.const_mul(&Dims::L).const_eq(&Dims::new(1, 1, 0, 0, 0, 0, 0)));
const _: () = assert!(Dims::T.const_div(&Dims::L).const_eq(&Dims::new(1, -1, 0, 0, 0, 0, 0)));
