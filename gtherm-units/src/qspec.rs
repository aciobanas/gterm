use crate::dimensions::Dims;

/// A dependency graph node describing how a derived quantity is built from others, e.g. velocity = length / time.
// `Box` can't be used here since heap allocation isn't allowed in const context; use `&'static` references instead.
pub enum QSpecEquation {
    Term(&'static QSpec),
    Mul(&'static QSpecEquation, &'static QSpecEquation),
    Div(&'static QSpecEquation, &'static QSpecEquation),
    Pow(&'static QSpecEquation, i32),
}

impl QSpecEquation {
    pub const fn dims(&self) -> Dims {
        match self {
            QSpecEquation::Term(spec) => spec.dims,
            QSpecEquation::Mul(a, b) => a.dims().const_mul(&b.dims()),
            QSpecEquation::Div(a, b) => a.dims().const_div(&b.dims()),
            QSpecEquation::Pow(a, exp) => a.dims().pow(*exp),
        }
    }
}

pub struct QSpec {
    pub name: &'static str,
    pub dims: Dims,
    pub equation: Option<&'static QSpecEquation>,
}

impl QSpec {
    pub const fn new(name: &'static str, dims: Dims, equation: Option<&'static QSpecEquation>) -> Self {
        Self { name, dims, equation }
    }

    /// Constructs a derived spec, computing `dims` from `equation` rather than requiring it be passed in.
    pub const fn derived(name: &'static str, equation: &'static QSpecEquation) -> Self {
        Self { name, dims: equation.dims(), equation: Some(equation) }
    }

    pub const fn is_base(&self) -> bool {
        self.equation.is_none()
    }

    pub const fn is_derived(&self) -> bool {
        self.equation.is_some()
    }
}

pub static Q_LENGTH: QSpec = QSpec::new("length", Dims::L, None);
pub static Q_TIME: QSpec = QSpec::new("time", Dims::T, None);

pub static Q_WIDTH: QSpec = QSpec::derived("width", &QSpecEquation::Term(&Q_LENGTH));

pub static Q_VELOCITY: QSpec = QSpec::derived(
    "velocity",
    &QSpecEquation::Div(&QSpecEquation::Term(&Q_LENGTH), &QSpecEquation::Term(&Q_TIME)),
);
