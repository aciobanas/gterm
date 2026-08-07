use gtherm_units::dimensions::*;

#[test]
fn test_dimensions_creation() {
    assert_eq!(Dims::T.time, 1);
    assert_eq!(Dims::T.length, 0);
    assert_eq!(Dims::T.mass, 0);
    assert_eq!(Dims::T.electric_current, 0);
    assert_eq!(Dims::T.thermodynamic_temperature, 0);
    assert_eq!(Dims::T.amount_of_substance, 0);
    assert_eq!(Dims::T.luminous_intensity, 0);
    assert!(Dims::T.are_base());

    assert_eq!(Dims::L.length, 1);
    assert!(Dims::L.are_base());

    assert_eq!(Dims::M.mass, 1);
    assert!(Dims::M.are_base());

    assert_eq!(Dims::I.electric_current, 1);
    assert!(Dims::I.are_base());

    assert_eq!(Dims::THETA.thermodynamic_temperature, 1);
    assert!(Dims::THETA.are_base());

    assert_eq!(Dims::N.amount_of_substance, 1);
    assert!(Dims::N.are_base());

    assert_eq!(Dims::J.luminous_intensity, 1);
    assert!(Dims::J.are_base());

    assert_eq!(Dims::ZERO.time, 0);
    assert_eq!(Dims::ZERO.length, 0);
    assert_eq!(Dims::ZERO.mass, 0);
    assert_eq!(Dims::ZERO.electric_current, 0);
    assert_eq!(Dims::ZERO.thermodynamic_temperature, 0);
    assert_eq!(Dims::ZERO.amount_of_substance, 0);
    assert_eq!(Dims::ZERO.luminous_intensity, 0);
    assert!(!Dims::ZERO.are_base());
}

#[test]
fn test_dimension_operations() {
    let dims1 = Dims::T; // Time
    let dims2 = Dims::L; // Length
    let dims3 = Dims::M; // Mass

    let mul_dims = dims1 * dims2;
    assert_eq!(mul_dims.time, 1);
    assert_eq!(mul_dims.length, 1);
    assert_eq!(mul_dims.mass, 0);
    assert!(!mul_dims.are_base());

    let div_dims = dims1 / dims2;
    assert_eq!(div_dims.time, 1);
    assert_eq!(div_dims.length, -1);
    assert_eq!(div_dims.mass, 0);
    assert!(!div_dims.are_base());

    let pow_dims = div_dims.pow(2);
    assert_eq!(pow_dims.time, 2);
    assert_eq!(pow_dims.length, -2);
    assert_eq!(pow_dims.mass, 0);
    assert!(!pow_dims.are_base());

    let mul_pow_dims = dims1 * dims2.pow(2);
    assert_eq!(mul_pow_dims.time, 1);
    assert_eq!(mul_pow_dims.length, 2);
    assert_eq!(mul_pow_dims.mass, 0);
    assert!(!mul_pow_dims.are_base());

    let div_pow_dims = dims1 / dims3.pow(2);
    assert_eq!(div_pow_dims.time, 1);
    assert_eq!(div_pow_dims.length, 0);
    assert_eq!(div_pow_dims.mass, -2);
    assert!(!div_pow_dims.are_base());
}
