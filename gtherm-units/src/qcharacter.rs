#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FieldType {
    Scalar,
    Vector,
    Tensor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalarDomain {
    Real,
    Complex,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QCharacter {
    pub field_type: FieldType,
    pub scalar_domain: ScalarDomain,
}

impl QCharacter {
    pub const fn new(field_type: FieldType, scalar_domain: ScalarDomain) -> Self {
        Self {
            field_type,
            scalar_domain,
        }
    }

    pub const fn mul(&self, other: &Self) -> Self {
        let field_type = match (self.field_type, other.field_type) {
            (FieldType::Scalar, FieldType::Scalar) => FieldType::Scalar,

            (FieldType::Scalar, FieldType::Vector)
            | (FieldType::Vector, FieldType::Scalar)
            | (FieldType::Vector, FieldType::Vector) => FieldType::Vector,

            _ => FieldType::Tensor,
        };

        let scalar_domain = match (self.scalar_domain, other.scalar_domain) {
            (ScalarDomain::Real, ScalarDomain::Real) => ScalarDomain::Real,
            _ => ScalarDomain::Complex,
        };

        Self {
            field_type,
            scalar_domain,
        }
    }

    pub const fn div(&self, other: &Self) -> Self {
        let field_type = match (self.field_type, other.field_type) {
            (FieldType::Scalar, FieldType::Scalar) => FieldType::Scalar,

            (FieldType::Scalar, FieldType::Vector)
            | (FieldType::Vector, FieldType::Scalar)
            | (FieldType::Vector, FieldType::Vector) => FieldType::Vector,

            _ => FieldType::Tensor,
        };

        let scalar_domain = match (self.scalar_domain, other.scalar_domain) {
            (ScalarDomain::Real, ScalarDomain::Real) => ScalarDomain::Real,
            _ => ScalarDomain::Complex,
        };

        Self {
            field_type,
            scalar_domain,
        }
    }

    pub const fn pow(&self, exp: i32) -> Self {
        let field_type = match self.field_type {
            FieldType::Scalar => FieldType::Scalar,
            FieldType::Vector => {
                if exp == 0 {
                    FieldType::Scalar
                } else {
                    FieldType::Vector
                }
            }
            FieldType::Tensor => {
                if exp == 0 {
                    FieldType::Scalar
                } else if exp == 1 {
                    FieldType::Tensor
                } else {
                    FieldType::Tensor
                }
            }
        };

        let scalar_domain = match self.scalar_domain {
            ScalarDomain::Real => ScalarDomain::Real,
            ScalarDomain::Complex => ScalarDomain::Complex,
        };

        Self {
            field_type,
            scalar_domain,
        }
    }
}