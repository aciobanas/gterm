use gtherm_units::dimensions::*;

#[test]
fn test_dims_new() {
    let d = Dims::new(1, -2, 3, 0, 0, 0, 0);
    assert_eq!(d.time, 1);
    assert_eq!(d.length, -2);
    assert_eq!(d.mass, 3);
    assert_eq!(d.electric_current, 0);
    assert_eq!(d.thermodynamic_temperature, 0);
    assert_eq!(d.amount_of_substance, 0);
    assert_eq!(d.luminous_intensity, 0);
}

#[test]
fn test_dims_zero_is_dimensionless() {
    assert_eq!(Dims::ZERO, Dims::new(0, 0, 0, 0, 0, 0, 0));
    assert!(!Dims::ZERO.are_base());
}

#[test]
fn test_dims_base_constants_have_single_exponent() {
    assert_eq!(Dims::T, Dims::new(1, 0, 0, 0, 0, 0, 0));
    assert_eq!(Dims::L, Dims::new(0, 1, 0, 0, 0, 0, 0));
    assert_eq!(Dims::M, Dims::new(0, 0, 1, 0, 0, 0, 0));
    assert_eq!(Dims::I, Dims::new(0, 0, 0, 1, 0, 0, 0));
    assert_eq!(Dims::THETA, Dims::new(0, 0, 0, 0, 1, 0, 0));
    assert_eq!(Dims::N, Dims::new(0, 0, 0, 0, 0, 1, 0));
    assert_eq!(Dims::J, Dims::new(0, 0, 0, 0, 0, 0, 1));
}

#[test]
fn test_dims_are_base() {
    for base in [Dims::T, Dims::L, Dims::M, Dims::I, Dims::THETA, Dims::N, Dims::J] {
        assert!(base.are_base());
    }

    assert!(!Dims::ZERO.are_base());
    assert!(!(Dims::T * Dims::L).are_base()); // composite dimension
    assert!(!Dims::T.pow(2).are_base()); // T^2 isn't itself a base dimension
}

#[test]
fn test_dims_eq() {
    assert_eq!(Dims::T, Dims::T);
    assert_ne!(Dims::T, Dims::L);
    assert!(Dims::T.const_eq(&Dims::T));
    assert!(!Dims::T.const_eq(&Dims::L));
}

#[test]
fn test_dims_mul_and_div_operators() {
    // velocity: L / T
    let velocity = Dims::L / Dims::T;
    assert_eq!(velocity, Dims::new(-1, 1, 0, 0, 0, 0, 0));

    // area: L * L
    let area = Dims::L * Dims::L;
    assert_eq!(area, Dims::new(0, 2, 0, 0, 0, 0, 0));

    // force: M * L * T^-2
    let force = Dims::M * Dims::L * Dims::T.pow(-2);
    assert_eq!(force, Dims::new(-2, 1, 1, 0, 0, 0, 0));
}

#[test]
fn test_dims_pow() {
    assert_eq!(Dims::L.pow(0), Dims::ZERO);
    assert_eq!(Dims::L.pow(1), Dims::L);
    assert_eq!(Dims::L.pow(2), Dims::new(0, 2, 0, 0, 0, 0, 0));
    assert_eq!(Dims::L.pow(-1), Dims::new(0, -1, 0, 0, 0, 0, 0));
    assert_eq!(Dims::ZERO.pow(5), Dims::ZERO);
}
