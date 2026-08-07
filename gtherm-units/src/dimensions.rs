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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dimensions_creation() {
        assert_eq!(Dims::T.time, 1);
        assert_eq!(Dims::T.length, 0);
        assert_eq!(Dims::T.mass, 0);
        assert_eq!(Dims::T.electric_current, 0);
        assert_eq!(Dims::T.thermodynamic_temperature, 0);
        assert_eq!(Dims::T.amount_of_substance, 0);
        assert_eq!(Dims::T.luminous_intensity, 0);
        assert!(Dims::T.are_base());

        assert_eq!(Dims::L.length, 1);
        assert!(Dims::L.are_base());

        assert_eq!(Dims::M.mass, 1);
        assert!(Dims::M.are_base());

        assert_eq!(Dims::I.electric_current, 1);
        assert!(Dims::I.are_base());

        assert_eq!(Dims::THETA.thermodynamic_temperature, 1);
        assert!(Dims::THETA.are_base());

        assert_eq!(Dims::N.amount_of_substance, 1);
        assert!(Dims::N.are_base());

        assert_eq!(Dims::J.luminous_intensity, 1);
        assert!(Dims::J.are_base());

        assert_eq!(Dims::ZERO.time, 0);
        assert_eq!(Dims::ZERO.length, 0);
        assert_eq!(Dims::ZERO.mass, 0);
        assert_eq!(Dims::ZERO.electric_current, 0);
        assert_eq!(Dims::ZERO.thermodynamic_temperature, 0);
        assert_eq!(Dims::ZERO.amount_of_substance, 0);
        assert_eq!(Dims::ZERO.luminous_intensity, 0);
        assert!(!Dims::ZERO.are_base());
    }

    #[test]
    fn test_dimension_operations() {
        let dims1 = Dims::T; // Time
        let dims2 = Dims::L; // Length
        let dims3 = Dims::M; // Mass

        let mul_dims = dims1 * dims2;
        assert_eq!(mul_dims.time, 1);
        assert_eq!(mul_dims.length, 1);
        assert_eq!(mul_dims.mass, 0);
        assert!(!mul_dims.are_base());

        let div_dims = dims1 / dims2;
        assert_eq!(div_dims.time, 1);
        assert_eq!(div_dims.length, -1);
        assert_eq!(div_dims.mass, 0);
        assert!(!div_dims.are_base());

        let pow_dims = div_dims.pow(2);
        assert_eq!(pow_dims.time, 2);
        assert_eq!(pow_dims.length, -2);
        assert_eq!(pow_dims.mass, 0);
        assert!(!pow_dims.are_base());

        let mul_pow_dims = dims1 * dims2.pow(2);
        assert_eq!(mul_pow_dims.time, 1);
        assert_eq!(mul_pow_dims.length, 2);
        assert_eq!(mul_pow_dims.mass, 0);
        assert!(!mul_pow_dims.are_base());

        let div_pow_dims = dims1 / dims3.pow(2);
        assert_eq!(div_pow_dims.time, 1);
        assert_eq!(div_pow_dims.length, 0);
        assert_eq!(div_pow_dims.mass, -2);
        assert!(!div_pow_dims.are_base());
    }
}
