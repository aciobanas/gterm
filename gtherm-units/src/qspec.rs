use crate::dims::Dims;

/// A dependency graph node describing how a derived quantity is built from others, e.g. velocity = length / time.
// `Box` can't be used here since heap allocation isn't allowed in const context; use `&'static` references instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QSpecEq {
    Term(&'static QSpec),
    Mul(&'static QSpecEq, &'static QSpecEq),
    Div(&'static QSpecEq, &'static QSpecEq),
    Pow(&'static QSpecEq, i32),
}

impl QSpecEq {
    /// Computes the resulting `Dims` of this equation by recursively combining its operands' dimensions.
    pub const fn dims(&self) -> Dims {
        match self {
            QSpecEq::Term(spec) => spec.dims,
            QSpecEq::Mul(a, b) => a.dims().const_mul(&b.dims()),
            QSpecEq::Div(a, b) => a.dims().const_div(&b.dims()),
            QSpecEq::Pow(a, exp) => a.dims().pow(*exp),
        }
    }
}

/// A physical quantity's specification: its name, dimensionality, and (if derived) the equation it comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QSpec {
    pub name: &'static str,
    pub dims: Dims,
    pub equation: Option<&'static QSpecEq>,
}

impl QSpec {
    /// Constructs a base spec (no equation); `dims` is one of the seven SI base dimensions.
    const fn base(name: &'static str, dims: Dims) -> Self {
        Self { name, dims, equation: None }
    }

    // base quantity spec constants for the seven SI base quantities
    /// Time.
    pub const T: QSpec = QSpec::base("time", Dims::T);
    /// Length.
    pub const L: QSpec = QSpec::base("length", Dims::L);
    /// Mass.
    pub const M: QSpec = QSpec::base("mass", Dims::M);
    /// Electric current.
    pub const I: QSpec = QSpec::base("electric_current", Dims::I);
    /// Thermodynamic temperature.
    pub const THETA: QSpec = QSpec::base("thermodynamic_temperature", Dims::THETA);
    /// Amount of substance.
    pub const N: QSpec = QSpec::base("amount_of_substance", Dims::N);
    /// Luminous intensity.
    pub const J: QSpec = QSpec::base("luminous_intensity", Dims::J);

    /// Constructs a derived spec, computing `dims` from `equation` rather than requiring it be passed in.
    pub const fn derived(name: &'static str, equation: &'static QSpecEq) -> Self {
        Self { name, dims: equation.dims(), equation: Some(equation) }
    }

    /// Whether this is one of the seven SI base quantities, as opposed to derived from an equation.
    pub const fn is_base(&self) -> bool {
        self.equation.is_none()
    }

    /// Whether this is derived from an equation, as opposed to one of the seven SI base quantities.
    pub const fn is_derived(&self) -> bool {
        self.equation.is_some()
    }
}

pub const Q_WIDTH: QSpec = QSpec::derived("width", &QSpecEq::Term(&QSpec::L));

pub const Q_VELOCITY: QSpec = QSpec::derived(
    "velocity",
    &QSpecEq::Div(&QSpecEq::Term(&QSpec::L), &QSpecEq::Term(&QSpec::T)),
);
