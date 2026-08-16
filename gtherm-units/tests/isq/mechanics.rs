use gtherm_units::dims::Dims;
use gtherm_units::isq::Isq;
use gtherm_units::qcharacter::{QCharacter, TensorOrder};

#[test]
fn test_mass_density_and_specific_volume() {
    assert_eq!(Isq::MASS_DENSITY.dims, Dims::M / Isq::VOLUME.dims);
    assert_eq!(Isq::DENSITY, Isq::MASS_DENSITY);
    assert_eq!(Isq::SPECIFIC_VOLUME.dims, Isq::MASS_DENSITY.dims.pow(-1));
}

#[test]
fn test_relative_and_surface_and_linear_densities() {
    assert_eq!(Isq::RELATIVE_MASS_DENSITY.dims, Dims::ZERO);
    assert_eq!(Isq::RELATIVE_DENSITY, Isq::RELATIVE_MASS_DENSITY);

    assert_eq!(Isq::SURFACE_MASS_DENSITY.dims, Dims::M / Isq::AREA.dims);
    assert_eq!(Isq::SURFACE_DENSITY, Isq::SURFACE_MASS_DENSITY);

    assert_eq!(Isq::LINEAR_MASS_DENSITY.dims, Dims::M / Dims::L);
    assert_eq!(Isq::LINEAR_DENSITY, Isq::LINEAR_MASS_DENSITY);
}

#[test]
fn test_momentum_and_weight_dims() {
    assert_eq!(Isq::MOMENTUM.dims, Dims::M * Isq::VELOCITY.dims);
    assert_eq!(Isq::WEIGHT.dims, Dims::M * Isq::ACCELERATION_OF_FREE_FALL.dims);
}

#[test]
fn test_friction_and_drag_forces_share_force_dimension() {
    assert_eq!(Isq::STATIC_FRICTION_FORCE.dims, Isq::FORCE.dims);
    assert_eq!(Isq::STATIC_FRICTION, Isq::STATIC_FRICTION_FORCE);

    assert_eq!(Isq::KINETIC_FRICTION_FORCE.dims, Isq::FORCE.dims);
    assert_eq!(Isq::DYNAMIC_FRICTION_FORCE, Isq::KINETIC_FRICTION_FORCE);

    assert_eq!(Isq::ROLLING_RESISTANCE.dims, Isq::FORCE.dims);
    assert_eq!(Isq::ROLLING_DRAG, Isq::ROLLING_RESISTANCE);
    assert_eq!(Isq::ROLLING_FRICTION_FORCE, Isq::ROLLING_RESISTANCE);

    assert_eq!(Isq::DRAG_FORCE.dims, Isq::FORCE.dims);
}

#[test]
fn test_impulse_dims() {
    assert_eq!(Isq::IMPULSE.dims, Isq::FORCE.dims * Dims::T);
}

#[test]
fn test_angular_momentum_and_moment_of_inertia() {
    assert_eq!(Isq::ANGULAR_MOMENTUM.dims, Isq::POSITION_VECTOR.dims * Isq::MOMENTUM.dims);
    assert_eq!(Isq::ANGULAR_MOMENTUM.character.tensor_order, Some(TensorOrder::Vector));

    assert_eq!(Isq::MOMENT_OF_INERTIA.dims, Isq::ANGULAR_MOMENTUM.dims / Isq::ANGULAR_VELOCITY.dims);
    assert_eq!(Isq::MOMENT_OF_INERTIA.character.tensor_order, Some(TensorOrder::Tensor));
}

#[test]
fn test_moment_of_force_torque_and_angular_impulse() {
    assert_eq!(Isq::MOMENT_OF_FORCE.dims, Isq::POSITION_VECTOR.dims * Isq::FORCE.dims);
    assert_eq!(Isq::MOMENT_OF_FORCE.character.tensor_order, Some(TensorOrder::Vector));

    assert_eq!(Isq::TORQUE.dims, Isq::MOMENT_OF_FORCE.dims);
    // no character override: derived from the vector `moment_of_force` but stays the default scalar
    assert_eq!(Isq::TORQUE.character, QCharacter::REAL_SCALAR);

    assert_eq!(Isq::ANGULAR_IMPULSE.dims, Isq::MOMENT_OF_FORCE.dims * Dims::T);
}

#[test]
fn test_pressure_derived_stress_family() {
    assert_eq!(Isq::GAUGE_PRESSURE.dims, Isq::PRESSURE.dims);

    assert_eq!(Isq::STRESS.dims, Isq::PRESSURE.dims);
    assert_eq!(Isq::STRESS.character.tensor_order, Some(TensorOrder::Tensor));

    assert_eq!(Isq::NORMAL_STRESS.dims, Isq::STRESS.dims);
    assert_eq!(Isq::SHEAR_STRESS.dims, Isq::STRESS.dims);
}

#[test]
fn test_strain_family_is_dimensionless() {
    assert_eq!(Isq::STRAIN.dims, Dims::ZERO);
    assert_eq!(Isq::STRAIN.character.tensor_order, Some(TensorOrder::Tensor));

    assert_eq!(Isq::RELATIVE_LINEAR_STRAIN.dims, Dims::ZERO);
    assert_eq!(Isq::SHEAR_STRAIN.dims, Dims::ZERO);
    assert_eq!(Isq::RELATIVE_VOLUME_STRAIN.dims, Dims::ZERO);
    assert_eq!(Isq::POISSON_NUMBER.dims, Dims::ZERO);
}

