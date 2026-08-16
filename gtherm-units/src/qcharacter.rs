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
}
