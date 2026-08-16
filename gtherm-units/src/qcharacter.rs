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
pub enum ValuesDomain {
    Real,
    Complex,
}

/// The two orthogonal axes of a quantity's ISO 80000 character (tensor order and numeric
/// domain); per ISO 80000-1, character is independent of the quantity's dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QCharacter {
    pub tensor_order: Option<TensorOrder>,
    pub values_domain: ValuesDomain,
}

impl QCharacter {
    pub const REAL_SCALAR: Self = Self {
        tensor_order: Some(TensorOrder::Scalar),
        values_domain: ValuesDomain::Real,
    };
    pub const COMPLEX_SCALAR: Self = Self {
        tensor_order: Some(TensorOrder::Scalar),
        values_domain: ValuesDomain::Complex,
    };
    pub const REAL_VECTOR: Self = Self {
        tensor_order: Some(TensorOrder::Vector),
        values_domain: ValuesDomain::Real,
    };
    pub const COMPLEX_VECTOR: Self = Self {
        tensor_order: Some(TensorOrder::Vector),
        values_domain: ValuesDomain::Complex,
    };
    pub const REAL_TENSOR: Self = Self {
        tensor_order: Some(TensorOrder::Tensor),
        values_domain: ValuesDomain::Real,
    };
    pub const COMPLEX_TENSOR: Self = Self {
        tensor_order: Some(TensorOrder::Tensor),
        values_domain: ValuesDomain::Complex,
    };

    /// Whether this has a tensor order explicitly set.
    pub const fn has_tensor_order(&self) -> bool {
        self.tensor_order.is_some()
    }

    /// Combines two characters as for a product; panics if both operands are non-scalar.
    pub const fn mul(self, other: Self) -> Self {
        use TensorOrder::*;
        let tensor_order = match (self.tensor_order, other.tensor_order) {
            (Some(Scalar), x) | (x, Some(Scalar)) => x,
            _ => None,
        };
        Self {
            tensor_order,
            values_domain: QCharacter::combine_scalar_domains(
                self.values_domain,
                other.values_domain,
            ),
        }
    }

    /// Combines two characters as for a quotient; panics unless the divisor is scalar.
    pub const fn div(self, other: Self) -> Self {
        use TensorOrder::*;
        let tensor_order = match (self.tensor_order, other.tensor_order) {
            (x, Some(Scalar)) => x,
            _ => None,
        };
        Self {
            tensor_order,
            values_domain: QCharacter::combine_scalar_domains(
                self.values_domain,
                other.values_domain,
            ),
        }
    }

    /// Raises the tensor order to `exp`; any tensor order raised to the 0th power is scalar.
    pub const fn pow(self, exp: i32) -> Self {
        use TensorOrder::*;
        let tensor_order = match self.tensor_order {
            Some(Scalar) => Some(Scalar),
            _ if exp == 0 => Some(Scalar),
            x => x,
        };
        Self {
            tensor_order,
            ..self
        }
    }

    /// Combines two scalar domains; the result is `Complex` unless both are `Real`.
    const fn combine_scalar_domains(a: ValuesDomain, b: ValuesDomain) -> ValuesDomain {
        use ValuesDomain::*;
        match (a, b) {
            (Real, Real) => Real,
            _ => Complex,
        }
    }
}
