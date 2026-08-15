#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FieldType {
    Scalar,
    Vector,
    Tensor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScalarDomain {
    Real,
    Complex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QCharacter {
    pub field_type: FieldType,
    pub scalar_domain: ScalarDomain,
}

impl QCharacter {
    pub const REAL_SCALAR: Self = Self { field_type: FieldType::Scalar, scalar_domain: ScalarDomain::Real };

    /// Outer-product convention: rank(a⊗b) = rank(a)+rank(b), capped at Tensor.
    pub const fn mul(self, other: Self) -> Self {
        Self {
            field_type: combine_field_types(self.field_type, other.field_type),
            scalar_domain: combine_scalar_domains(self.scalar_domain, other.scalar_domain),
        }
    }

    /// Division only makes physical sense when the divisor is Scalar;
    /// for same-rank operands we return Scalar (ratio); otherwise preserve LHS rank.
    pub const fn div(self, other: Self) -> Self {
        let field_type = match (self.field_type, other.field_type) {
            (a, FieldType::Scalar) => a,

            (a, b) if matches!(
                (a, b),
                (FieldType::Vector, FieldType::Vector) | (FieldType::Tensor, FieldType::Tensor)
            ) => FieldType::Scalar,

            (a, _) => a, // best-effort; caller responsibility
        };
        Self { field_type, scalar_domain: combine_scalar_domains(self.scalar_domain, other.scalar_domain) }
    }

    pub const fn pow(self, exp: i32) -> Self {
        let field_type = match self.field_type {
            FieldType::Scalar => FieldType::Scalar,
            _ if exp == 0    => FieldType::Scalar,
            other            => other,
        };
        Self { field_type, ..self }
    }
}


const fn combine_field_types(a: FieldType, b: FieldType) -> FieldType {
    use FieldType::*;
    match (a, b) {
        (Scalar, x) | (x, Scalar) => x,
        (Vector, Vector) => Tensor, // outer product of two vectors is a tensor
        _ => Tensor, // any combination involving a tensor results in a tensor
    }
}

const fn combine_scalar_domains(a: ScalarDomain, b: ScalarDomain) -> ScalarDomain {
    use ScalarDomain::*;
    match (a, b) {
        (Real, Real) => Real,
        _ => Complex,
    }
}