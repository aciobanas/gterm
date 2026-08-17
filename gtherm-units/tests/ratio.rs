use gtherm_units::errors::ZeroDenominatorError;
use gtherm_units::ratio::*;
use gtherm_units::si::prefix::Prefix;

#[test]
fn test_ratio_creation() {
    let r = Ratio::new(3, 4);
    assert_eq!(r.numerator, 3);
    assert_eq!(r.denominator, 4);

    assert_eq!(Ratio::ONE.numerator, 1);
    assert_eq!(Ratio::ONE.denominator, 1);

    assert_eq!(Ratio::ZERO.numerator, 0);
    assert_eq!(Ratio::ZERO.denominator, 1);
}

#[test]
#[should_panic(expected = "ratio denominator cannot be zero")]
fn test_ratio_new_panics_on_zero_denominator() {
    Ratio::new(1, 0);
}

#[test]
fn test_ratio_try_new() {
    assert_eq!(Ratio::try_new(1, 2), Ok(Ratio::new(1, 2)));
    assert_eq!(Ratio::try_new(1, 0), Err(ZeroDenominatorError));
}

#[test]
fn test_ratio_eq() {
    // `Ratio`'s derived `PartialEq` is structural (field-by-field), not the reduced mathematical value.
    assert_eq!(Ratio::new(1, 2), Ratio::new(1, 2));
    assert_ne!(Ratio::new(1, 2), Ratio::new(1, 3));

    // `const_eq` compares the mathematical ratio, so differently-represented equal fractions match.
    assert!(Ratio::new(1, 2).const_eq(&Ratio::new(2, 4)));
    assert!(Ratio::ONE.const_eq(&Ratio::ONE));
    assert!(!Ratio::ONE.const_eq(&Ratio::ZERO));
}

#[test]
fn test_ratio_mul_and_div_operators() {
    let product = Ratio::new(1, 2) * Ratio::new(2, 3);
    assert!(product.const_eq(&Ratio::new(1, 3)));

    let quotient = Ratio::new(1, 2) / Ratio::new(1, 3);
    assert!(quotient.const_eq(&Ratio::new(3, 2)));
}

#[test]
#[should_panic(expected = "cannot divide by zero ratio")]
fn test_ratio_div_by_zero_panics() {
    let _ = Ratio::new(1, 2) / Ratio::ZERO;
}

#[test]
fn test_ratio_is_zero_and_is_one() {
    assert!(Ratio::ZERO.is_zero());
    assert!(!Ratio::ONE.is_zero());

    assert!(Ratio::ONE.is_one());
    assert!(Ratio::new(3, 3).is_one());
    assert!(!Ratio::new(1, 2).is_one());
}

#[test]
fn test_ratio_to_double() {
    assert_eq!(Ratio::ONE.to_double(), 1.0);
    assert_eq!(Ratio::ZERO.to_double(), 0.0);
    assert_eq!(Ratio::new(1, 2).to_double(), 0.5);
}

#[test]
fn test_ratio_simplify() {
    assert!(Ratio::new(2, 4).simplify().const_eq(&Ratio::new(1, 2)));
    assert!(Ratio::new(0, 5).simplify().const_eq(&Ratio::ZERO));
    assert!(Ratio::new(7, 1).simplify().const_eq(&Ratio::new(7, 1)));
    assert_eq!(Ratio::new(6, 9).simplify().numerator, 2);
    assert_eq!(Ratio::new(6, 9).simplify().denominator, 3);
}

#[test]
fn test_ratio_const_pow_positive_and_zero_exponents() {
    assert!(Ratio::new(2, 3).const_pow(0).const_eq(&Ratio::ONE));
    assert!(Ratio::new(2, 3).const_pow(1).const_eq(&Ratio::new(2, 3)));
    assert!(Ratio::new(2, 3).const_pow(3).const_eq(&Ratio::new(8, 27)));
    assert!(Ratio::ZERO.const_pow(3).const_eq(&Ratio::ZERO));
}

#[test]
fn test_ratio_const_pow_negative_exponents() {
    assert!(Ratio::new(2, 3).const_pow(-1).const_eq(&Ratio::new(3, 2)));
    assert!(Ratio::new(2, 3).const_pow(-2).const_eq(&Ratio::new(9, 4)));
}

#[test]
fn test_ratio_const_pow_edge_cases() {
    // exp == i32::MIN must not overflow when negated internally.
    assert!(Ratio::ONE.const_pow(i32::MIN).const_eq(&Ratio::ONE));
}

#[test]
#[should_panic(expected = "cannot divide by zero ratio")]
fn test_ratio_const_pow_zero_to_negative_exponent_panics() {
    Ratio::ZERO.const_pow(-1);
}

#[test]
fn test_ratio_const_mul_and_const_div() {
    assert!(
        Ratio::new(1, 2)
            .const_mul(&Ratio::new(2, 3))
            .const_eq(&Ratio::new(1, 3))
    );
    assert!(
        Ratio::new(1, 2)
            .const_div(&Ratio::new(1, 3))
            .const_eq(&Ratio::new(3, 2))
    );
}

#[test]
#[should_panic(expected = "cannot divide by zero ratio")]
fn test_ratio_const_div_by_zero_panics() {
    Ratio::ONE.const_div(&Ratio::ZERO);
}

#[test]
fn test_ratio_metric_prefix_constants() {
    assert_eq!(Prefix::FEMTO.to_double(), 1e-15);
    assert_eq!(Prefix::PICO.to_double(), 1e-12);
    assert_eq!(Prefix::NANO.to_double(), 1e-9);
    assert_eq!(Prefix::MICRO.to_double(), 1e-6);
    assert_eq!(Prefix::MILLI.to_double(), 1e-3);
    assert_eq!(Prefix::CENTI.to_double(), 1e-2);
    assert_eq!(Prefix::DECI.to_double(), 1e-1);
    assert_eq!(Prefix::DECA.to_double(), 1e1);
    assert_eq!(Prefix::HECTO.to_double(), 1e2);
    assert_eq!(Prefix::KILO.to_double(), 1e3);
    assert_eq!(Prefix::MEGA.to_double(), 1e6);
    assert_eq!(Prefix::GIGA.to_double(), 1e9);
    assert_eq!(Prefix::TERA.to_double(), 1e12);
    assert_eq!(Prefix::PETA.to_double(), 1e15);

    // adjacent prefixes should be inverses of each other
    assert!(Prefix::MILLI.const_mul(&Prefix::KILO).const_eq(&Ratio::ONE));
    assert!(
        Prefix::CENTI
            .const_mul(&Prefix::HECTO)
            .const_eq(&Ratio::ONE)
    );
    assert!(Prefix::DECI.const_mul(&Prefix::DECA).const_eq(&Ratio::ONE));
}

#[test]
fn test_ratio_clone_copy_and_debug() {
    let r = Ratio::new(3, 4);
    let cloned = r.clone();
    let copied = r;
    assert_eq!(r, cloned);
    assert_eq!(r, copied);
    assert_eq!(format!("{r:?}"), "Ratio { numerator: 3, denominator: 4 }");
}
