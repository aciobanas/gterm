use gtherm_units::qcharacter::*;

#[test]
fn test_qcharacter_named_constants() {
    assert_eq!(QCharacter::REAL_SCALAR.tensor_order, Some(TensorOrder::Scalar));
    assert_eq!(QCharacter::REAL_SCALAR.values_domain, ValuesDomain::Real);

    assert_eq!(QCharacter::COMPLEX_SCALAR.tensor_order, Some(TensorOrder::Scalar));
    assert_eq!(QCharacter::COMPLEX_SCALAR.values_domain, ValuesDomain::Complex);

    assert_eq!(QCharacter::REAL_VECTOR.tensor_order, Some(TensorOrder::Vector));
    assert_eq!(QCharacter::REAL_VECTOR.values_domain, ValuesDomain::Real);

    assert_eq!(QCharacter::COMPLEX_VECTOR.tensor_order, Some(TensorOrder::Vector));
    assert_eq!(QCharacter::COMPLEX_VECTOR.values_domain, ValuesDomain::Complex);

    assert_eq!(QCharacter::REAL_TENSOR.tensor_order, Some(TensorOrder::Tensor));
    assert_eq!(QCharacter::REAL_TENSOR.values_domain, ValuesDomain::Real);

    assert_eq!(QCharacter::COMPLEX_TENSOR.tensor_order, Some(TensorOrder::Tensor));
    assert_eq!(QCharacter::COMPLEX_TENSOR.values_domain, ValuesDomain::Complex);
}

#[test]
fn test_qcharacter_eq() {
    assert_eq!(QCharacter::REAL_SCALAR, QCharacter::REAL_SCALAR);
    assert_ne!(QCharacter::REAL_SCALAR, QCharacter::REAL_VECTOR);
    assert_ne!(QCharacter::REAL_VECTOR, QCharacter::COMPLEX_VECTOR);
}

#[test]
fn test_qcharacter_has_tensor_order() {
    assert!(QCharacter::REAL_SCALAR.has_tensor_order());
    assert!(QCharacter::REAL_VECTOR.has_tensor_order());
    assert!(QCharacter::REAL_TENSOR.has_tensor_order());

    let untyped = QCharacter { tensor_order: None, values_domain: ValuesDomain::Real };
    assert!(!untyped.has_tensor_order());
}

#[test]
fn test_qcharacter_mul_scalar_absorbs_into_other_operand() {
    // scalar * vector == vector, regardless of operand order
    assert_eq!(
        QCharacter::REAL_SCALAR.mul(QCharacter::REAL_VECTOR).tensor_order,
        Some(TensorOrder::Vector)
    );
    assert_eq!(
        QCharacter::REAL_VECTOR.mul(QCharacter::REAL_SCALAR).tensor_order,
        Some(TensorOrder::Vector)
    );

    // scalar * tensor == tensor
    assert_eq!(
        QCharacter::REAL_SCALAR.mul(QCharacter::REAL_TENSOR).tensor_order,
        Some(TensorOrder::Tensor)
    );

    // scalar * scalar == scalar
    assert_eq!(
        QCharacter::REAL_SCALAR.mul(QCharacter::REAL_SCALAR).tensor_order,
        Some(TensorOrder::Scalar)
    );
}

#[test]
fn test_qcharacter_mul_of_two_non_scalars_has_no_tensor_order() {
    // neither operand is scalar, so there's no well-defined resulting tensor order
    assert_eq!(QCharacter::REAL_VECTOR.mul(QCharacter::REAL_VECTOR).tensor_order, None);
    assert_eq!(QCharacter::REAL_VECTOR.mul(QCharacter::REAL_TENSOR).tensor_order, None);
}

#[test]
fn test_qcharacter_mul_domain_combination() {
    assert_eq!(
        QCharacter::REAL_SCALAR.mul(QCharacter::REAL_SCALAR).values_domain,
        ValuesDomain::Real
    );
    assert_eq!(
        QCharacter::REAL_SCALAR.mul(QCharacter::COMPLEX_SCALAR).values_domain,
        ValuesDomain::Complex
    );
    assert_eq!(
        QCharacter::COMPLEX_SCALAR.mul(QCharacter::COMPLEX_SCALAR).values_domain,
        ValuesDomain::Complex
    );
}

#[test]
fn test_qcharacter_div_by_scalar_preserves_dividend_order() {
    assert_eq!(
        QCharacter::REAL_VECTOR.div(QCharacter::REAL_SCALAR).tensor_order,
        Some(TensorOrder::Vector)
    );
    assert_eq!(
        QCharacter::REAL_TENSOR.div(QCharacter::REAL_SCALAR).tensor_order,
        Some(TensorOrder::Tensor)
    );
    assert_eq!(
        QCharacter::REAL_SCALAR.div(QCharacter::REAL_SCALAR).tensor_order,
        Some(TensorOrder::Scalar)
    );
}

#[test]
fn test_qcharacter_div_by_non_scalar_has_no_tensor_order() {
    assert_eq!(QCharacter::REAL_SCALAR.div(QCharacter::REAL_VECTOR).tensor_order, None);
    assert_eq!(QCharacter::REAL_VECTOR.div(QCharacter::REAL_VECTOR).tensor_order, None);
}

#[test]
fn test_qcharacter_div_domain_combination() {
    assert_eq!(
        QCharacter::REAL_SCALAR.div(QCharacter::REAL_SCALAR).values_domain,
        ValuesDomain::Real
    );
    assert_eq!(
        QCharacter::COMPLEX_SCALAR.div(QCharacter::REAL_SCALAR).values_domain,
        ValuesDomain::Complex
    );
}

#[test]
fn test_qcharacter_pow_zero_collapses_to_scalar() {
    assert_eq!(QCharacter::REAL_VECTOR.pow(0).tensor_order, Some(TensorOrder::Scalar));
    assert_eq!(QCharacter::REAL_TENSOR.pow(0).tensor_order, Some(TensorOrder::Scalar));
}

#[test]
fn test_qcharacter_pow_nonzero_preserves_tensor_order() {
    assert_eq!(QCharacter::REAL_VECTOR.pow(2).tensor_order, Some(TensorOrder::Vector));
    assert_eq!(QCharacter::REAL_VECTOR.pow(-1).tensor_order, Some(TensorOrder::Vector));
    assert_eq!(QCharacter::REAL_SCALAR.pow(5).tensor_order, Some(TensorOrder::Scalar));
}

#[test]
fn test_qcharacter_pow_preserves_values_domain() {
    assert_eq!(QCharacter::COMPLEX_VECTOR.pow(2).values_domain, ValuesDomain::Complex);
    assert_eq!(QCharacter::REAL_VECTOR.pow(0).values_domain, ValuesDomain::Real);
}
