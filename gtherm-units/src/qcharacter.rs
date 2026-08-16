/// Tensor order of a quantity per ISO 80000-2: a scalar is a tensor of order zero and a
/// vector a tensor of order one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TensorOrder {
    Scalar,
    Vector,
    Tensor,
}

/// Numeric field (real or complex) a scalar component is drawn from, e.g. a voltage phasor
/// is a complex scalar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScalarDomain {
    Real,
    Complex,
}

/// The two orthogonal axes of a quantity's ISO 80000 character (tensor order and numeric
/// domain); per ISO 80000-1, character is independent of the quantity's dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QCharacter {
    pub tensor_order: TensorOrder,
    pub scalar_domain: ScalarDomain,
}

impl QCharacter {
    pub const REAL_SCALAR: Self = Self { tensor_order: TensorOrder::Scalar, scalar_domain: ScalarDomain::Real };
    pub const COMPLEX_SCALAR: Self = Self { tensor_order: TensorOrder::Scalar, scalar_domain: ScalarDomain::Complex };
    pub const REAL_VECTOR: Self = Self { tensor_order: TensorOrder::Vector, scalar_domain: ScalarDomain::Real };
    pub const COMPLEX_VECTOR: Self = Self { tensor_order: TensorOrder::Vector, scalar_domain: ScalarDomain::Complex };
    pub const REAL_TENSOR: Self = Self { tensor_order: TensorOrder::Tensor, scalar_domain: ScalarDomain::Real };
    pub const COMPLEX_TENSOR: Self = Self { tensor_order: TensorOrder::Tensor, scalar_domain: ScalarDomain::Complex };

    /// Combines two characters as for a product; panics if both operands are non-scalar.
    pub const fn mul(self, other: Self) -> Self {
        use TensorOrder::*;
        let tensor_order = match (self.tensor_order, other.tensor_order) {
            (Scalar, x) | (x, Scalar) => x,
            _ => panic!("Please specify explicitly the quantity tensor order."),
        };
        Self {
            tensor_order,
            scalar_domain: QCharacter::combine_scalar_domains(self.scalar_domain, other.scalar_domain),
        }
    }

    /// Combines two characters as for a quotient; panics unless the divisor is scalar.
    pub const fn div(self, other: Self) -> Self {
        use TensorOrder::*;
        let tensor_order = match (self.tensor_order, other.tensor_order) {
            (x, Scalar) => x,
            _ => panic!("Please specify explicitly the quantity tensor order."),
        };
        Self {
            tensor_order,
            scalar_domain: QCharacter::combine_scalar_domains(self.scalar_domain, other.scalar_domain),
        }
    }

    /// Raises the tensor order to `exp`; any tensor order raised to the 0th power is scalar.
    pub const fn pow(self, exp: i32) -> Self {
        let tensor_order = match self.tensor_order {
            TensorOrder::Scalar => TensorOrder::Scalar,
            _ if exp == 0    => TensorOrder::Scalar,
            other            => other,
        };
        Self { tensor_order, ..self }
    }

    /// Combines two scalar domains; the result is `Complex` unless both are `Real`.
    const fn combine_scalar_domains(a: ScalarDomain, b: ScalarDomain) -> ScalarDomain {
        use ScalarDomain::*;
        match (a, b) {
            (Real, Real) => Real,
            _ => Complex,
        }
    }
}




