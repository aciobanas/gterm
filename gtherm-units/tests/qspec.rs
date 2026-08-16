use gtherm_units::dims::Dims;
use gtherm_units::qcharacter::{QCharacter, TensorOrder};
use gtherm_units::qspec::*;
use QSpecEq::{Div, Mul, Pow, Term};

const LENGTH: QSpec = QSpec::new("length").dims(Dims::L);
const TIME: QSpec = QSpec::new("time").dims(Dims::T);
const MASS: QSpec = QSpec::new("mass").dims(Dims::M);

const WIDTH: QSpec = QSpec::new("width").equation(&Term(&LENGTH));
const RADIUS: QSpec = QSpec::new("radius").equation(&Term(&WIDTH));
const DIAMETER: QSpec = QSpec::new("diameter").equation(&Term(&WIDTH));
const HEIGHT: QSpec = QSpec::new("height").equation(&Term(&LENGTH));

const VELOCITY_EQ: QSpecEq = Div(&Term(&LENGTH), &Term(&TIME));
const VELOCITY: QSpec = QSpec::new("velocity").equation(&VELOCITY_EQ);

const FORCE: QSpec = QSpec::new("force")
    .equation(&Div(&Mul(&Term(&MASS), &Term(&LENGTH)), &Pow(&Term(&TIME), 2)))
    .character(QCharacter::REAL_VECTOR);

// same dims as `AREA` but derived through an unrelated equation, so it isn't the same "kind"
const AREA: QSpec = QSpec::new("area").equation(&Pow(&Term(&LENGTH), 2));
const UNRELATED_AREA: QSpec = QSpec::new("unrelated_area").equation(&Mul(&Term(&LENGTH), &Term(&LENGTH)));

#[test]
fn test_qspec_new_defaults_to_dimensionless_scalar_base() {
    let s = QSpec::new("dimensionless");
    assert_eq!(s.name, "dimensionless");
    assert_eq!(s.dims, Dims::ZERO);
    assert_eq!(s.character, QCharacter::REAL_SCALAR);
    assert!(s.equation.is_none());
    assert!(s.is_base());
    assert!(!s.is_derived());
}

#[test]
fn test_qspec_dims_overrides_default() {
    assert_eq!(LENGTH.dims, Dims::L);
    assert!(LENGTH.is_base());
}

#[test]
fn test_qspec_character_overrides_default() {
    assert_eq!(FORCE.character, QCharacter::REAL_VECTOR);
}

#[test]
fn test_qspec_equation_marks_derived_and_computes_dims() {
    assert!(VELOCITY.is_derived());
    assert!(!VELOCITY.is_base());
    assert_eq!(VELOCITY.dims, Dims::L / Dims::T);
}

#[test]
fn test_qspeceq_term_passes_through_spec_dims_and_character() {
    assert_eq!(Term(&LENGTH).dims(), LENGTH.dims);
    assert_eq!(Term(&FORCE).character(), FORCE.character);
}

#[test]
fn test_qspeceq_mul_div_pow_compose_dims() {
    assert_eq!(AREA.dims, Dims::L * Dims::L);
    assert_eq!(VELOCITY_EQ.dims(), Dims::L / Dims::T);
    assert_eq!(FORCE.dims, Dims::M * Dims::L * Dims::T.pow(-2));
}

#[test]
fn test_qspeceq_character_composes_through_operators() {
    // mass (scalar) * velocity (scalar) stays scalar
    let momentum_eq = Mul(&Term(&MASS), &Term(&VELOCITY));
    assert_eq!(momentum_eq.character().tensor_order, Some(TensorOrder::Scalar));

    // force (vector) * time (scalar) stays a vector
    let impulse_eq = Mul(&Term(&FORCE), &Term(&TIME));
    assert_eq!(impulse_eq.character().tensor_order, Some(TensorOrder::Vector));
}

#[test]
fn test_qspec_eq_is_structural() {
    assert_eq!(LENGTH, LENGTH);
    assert_ne!(LENGTH, TIME);
    // same dims, but a distinct name/equation, so not equal
    assert_ne!(AREA, UNRELATED_AREA);
}

#[test]
fn test_find_same_kind_returns_nearest_common_term_ancestor() {
    // radius and diameter are both directly rooted in width
    assert_eq!(RADIUS.find_same_kind(&DIAMETER), Some(&WIDTH));

    // radius and height only share the more distant `length` ancestor
    assert_eq!(RADIUS.find_same_kind(&HEIGHT), Some(&LENGTH));
}

#[test]
fn test_find_same_kind_none_for_different_dims() {
    assert_eq!(LENGTH.find_same_kind(&TIME), None);
}

#[test]
fn test_find_same_kind_none_without_shared_term_chain() {
    // same dims (L^2), but neither is built from a `Term` chain through the other
    assert_eq!(AREA.find_same_kind(&UNRELATED_AREA), None);
}
