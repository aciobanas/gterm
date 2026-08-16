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
    pub const COMPLEX_SCALAR: Self = Self { field_type: FieldType::Scalar, scalar_domain: ScalarDomain::Complex };
    pub const REAL_VECTOR: Self = Self { field_type: FieldType::Vector, scalar_domain: ScalarDomain::Real };
    pub const COMPLEX_VECTOR: Self = Self { field_type: FieldType::Vector, scalar_domain: ScalarDomain::Complex };
    pub const REAL_TENSOR: Self = Self { field_type: FieldType::Tensor, scalar_domain: ScalarDomain::Real };
    pub const COMPLEX_TENSOR: Self = Self { field_type: FieldType::Tensor, scalar_domain: ScalarDomain::Complex };

    pub const fn mul(self, other: Self) -> Self {
        use FieldType::*;
        let field_type = match (self.field_type, other.field_type) {
            (Scalar, x) | (x, Scalar) => x,
            _ => panic!("Please specify explicitly the quantity field type."),
        };
        Self {
            field_type,
            scalar_domain: QCharacter::combine_scalar_domains(self.scalar_domain, other.scalar_domain),
        }
    }

    pub const fn div(self, other: Self) -> Self {
        use FieldType::*;
        let field_type = match (self.field_type, other.field_type) {
            (x, Scalar) => x,
            _ => panic!("Please specify explicitly the quantity field type."),
        };
        Self {
            field_type,
            scalar_domain: QCharacter::combine_scalar_domains(self.scalar_domain, other.scalar_domain),
        }
    }

    pub const fn pow(self, exp: i32) -> Self {
        let field_type = match self.field_type {
            FieldType::Scalar => FieldType::Scalar,
            _ if exp == 0    => FieldType::Scalar,
            other            => other,
        };
        Self { field_type, ..self }
    }

    const fn combine_scalar_domains(a: ScalarDomain, b: ScalarDomain) -> ScalarDomain {
        use ScalarDomain::*;
        match (a, b) {
            (Real, Real) => Real,
            _ => Complex,
        }
    }
}




