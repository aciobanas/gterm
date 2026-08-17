use gtherm_units::dims::Dims;
use gtherm_units::isq::Isq;
use gtherm_units::qcharacter::{QCharacter, TensorOrder, ValuesDomain};

#[test]
fn test_width_breadth_radius_share_length_dimension() {
    assert_eq!(Isq::WIDTH.dims, Dims::L);
    assert_eq!(Isq::BREADTH, Isq::WIDTH);
    assert_eq!(Isq::RADIUS.dims, Dims::L);
}

#[test]
fn test_path_length_and_arc_length_are_aliases_of_length() {
    assert_eq!(Isq::PATH_LENGTH.dims, Dims::L);
    assert_eq!(Isq::ARC_LENGTH, Isq::PATH_LENGTH);
}

#[test]
fn test_area_is_length_squared() {
    assert_eq!(Isq::AREA.dims, Dims::L.pow(2));
}

#[test]
fn test_angular_measures_are_dimensionless() {
    // arc_length / radius, and area / radius^2, both cancel out to dimensionless
    assert_eq!(Isq::ANGULAR_MEASURE.dims, Dims::ZERO);
    assert_eq!(Isq::SOLID_ANGULAR_MEASURE.dims, Dims::ZERO);
}

#[test]
fn test_period_duration_and_period_are_time() {
    assert_eq!(Isq::PERIOD_DURATION.dims, Dims::T);
    assert_eq!(Isq::PERIOD, Isq::PERIOD_DURATION);
}

#[test]
fn test_frequency_is_inverse_time() {
    assert_eq!(Isq::FREQUENCY.dims, Dims::T.pow(-1));
}

#[test]
fn test_energy_dims() {
    assert_eq!(Isq::ENERGY.dims, Dims::M * Dims::L.pow(2) / Dims::T.pow(2));
}

#[test]
fn test_force_dims_and_character() {
    assert_eq!(Isq::FORCE.dims, Dims::M * Dims::L / Dims::T.pow(2));
    assert_eq!(Isq::FORCE.character, QCharacter::REAL_VECTOR);
}

#[test]
fn test_pressure_dims_and_character_override_to_scalar() {
    assert_eq!(Isq::PRESSURE.dims, Isq::FORCE.dims / Isq::AREA.dims);
    // force is a vector, but pressure is explicitly overridden back to scalar
    assert_eq!(Isq::PRESSURE.character, QCharacter::REAL_SCALAR);
}

#[test]
fn test_electric_potential_and_capacitance_dims() {
    assert_eq!(
        Isq::ELECTRIC_POTENTIAL.dims,
        Isq::ENERGY.dims / (Dims::I * Dims::T)
    );
    assert_eq!(
        Isq::CAPACITANCE.dims,
        (Dims::I * Dims::T) / Isq::ELECTRIC_POTENTIAL.dims
    );
}

#[test]
fn test_impedance_is_complex_and_admittance_is_its_inverse() {
    assert_eq!(
        Isq::IMPEDANCE.character.values_domain,
        ValuesDomain::Complex
    );
    assert_eq!(Isq::ADMITTANCE.dims, Isq::IMPEDANCE.dims.pow(-1));
}

#[test]
fn test_magnetic_flux_density_is_a_vector() {
    assert_eq!(
        Isq::MAGNETIC_FLUX_DENSITY.character.tensor_order,
        Some(TensorOrder::Vector)
    );
}

#[test]
fn test_luminous_flux_and_illuminance_dims() {
    assert_eq!(
        Isq::LUMINOUS_FLUX.dims,
        Dims::J * Isq::SOLID_ANGULAR_MEASURE.dims
    );
    assert_eq!(
        Isq::ILLUMINANCE.dims,
        Isq::LUMINOUS_FLUX.dims / Isq::AREA.dims
    );
}

#[test]
fn test_catalytic_activity_dims() {
    assert_eq!(Isq::CATALYTIC_ACTIVITY.dims, Dims::N / Dims::T);
}

#[test]
fn test_activity_is_inverse_time() {
    assert_eq!(Isq::ACTIVITY.dims, Dims::T.pow(-1));
}

#[test]
fn test_absorbed_dose_and_dose_equivalent_dims() {
    assert_eq!(Isq::ABSORBED_DOSE.dims, Isq::ENERGY.dims / Dims::M);
    assert_eq!(
        Isq::DOSE_EQUIVALENT.dims,
        Isq::ABSORBED_DOSE.dims * Isq::IONIZING_RADIATION_QUALITY_FACTOR.dims
    );
}

#[test]
fn test_power_and_energy_density_dims() {
    assert_eq!(Isq::POWER.dims, Dims::M * Dims::L.pow(2) / Dims::T.pow(3));
    assert_eq!(
        Isq::ENERGY_DENSITY.dims,
        Dims::M / (Dims::L * Dims::T.pow(2))
    );
}

#[test]
fn test_find_same_kind_across_a_term_chain() {
    // radius is directly rooted in width
    assert_eq!(Isq::RADIUS.find_same_kind(&Isq::WIDTH), Some(&Isq::WIDTH));

    // path_length and width share the more distant `length` ancestor
    assert_eq!(Isq::PATH_LENGTH.find_same_kind(&Isq::WIDTH), Some(&Isq::L));
}

#[test]
fn test_find_same_kind_none_for_different_dims() {
    assert_eq!(Isq::ENERGY.find_same_kind(&Isq::POWER), None);
}