#[test]
fn test_elastic_moduli_reduce_to_pressure_dimension() {
    // each modulus divides a stress/pressure by a dimensionless strain, so it stays pressure-dimensioned
    assert_eq!(Isq::MODULUS_OF_ELASTICITY.dims, Isq::PRESSURE.dims);
    assert_eq!(Isq::YOUNG_MODULUS, Isq::MODULUS_OF_ELASTICITY);

    assert_eq!(Isq::MODULUS_OF_RIGIDITY.dims, Isq::PRESSURE.dims);
    assert_eq!(Isq::SHEAR_MODULUS, Isq::MODULUS_OF_RIGIDITY);

    assert_eq!(Isq::MODULUS_OF_COMPRESSION.dims, Isq::PRESSURE.dims);
    assert_eq!(Isq::BULK_MODULUS, Isq::MODULUS_OF_COMPRESSION);
}

#[test]
fn test_compressibility_is_inverse_pressure() {
    assert_eq!(Isq::COMPRESSIBILITY.dims, Isq::PRESSURE.dims.pow(-1));
}

#[test]
fn test_second_moments_of_area_and_section_modulus() {
    let expected_moment = Isq::RADIAL_DISTANCE.dims.pow(2) * Isq::AREA.dims;
    assert_eq!(Isq::SECOND_AXIAL_MOMENT_OF_AREA.dims, expected_moment);
    assert_eq!(Isq::SECOND_POLAR_MOMENT_OF_AREA.dims, expected_moment);

    assert_eq!(
        Isq::SECTION_MODULUS.dims,
        Isq::SECOND_AXIAL_MOMENT_OF_AREA.dims / Isq::RADIAL_DISTANCE.dims
    );
}

#[test]
fn test_friction_and_drag_coefficients_are_dimensionless() {
    assert_eq!(Isq::STATIC_FRICTION_COEFFICIENT.dims, Dims::ZERO);
    assert_eq!(Isq::STATIC_FRICTION_FACTOR, Isq::STATIC_FRICTION_COEFFICIENT);
    assert_eq!(Isq::COEFFICIENT_OF_STATIC_FRICTION, Isq::STATIC_FRICTION_COEFFICIENT);

    assert_eq!(Isq::KINETIC_FRICTION_FACTOR.dims, Dims::ZERO);
    assert_eq!(Isq::DYNAMIC_FRICTION_FACTOR, Isq::KINETIC_FRICTION_FACTOR);

    assert_eq!(Isq::ROLLING_RESISTANCE_FACTOR.dims, Dims::ZERO);

    assert_eq!(Isq::DRAG_COEFFICIENT.dims, Dims::ZERO);
    assert_eq!(Isq::DRAG_FACTOR, Isq::DRAG_COEFFICIENT);
}

#[test]
fn test_viscosity_dims() {
    let expected_dynamic = Isq::SHEAR_STRESS.dims * Dims::L / Isq::VELOCITY.dims;
    assert_eq!(Isq::DYNAMIC_VISCOSITY.dims, expected_dynamic);
    assert_eq!(Isq::KINEMATIC_VISCOSITY.dims, Isq::DYNAMIC_VISCOSITY.dims / Isq::MASS_DENSITY.dims);
}

#[test]
fn test_surface_tension_dims() {
    assert_eq!(Isq::SURFACE_TENSION.dims, Isq::FORCE.dims / Dims::L);
}

#[test]
fn test_mechanical_power_and_work() {
    assert_eq!(Isq::MECHANICAL_POWER.dims, Isq::FORCE.dims * Isq::VELOCITY.dims);
    assert_eq!(Isq::MECHANICAL_WORK.dims, Isq::FORCE.dims * Isq::DISPLACEMENT.dims);
    assert_eq!(Isq::WORK, Isq::MECHANICAL_WORK);
}

#[test]
fn test_mechanical_energy_family() {
    assert_eq!(Isq::MECHANICAL_ENERGY.dims, Dims::M * Dims::L.pow(2) / Dims::T.pow(2));
    assert_eq!(Isq::POTENTIAL_ENERGY.dims, Isq::MECHANICAL_ENERGY.dims);
    assert_eq!(Isq::KINETIC_ENERGY.dims, Dims::M * Isq::SPEED.dims.pow(2));
    assert_eq!(Isq::MECHANICAL_EFFICIENCY.dims, Dims::ZERO);
}

#[test]
fn test_mass_flow_and_rates() {
    assert_eq!(Isq::MASS_FLOW.dims, Isq::MASS_DENSITY.dims * Isq::VELOCITY.dims);
    assert_eq!(Isq::MASS_FLOW.character.tensor_order, Some(TensorOrder::Vector));

    assert_eq!(Isq::MASS_FLOW_RATE.dims, Isq::MASS_FLOW.dims * Isq::AREA.dims);
    assert_eq!(Isq::MASS_CHANGE_RATE.dims, Dims::M / Dims::T);
    assert_eq!(Isq::VOLUME_FLOW_RATE.dims, Isq::VELOCITY.dims * Isq::AREA.dims);
}

#[test]
fn test_action_dims() {
    assert_eq!(Isq::ACTION.dims, Isq::ENERGY.dims * Dims::T);
}

#[test]
fn test_find_same_kind_across_a_term_chain() {
    // static and kinetic friction forces are both directly rooted in force
    assert_eq!(Isq::STATIC_FRICTION_FORCE.find_same_kind(&Isq::KINETIC_FRICTION_FORCE), Some(&Isq::FORCE));

    // normal and shear stress are both directly rooted in stress
    assert_eq!(Isq::NORMAL_STRESS.find_same_kind(&Isq::SHEAR_STRESS), Some(&Isq::STRESS));
}
